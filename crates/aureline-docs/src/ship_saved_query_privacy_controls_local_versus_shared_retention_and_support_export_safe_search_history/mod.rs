//! Saved-query privacy controls, local-versus-shared retention, and
//! support-export-safe search history.
//!
//! This module implements the M5 saved-query privacy boundary: the records
//! that let a saved search query, a recent history entry, or a pinned query
//! carry an explicit *privacy control* (who may see it), an explicit
//! *local-versus-shared retention* posture (where it is stored and how widely
//! that storage exposes it), and an explicit *support-export safety* class
//! (whether the entry may travel in a support export, and how it is redacted)
//! — without ever letting a private query leak into a shared store, widen its
//! audience beyond what the user granted, or carry a raw query body into an
//! export.
//!
//! Each [`SavedQueryEntry`] carries one [`QueryPrivacyClass`] (the qualified
//! `private_local` / `private_synced` / `shared_team` / `shared_org` control;
//! `public_listing` and `unscoped_export` are recorded and block promotion), a
//! granted-vs-effective [`VisibilityGrant`] (*the no-hidden-visibility-expansion
//! guarantee*), a [`RetentionDisclosure`] (*what storage tier the query lives in
//! and whether a shared tier is disclosed* — the local-versus-shared retention
//! guarantee), a [`SupportExportSafety`] block (*whether the entry is
//! export-safe and how it is redacted* — the support-export-safe history
//! guarantee), one [`QueryTrustClass`] disclosure, the same
//! source/version/freshness/locality/confidence chip set the other docs lanes
//! use, a [`SharePosture`], the live-vs-captured state, citation state, and the
//! open-raw / open-source escapes.
//!
//! Three invariants make a saved query honest:
//!
//! - **Privacy control / no hidden visibility expansion.** A query's effective
//!   visibility may never exceed the visibility the user granted, and may never
//!   exceed the visibility its privacy class permits.
//! - **Local-versus-shared retention truth.** The storage tier may not expose a
//!   query more widely than its privacy class allows (a `private_local` query in
//!   a shared store is a leak), and a shared or synced tier must be disclosed.
//! - **Support-export safety.** A history entry marked export-safe must carry a
//!   redaction class that is actually safe to export; a raw query body never
//!   crosses the boundary.
//!
//! The [`SavedQueryHistoryExport`] is the projection support, AI evidence, and
//! diagnostics surfaces ingest: one [`SavedQueryExportRow`] per entry preserving
//! privacy class, retention posture, granted/effective visibility, trust class,
//! source class, confidence, redaction class, export-safe flag, retention
//! disclosure, citation state, and the open-raw / open-source escapes.
//!
//! [`SavedQueryPrivacyPacket::materialize`] computes the validation findings and
//! the promotion state (`stable`, `narrowed_below_stable`, or `blocks_stable`)
//! from the input, so a privacy-leaking, visibility-expanding, export-unsafe,
//! trust-collapsed, uncited, or unattributed entry set automatically narrows or
//! blocks before it reaches a consumer surface. The packet is an inspectable,
//! serde-serializable truth packet: it carries no raw query bodies, no raw URLs,
//! no raw history payloads, no raw source files, no raw provider payloads, and
//! no credentials — only metadata, privacy truth, retention truth, redaction
//! truth, visibility truth, chip truth, cited refs, provenance, finding
//! summaries, and contract refs.
//!
//! The boundary schema is
//! [`schemas/docs/ship-saved-query-privacy-controls-local-versus-shared-retention-and-support-export-safe-search-history.schema.json`](../../../../schemas/docs/ship-saved-query-privacy-controls-local-versus-shared-retention-and-support-export-safe-search-history.schema.json).
//! The contract doc is
//! [`docs/docs/m5/ship_saved_query_privacy_controls_local_versus_shared_retention_and_support_export_safe_search_history.md`](../../../../docs/docs/m5/ship_saved_query_privacy_controls_local_versus_shared_retention_and_support_export_safe_search_history.md).
//! The protected fixture directory is
//! [`fixtures/docs/m5/ship_saved_query_privacy_controls_local_versus_shared_retention_and_support_export_safe_search_history/`](../../../../fixtures/docs/m5/ship_saved_query_privacy_controls_local_versus_shared_retention_and_support_export_safe_search_history/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`SavedQueryPrivacyPacket`].
pub const SAVED_QUERY_PRIVACY_RECORD_KIND: &str = "saved_query_privacy_controls";

/// Record-kind tag carried by the support-export wrapper.
pub const SAVED_QUERY_PRIVACY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "saved_query_privacy_controls_support_export";

/// Schema version for saved-query-privacy records.
pub const SAVED_QUERY_PRIVACY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const SAVED_QUERY_PRIVACY_SCHEMA_REF: &str =
    "schemas/docs/ship-saved-query-privacy-controls-local-versus-shared-retention-and-support-export-safe-search-history.schema.json";

/// Repo-relative path of the saved-query-privacy contract doc.
pub const SAVED_QUERY_PRIVACY_DOC_REF: &str =
    "docs/docs/m5/ship_saved_query_privacy_controls_local_versus_shared_retention_and_support_export_safe_search_history.md";

/// Repo-relative path of the protected fixture directory.
pub const SAVED_QUERY_PRIVACY_FIXTURE_DIR: &str =
    "fixtures/docs/m5/ship_saved_query_privacy_controls_local_versus_shared_retention_and_support_export_safe_search_history";

/// Repo-relative path of the checked support-export artifact.
pub const SAVED_QUERY_PRIVACY_ARTIFACT_REF: &str =
    "artifacts/docs/m5/ship_saved_query_privacy_controls_local_versus_shared_retention_and_support_export_safe_search_history/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const SAVED_QUERY_PRIVACY_SUMMARY_REF: &str =
    "artifacts/docs/m5/ship_saved_query_privacy_controls_local_versus_shared_retention_and_support_export_safe_search_history.md";

/// The audience a saved query may be exposed to, ordered from narrowest to
/// widest.
///
/// The variants are declared in ascending order, so the derived [`Ord`] lets the
/// validator compare an effective visibility against a granted visibility, a
/// privacy-class ceiling, or a storage-tier exposure: a larger value is a wider
/// audience.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    /// Visible to the owner only.
    OwnerOnly,
    /// Visible across the owner's own devices.
    OwnerDevices,
    /// Visible to the owner's team.
    Team,
    /// Visible across the organization.
    Organization,
    /// Visible to everyone (public).
    Everyone,
}

impl Visibility {
    /// Stable token recorded in the surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OwnerOnly => "owner_only",
            Self::OwnerDevices => "owner_devices",
            Self::Team => "team",
            Self::Organization => "organization",
            Self::Everyone => "everyone",
        }
    }
}

/// The declared privacy control of a saved query.
///
/// Only `private_local`, `private_synced`, `shared_team`, and `shared_org` are
/// inside the qualified M5 scope. `public_listing` and `unscoped_export` are
/// recorded so the validator can detect and block a query that overruns the
/// qualified privacy controls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryPrivacyClass {
    /// Private to the owner, stored locally.
    PrivateLocal,
    /// Private to the owner, synced across their devices.
    PrivateSynced,
    /// Shared with the owner's team.
    SharedTeam,
    /// Shared across the organization.
    SharedOrg,
    /// Published as a public listing — outside the qualified M5 scope.
    PublicListing,
    /// Exported with no privacy scope — outside the qualified M5 scope.
    UnscopedExport,
}

impl QueryPrivacyClass {
    /// The privacy classes a packet must cover (`private_local` and
    /// `shared_team`).
    pub const REQUIRED: [Self; 2] = [Self::PrivateLocal, Self::SharedTeam];

    /// Stable token recorded in the entry.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrivateLocal => "private_local",
            Self::PrivateSynced => "private_synced",
            Self::SharedTeam => "shared_team",
            Self::SharedOrg => "shared_org",
            Self::PublicListing => "public_listing",
            Self::UnscopedExport => "unscoped_export",
        }
    }

    /// Whether this privacy class is inside the qualified saved-query scope.
    pub const fn is_within_qualified_scope(self) -> bool {
        matches!(
            self,
            Self::PrivateLocal | Self::PrivateSynced | Self::SharedTeam | Self::SharedOrg
        )
    }

    /// The widest audience a query of this privacy class may be exposed to. An
    /// effective visibility or storage exposure above this ceiling is a privacy
    /// leak.
    pub const fn max_visibility(self) -> Visibility {
        match self {
            Self::PrivateLocal => Visibility::OwnerOnly,
            Self::PrivateSynced => Visibility::OwnerDevices,
            Self::SharedTeam => Visibility::Team,
            Self::SharedOrg => Visibility::Organization,
            Self::PublicListing | Self::UnscopedExport => Visibility::Everyone,
        }
    }
}

/// Where a saved query / history entry is retained, projected for the
/// local-versus-shared retention disclosure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionPosture {
    /// Held only for the current session; not retained.
    EphemeralSession,
    /// Retained only on the local device.
    LocalOnly,
    /// Synced privately across the owner's devices.
    SyncedPrivate,
    /// Retained in a shared (team) store.
    SharedStore,
    /// Retained under an org-managed retention policy.
    ManagedRetention,
}

impl RetentionPosture {
    /// Stable token recorded in the disclosure.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EphemeralSession => "ephemeral_session",
            Self::LocalOnly => "local_only",
            Self::SyncedPrivate => "synced_private",
            Self::SharedStore => "shared_store",
            Self::ManagedRetention => "managed_retention",
        }
    }

    /// The widest audience this storage tier exposes a retained query to. A
    /// boundary above the query's privacy ceiling is a retention leak.
    pub const fn shared_boundary(self) -> Visibility {
        match self {
            Self::EphemeralSession | Self::LocalOnly => Visibility::OwnerOnly,
            Self::SyncedPrivate => Visibility::OwnerDevices,
            Self::SharedStore => Visibility::Team,
            Self::ManagedRetention => Visibility::Organization,
        }
    }

    /// Whether this storage tier requires an explicit retention disclosure. A
    /// synced or shared tier moves the query off the local device, so it must be
    /// disclosed (the local-versus-shared retention guarantee).
    pub const fn requires_disclosure(self) -> bool {
        matches!(
            self,
            Self::SyncedPrivate | Self::SharedStore | Self::ManagedRetention
        )
    }
}

/// Redaction class for a history entry under support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryRedactionClass {
    /// Only a human label is exported; the raw query is dropped.
    RedactedLabelOnly,
    /// Only an opaque digest is exported.
    DigestOnly,
    /// The raw query is withheld entirely.
    RawWithheld,
    /// The entry still needs redaction before it is export-safe.
    NeedsRedaction,
    /// The entry is not exportable at all.
    NotExportable,
}

impl QueryRedactionClass {
    /// Stable token recorded in the safety block.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RedactedLabelOnly => "redacted_label_only",
            Self::DigestOnly => "digest_only",
            Self::RawWithheld => "raw_withheld",
            Self::NeedsRedaction => "needs_redaction",
            Self::NotExportable => "not_exportable",
        }
    }

    /// Whether an entry of this redaction class may actually travel in a support
    /// export.
    pub const fn is_export_safe(self) -> bool {
        matches!(
            self,
            Self::RedactedLabelOnly | Self::DigestOnly | Self::RawWithheld
        )
    }
}

/// Kind of saved-query / history entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryEntryKind {
    /// A query the user explicitly saved.
    SavedQuery,
    /// A recent search-history entry.
    RecentHistory,
    /// A pinned query.
    PinnedQuery,
    /// A suggested query.
    SuggestedQuery,
}

impl QueryEntryKind {
    /// Stable token recorded in the entry.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SavedQuery => "saved_query",
            Self::RecentHistory => "recent_history",
            Self::PinnedQuery => "pinned_query",
            Self::SuggestedQuery => "suggested_query",
        }
    }
}

/// Trust class of the saved query's origin, projected as a disclosure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryTrustClass {
    /// A first-party query the user saved themselves.
    FirstPartyUserSaved,
    /// A pinned, signed shared-library query.
    SignedSharedLibrary,
    /// A query imported from a signed extension set.
    ExtensionImportedSet,
    /// A live-synced suggestion — not verified at materialization time.
    LiveSyncedSuggestion,
    /// A derived / inferred suggestion only.
    DerivedSuggestionOnly,
}

impl QueryTrustClass {
    /// Stable token recorded in the disclosure.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyUserSaved => "first_party_user_saved",
            Self::SignedSharedLibrary => "signed_shared_library",
            Self::ExtensionImportedSet => "extension_imported_set",
            Self::LiveSyncedSuggestion => "live_synced_suggestion",
            Self::DerivedSuggestionOnly => "derived_suggestion_only",
        }
    }

    /// Whether this trust class may back a high-confidence / authoritative claim.
    /// A live-synced suggestion or derived suggestion may not.
    pub const fn may_be_authoritative(self) -> bool {
        matches!(
            self,
            Self::FirstPartyUserSaved | Self::SignedSharedLibrary | Self::ExtensionImportedSet
        )
    }

    /// Whether an entry of this trust class must stay cited.
    pub const fn needs_citation(self) -> bool {
        !matches!(self, Self::FirstPartyUserSaved)
    }
}

/// Whether and how a saved query may be shared.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SharePosture {
    /// The query stays local and private; no share action is offered.
    LocalPrivateOnly,
    /// A share action is available and explicit.
    ShareAvailable,
    /// Sharing is blocked by policy.
    ShareBlockedByPolicy,
    /// Sharing is unavailable and disclosed as such.
    ShareUnavailableDisclosed,
}

impl SharePosture {
    /// Stable token recorded in the entry.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalPrivateOnly => "local_private_only",
            Self::ShareAvailable => "share_available",
            Self::ShareBlockedByPolicy => "share_blocked_by_policy",
            Self::ShareUnavailableDisclosed => "share_unavailable_disclosed",
        }
    }

    /// Whether the entry may present sharing as an available action.
    pub const fn is_available(self) -> bool {
        matches!(self, Self::ShareAvailable)
    }
}

/// Whether the entry is live, a captured snapshot, or a narrowed rerun.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapturedVsLive {
    /// A live entry.
    Live,
    /// A captured snapshot of an earlier view.
    CapturedSnapshot,
    /// A rerun narrowed to a smaller scope.
    NarrowedScopeRerun,
}

impl CapturedVsLive {
    /// Stable token recorded in the entry.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::CapturedSnapshot => "captured_snapshot",
            Self::NarrowedScopeRerun => "narrowed_scope_rerun",
        }
    }
}

/// Source class for an entry's underlying material, projected as the source chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuerySourceClass {
    /// A query the user saved locally.
    UserSavedQuery,
    /// A query from a shared team library.
    TeamSharedLibrary,
    /// A synced search-history entry.
    SyncedHistory,
    /// An imported / extension query set.
    ImportedQuerySet,
    /// A suggested query.
    SuggestedQuery,
    /// A derived / inferred suggestion.
    DerivedSuggestion,
}

impl QuerySourceClass {
    /// Stable token recorded in the chip.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserSavedQuery => "user_saved_query",
            Self::TeamSharedLibrary => "team_shared_library",
            Self::SyncedHistory => "synced_history",
            Self::ImportedQuerySet => "imported_query_set",
            Self::SuggestedQuery => "suggested_query",
            Self::DerivedSuggestion => "derived_suggestion",
        }
    }
}

/// Version-match state for an entry, projected as the version chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryVersionMatch {
    /// Entry matches the active build/workspace revision exactly.
    ExactBuildMatch,
    /// Entry is within an accepted compatible drift window.
    CompatibleMinorDrift,
    /// Entry drifted incompatibly from the active target.
    IncompatibleDriftDetected,
    /// Pre-release entry has not completed verification.
    PreReleaseUnverified,
    /// The target build/workspace revision could not be verified.
    UnknownTargetBuild,
}

impl QueryVersionMatch {
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

/// Freshness state for an entry, projected as the freshness chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryFreshness {
    /// Entry was live and authoritative at materialization time.
    AuthoritativeLive,
    /// Cached entry within its freshness window.
    WarmCached,
    /// Cached entry usable only with degraded disclosure.
    DegradedCached,
    /// Entry is stale and must not claim current authority.
    Stale,
    /// Freshness could not be verified.
    Unverified,
    /// A refresh is pending; the source has not yet re-synced.
    RefreshPending,
}

impl QueryFreshness {
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

/// Locality / install posture for an entry, projected as the locality chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryLocality {
    /// Resolved from local content or the in-repo index.
    Local,
    /// Resolved through a synced private store.
    SyncedPrivate,
    /// Resolved through a shared team store.
    SharedStore,
    /// Resolved through a managed (org-hosted) service.
    Managed,
}

impl QueryLocality {
    /// Stable token recorded in the chip.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::SyncedPrivate => "synced_private",
            Self::SharedStore => "shared_store",
            Self::Managed => "managed",
        }
    }
}

/// Confidence label for an entry, projected as the confidence chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryConfidence {
    /// High confidence.
    High,
    /// Medium confidence.
    Medium,
    /// Low confidence.
    Low,
    /// Heuristic only; not a verified claim.
    Heuristic,
}

impl QueryConfidence {
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
pub enum SavedQueryFindingSeverity {
    /// Blocks a Stable claim; the set must block.
    Blocking,
    /// Narrows below Stable but the set stays valid and attributable.
    Narrowing,
    /// Advisory only.
    Advisory,
}

impl SavedQueryFindingSeverity {
    /// Stable token recorded in the finding.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Blocking => "blocking",
            Self::Narrowing => "narrowing",
            Self::Advisory => "advisory",
        }
    }
}

/// Consumer surface that must project the saved-query-privacy packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedQueryConsumerSurface {
    /// The search surface.
    SearchSurface,
    /// The docs browser shell.
    DocsBrowserShell,
    /// The saved-query library.
    SavedQueryLibrary,
    /// The recent-history panel.
    HistoryPanel,
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

impl SavedQueryConsumerSurface {
    /// Stable token recorded in the projection.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchSurface => "search_surface",
            Self::DocsBrowserShell => "docs_browser_shell",
            Self::SavedQueryLibrary => "saved_query_library",
            Self::HistoryPanel => "history_panel",
            Self::AiContextInspector => "ai_context_inspector",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
        }
    }
}

/// Class of a packet-level saved-query degradation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedQueryDegradationClass {
    /// Sync is offline; the entry is served from the last snapshot.
    SyncOfflineSnapshot,
    /// A shared store was unreachable; a captured snapshot is shown instead.
    SharedStoreUnreachableCapturedSnapshot,
    /// Sharing was blocked by policy.
    ShareBlockedByPolicy,
    /// The retention tier is degraded (still present, but reduced).
    RetentionTierDegraded,
    /// The entry was rerun at a narrowed scope.
    ScopeNarrowedRerun,
    /// The entry's privacy was narrowed before publication.
    PrivacyNarrowed,
    /// The entry's history was redacted more aggressively before export.
    RedactionUpgraded,
    /// A referenced anchor is broken.
    BrokenAnchor,
    /// The owning source is quarantined.
    QuarantinedSource,
}

impl SavedQueryDegradationClass {
    /// Stable token recorded in the degradation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SyncOfflineSnapshot => "sync_offline_snapshot",
            Self::SharedStoreUnreachableCapturedSnapshot => {
                "shared_store_unreachable_captured_snapshot"
            }
            Self::ShareBlockedByPolicy => "share_blocked_by_policy",
            Self::RetentionTierDegraded => "retention_tier_degraded",
            Self::ScopeNarrowedRerun => "scope_narrowed_rerun",
            Self::PrivacyNarrowed => "privacy_narrowed",
            Self::RedactionUpgraded => "redaction_upgraded",
            Self::BrokenAnchor => "broken_anchor",
            Self::QuarantinedSource => "quarantined_source",
        }
    }
}

/// Scope a saved-query export covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedQueryExportScope {
    /// Every entry in the packet.
    AllEntries,
    /// Saved queries only.
    SavedQueriesOnly,
    /// Recent history only.
    HistoryOnly,
    /// Shared entries only.
    SharedOnly,
}

impl SavedQueryExportScope {
    /// Stable token recorded in the export.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllEntries => "all_entries",
            Self::SavedQueriesOnly => "saved_queries_only",
            Self::HistoryOnly => "history_only",
            Self::SharedOnly => "shared_only",
        }
    }
}

/// Promotion state computed for the saved-query-privacy packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedQueryPromotionState {
    /// Set qualifies for the Stable claim.
    Stable,
    /// Set narrowed below Stable but stays valid and attributable.
    NarrowedBelowStable,
    /// Set has a blocking finding and must not present as Stable.
    BlocksStable,
}

impl SavedQueryPromotionState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Validation finding kind emitted by [`SavedQueryPrivacyPacket::materialize`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedQueryFindingKind {
    /// A required identity field is missing.
    MissingIdentity,
    /// The entry set is empty.
    EntriesEmpty,
    /// An entry id is duplicated.
    DuplicateEntryId,
    /// A required privacy class (private-local / shared-team) is missing.
    RequiredPrivacyClassMissing,
    /// An entry declares a privacy class outside the qualified scope.
    PrivacyClassOutOfBounds,
    /// An entry is missing its title or query label.
    EntryTitleOrLabelMissing,
    /// A synced or shared entry is missing its retention disclosure.
    RetentionDisclosureMissing,
    /// An entry is missing its trust-class disclosure note.
    TrustClassDisclosureMissing,
    /// An untrusted entry is presented as a high-confidence claim.
    TrustClassDisclosureCollapsed,
    /// A share blocked by policy is presented as available.
    BlockedSharePresentedAvailable,
    /// An entry is missing an open-raw / open-source escape ref.
    OpenRawOpenSourceEscapeMissing,
    /// An imported / synced / derived entry is not cited.
    EntryNotCited,
    /// A non-current version-match is presented as a confident live match.
    VersionTruthCollapsed,
    /// The effective visibility exceeds the granted visibility (hidden share).
    VisibilityExpansionDetected,
    /// The effective visibility exceeds what the privacy class permits.
    PrivacyVisibilityMismatch,
    /// The storage tier exposes the query more widely than the privacy allows.
    RetentionPrivacyMismatch,
    /// An entry marked export-safe carries an unsafe redaction class.
    SupportExportUnsafe,
    /// An export row references an entry id absent from the entries.
    ExportRowOrphan,
    /// An entry has no matching export row.
    ExportCoverageMissing,
    /// The export drops a required preservation flag.
    ExportDropsPreservation,
    /// An export row's privacy class disagrees with the entry.
    ExportPrivacyClassMismatch,
    /// An export row's retention posture disagrees with the entry.
    ExportRetentionMismatch,
    /// An export row's granted/effective visibility disagrees with the entry.
    ExportVisibilityMismatch,
    /// An export row's trust class disagrees with the entry.
    ExportTrustClassMismatch,
    /// An export row's source class disagrees with the entry's chip.
    ExportSourceClassMismatch,
    /// An export row's confidence disagrees with the entry's chip.
    ExportConfidenceMismatch,
    /// An export row's redaction class disagrees with the entry.
    ExportRedactionMismatch,
    /// An export row's export-safe flag disagrees with the entry.
    ExportExportSafeMismatch,
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
    /// Raw bodies, raw URLs, raw history payloads, or secrets crossed the boundary.
    RawBoundaryMaterialPresent,
}

impl SavedQueryFindingKind {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingIdentity => "missing_identity",
            Self::EntriesEmpty => "entries_empty",
            Self::DuplicateEntryId => "duplicate_entry_id",
            Self::RequiredPrivacyClassMissing => "required_privacy_class_missing",
            Self::PrivacyClassOutOfBounds => "privacy_class_out_of_bounds",
            Self::EntryTitleOrLabelMissing => "entry_title_or_label_missing",
            Self::RetentionDisclosureMissing => "retention_disclosure_missing",
            Self::TrustClassDisclosureMissing => "trust_class_disclosure_missing",
            Self::TrustClassDisclosureCollapsed => "trust_class_disclosure_collapsed",
            Self::BlockedSharePresentedAvailable => "blocked_share_presented_available",
            Self::OpenRawOpenSourceEscapeMissing => "open_raw_open_source_escape_missing",
            Self::EntryNotCited => "entry_not_cited",
            Self::VersionTruthCollapsed => "version_truth_collapsed",
            Self::VisibilityExpansionDetected => "visibility_expansion_detected",
            Self::PrivacyVisibilityMismatch => "privacy_visibility_mismatch",
            Self::RetentionPrivacyMismatch => "retention_privacy_mismatch",
            Self::SupportExportUnsafe => "support_export_unsafe",
            Self::ExportRowOrphan => "export_row_orphan",
            Self::ExportCoverageMissing => "export_coverage_missing",
            Self::ExportDropsPreservation => "export_drops_preservation",
            Self::ExportPrivacyClassMismatch => "export_privacy_class_mismatch",
            Self::ExportRetentionMismatch => "export_retention_mismatch",
            Self::ExportVisibilityMismatch => "export_visibility_mismatch",
            Self::ExportTrustClassMismatch => "export_trust_class_mismatch",
            Self::ExportSourceClassMismatch => "export_source_class_mismatch",
            Self::ExportConfidenceMismatch => "export_confidence_mismatch",
            Self::ExportRedactionMismatch => "export_redaction_mismatch",
            Self::ExportExportSafeMismatch => "export_export_safe_mismatch",
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
    pub const fn default_severity(self) -> SavedQueryFindingSeverity {
        SavedQueryFindingSeverity::Blocking
    }
}

/// The chip set rendered for one saved-query entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueryChipSet {
    /// Source-class chip.
    pub source_class: QuerySourceClass,
    /// Version-match chip.
    pub version_match: QueryVersionMatch,
    /// Freshness chip.
    pub freshness: QueryFreshness,
    /// Locality chip.
    pub locality: QueryLocality,
    /// Confidence chip (the confidence label).
    pub confidence: QueryConfidence,
}

/// The granted vs. effective visibility of a saved query (no hidden share).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisibilityGrant {
    /// The visibility the user/policy granted.
    pub granted: Visibility,
    /// The visibility the query actually exposes.
    pub effective: Visibility,
}

impl VisibilityGrant {
    /// Whether the effective visibility exceeds the granted visibility.
    pub fn is_expansion(&self) -> bool {
        self.effective > self.granted
    }
}

/// The storage tier a saved query is retained in (local-versus-shared retention).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetentionDisclosure {
    /// The retention posture.
    pub posture: RetentionPosture,
    /// Whether a synced or shared tier is disclosed to the reader.
    pub disclosed: bool,
    /// Human-readable disclosure note (no raw bodies).
    pub note: String,
}

/// The support-export safety block for a history entry (export-safe history).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportSafety {
    /// The redaction class applied for export.
    pub redaction_class: QueryRedactionClass,
    /// Whether the entry is marked export-safe.
    pub export_safe: bool,
    /// Human-readable safety note (no raw bodies).
    pub note: String,
}

/// One saved-query / history entry — one bounded, privacy-scoped record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedQueryEntry {
    /// Stable entry id within this packet.
    pub entry_id: String,
    /// The kind of entry.
    pub entry_kind: QueryEntryKind,
    /// The declared privacy control for this entry.
    pub privacy_class: QueryPrivacyClass,
    /// Subject ref the entry points at (no raw query / no raw body).
    pub subject_ref: String,
    /// Human-readable title.
    pub title: String,
    /// Human-readable query label / summary (no raw query bodies).
    pub query_label: String,
    /// Source/version/freshness/locality/confidence chips.
    pub chips: QueryChipSet,
    /// The trust-class disclosure for the origin.
    pub trust_class: QueryTrustClass,
    /// Human-readable trust-class disclosure note.
    pub trust_disclosure_note: String,
    /// The granted vs. effective visibility (no hidden visibility expansion).
    pub visibility: VisibilityGrant,
    /// The retention disclosure (local-versus-shared retention).
    pub retention: RetentionDisclosure,
    /// The support-export safety block (export-safe history).
    pub export_safety: SupportExportSafety,
    /// Whether and how the query may be shared.
    pub share_posture: SharePosture,
    /// Whether the entry is live, captured, or a narrowed rerun.
    pub captured_vs_live: CapturedVsLive,
    /// Whether the entry is cited back to its source.
    pub cited: bool,
    /// Citation ref when cited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub citation_ref: Option<String>,
    /// Open-raw escape ref (open the underlying node/subject).
    pub open_raw_escape_ref: String,
    /// Open-source escape ref (open the upstream/source).
    pub open_source_escape_ref: String,
}

/// One export row, mirroring an entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedQueryExportRow {
    /// The entry this export row mirrors.
    pub entry_id_ref: String,
    /// Entry kind (must match the entry).
    pub entry_kind: QueryEntryKind,
    /// Privacy class (must match the entry).
    pub privacy_class: QueryPrivacyClass,
    /// Retention posture (must match the entry).
    pub retention_posture: RetentionPosture,
    /// Granted visibility (must match the entry).
    pub granted_visibility: Visibility,
    /// Effective visibility (must match the entry).
    pub effective_visibility: Visibility,
    /// Trust class (must match the entry).
    pub trust_class: QueryTrustClass,
    /// Source class (must match the entry's chip).
    pub source_class: QuerySourceClass,
    /// Confidence (must match the entry's chip).
    pub confidence: QueryConfidence,
    /// Redaction class (must match the entry).
    pub redaction_class: QueryRedactionClass,
    /// Export-safe flag (must match the entry).
    pub export_safe: bool,
    /// Whether the entry's retention tier is disclosed (must match the entry).
    pub retention_disclosed: bool,
    /// Whether the entry is cited.
    pub cited: bool,
    /// Open-raw escape ref.
    pub open_raw_escape_ref: String,
    /// Open-source escape ref.
    pub open_source_escape_ref: String,
}

/// The saved-query history export projection for the entry set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedQueryHistoryExport {
    /// Scope this export covers.
    pub scope: SavedQueryExportScope,
    /// Whether the export preserves each entry's privacy class.
    pub preserves_privacy_class: bool,
    /// Whether the export preserves each entry's retention posture.
    pub preserves_retention: bool,
    /// Whether the export preserves each entry's visibility truth.
    pub preserves_visibility: bool,
    /// Whether the export preserves each entry's trust class.
    pub preserves_trust_class: bool,
    /// Whether the export preserves each entry's source class.
    pub preserves_source_class: bool,
    /// Whether the export preserves each entry's confidence label.
    pub preserves_confidence: bool,
    /// Whether the export preserves each entry's redaction class.
    pub preserves_redaction: bool,
    /// Whether the export preserves each entry's export-safe flag.
    pub preserves_export_safe: bool,
    /// Whether the export preserves the open-raw / open-source escapes.
    pub preserves_open_raw_open_source_escape: bool,
    /// Per-entry export rows.
    pub rows: Vec<SavedQueryExportRow>,
}

impl SavedQueryHistoryExport {
    /// Whether the export preserves every required field.
    pub const fn preserves_all(&self) -> bool {
        self.preserves_privacy_class
            && self.preserves_retention
            && self.preserves_visibility
            && self.preserves_trust_class
            && self.preserves_source_class
            && self.preserves_confidence
            && self.preserves_redaction
            && self.preserves_export_safe
            && self.preserves_open_raw_open_source_escape
    }
}

/// A packet-level saved-query degradation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedQueryDegradation {
    /// Degradation class.
    pub degradation_class: SavedQueryDegradationClass,
    /// Severity.
    pub severity: SavedQueryFindingSeverity,
    /// Human-readable summary (no raw bodies).
    pub summary: String,
    /// The entry this degradation annotates, if scoped to one entry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry_id_ref: Option<String>,
    /// Optional supporting evidence ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<String>,
}

/// How a consumer surface projects the saved-query set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedQueryConsumerProjection {
    /// Surface that consumes the set.
    pub surface: SavedQueryConsumerSurface,
    /// Packet id this projection mirrors.
    pub packet_id_ref: String,
    /// Whether the surface preserves the chip set verbatim.
    pub preserves_chips: bool,
    /// Whether the surface preserves all privacy classes.
    pub preserves_privacy_classes: bool,
    /// Whether the surface preserves the retention postures.
    pub preserves_retention: bool,
    /// Whether the surface preserves the visibility truth.
    pub preserves_visibility: bool,
    /// Whether the surface preserves the trust classes.
    pub preserves_trust_classes: bool,
    /// Whether the surface preserves the redaction classes.
    pub preserves_redaction: bool,
    /// Whether the surface preserves the export-safe flags.
    pub preserves_export_safe: bool,
    /// Whether the surface preserves the open-raw / open-source escapes.
    pub preserves_open_raw_open_source_escape: bool,
}

impl SavedQueryConsumerProjection {
    /// Whether the projection preserves every required field.
    pub const fn preserves_all(&self) -> bool {
        self.preserves_chips
            && self.preserves_privacy_classes
            && self.preserves_retention
            && self.preserves_visibility
            && self.preserves_trust_classes
            && self.preserves_redaction
            && self.preserves_export_safe
            && self.preserves_open_raw_open_source_escape
    }
}

/// A single validation finding on the saved-query set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedQueryValidationFinding {
    /// Finding kind.
    pub finding_kind: SavedQueryFindingKind,
    /// Finding severity.
    pub severity: SavedQueryFindingSeverity,
    /// Human-readable summary.
    pub summary: String,
}

/// Constructor input for [`SavedQueryPrivacyPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedQueryPrivacyPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable session label (no raw URLs / no raw query text).
    pub session_label: String,
    /// Opaque digest/ref for the session.
    pub session_digest_ref: String,
    /// The saved-query / history entries.
    pub entries: Vec<SavedQueryEntry>,
    /// The export projection.
    pub export: SavedQueryHistoryExport,
    /// Packet-level degradations.
    pub query_degradations: Vec<SavedQueryDegradation>,
    /// Consumer projections.
    pub consumer_projections: Vec<SavedQueryConsumerProjection>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp (RFC 3339).
    pub minted_at: String,
}

/// Export-safe saved-query-privacy packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedQueryPrivacyPacket {
    /// Record kind; must equal [`SAVED_QUERY_PRIVACY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`SAVED_QUERY_PRIVACY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable session label.
    pub session_label: String,
    /// Opaque digest/ref for the session.
    pub session_digest_ref: String,
    /// The saved-query / history entries.
    pub entries: Vec<SavedQueryEntry>,
    /// The export projection.
    pub export: SavedQueryHistoryExport,
    /// Packet-level degradations.
    pub query_degradations: Vec<SavedQueryDegradation>,
    /// Consumer projections.
    pub consumer_projections: Vec<SavedQueryConsumerProjection>,
    /// Computed promotion state.
    pub promotion_state: SavedQueryPromotionState,
    /// Computed validation findings.
    pub validation_findings: Vec<SavedQueryValidationFinding>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Required consumer surfaces that every saved-query packet must project.
const REQUIRED_SURFACES: [SavedQueryConsumerSurface; 4] = [
    SavedQueryConsumerSurface::SearchSurface,
    SavedQueryConsumerSurface::SavedQueryLibrary,
    SavedQueryConsumerSurface::HistoryPanel,
    SavedQueryConsumerSurface::SupportExport,
];

impl SavedQueryPrivacyPacket {
    /// Materializes a saved-query-privacy packet, computing validation findings
    /// and the promotion state from the input.
    pub fn materialize(input: SavedQueryPrivacyPacketInput) -> Self {
        let mut findings = Vec::new();

        check_identity(&input, &mut findings);
        check_entries(&input, &mut findings);
        check_export(&input, &mut findings);
        check_degradations(&input, &mut findings);
        check_consumer_projections(&input, &mut findings);
        check_boundary(&input, &mut findings);

        let promotion_state = promotion_state(&findings, &input.query_degradations);

        Self {
            record_kind: SAVED_QUERY_PRIVACY_RECORD_KIND.to_owned(),
            schema_version: SAVED_QUERY_PRIVACY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            session_label: input.session_label,
            session_digest_ref: input.session_digest_ref,
            entries: input.entries,
            export: input.export,
            query_degradations: input.query_degradations,
            consumer_projections: input.consumer_projections,
            promotion_state,
            validation_findings: findings,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Whether the set qualifies for the Stable claim with no findings.
    pub fn is_clean_stable(&self) -> bool {
        self.promotion_state == SavedQueryPromotionState::Stable
            && self.validation_findings.is_empty()
    }

    /// Wraps the packet in a support-export envelope.
    pub fn support_export(
        &self,
        export_id: &str,
        exported_at: &str,
    ) -> SavedQueryPrivacySupportExport {
        SavedQueryPrivacySupportExport {
            record_kind: SAVED_QUERY_PRIVACY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: SAVED_QUERY_PRIVACY_SCHEMA_VERSION,
            export_id: export_id.to_owned(),
            exported_at: exported_at.to_owned(),
            schema_ref: SAVED_QUERY_PRIVACY_SCHEMA_REF.to_owned(),
            doc_ref: SAVED_QUERY_PRIVACY_DOC_REF.to_owned(),
            packet: self.clone(),
        }
    }

    /// Deterministic export-safe pretty JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("saved-query-privacy packet serializes")
    }

    /// Deterministic Markdown summary for docs, support, or library handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# Saved-Query Privacy Controls (local-versus-shared retention, export-safe history)\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Session: {}\n", self.session_label));
        out.push_str(&format!(
            "- Promotion: `{}` ({} findings)\n",
            self.promotion_state.as_str(),
            self.validation_findings.len()
        ));
        out.push_str(&format!(
            "- Entries: {} | Degradations: {}\n",
            self.entries.len(),
            self.query_degradations.len()
        ));
        out.push_str("\n## Entries\n\n");
        for entry in &self.entries {
            out.push_str(&format!(
                "- [{}] `{}` ({}) — trust `{}` — {} / {} / {} / {} / {}\n",
                entry.privacy_class.as_str(),
                entry.entry_id,
                entry.title,
                entry.trust_class.as_str(),
                entry.chips.source_class.as_str(),
                entry.chips.version_match.as_str(),
                entry.chips.freshness.as_str(),
                entry.chips.locality.as_str(),
                entry.chips.confidence.as_str(),
            ));
            out.push_str(&format!(
                "  - Visibility: granted `{}` / effective `{}`\n",
                entry.visibility.granted.as_str(),
                entry.visibility.effective.as_str(),
            ));
            out.push_str(&format!(
                "  - Retention: [{}] disclosed {}\n",
                entry.retention.posture.as_str(),
                entry.retention.disclosed,
            ));
            out.push_str(&format!(
                "  - Export safety: [{}] export_safe {}\n",
                entry.export_safety.redaction_class.as_str(),
                entry.export_safety.export_safe,
            ));
            out.push_str(&format!(
                "  - Share: {} | Captured/live: {} | Cited: {}\n",
                entry.share_posture.as_str(),
                entry.captured_vs_live.as_str(),
                entry.cited,
            ));
        }
        if !self.query_degradations.is_empty() {
            out.push_str("\n## Degradations\n\n");
            for degradation in &self.query_degradations {
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

/// Support-export envelope for the saved-query-privacy packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedQueryPrivacySupportExport {
    /// Record kind; must equal [`SAVED_QUERY_PRIVACY_SUPPORT_EXPORT_RECORD_KIND`].
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
    /// The wrapped saved-query-privacy packet.
    pub packet: SavedQueryPrivacyPacket,
}

/// Errors emitted when reading the checked-in saved-query-privacy support export.
#[derive(Debug)]
pub enum SavedQueryPrivacyArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Re-materialization disagreed with the checked-in promotion state.
    PromotionDrift {
        /// Promotion state recorded in the export.
        recorded: SavedQueryPromotionState,
        /// Promotion state computed by re-materialization.
        computed: SavedQueryPromotionState,
    },
    /// The checked-in packet should be clean Stable but is not.
    NotCleanStable(Vec<SavedQueryValidationFinding>),
}

impl fmt::Display for SavedQueryPrivacyArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "saved-query-privacy export parse failed: {error}"
                )
            }
            Self::PromotionDrift { recorded, computed } => write!(
                formatter,
                "saved-query-privacy promotion drift: recorded {} but computed {}",
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
                    "saved-query-privacy export is not clean stable: {tokens}"
                )
            }
        }
    }
}

impl Error for SavedQueryPrivacyArtifactError {}

/// Reads and re-validates the checked-in stable saved-query-privacy support export.
pub fn current_stable_saved_query_privacy_export(
) -> Result<SavedQueryPrivacySupportExport, SavedQueryPrivacyArtifactError> {
    let export: SavedQueryPrivacySupportExport = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/ship_saved_query_privacy_controls_local_versus_shared_retention_and_support_export_safe_search_history/support_export.json"
    )))
    .map_err(SavedQueryPrivacyArtifactError::SupportExport)?;

    let recomputed = SavedQueryPrivacyPacket::materialize(packet_to_input(&export.packet));
    if recomputed.promotion_state != export.packet.promotion_state {
        return Err(SavedQueryPrivacyArtifactError::PromotionDrift {
            recorded: export.packet.promotion_state,
            computed: recomputed.promotion_state,
        });
    }
    if !export.packet.is_clean_stable() {
        return Err(SavedQueryPrivacyArtifactError::NotCleanStable(
            export.packet.validation_findings.clone(),
        ));
    }
    Ok(export)
}

/// Rebuilds the materialization input from a packet (used for re-validation).
pub fn packet_to_input(packet: &SavedQueryPrivacyPacket) -> SavedQueryPrivacyPacketInput {
    SavedQueryPrivacyPacketInput {
        packet_id: packet.packet_id.clone(),
        session_label: packet.session_label.clone(),
        session_digest_ref: packet.session_digest_ref.clone(),
        entries: packet.entries.clone(),
        export: packet.export.clone(),
        query_degradations: packet.query_degradations.clone(),
        consumer_projections: packet.consumer_projections.clone(),
        redaction_class_token: packet.redaction_class_token.clone(),
        minted_at: packet.minted_at.clone(),
    }
}

fn push_finding(
    findings: &mut Vec<SavedQueryValidationFinding>,
    kind: SavedQueryFindingKind,
    summary: impl Into<String>,
) {
    findings.push(SavedQueryValidationFinding {
        finding_kind: kind,
        severity: kind.default_severity(),
        summary: summary.into(),
    });
}

fn check_identity(
    input: &SavedQueryPrivacyPacketInput,
    findings: &mut Vec<SavedQueryValidationFinding>,
) {
    if input.packet_id.trim().is_empty()
        || input.session_label.trim().is_empty()
        || input.session_digest_ref.trim().is_empty()
        || input.redaction_class_token.trim().is_empty()
        || input.minted_at.trim().is_empty()
    {
        push_finding(
            findings,
            SavedQueryFindingKind::MissingIdentity,
            "packet identity fields must all be present",
        );
    }
}

fn check_entries(
    input: &SavedQueryPrivacyPacketInput,
    findings: &mut Vec<SavedQueryValidationFinding>,
) {
    if input.entries.is_empty() {
        push_finding(
            findings,
            SavedQueryFindingKind::EntriesEmpty,
            "the saved-query set must carry at least one entry",
        );
        return;
    }

    let present_classes: BTreeSet<QueryPrivacyClass> = input
        .entries
        .iter()
        .map(|entry| entry.privacy_class)
        .collect();
    for required in QueryPrivacyClass::REQUIRED {
        if !present_classes.contains(&required) {
            push_finding(
                findings,
                SavedQueryFindingKind::RequiredPrivacyClassMissing,
                format!("required privacy class `{}` is missing", required.as_str()),
            );
        }
    }

    let mut seen_entry_ids: BTreeSet<&str> = BTreeSet::new();
    for entry in &input.entries {
        if !seen_entry_ids.insert(entry.entry_id.as_str()) {
            push_finding(
                findings,
                SavedQueryFindingKind::DuplicateEntryId,
                format!("duplicate entry id `{}`", entry.entry_id),
            );
        }
        check_one_entry(entry, findings);
    }
}

fn check_one_entry(entry: &SavedQueryEntry, findings: &mut Vec<SavedQueryValidationFinding>) {
    if !entry.privacy_class.is_within_qualified_scope() {
        push_finding(
            findings,
            SavedQueryFindingKind::PrivacyClassOutOfBounds,
            format!(
                "entry `{}` declares out-of-scope `{}`; only private-local/private-synced/shared-team/shared-org are qualified",
                entry.entry_id,
                entry.privacy_class.as_str()
            ),
        );
    }
    if entry.title.trim().is_empty() || entry.query_label.trim().is_empty() {
        push_finding(
            findings,
            SavedQueryFindingKind::EntryTitleOrLabelMissing,
            format!(
                "entry `{}` is missing a title or query label",
                entry.entry_id
            ),
        );
    }
    if entry.trust_disclosure_note.trim().is_empty() {
        push_finding(
            findings,
            SavedQueryFindingKind::TrustClassDisclosureMissing,
            format!(
                "entry `{}` is missing its trust-class disclosure",
                entry.entry_id
            ),
        );
    }
    if entry.open_raw_escape_ref.trim().is_empty() || entry.open_source_escape_ref.trim().is_empty()
    {
        push_finding(
            findings,
            SavedQueryFindingKind::OpenRawOpenSourceEscapeMissing,
            format!(
                "entry `{}` must keep open-raw and open-source escapes",
                entry.entry_id
            ),
        );
    }

    // An untrusted origin must stay cited.
    if entry.trust_class.needs_citation() && !entry.cited {
        push_finding(
            findings,
            SavedQueryFindingKind::EntryNotCited,
            format!(
                "entry `{}` is `{}` but is not cited",
                entry.entry_id,
                entry.trust_class.as_str()
            ),
        );
    }
    // An untrusted origin may never be presented at high confidence.
    if !entry.trust_class.may_be_authoritative() && entry.chips.confidence == QueryConfidence::High
    {
        push_finding(
            findings,
            SavedQueryFindingKind::TrustClassDisclosureCollapsed,
            format!(
                "entry `{}` is `{}` but presented as high confidence",
                entry.entry_id,
                entry.trust_class.as_str()
            ),
        );
    }
    // A share blocked by policy may not present as a live shareable action.
    if entry.share_posture == SharePosture::ShareBlockedByPolicy
        && entry.captured_vs_live == CapturedVsLive::Live
        && entry.visibility.effective > Visibility::OwnerDevices
    {
        push_finding(
            findings,
            SavedQueryFindingKind::BlockedSharePresentedAvailable,
            format!(
                "entry `{}` is share-blocked by policy but presented as a live shared entry",
                entry.entry_id
            ),
        );
    }
    // A non-current version may not be presented as a confident live match.
    if !entry.chips.version_match.is_confident_current()
        && entry.chips.confidence == QueryConfidence::High
        && entry.chips.freshness.is_authoritative_live()
    {
        push_finding(
            findings,
            SavedQueryFindingKind::VersionTruthCollapsed,
            format!(
                "entry `{}` presents version `{}` as a confident live match",
                entry.entry_id,
                entry.chips.version_match.as_str()
            ),
        );
    }

    // Privacy control: effective visibility may not exceed the granted visibility.
    if entry.visibility.is_expansion() {
        push_finding(
            findings,
            SavedQueryFindingKind::VisibilityExpansionDetected,
            format!(
                "entry `{}` exposes `{}` visibility but was granted only `{}`",
                entry.entry_id,
                entry.visibility.effective.as_str(),
                entry.visibility.granted.as_str()
            ),
        );
    }
    // Privacy control: effective visibility may not exceed the privacy ceiling.
    if entry.visibility.effective > entry.privacy_class.max_visibility() {
        push_finding(
            findings,
            SavedQueryFindingKind::PrivacyVisibilityMismatch,
            format!(
                "entry `{}` exposes `{}` visibility beyond what privacy `{}` permits (`{}`)",
                entry.entry_id,
                entry.visibility.effective.as_str(),
                entry.privacy_class.as_str(),
                entry.privacy_class.max_visibility().as_str()
            ),
        );
    }

    // Local-versus-shared retention truth: the storage tier may not expose the
    // query more widely than the privacy class allows.
    if entry.retention.posture.shared_boundary() > entry.privacy_class.max_visibility() {
        push_finding(
            findings,
            SavedQueryFindingKind::RetentionPrivacyMismatch,
            format!(
                "entry `{}` is `{}` but retained in `{}`, which exposes it to `{}` beyond the privacy ceiling `{}`",
                entry.entry_id,
                entry.privacy_class.as_str(),
                entry.retention.posture.as_str(),
                entry.retention.posture.shared_boundary().as_str(),
                entry.privacy_class.max_visibility().as_str()
            ),
        );
    }
    // Local-versus-shared retention truth: a synced or shared tier must be disclosed.
    if entry.retention.posture.requires_disclosure()
        && (!entry.retention.disclosed || entry.retention.note.trim().is_empty())
    {
        push_finding(
            findings,
            SavedQueryFindingKind::RetentionDisclosureMissing,
            format!(
                "entry `{}` is retained in `{}` but does not disclose the shared/synced tier",
                entry.entry_id,
                entry.retention.posture.as_str()
            ),
        );
    }

    // Support-export safety: an export-safe entry must carry a safe redaction class.
    if entry.export_safety.export_safe && !entry.export_safety.redaction_class.is_export_safe() {
        push_finding(
            findings,
            SavedQueryFindingKind::SupportExportUnsafe,
            format!(
                "entry `{}` is marked export-safe but its redaction class `{}` is not export-safe",
                entry.entry_id,
                entry.export_safety.redaction_class.as_str()
            ),
        );
    }
    if entry.export_safety.note.trim().is_empty() {
        push_finding(
            findings,
            SavedQueryFindingKind::SupportExportUnsafe,
            format!(
                "entry `{}` is missing its support-export safety note",
                entry.entry_id
            ),
        );
    }
}

fn check_export(
    input: &SavedQueryPrivacyPacketInput,
    findings: &mut Vec<SavedQueryValidationFinding>,
) {
    let export = &input.export;
    if !export.preserves_all() {
        push_finding(
            findings,
            SavedQueryFindingKind::ExportDropsPreservation,
            "the export must preserve privacy class, retention, visibility, trust class, source class, confidence, redaction, export-safe, and escapes",
        );
    }

    let mut export_ids: BTreeSet<&str> = BTreeSet::new();
    for row in &export.rows {
        export_ids.insert(row.entry_id_ref.as_str());
        let entry = input
            .entries
            .iter()
            .find(|entry| entry.entry_id == row.entry_id_ref);
        match entry {
            None => push_finding(
                findings,
                SavedQueryFindingKind::ExportRowOrphan,
                format!("export row references unknown entry `{}`", row.entry_id_ref),
            ),
            Some(entry) => check_export_row(entry, row, findings),
        }
    }

    for entry in &input.entries {
        if !export_ids.contains(entry.entry_id.as_str()) {
            push_finding(
                findings,
                SavedQueryFindingKind::ExportCoverageMissing,
                format!("entry `{}` has no export row", entry.entry_id),
            );
        }
    }
}

fn check_export_row(
    entry: &SavedQueryEntry,
    row: &SavedQueryExportRow,
    findings: &mut Vec<SavedQueryValidationFinding>,
) {
    if entry.privacy_class != row.privacy_class {
        push_finding(
            findings,
            SavedQueryFindingKind::ExportPrivacyClassMismatch,
            format!(
                "export for `{}` records privacy `{}` but the entry is `{}`",
                row.entry_id_ref,
                row.privacy_class.as_str(),
                entry.privacy_class.as_str()
            ),
        );
    }
    if entry.retention.posture != row.retention_posture
        || entry.retention.disclosed != row.retention_disclosed
    {
        push_finding(
            findings,
            SavedQueryFindingKind::ExportRetentionMismatch,
            format!(
                "export for `{}` records retention `{}`/disclosed `{}` but the entry is `{}`/disclosed `{}`",
                row.entry_id_ref,
                row.retention_posture.as_str(),
                row.retention_disclosed,
                entry.retention.posture.as_str(),
                entry.retention.disclosed
            ),
        );
    }
    if entry.visibility.granted != row.granted_visibility
        || entry.visibility.effective != row.effective_visibility
    {
        push_finding(
            findings,
            SavedQueryFindingKind::ExportVisibilityMismatch,
            format!(
                "export for `{}` records visibility granted `{}`/effective `{}` but the entry is granted `{}`/effective `{}`",
                row.entry_id_ref,
                row.granted_visibility.as_str(),
                row.effective_visibility.as_str(),
                entry.visibility.granted.as_str(),
                entry.visibility.effective.as_str()
            ),
        );
    }
    if entry.trust_class != row.trust_class {
        push_finding(
            findings,
            SavedQueryFindingKind::ExportTrustClassMismatch,
            format!(
                "export for `{}` records trust `{}` but the entry is `{}`",
                row.entry_id_ref,
                row.trust_class.as_str(),
                entry.trust_class.as_str()
            ),
        );
    }
    if entry.chips.source_class != row.source_class {
        push_finding(
            findings,
            SavedQueryFindingKind::ExportSourceClassMismatch,
            format!(
                "export for `{}` records source `{}` but the entry chip is `{}`",
                row.entry_id_ref,
                row.source_class.as_str(),
                entry.chips.source_class.as_str()
            ),
        );
    }
    if entry.chips.confidence != row.confidence {
        push_finding(
            findings,
            SavedQueryFindingKind::ExportConfidenceMismatch,
            format!(
                "export for `{}` records confidence `{}` but the entry chip is `{}`",
                row.entry_id_ref,
                row.confidence.as_str(),
                entry.chips.confidence.as_str()
            ),
        );
    }
    if entry.export_safety.redaction_class != row.redaction_class {
        push_finding(
            findings,
            SavedQueryFindingKind::ExportRedactionMismatch,
            format!(
                "export for `{}` records redaction `{}` but the entry is `{}`",
                row.entry_id_ref,
                row.redaction_class.as_str(),
                entry.export_safety.redaction_class.as_str()
            ),
        );
    }
    if entry.export_safety.export_safe != row.export_safe {
        push_finding(
            findings,
            SavedQueryFindingKind::ExportExportSafeMismatch,
            format!(
                "export for `{}` records export-safe `{}` but the entry is `{}`",
                row.entry_id_ref, row.export_safe, entry.export_safety.export_safe
            ),
        );
    }
}

fn check_degradations(
    input: &SavedQueryPrivacyPacketInput,
    findings: &mut Vec<SavedQueryValidationFinding>,
) {
    let entry_ids: BTreeSet<&str> = input
        .entries
        .iter()
        .map(|entry| entry.entry_id.as_str())
        .collect();

    for degradation in &input.query_degradations {
        if degradation.summary.trim().is_empty() {
            push_finding(
                findings,
                SavedQueryFindingKind::DegradationIncomplete,
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
                    SavedQueryFindingKind::DegradationOrphan,
                    format!("degradation references unknown entry `{}`", entry_id),
                );
            }
        }
    }
}

fn check_consumer_projections(
    input: &SavedQueryPrivacyPacketInput,
    findings: &mut Vec<SavedQueryValidationFinding>,
) {
    let present: BTreeSet<SavedQueryConsumerSurface> = input
        .consumer_projections
        .iter()
        .map(|projection| projection.surface)
        .collect();
    for required in REQUIRED_SURFACES {
        if !present.contains(&required) {
            push_finding(
                findings,
                SavedQueryFindingKind::RequiredSurfaceCoverageMissing,
                format!("required surface `{}` is missing", required.as_str()),
            );
        }
    }

    for projection in &input.consumer_projections {
        if projection.packet_id_ref != input.packet_id {
            push_finding(
                findings,
                SavedQueryFindingKind::ConsumerProjectionPacketIdMismatch,
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
                SavedQueryFindingKind::ConsumerProjectionDrift,
                format!(
                    "surface `{}` drops a required preservation flag",
                    projection.surface.as_str()
                ),
            );
        }
    }
}

fn check_boundary(
    input: &SavedQueryPrivacyPacketInput,
    findings: &mut Vec<SavedQueryValidationFinding>,
) {
    let value = serde_json::to_value(input).expect("saved-query-privacy input serializes");
    if json_contains_forbidden_boundary_material(&value) {
        push_finding(
            findings,
            SavedQueryFindingKind::RawBoundaryMaterialPresent,
            "export must not carry raw bodies, raw URLs, raw history payloads, or secrets",
        );
    }
}

/// Computes the promotion state from the worst severity across both the
/// validation findings and the attached degradations.
///
/// A blocking finding (privacy, retention, export-safety, visibility, trust,
/// citation, or boundary violation) blocks the Stable claim; an otherwise-clean
/// set that carries a narrowing degradation narrows below Stable rather than
/// hiding the entries.
fn promotion_state(
    findings: &[SavedQueryValidationFinding],
    degradations: &[SavedQueryDegradation],
) -> SavedQueryPromotionState {
    let any_blocking = findings
        .iter()
        .any(|finding| finding.severity == SavedQueryFindingSeverity::Blocking)
        || degradations
            .iter()
            .any(|degradation| degradation.severity == SavedQueryFindingSeverity::Blocking);
    if any_blocking {
        return SavedQueryPromotionState::BlocksStable;
    }

    let any_narrowing = findings
        .iter()
        .any(|finding| finding.severity == SavedQueryFindingSeverity::Narrowing)
        || degradations
            .iter()
            .any(|degradation| degradation.severity == SavedQueryFindingSeverity::Narrowing);
    if any_narrowing {
        SavedQueryPromotionState::NarrowedBelowStable
    } else {
        SavedQueryPromotionState::Stable
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
                || lower.contains("raw_query:")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Seeded stable saved-query-privacy input used by the producer, tests, and fixtures.
pub fn seeded_stable_saved_query_privacy_input() -> SavedQueryPrivacyPacketInput {
    let packet_id = "packet:m5:saved_query_privacy:retry_backoff_searches".to_owned();
    SavedQueryPrivacyPacketInput {
        packet_id: packet_id.clone(),
        session_label: "saved-query privacy: the networking retry backoff search set".to_owned(),
        session_digest_ref: "sessiondigest:sha256:net-retry-backoff-searches".to_owned(),
        entries: vec![
            private_local_saved_query_entry(),
            private_synced_history_entry(),
            shared_team_pinned_entry(),
        ],
        export: seeded_export(),
        query_degradations: vec![SavedQueryDegradation {
            degradation_class: SavedQueryDegradationClass::SyncOfflineSnapshot,
            severity: SavedQueryFindingSeverity::Advisory,
            summary: "the private sync was last reconciled two days ago; the synced history entry is served from the cached snapshot".to_owned(),
            entry_id_ref: Some("entry:recent_history:retry_backoff_log_search".to_owned()),
            evidence_ref: Some("evidence:saved-query-privacy:sync-state".to_owned()),
        }],
        consumer_projections: required_projections(&packet_id),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-10T00:00:00Z".to_owned(),
    }
}

fn private_local_saved_query_entry() -> SavedQueryEntry {
    SavedQueryEntry {
        entry_id: "entry:saved_query:retry_backoff_symbol_search".to_owned(),
        entry_kind: QueryEntryKind::SavedQuery,
        privacy_class: QueryPrivacyClass::PrivateLocal,
        subject_ref: "query:project:retry_with_backoff_symbol_search".to_owned(),
        title: "Saved query: retry_with_backoff usages".to_owned(),
        query_label: "a private-local saved query for the local retry backoff symbol usages"
            .to_owned(),
        chips: QueryChipSet {
            source_class: QuerySourceClass::UserSavedQuery,
            version_match: QueryVersionMatch::ExactBuildMatch,
            freshness: QueryFreshness::AuthoritativeLive,
            locality: QueryLocality::Local,
            confidence: QueryConfidence::High,
        },
        trust_class: QueryTrustClass::FirstPartyUserSaved,
        trust_disclosure_note: "first-party query the user saved; it stays local and authoritative"
            .to_owned(),
        visibility: VisibilityGrant {
            granted: Visibility::OwnerOnly,
            effective: Visibility::OwnerOnly,
        },
        retention: RetentionDisclosure {
            posture: RetentionPosture::LocalOnly,
            disclosed: true,
            note: "retained only on this device; never synced or shared".to_owned(),
        },
        export_safety: SupportExportSafety {
            redaction_class: QueryRedactionClass::RedactedLabelOnly,
            export_safe: true,
            note: "support export carries the human label only; the raw query is dropped"
                .to_owned(),
        },
        share_posture: SharePosture::LocalPrivateOnly,
        captured_vs_live: CapturedVsLive::Live,
        cited: true,
        citation_ref: Some("cite:query:project:retry_with_backoff_symbol_search".to_owned()),
        open_raw_escape_ref: "open-raw:query:project:retry_with_backoff_symbol_search".to_owned(),
        open_source_escape_ref: "open-source:repo:crates/aureline-net/src/retry.rs".to_owned(),
    }
}

fn private_synced_history_entry() -> SavedQueryEntry {
    SavedQueryEntry {
        entry_id: "entry:recent_history:retry_backoff_log_search".to_owned(),
        entry_kind: QueryEntryKind::RecentHistory,
        privacy_class: QueryPrivacyClass::PrivateSynced,
        subject_ref: "query:history:retry_backoff_log_search".to_owned(),
        title: "Recent search: retry backoff log lines".to_owned(),
        query_label: "a private-synced recent search across the retry backoff log lines".to_owned(),
        chips: QueryChipSet {
            source_class: QuerySourceClass::SyncedHistory,
            version_match: QueryVersionMatch::CompatibleMinorDrift,
            freshness: QueryFreshness::WarmCached,
            locality: QueryLocality::SyncedPrivate,
            confidence: QueryConfidence::Medium,
        },
        trust_class: QueryTrustClass::LiveSyncedSuggestion,
        trust_disclosure_note: "synced from the user's account; not verified at materialization time, held to medium and disclosed as a warm snapshot".to_owned(),
        visibility: VisibilityGrant {
            granted: Visibility::OwnerDevices,
            effective: Visibility::OwnerDevices,
        },
        retention: RetentionDisclosure {
            posture: RetentionPosture::SyncedPrivate,
            disclosed: true,
            note: "synced privately across the user's own devices; reconciled two days ago".to_owned(),
        },
        export_safety: SupportExportSafety {
            redaction_class: QueryRedactionClass::DigestOnly,
            export_safe: true,
            note: "support export carries an opaque digest only".to_owned(),
        },
        share_posture: SharePosture::ShareAvailable,
        captured_vs_live: CapturedVsLive::Live,
        cited: true,
        citation_ref: Some("cite:query:history:retry_backoff_log_search".to_owned()),
        open_raw_escape_ref: "open-raw:query:history:retry_backoff_log_search".to_owned(),
        open_source_escape_ref: "open-source:sync:retry_backoff_log_search".to_owned(),
    }
}

fn shared_team_pinned_entry() -> SavedQueryEntry {
    SavedQueryEntry {
        entry_id: "entry:pinned_query:retry_backoff_team_query".to_owned(),
        entry_kind: QueryEntryKind::PinnedQuery,
        privacy_class: QueryPrivacyClass::SharedTeam,
        subject_ref: "query:team:retry_backoff_team_query".to_owned(),
        title: "Pinned team query: retry/backoff regressions".to_owned(),
        query_label: "a shared-team pinned query for retry/backoff regression triage".to_owned(),
        chips: QueryChipSet {
            source_class: QuerySourceClass::TeamSharedLibrary,
            version_match: QueryVersionMatch::ExactBuildMatch,
            freshness: QueryFreshness::AuthoritativeLive,
            locality: QueryLocality::SharedStore,
            confidence: QueryConfidence::Medium,
        },
        trust_class: QueryTrustClass::SignedSharedLibrary,
        trust_disclosure_note:
            "pinned from the signed shared-team library; attributable to the team store".to_owned(),
        visibility: VisibilityGrant {
            granted: Visibility::Team,
            effective: Visibility::Team,
        },
        retention: RetentionDisclosure {
            posture: RetentionPosture::SharedStore,
            disclosed: true,
            note: "retained in the shared team store; visible to the team and disclosed as shared"
                .to_owned(),
        },
        export_safety: SupportExportSafety {
            redaction_class: QueryRedactionClass::RawWithheld,
            export_safe: true,
            note: "support export withholds the raw query; only metadata travels".to_owned(),
        },
        share_posture: SharePosture::ShareAvailable,
        captured_vs_live: CapturedVsLive::Live,
        cited: true,
        citation_ref: Some("cite:query:team:retry_backoff_team_query".to_owned()),
        open_raw_escape_ref: "open-raw:query:team:retry_backoff_team_query".to_owned(),
        open_source_escape_ref: "open-source:team-library:retry_backoff_team_query".to_owned(),
    }
}

fn seeded_export() -> SavedQueryHistoryExport {
    SavedQueryHistoryExport {
        scope: SavedQueryExportScope::AllEntries,
        preserves_privacy_class: true,
        preserves_retention: true,
        preserves_visibility: true,
        preserves_trust_class: true,
        preserves_source_class: true,
        preserves_confidence: true,
        preserves_redaction: true,
        preserves_export_safe: true,
        preserves_open_raw_open_source_escape: true,
        rows: vec![
            SavedQueryExportRow {
                entry_id_ref: "entry:saved_query:retry_backoff_symbol_search".to_owned(),
                entry_kind: QueryEntryKind::SavedQuery,
                privacy_class: QueryPrivacyClass::PrivateLocal,
                retention_posture: RetentionPosture::LocalOnly,
                granted_visibility: Visibility::OwnerOnly,
                effective_visibility: Visibility::OwnerOnly,
                trust_class: QueryTrustClass::FirstPartyUserSaved,
                source_class: QuerySourceClass::UserSavedQuery,
                confidence: QueryConfidence::High,
                redaction_class: QueryRedactionClass::RedactedLabelOnly,
                export_safe: true,
                retention_disclosed: true,
                cited: true,
                open_raw_escape_ref: "open-raw:query:project:retry_with_backoff_symbol_search"
                    .to_owned(),
                open_source_escape_ref: "open-source:repo:crates/aureline-net/src/retry.rs"
                    .to_owned(),
            },
            SavedQueryExportRow {
                entry_id_ref: "entry:recent_history:retry_backoff_log_search".to_owned(),
                entry_kind: QueryEntryKind::RecentHistory,
                privacy_class: QueryPrivacyClass::PrivateSynced,
                retention_posture: RetentionPosture::SyncedPrivate,
                granted_visibility: Visibility::OwnerDevices,
                effective_visibility: Visibility::OwnerDevices,
                trust_class: QueryTrustClass::LiveSyncedSuggestion,
                source_class: QuerySourceClass::SyncedHistory,
                confidence: QueryConfidence::Medium,
                redaction_class: QueryRedactionClass::DigestOnly,
                export_safe: true,
                retention_disclosed: true,
                cited: true,
                open_raw_escape_ref: "open-raw:query:history:retry_backoff_log_search".to_owned(),
                open_source_escape_ref: "open-source:sync:retry_backoff_log_search".to_owned(),
            },
            SavedQueryExportRow {
                entry_id_ref: "entry:pinned_query:retry_backoff_team_query".to_owned(),
                entry_kind: QueryEntryKind::PinnedQuery,
                privacy_class: QueryPrivacyClass::SharedTeam,
                retention_posture: RetentionPosture::SharedStore,
                granted_visibility: Visibility::Team,
                effective_visibility: Visibility::Team,
                trust_class: QueryTrustClass::SignedSharedLibrary,
                source_class: QuerySourceClass::TeamSharedLibrary,
                confidence: QueryConfidence::Medium,
                redaction_class: QueryRedactionClass::RawWithheld,
                export_safe: true,
                retention_disclosed: true,
                cited: true,
                open_raw_escape_ref: "open-raw:query:team:retry_backoff_team_query".to_owned(),
                open_source_escape_ref: "open-source:team-library:retry_backoff_team_query"
                    .to_owned(),
            },
        ],
    }
}

fn required_projections(packet_id: &str) -> Vec<SavedQueryConsumerProjection> {
    [
        SavedQueryConsumerSurface::SearchSurface,
        SavedQueryConsumerSurface::DocsBrowserShell,
        SavedQueryConsumerSurface::SavedQueryLibrary,
        SavedQueryConsumerSurface::HistoryPanel,
        SavedQueryConsumerSurface::AiContextInspector,
        SavedQueryConsumerSurface::CliHeadless,
        SavedQueryConsumerSurface::SupportExport,
        SavedQueryConsumerSurface::Diagnostics,
        SavedQueryConsumerSurface::HelpAbout,
    ]
    .into_iter()
    .map(|surface| SavedQueryConsumerProjection {
        surface,
        packet_id_ref: packet_id.to_owned(),
        preserves_chips: true,
        preserves_privacy_classes: true,
        preserves_retention: true,
        preserves_visibility: true,
        preserves_trust_classes: true,
        preserves_redaction: true,
        preserves_export_safe: true,
        preserves_open_raw_open_source_escape: true,
    })
    .collect()
}
