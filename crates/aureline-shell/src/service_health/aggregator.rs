//! Service-health aggregator and contract-state cards.
//!
//! ## Why one aggregator, not seven surface-local status badges
//!
//! Service-health, Help / About, diagnostics, headless inspect, support
//! exports, and release-truth packets all need the same answer when a user
//! or reviewer asks "which service family is unhealthy, what does that mean
//! for my work right now, and is the rest of the product still safe?".
//! Surface-local copy ("service degraded", "unavailable", "we hit an
//! error") drifts the moment one surface lags a change in another. The
//! result is a chrome that paints the whole product as broken because the
//! `framework_lsp` family lost its remote indexer, or a CLI that calls a
//! known-stale snapshot `unavailable` while the desktop reports it as
//! `degraded`.
//!
//! This module mints a single [`ServiceHealthAggregator`] record that
//! every surface reads. Each member service family is captured as one
//! [`ServiceHealthCard`] with a closed
//! [`ServiceContractStateClass`] (`ready`, `degraded`, `local_only`,
//! `stale`, `contract_mismatch`, `policy_blocked`, `unavailable`), the
//! affected workflows the user actually feels, a last-checked age, the
//! boundary class the family sits behind, and the local-continuity
//! posture the rest of the product can rely on while this one family is
//! impaired.
//!
//! ## What a card carries
//!
//! - `service_family` and `service_family_label` — the user-visible name
//!   ("Language services", "AI assist", "Sync") so the chrome never has to
//!   invent copy.
//! - `boundary_class` — `local_only`, `local_with_remote_optional`,
//!   `local_with_remote_required`, `hosted`, or `vendor_provider`. Used by
//!   the chrome to decide which families are immune to network outages.
//! - `contract_state` — the 7-state vocabulary. The chrome MUST quote the
//!   stable token and the human label verbatim; rendering a different
//!   token (e.g. "broken", "down", "error") here is a contract violation.
//! - `affected_workflows` — a closed
//!   [`AffectedWorkflowClass`] set quoting which launch-critical workflows
//!   the user will feel. Empty when the family is `ready` or when only
//!   secondary affordances are affected (the card explains in
//!   `state_explanation` instead).
//! - `last_checked` — the last time the aggregator probed the family and
//!   the age bucket the chrome reads (`fresh`, `recent`, `stale`,
//!   `very_stale`, `never_checked`).
//! - `local_continuity` — `local_safe`, `local_safe_read_only`,
//!   `local_review_only`, or `local_unsafe`. The chrome reads this to
//!   light the "you can keep working locally" continuity strip.
//! - `diagnostics_action` — a stable, copyable command-ref string the
//!   chrome can wire into a "Run diagnostics" button. Never contains raw
//!   endpoint URLs or credentials.
//! - `state_explanation` — a short reviewable sentence (<= 240 chars)
//!   explaining the state without leaking endpoint trivia.
//! - `card_id` — stable object identity for support exports and release
//!   truth packets so an incident shows up under the same ref across
//!   surfaces.
//!
//! ## Local-continuity invariant
//!
//! Acceptance criterion: "A single failed service cannot silently flip
//! the whole product into broken or unavailable messaging when local work
//! remains safe." The aggregator enforces this by computing
//! [`ServiceHealthAggregator::overall_local_continuity`] as the *worst*
//! `local_continuity` across cards whose `boundary_class` includes a
//! local fallback. Hosted-only or vendor-only failures cannot downgrade
//! the overall continuity below `local_safe` — they only contribute to
//! the affected-family counters.
//!
//! ## What the aggregator does not do
//!
//! - It does not own the live probes. The
//!   [`ServiceHealthProbeReading`] inputs are minted by the runtime probes
//!   (network, license, AI provider, framework LSP, etc.) and fed in by
//!   the shell's service-health bus. The aggregator only normalizes them.
//! - It does not run incident management. Once a family flips to
//!   `degraded`, `contract_mismatch`, or `unavailable`, the
//!   `diagnostics_action` points users at the existing inspector;
//!   responder workflows live in
//!   `crates/aureline-support/src/incident_workspace_beta` and are out of
//!   scope here.
//! - It does not invent fallback copy. Every visible string is quoted
//!   from a stable token + label table on this module, so support exports
//!   and headless inspect read the same vocabulary the desktop does.

use std::cmp::Ordering;

use aureline_service_health_feed::{
    ServiceHealthContractState, ServiceHealthFeed, ServiceHealthFeedItem, ServiceHealthFreshness,
    ServiceHealthOutageScope, ServiceHealthSourceClass, ServiceHealthSurface,
    ServiceHealthSurfaceBinding, SERVICE_HEALTH_FEED_ITEM_RECORD_KIND,
    SERVICE_HEALTH_FEED_RECORD_KIND, SERVICE_HEALTH_FEED_SCHEMA_REF,
    SERVICE_HEALTH_FEED_SCHEMA_VERSION, SERVICE_HEALTH_FEED_SHARED_CONTRACT_REF,
};
use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried in serialized aggregator payloads.
pub const SERVICE_HEALTH_AGGREGATOR_RECORD_KIND: &str = "service_health_aggregator_record";

/// Schema version for the [`ServiceHealthAggregator`] payload shape.
pub const SERVICE_HEALTH_AGGREGATOR_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for an individual card payload.
pub const SERVICE_HEALTH_CARD_RECORD_KIND: &str = "service_health_card_record";

/// Schema version for an individual card payload.
pub const SERVICE_HEALTH_CARD_SCHEMA_VERSION: u32 = 1;

/// Reviewer-facing notice rendered on every aggregator-truth surface.
pub const SERVICE_HEALTH_AGGREGATOR_NOTICE: &str =
    "Service-health aggregator: every card carries a stable contract-state token, the affected \
     workflows the user feels, the boundary class the service family sits behind, and the safe \
     local-continuity posture. Shell, About, CLI/headless inspect, diagnostics, and support exports \
     read this record verbatim — surface-local degraded copy is not admitted.";

/// Service family vocabulary. The aggregator pins one card per family it
/// has ever heard about. Surfaces MUST not invent families outside this
/// set; if a family is added later it is additive-minor and bumps
/// [`SERVICE_HEALTH_AGGREGATOR_SCHEMA_VERSION`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceFamilyClass {
    /// Language servers, framework adapters, on-disk indexer.
    LanguageServices,
    /// AI assistants and providers — completion, chat, edit suggestions.
    AiAssist,
    /// Workspace sync, remote workspace, mirror sync.
    Sync,
    /// Account, license, entitlement broker.
    LicenseEntitlement,
    /// Telemetry, crash reporting, support packet upload.
    Telemetry,
    /// Marketplace, extension catalog, recipe discovery.
    Marketplace,
    /// Remote container, dev container, build farm.
    RemoteRuntime,
    /// Update channel, docs mirror, claim manifest fetch.
    ReleaseChannel,
    /// Docs / help knowledge pack.
    DocsKnowledge,
    /// Status-feed/incident page aggregation (vendor + project).
    StatusFeed,
}

impl ServiceFamilyClass {
    /// Stable string token (snake_case).
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LanguageServices => "language_services",
            Self::AiAssist => "ai_assist",
            Self::Sync => "sync",
            Self::LicenseEntitlement => "license_entitlement",
            Self::Telemetry => "telemetry",
            Self::Marketplace => "marketplace",
            Self::RemoteRuntime => "remote_runtime",
            Self::ReleaseChannel => "release_channel",
            Self::DocsKnowledge => "docs_knowledge",
            Self::StatusFeed => "status_feed",
        }
    }

    /// Stable user-visible label. Quoted verbatim across surfaces.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LanguageServices => "Language services",
            Self::AiAssist => "AI assist",
            Self::Sync => "Workspace sync",
            Self::LicenseEntitlement => "License & entitlement",
            Self::Telemetry => "Telemetry & crash",
            Self::Marketplace => "Marketplace",
            Self::RemoteRuntime => "Remote runtime",
            Self::ReleaseChannel => "Release channel",
            Self::DocsKnowledge => "Docs & knowledge",
            Self::StatusFeed => "Status feed",
        }
    }
}

/// Boundary class: how this service family sits relative to the local
/// editor. Used by the aggregator to decide whether the family can
/// degrade overall local-continuity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryClass {
    /// Entirely local. Loss is a real local failure.
    LocalOnly,
    /// Local with remote helpers that are optional. Remote outage cannot
    /// flip overall continuity below `local_safe`.
    LocalWithRemoteOptional,
    /// Local with remote pieces required for full function. Remote
    /// outage degrades overall continuity to `local_safe_read_only` or
    /// `local_review_only` depending on the card.
    LocalWithRemoteRequired,
    /// Hosted service with no local fallback (e.g. marketplace fetch).
    /// Outage never downgrades local-continuity by itself; the chrome
    /// just reports the family as unavailable.
    Hosted,
    /// Vendor / third-party provider. Treated like `Hosted` for the
    /// local-continuity calculation but distinguished in copy so users
    /// understand the failure is upstream.
    VendorProvider,
}

impl BoundaryClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::LocalWithRemoteOptional => "local_with_remote_optional",
            Self::LocalWithRemoteRequired => "local_with_remote_required",
            Self::Hosted => "hosted",
            Self::VendorProvider => "vendor_provider",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalOnly => "Local-only",
            Self::LocalWithRemoteOptional => "Local — remote optional",
            Self::LocalWithRemoteRequired => "Local — remote required",
            Self::Hosted => "Hosted",
            Self::VendorProvider => "Vendor / provider",
        }
    }

    /// True when an outage in this family can downgrade overall local
    /// continuity for the user. Hosted and vendor-provider outages do
    /// not; the rest of the product stays `local_safe`.
    pub const fn can_downgrade_local_continuity(self) -> bool {
        matches!(
            self,
            Self::LocalOnly | Self::LocalWithRemoteOptional | Self::LocalWithRemoteRequired
        )
    }
}

/// Closed contract-state vocabulary. Surfaces MUST quote a token from
/// this set; rendering "broken", "error", or "service degraded" is a
/// contract violation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceContractStateClass {
    /// Healthy: probes are current, contract is honoured.
    Ready,
    /// Functioning but at reduced capacity (slow, partial features,
    /// retries firing).
    Degraded,
    /// The service is unreachable but the product remains useful through
    /// its local fallback. The user keeps working locally.
    LocalOnly,
    /// Cached data is being served because the last fresh probe is past
    /// its review window. The contract is honoured but the data is old.
    Stale,
    /// The remote side responded with a payload outside the agreed
    /// contract (schema mismatch, unknown discriminator, version skew).
    ContractMismatch,
    /// Policy / governance blocked the service (admin disabled, region
    /// gate, license, sandbox).
    PolicyBlocked,
    /// Service is unreachable AND there is no admissible local fallback.
    /// Only contributes to overall continuity downgrade when the boundary
    /// class can downgrade it.
    Unavailable,
}

impl ServiceContractStateClass {
    /// Stable string token (snake_case).
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Degraded => "degraded",
            Self::LocalOnly => "local_only",
            Self::Stale => "stale",
            Self::ContractMismatch => "contract_mismatch",
            Self::PolicyBlocked => "policy_blocked",
            Self::Unavailable => "unavailable",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Ready => "Ready",
            Self::Degraded => "Degraded",
            Self::LocalOnly => "Local-only",
            Self::Stale => "Stale",
            Self::ContractMismatch => "Contract mismatch",
            Self::PolicyBlocked => "Policy blocked",
            Self::Unavailable => "Unavailable",
        }
    }

    /// True when the chrome MUST light a yellow chip on the card.
    pub const fn is_honest_warning(self) -> bool {
        !matches!(self, Self::Ready)
    }

    /// Severity for sorting and rollup. Higher value = more severe.
    /// `ready` is 0; `unavailable` and `contract_mismatch` are the
    /// strongest signals.
    pub const fn severity(self) -> u8 {
        match self {
            Self::Ready => 0,
            Self::Degraded => 2,
            Self::LocalOnly => 1,
            Self::Stale => 1,
            Self::ContractMismatch => 4,
            Self::PolicyBlocked => 3,
            Self::Unavailable => 4,
        }
    }

    /// Parse from a token. Unknown tokens map to `Unavailable` rather
    /// than fabricating a `Ready` state, so a misconfigured probe never
    /// hides an outage.
    pub fn from_token(token: &str) -> Option<Self> {
        Some(match token {
            "ready" => Self::Ready,
            "degraded" => Self::Degraded,
            "local_only" => Self::LocalOnly,
            "stale" => Self::Stale,
            "contract_mismatch" => Self::ContractMismatch,
            "policy_blocked" => Self::PolicyBlocked,
            "unavailable" => Self::Unavailable,
            _ => return None,
        })
    }
}

/// Local-continuity vocabulary. Says what the user can keep doing while
/// this card is impaired. `LocalSafe` is the "no impact on local work"
/// floor; lower values mean less of the local workflow is usable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalContinuityClass {
    /// Less of the local workflow than `LocalReviewOnly` is usable — for
    /// example, the editor itself cannot persist edits. Aggregates to
    /// the worst floor.
    LocalUnsafe,
    /// Read & review only. Edits / commits are blocked until the family
    /// recovers (e.g. policy lock).
    LocalReviewOnly,
    /// Edits work but external writes (sync push, publish, share) are
    /// blocked.
    LocalSafeReadOnly,
    /// Full local functionality. The user does not feel this card.
    LocalSafe,
}

impl LocalContinuityClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalSafe => "local_safe",
            Self::LocalSafeReadOnly => "local_safe_read_only",
            Self::LocalReviewOnly => "local_review_only",
            Self::LocalUnsafe => "local_unsafe",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalSafe => "Local work safe",
            Self::LocalSafeReadOnly => "Local edits safe — no external writes",
            Self::LocalReviewOnly => "Local review only — edits paused",
            Self::LocalUnsafe => "Local work blocked",
        }
    }
}

/// Closed set of launch-critical workflows a card can name as affected.
/// Mirrors the workflow vocabulary the failover-continuity banner reads
/// from `schemas/ops/local_safe_baseline.schema.json`, kept narrow on
/// purpose so cards never name micro-features.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AffectedWorkflowClass {
    Edit,
    Save,
    Search,
    GitOperations,
    Build,
    TestRun,
    Debug,
    OpenRecent,
    Export,
    Diagnostics,
    UndoRedo,
    LocalDocsInspect,
    CachedProviderInspect,
    AiCompletion,
    AiChat,
    AiInlineEdit,
    LanguageDefinition,
    LanguageReferences,
    LanguageRename,
    LanguageFormatting,
    WorkspaceSync,
    RemoteShell,
    MarketplaceBrowse,
    ExtensionInstall,
    LicenseRefresh,
    SupportExportUpload,
    TelemetryUpload,
    DocsBrowseLocal,
    DocsBrowseRemote,
}

impl AffectedWorkflowClass {
    /// Stable token (snake_case).
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Edit => "edit",
            Self::Save => "save",
            Self::Search => "search",
            Self::GitOperations => "git_operations",
            Self::Build => "build",
            Self::TestRun => "test_run",
            Self::Debug => "debug",
            Self::OpenRecent => "open_recent",
            Self::Export => "export",
            Self::Diagnostics => "diagnostics",
            Self::UndoRedo => "undo_redo",
            Self::LocalDocsInspect => "local_docs_inspect",
            Self::CachedProviderInspect => "cached_provider_inspect",
            Self::AiCompletion => "ai_completion",
            Self::AiChat => "ai_chat",
            Self::AiInlineEdit => "ai_inline_edit",
            Self::LanguageDefinition => "language_definition",
            Self::LanguageReferences => "language_references",
            Self::LanguageRename => "language_rename",
            Self::LanguageFormatting => "language_formatting",
            Self::WorkspaceSync => "workspace_sync",
            Self::RemoteShell => "remote_shell",
            Self::MarketplaceBrowse => "marketplace_browse",
            Self::ExtensionInstall => "extension_install",
            Self::LicenseRefresh => "license_refresh",
            Self::SupportExportUpload => "support_export_upload",
            Self::TelemetryUpload => "telemetry_upload",
            Self::DocsBrowseLocal => "docs_browse_local",
            Self::DocsBrowseRemote => "docs_browse_remote",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Edit => "Editing",
            Self::Save => "Save",
            Self::Search => "Search",
            Self::GitOperations => "Git operations",
            Self::Build => "Build",
            Self::TestRun => "Run tests",
            Self::Debug => "Debug",
            Self::OpenRecent => "Open recent",
            Self::Export => "Export",
            Self::Diagnostics => "Diagnostics",
            Self::UndoRedo => "Undo / redo",
            Self::LocalDocsInspect => "Local docs inspect",
            Self::CachedProviderInspect => "Cached provider inspect",
            Self::AiCompletion => "AI completion",
            Self::AiChat => "AI chat",
            Self::AiInlineEdit => "AI inline edit",
            Self::LanguageDefinition => "Go to definition",
            Self::LanguageReferences => "Find references",
            Self::LanguageRename => "Rename symbol",
            Self::LanguageFormatting => "Formatting",
            Self::WorkspaceSync => "Workspace sync",
            Self::RemoteShell => "Remote shell",
            Self::MarketplaceBrowse => "Marketplace browse",
            Self::ExtensionInstall => "Install extension",
            Self::LicenseRefresh => "Refresh license",
            Self::SupportExportUpload => "Upload support export",
            Self::TelemetryUpload => "Upload telemetry",
            Self::DocsBrowseLocal => "Browse docs (local)",
            Self::DocsBrowseRemote => "Browse docs (remote mirror)",
        }
    }
}

/// Age bucket for the last-checked time. The aggregator computes it from
/// the probe's `last_checked` minus a caller-supplied `as_of` so the
/// chrome reads a stable token instead of formatting age each frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LastCheckedAgeClass {
    /// Probed within the last 5 minutes.
    Fresh,
    /// Probed within the last hour.
    Recent,
    /// Probed within the last 24 hours.
    Stale,
    /// Probed more than 24 hours ago.
    VeryStale,
    /// No probe has ever landed for this card (or the probe time was
    /// unparseable). The chrome MUST not paint such a card "ready".
    NeverChecked,
}

impl LastCheckedAgeClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Recent => "recent",
            Self::Stale => "stale",
            Self::VeryStale => "very_stale",
            Self::NeverChecked => "never_checked",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Fresh => "Just now",
            Self::Recent => "Within the hour",
            Self::Stale => "Hours ago",
            Self::VeryStale => "More than a day ago",
            Self::NeverChecked => "Never checked",
        }
    }

    /// True when the chrome should light a freshness warning chip even
    /// if the contract state is `ready`.
    pub const fn is_freshness_warning(self) -> bool {
        matches!(self, Self::Stale | Self::VeryStale | Self::NeverChecked)
    }
}

const MAX_STATE_EXPLANATION_CHARS: usize = 240;
const MAX_DIAGNOSTICS_ACTION_CHARS: usize = 120;
const MAX_DETAIL_TOKEN_CHARS: usize = 64;

/// Probe-reading input minted by the runtime probes. The aggregator
/// normalizes a vec of these into [`ServiceHealthCard`]s.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceHealthProbeReading {
    /// Caller-supplied stable card id. Surfaces use this for object
    /// identity (e.g. support export and release-truth refs).
    pub card_id: String,
    pub service_family: ServiceFamilyClass,
    pub boundary_class: BoundaryClass,
    pub contract_state: ServiceContractStateClass,
    pub local_continuity: LocalContinuityClass,
    #[serde(default)]
    pub affected_workflows: Vec<AffectedWorkflowClass>,
    /// ISO-8601 UTC timestamp from a monotonic clock source. The
    /// aggregator parses the leading `YYYY-MM-DDTHH:MM` and computes the
    /// age bucket relative to `as_of`.
    #[serde(default)]
    pub last_checked: Option<String>,
    pub state_explanation: String,
    pub diagnostics_action: String,
    /// Stable, non-secret tokens the support-export reader may quote
    /// (e.g. provider class, sandbox class). Never carries URLs or
    /// credentials. Each token is capped to 64 chars.
    #[serde(default)]
    pub detail_tokens: Vec<String>,
}

/// One card. The aggregator produces one of these per probe reading
/// after normalizing copy, computing age, and validating the input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceHealthCard {
    pub record_kind: String,
    pub schema_version: u32,
    pub card_id: String,
    pub service_family: ServiceFamilyClass,
    pub service_family_token: String,
    pub service_family_label: String,
    pub boundary_class: BoundaryClass,
    pub boundary_token: String,
    pub boundary_label: String,
    pub contract_state: ServiceContractStateClass,
    pub contract_state_token: String,
    pub contract_state_label: String,
    pub local_continuity: LocalContinuityClass,
    pub local_continuity_token: String,
    pub local_continuity_label: String,
    pub affected_workflows: Vec<AffectedWorkflowClass>,
    pub affected_workflow_tokens: Vec<String>,
    pub last_checked: Option<String>,
    pub last_checked_age: LastCheckedAgeClass,
    pub last_checked_age_token: String,
    pub last_checked_age_label: String,
    pub state_explanation: String,
    pub diagnostics_action: String,
    pub detail_tokens: Vec<String>,
    pub honesty_marker_present: bool,
    /// True when this card alone, by virtue of its boundary class, is
    /// allowed to drag overall local-continuity below `LocalSafe`.
    pub contributes_to_local_continuity: bool,
}

impl ServiceHealthCard {
    /// True when the card lights a yellow chip in the chrome.
    pub const fn honest_warning(&self) -> bool {
        self.honesty_marker_present
    }
}

/// Aggregated counters across all cards on the aggregator.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ServiceHealthAggregatorSummary {
    pub total_card_count: u32,
    pub ready_card_count: u32,
    pub degraded_card_count: u32,
    pub local_only_card_count: u32,
    pub stale_card_count: u32,
    pub contract_mismatch_card_count: u32,
    pub policy_blocked_card_count: u32,
    pub unavailable_card_count: u32,
    pub never_checked_card_count: u32,
}

/// Top-level aggregator record every surface reads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceHealthAggregator {
    pub record_kind: String,
    pub schema_version: u32,
    pub notice: String,
    /// Stable id for this aggregator snapshot. Support exports quote it
    /// so an incident is referenced under the same id across surfaces.
    pub aggregator_id: String,
    /// `as_of` instant used to compute every card's age bucket. Quoted
    /// verbatim into the record.
    pub as_of: String,
    pub cards: Vec<ServiceHealthCard>,
    pub summary: ServiceHealthAggregatorSummary,
    pub overall_contract_state: ServiceContractStateClass,
    pub overall_contract_state_token: String,
    pub overall_contract_state_label: String,
    pub overall_local_continuity: LocalContinuityClass,
    pub overall_local_continuity_token: String,
    pub overall_local_continuity_label: String,
    /// True when at least one card lights an honest warning.
    pub honesty_marker_present: bool,
}

/// Error raised when an aggregator input fails validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AggregatorBuildError {
    EmptyCardId,
    DuplicateCardId(String),
    EmptyStateExplanation(String),
    StateExplanationTooLong(String),
    EmptyDiagnosticsAction(String),
    DiagnosticsActionTooLong(String),
    DetailTokenEmpty(String),
    DetailTokenTooLong(String),
    AsOfEmpty,
    ReadyCardCarriesAffectedWorkflows(String),
}

impl std::fmt::Display for AggregatorBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyCardId => write!(f, "card_id must not be empty"),
            Self::DuplicateCardId(id) => write!(f, "duplicate card_id: {id}"),
            Self::EmptyStateExplanation(id) => {
                write!(f, "state_explanation must not be empty for card {id}")
            }
            Self::StateExplanationTooLong(id) => write!(
                f,
                "state_explanation for card {id} exceeds {MAX_STATE_EXPLANATION_CHARS} chars",
            ),
            Self::EmptyDiagnosticsAction(id) => {
                write!(f, "diagnostics_action must not be empty for card {id}")
            }
            Self::DiagnosticsActionTooLong(id) => write!(
                f,
                "diagnostics_action for card {id} exceeds {MAX_DIAGNOSTICS_ACTION_CHARS} chars",
            ),
            Self::DetailTokenEmpty(id) => {
                write!(f, "detail_token must not be empty on card {id}")
            }
            Self::DetailTokenTooLong(id) => {
                write!(
                    f,
                    "detail_token on card {id} exceeds {MAX_DETAIL_TOKEN_CHARS} chars",
                )
            }
            Self::AsOfEmpty => write!(f, "as_of must not be empty"),
            Self::ReadyCardCarriesAffectedWorkflows(id) => write!(
                f,
                "card {id} is marked ready but carries affected_workflows; a ready card must \
                 have no affected_workflows",
            ),
        }
    }
}

impl std::error::Error for AggregatorBuildError {}

impl ServiceHealthAggregator {
    /// Build an aggregator from a vec of probe readings.
    ///
    /// `as_of` is the chrome's "now" the aggregator reads when computing
    /// `last_checked_age`. Tests pin it to a fixed timestamp.
    pub fn build(
        aggregator_id: impl Into<String>,
        as_of: impl Into<String>,
        readings: Vec<ServiceHealthProbeReading>,
    ) -> Result<Self, AggregatorBuildError> {
        let aggregator_id = aggregator_id.into();
        let as_of = as_of.into();
        if as_of.trim().is_empty() {
            return Err(AggregatorBuildError::AsOfEmpty);
        }

        validate_readings(&readings)?;

        let mut cards = Vec::with_capacity(readings.len());
        for reading in &readings {
            cards.push(project_card(reading, &as_of)?);
        }
        cards.sort_by(card_sort_key);

        let summary = compute_summary(&cards);
        let overall_contract_state = rollup_contract_state(&cards);
        let overall_local_continuity = rollup_local_continuity(&cards);
        let honesty_marker_present = cards.iter().any(|c| c.honesty_marker_present);

        Ok(Self {
            record_kind: SERVICE_HEALTH_AGGREGATOR_RECORD_KIND.to_owned(),
            schema_version: SERVICE_HEALTH_AGGREGATOR_SCHEMA_VERSION,
            notice: SERVICE_HEALTH_AGGREGATOR_NOTICE.to_owned(),
            aggregator_id,
            as_of,
            cards,
            summary,
            overall_contract_state,
            overall_contract_state_token: overall_contract_state.as_str().to_owned(),
            overall_contract_state_label: overall_contract_state.label().to_owned(),
            overall_local_continuity,
            overall_local_continuity_token: overall_local_continuity.as_str().to_owned(),
            overall_local_continuity_label: overall_local_continuity.label().to_owned(),
            honesty_marker_present,
        })
    }

    /// Cards in the same deterministic order rendered everywhere — by
    /// severity (worst first), then by service family.
    pub fn cards_for_render(&self) -> &[ServiceHealthCard] {
        &self.cards
    }

    /// Cards whose contract state is anything other than `ready`.
    pub fn impaired_cards(&self) -> Vec<&ServiceHealthCard> {
        self.cards
            .iter()
            .filter(|c| c.contract_state != ServiceContractStateClass::Ready)
            .collect()
    }

    /// Render a deterministic plaintext block for support-export and
    /// reviewer-facing previews. Stable for the same input snapshot.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str("Service-health aggregator\n");
        out.push_str(&format!("Aggregator: {}\n", self.aggregator_id));
        out.push_str(&format!("As of: {}\n", self.as_of));
        out.push_str(&format!(
            "Overall: {} ({}) | Local continuity: {} ({})\n",
            self.overall_contract_state.label(),
            self.overall_contract_state_token,
            self.overall_local_continuity.label(),
            self.overall_local_continuity_token,
        ));
        out.push_str(&format!(
            "Honesty marker: {}\n",
            if self.honesty_marker_present {
                "present"
            } else {
                "none"
            },
        ));
        out.push_str(&format!(
            "Summary: total={}, ready={}, degraded={}, local_only={}, stale={}, contract_mismatch={}, policy_blocked={}, unavailable={}, never_checked={}\n\n",
            self.summary.total_card_count,
            self.summary.ready_card_count,
            self.summary.degraded_card_count,
            self.summary.local_only_card_count,
            self.summary.stale_card_count,
            self.summary.contract_mismatch_card_count,
            self.summary.policy_blocked_card_count,
            self.summary.unavailable_card_count,
            self.summary.never_checked_card_count,
        ));

        for card in &self.cards {
            out.push_str(&format!(
                "- {} [{}] family={} boundary={} state={} continuity={} age={}\n",
                card.card_id,
                if card.honesty_marker_present {
                    "warn"
                } else {
                    "ok"
                },
                card.service_family_token,
                card.boundary_token,
                card.contract_state_token,
                card.local_continuity_token,
                card.last_checked_age_token,
            ));
            if !card.affected_workflows.is_empty() {
                out.push_str("    affected workflows: ");
                out.push_str(&card.affected_workflow_tokens.join(", "));
                out.push('\n');
            }
            out.push_str(&format!("    explain: {}\n", card.state_explanation));
            out.push_str(&format!(
                "    diagnostics action: {}\n",
                card.diagnostics_action,
            ));
        }

        out
    }

    /// Projects the aggregator into the shared service-health feed contract.
    pub fn shared_service_health_feed(&self) -> ServiceHealthFeed {
        let items = self
            .cards
            .iter()
            .map(ServiceHealthCard::to_shared_feed_item)
            .collect::<Vec<_>>();
        let item_refs = items
            .iter()
            .map(ServiceHealthFeedItem::item_ref)
            .collect::<Vec<_>>();

        ServiceHealthFeed {
            record_kind: SERVICE_HEALTH_FEED_RECORD_KIND.to_owned(),
            schema_version: SERVICE_HEALTH_FEED_SCHEMA_VERSION,
            feed_id: format!("{}:shared_feed", self.aggregator_id),
            shared_contract_ref: SERVICE_HEALTH_FEED_SHARED_CONTRACT_REF.to_owned(),
            schema_ref: SERVICE_HEALTH_FEED_SCHEMA_REF.to_owned(),
            items,
            surface_bindings: ServiceHealthSurface::REQUIRED
                .iter()
                .copied()
                .map(|surface| ServiceHealthSurfaceBinding {
                    surface,
                    feed_ref: self.aggregator_id.clone(),
                    item_refs: item_refs.clone(),
                    consumes_shared_feed: true,
                    last_checked_visible: true,
                    freshness_visible: true,
                    local_only_continuity_visible: true,
                    may_overclaim_live_reachability: false,
                    copyable_exportable: true,
                })
                .collect(),
        }
    }
}

impl ServiceHealthCard {
    fn to_shared_feed_item(&self) -> ServiceHealthFeedItem {
        let source_class = if self
            .detail_tokens
            .iter()
            .any(|token| token == "fallback_mode:mirror_only")
        {
            ServiceHealthSourceClass::MirroredNotice
        } else if matches!(
            self.last_checked_age,
            LastCheckedAgeClass::Stale
                | LastCheckedAgeClass::VeryStale
                | LastCheckedAgeClass::NeverChecked
        ) {
            ServiceHealthSourceClass::CachedData
        } else {
            ServiceHealthSourceClass::LivePolling
        };

        let outage_scope = if self.contract_state == ServiceContractStateClass::Ready {
            ServiceHealthOutageScope::None
        } else if self.contributes_to_local_continuity {
            ServiceHealthOutageScope::PartialService
        } else {
            ServiceHealthOutageScope::SingleService
        };

        let unaffected_workflows = if outage_scope == ServiceHealthOutageScope::PartialService {
            vec!["editing".to_owned(), "search".to_owned(), "git".to_owned()]
        } else {
            Vec::new()
        };

        ServiceHealthFeedItem {
            schema_version: SERVICE_HEALTH_FEED_SCHEMA_VERSION,
            record_kind: SERVICE_HEALTH_FEED_ITEM_RECORD_KIND.to_owned(),
            item_id: self.card_id.clone(),
            service_family: self.service_family_token.clone(),
            boundary_class: self.boundary_token.clone(),
            contract_state: self.contract_state.into(),
            outage_scope,
            affected_workflows: self.affected_workflow_tokens.clone(),
            unaffected_workflows,
            summary: self.state_explanation.clone(),
            freshness: ServiceHealthFreshness {
                freshness_ref: format!("freshness:{}", self.card_id),
                source_class,
                last_checked_at: self
                    .last_checked
                    .clone()
                    .unwrap_or_else(|| "unknown".to_owned()),
                stale_after: self
                    .last_checked
                    .clone()
                    .unwrap_or_else(|| "unknown".to_owned()),
                visible_freshness_label: self.last_checked_age_label.clone(),
                live_reachability_claim_allowed: !source_class.forbids_live_reachability(),
            },
            diagnostics_actions: vec![self.diagnostics_action.clone()],
            local_only_continuity_note: self.local_continuity_label.clone(),
            surfaced_on: ServiceHealthSurface::REQUIRED.to_vec(),
        }
    }
}

impl From<ServiceContractStateClass> for ServiceHealthContractState {
    fn from(value: ServiceContractStateClass) -> Self {
        match value {
            ServiceContractStateClass::Ready => Self::Ready,
            ServiceContractStateClass::Degraded => Self::Degraded,
            ServiceContractStateClass::LocalOnly => Self::LocalOnly,
            ServiceContractStateClass::Stale => Self::Stale,
            ServiceContractStateClass::ContractMismatch => Self::ContractMismatch,
            ServiceContractStateClass::PolicyBlocked => Self::PolicyBlocked,
            ServiceContractStateClass::Unavailable => Self::Unavailable,
        }
    }
}

fn validate_readings(readings: &[ServiceHealthProbeReading]) -> Result<(), AggregatorBuildError> {
    let mut seen = std::collections::BTreeSet::new();
    for reading in readings {
        if reading.card_id.trim().is_empty() {
            return Err(AggregatorBuildError::EmptyCardId);
        }
        if !seen.insert(reading.card_id.clone()) {
            return Err(AggregatorBuildError::DuplicateCardId(
                reading.card_id.clone(),
            ));
        }

        let explanation = reading.state_explanation.trim();
        if explanation.is_empty() {
            return Err(AggregatorBuildError::EmptyStateExplanation(
                reading.card_id.clone(),
            ));
        }
        if explanation.chars().count() > MAX_STATE_EXPLANATION_CHARS {
            return Err(AggregatorBuildError::StateExplanationTooLong(
                reading.card_id.clone(),
            ));
        }

        let action = reading.diagnostics_action.trim();
        if action.is_empty() {
            return Err(AggregatorBuildError::EmptyDiagnosticsAction(
                reading.card_id.clone(),
            ));
        }
        if action.chars().count() > MAX_DIAGNOSTICS_ACTION_CHARS {
            return Err(AggregatorBuildError::DiagnosticsActionTooLong(
                reading.card_id.clone(),
            ));
        }

        for token in &reading.detail_tokens {
            if token.trim().is_empty() {
                return Err(AggregatorBuildError::DetailTokenEmpty(
                    reading.card_id.clone(),
                ));
            }
            if token.chars().count() > MAX_DETAIL_TOKEN_CHARS {
                return Err(AggregatorBuildError::DetailTokenTooLong(
                    reading.card_id.clone(),
                ));
            }
        }

        if matches!(reading.contract_state, ServiceContractStateClass::Ready)
            && !reading.affected_workflows.is_empty()
        {
            return Err(AggregatorBuildError::ReadyCardCarriesAffectedWorkflows(
                reading.card_id.clone(),
            ));
        }
    }
    Ok(())
}

fn project_card(
    reading: &ServiceHealthProbeReading,
    as_of: &str,
) -> Result<ServiceHealthCard, AggregatorBuildError> {
    let last_checked_age = match &reading.last_checked {
        Some(ts) => derive_age(ts, as_of),
        None => LastCheckedAgeClass::NeverChecked,
    };

    // Sort + dedupe affected_workflows so two cards with the same input
    // intent serialize the same way.
    let mut affected_workflows = reading.affected_workflows.clone();
    affected_workflows.sort();
    affected_workflows.dedup();
    let affected_workflow_tokens = affected_workflows
        .iter()
        .map(|w| w.as_str().to_owned())
        .collect();

    let mut detail_tokens = reading.detail_tokens.clone();
    detail_tokens.sort();
    detail_tokens.dedup();

    let honesty_marker_present = reading.contract_state.is_honest_warning()
        || last_checked_age.is_freshness_warning()
        || reading.local_continuity != LocalContinuityClass::LocalSafe;

    let contributes_to_local_continuity = reading.boundary_class.can_downgrade_local_continuity();

    Ok(ServiceHealthCard {
        record_kind: SERVICE_HEALTH_CARD_RECORD_KIND.to_owned(),
        schema_version: SERVICE_HEALTH_CARD_SCHEMA_VERSION,
        card_id: reading.card_id.clone(),
        service_family: reading.service_family,
        service_family_token: reading.service_family.as_str().to_owned(),
        service_family_label: reading.service_family.label().to_owned(),
        boundary_class: reading.boundary_class,
        boundary_token: reading.boundary_class.as_str().to_owned(),
        boundary_label: reading.boundary_class.label().to_owned(),
        contract_state: reading.contract_state,
        contract_state_token: reading.contract_state.as_str().to_owned(),
        contract_state_label: reading.contract_state.label().to_owned(),
        local_continuity: reading.local_continuity,
        local_continuity_token: reading.local_continuity.as_str().to_owned(),
        local_continuity_label: reading.local_continuity.label().to_owned(),
        affected_workflows,
        affected_workflow_tokens,
        last_checked: reading.last_checked.clone(),
        last_checked_age,
        last_checked_age_token: last_checked_age.as_str().to_owned(),
        last_checked_age_label: last_checked_age.label().to_owned(),
        state_explanation: reading.state_explanation.trim().to_owned(),
        diagnostics_action: reading.diagnostics_action.trim().to_owned(),
        detail_tokens,
        honesty_marker_present,
        contributes_to_local_continuity,
    })
}

fn card_sort_key(a: &ServiceHealthCard, b: &ServiceHealthCard) -> Ordering {
    b.contract_state
        .severity()
        .cmp(&a.contract_state.severity())
        .then_with(|| a.service_family.cmp(&b.service_family))
        .then_with(|| a.card_id.cmp(&b.card_id))
}

fn compute_summary(cards: &[ServiceHealthCard]) -> ServiceHealthAggregatorSummary {
    let mut summary = ServiceHealthAggregatorSummary {
        total_card_count: cards.len() as u32,
        ..ServiceHealthAggregatorSummary::default()
    };
    for card in cards {
        match card.contract_state {
            ServiceContractStateClass::Ready => summary.ready_card_count += 1,
            ServiceContractStateClass::Degraded => summary.degraded_card_count += 1,
            ServiceContractStateClass::LocalOnly => summary.local_only_card_count += 1,
            ServiceContractStateClass::Stale => summary.stale_card_count += 1,
            ServiceContractStateClass::ContractMismatch => {
                summary.contract_mismatch_card_count += 1
            }
            ServiceContractStateClass::PolicyBlocked => summary.policy_blocked_card_count += 1,
            ServiceContractStateClass::Unavailable => summary.unavailable_card_count += 1,
        }
        if card.last_checked_age == LastCheckedAgeClass::NeverChecked {
            summary.never_checked_card_count += 1;
        }
    }
    summary
}

fn rollup_contract_state(cards: &[ServiceHealthCard]) -> ServiceContractStateClass {
    cards
        .iter()
        .map(|c| c.contract_state)
        .max_by_key(|s| s.severity())
        .unwrap_or(ServiceContractStateClass::Ready)
}

fn rollup_local_continuity(cards: &[ServiceHealthCard]) -> LocalContinuityClass {
    // Only cards whose boundary can downgrade local continuity
    // contribute. Hosted-only outages leave overall continuity at
    // `local_safe`.
    cards
        .iter()
        .filter(|c| c.contributes_to_local_continuity)
        .map(|c| c.local_continuity)
        .min()
        .unwrap_or(LocalContinuityClass::LocalSafe)
}

fn derive_age(last_checked: &str, as_of: &str) -> LastCheckedAgeClass {
    let last = match parse_timestamp_minutes(last_checked) {
        Some(v) => v,
        None => return LastCheckedAgeClass::NeverChecked,
    };
    let now = match parse_timestamp_minutes(as_of) {
        Some(v) => v,
        None => return LastCheckedAgeClass::NeverChecked,
    };
    if now < last {
        // The probe's last-checked is in the future relative to `as_of`.
        // Treat the card as never checked rather than fabricate a
        // freshness chip.
        return LastCheckedAgeClass::NeverChecked;
    }
    let delta_minutes = now - last;
    if delta_minutes <= 5 {
        LastCheckedAgeClass::Fresh
    } else if delta_minutes <= 60 {
        LastCheckedAgeClass::Recent
    } else if delta_minutes <= 60 * 24 {
        LastCheckedAgeClass::Stale
    } else {
        LastCheckedAgeClass::VeryStale
    }
}

// Parse `YYYY-MM-DDTHH:MM` (the optional `:SS` suffix and trailing `Z`
// are ignored). Returns total minutes from the Howard-Hinnant epoch so
// subtraction yields elapsed minutes. Returns None on malformed input.
fn parse_timestamp_minutes(input: &str) -> Option<i64> {
    let bytes = input.as_bytes();
    if bytes.len() < 16 {
        return None;
    }
    if bytes[4] != b'-' || bytes[7] != b'-' {
        return None;
    }
    if bytes[10] != b'T' && bytes[10] != b' ' {
        return None;
    }
    if bytes[13] != b':' {
        return None;
    }
    let year: i64 = std::str::from_utf8(&bytes[0..4]).ok()?.parse().ok()?;
    let month: u32 = std::str::from_utf8(&bytes[5..7]).ok()?.parse().ok()?;
    let day: u32 = std::str::from_utf8(&bytes[8..10]).ok()?.parse().ok()?;
    let hour: u32 = std::str::from_utf8(&bytes[11..13]).ok()?.parse().ok()?;
    let minute: u32 = std::str::from_utf8(&bytes[14..16]).ok()?.parse().ok()?;
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    if hour > 23 || minute > 59 {
        return None;
    }
    let day_number = days_from_civil(year, month, day);
    Some(day_number * 24 * 60 + i64::from(hour) * 60 + i64::from(minute))
}

fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = (if y >= 0 { y } else { y - 399 }) / 400;
    let yoe = y - era * 400;
    let m_i = m as i64;
    let doy = (153 * (if m_i > 2 { m_i - 3 } else { m_i + 9 }) + 2) / 5 + d as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ready_reading(id: &str, family: ServiceFamilyClass) -> ServiceHealthProbeReading {
        ServiceHealthProbeReading {
            card_id: id.to_owned(),
            service_family: family,
            boundary_class: BoundaryClass::LocalWithRemoteOptional,
            contract_state: ServiceContractStateClass::Ready,
            local_continuity: LocalContinuityClass::LocalSafe,
            affected_workflows: vec![],
            last_checked: Some("2026-05-19T12:00".to_owned()),
            state_explanation: "All probes current.".to_owned(),
            diagnostics_action: "shell.command:service_health.inspect".to_owned(),
            detail_tokens: vec![],
        }
    }

    #[test]
    fn ready_only_aggregator_is_local_safe_with_no_honesty_marker() {
        let readings = vec![
            ready_reading("card:language", ServiceFamilyClass::LanguageServices),
            ready_reading("card:ai", ServiceFamilyClass::AiAssist),
        ];
        let agg = ServiceHealthAggregator::build("agg:test:ready", "2026-05-19T12:01", readings)
            .expect("build");
        assert_eq!(agg.summary.total_card_count, 2);
        assert_eq!(agg.summary.ready_card_count, 2);
        assert_eq!(agg.overall_contract_state, ServiceContractStateClass::Ready);
        assert_eq!(
            agg.overall_local_continuity,
            LocalContinuityClass::LocalSafe
        );
        assert!(!agg.honesty_marker_present);
        for card in &agg.cards {
            assert!(!card.honesty_marker_present);
            assert_eq!(card.last_checked_age, LastCheckedAgeClass::Fresh);
        }
    }

    #[test]
    fn hosted_outage_does_not_downgrade_overall_local_continuity() {
        let mut hosted = ready_reading("card:marketplace", ServiceFamilyClass::Marketplace);
        hosted.boundary_class = BoundaryClass::Hosted;
        hosted.contract_state = ServiceContractStateClass::Unavailable;
        hosted.local_continuity = LocalContinuityClass::LocalSafe;
        hosted.state_explanation = "Marketplace fetch is unreachable; cached browse only.".into();

        let local = ready_reading("card:language", ServiceFamilyClass::LanguageServices);

        let agg = ServiceHealthAggregator::build(
            "agg:test:hosted",
            "2026-05-19T12:01",
            vec![hosted, local],
        )
        .expect("build");

        assert_eq!(agg.summary.unavailable_card_count, 1);
        assert_eq!(agg.summary.ready_card_count, 1);
        assert_eq!(
            agg.overall_contract_state,
            ServiceContractStateClass::Unavailable
        );
        // Overall local continuity stays safe because the only impaired
        // card has Hosted boundary.
        assert_eq!(
            agg.overall_local_continuity,
            LocalContinuityClass::LocalSafe
        );
        assert!(agg.honesty_marker_present);
    }

    #[test]
    fn local_with_remote_required_downgrades_overall_continuity() {
        let mut sync = ready_reading("card:sync", ServiceFamilyClass::Sync);
        sync.boundary_class = BoundaryClass::LocalWithRemoteRequired;
        sync.contract_state = ServiceContractStateClass::LocalOnly;
        sync.local_continuity = LocalContinuityClass::LocalSafeReadOnly;
        sync.state_explanation =
            "Workspace sync unreachable; local edits keep working but pushes pause.".into();
        sync.affected_workflows = vec![AffectedWorkflowClass::WorkspaceSync];

        let language = ready_reading("card:language", ServiceFamilyClass::LanguageServices);

        let agg = ServiceHealthAggregator::build(
            "agg:test:sync",
            "2026-05-19T12:01",
            vec![sync, language],
        )
        .expect("build");

        assert_eq!(
            agg.overall_contract_state,
            ServiceContractStateClass::LocalOnly
        );
        assert_eq!(
            agg.overall_local_continuity,
            LocalContinuityClass::LocalSafeReadOnly
        );
        assert!(agg.honesty_marker_present);
    }

    #[test]
    fn ready_card_with_affected_workflows_is_rejected() {
        let mut reading = ready_reading("card:language", ServiceFamilyClass::LanguageServices);
        reading.affected_workflows = vec![AffectedWorkflowClass::Search];
        let err = ServiceHealthAggregator::build("agg:test:bad", "2026-05-19T12:01", vec![reading])
            .unwrap_err();
        assert!(matches!(
            err,
            AggregatorBuildError::ReadyCardCarriesAffectedWorkflows(_)
        ));
    }

    #[test]
    fn duplicate_card_id_is_rejected() {
        let a = ready_reading("card:dup", ServiceFamilyClass::LanguageServices);
        let b = ready_reading("card:dup", ServiceFamilyClass::AiAssist);
        let err = ServiceHealthAggregator::build("agg:test:dup", "2026-05-19T12:01", vec![a, b])
            .unwrap_err();
        assert!(matches!(err, AggregatorBuildError::DuplicateCardId(_)));
    }

    #[test]
    fn age_bucketing_is_deterministic_across_thresholds() {
        let mut card = ready_reading("card:age", ServiceFamilyClass::Telemetry);
        card.last_checked = Some("2026-05-19T11:58".to_owned());
        let agg =
            ServiceHealthAggregator::build("agg", "2026-05-19T12:00", vec![card.clone()]).unwrap();
        assert_eq!(agg.cards[0].last_checked_age, LastCheckedAgeClass::Fresh);

        card.last_checked = Some("2026-05-19T11:30".to_owned());
        let agg =
            ServiceHealthAggregator::build("agg", "2026-05-19T12:00", vec![card.clone()]).unwrap();
        assert_eq!(agg.cards[0].last_checked_age, LastCheckedAgeClass::Recent);

        card.last_checked = Some("2026-05-19T03:00".to_owned());
        let agg =
            ServiceHealthAggregator::build("agg", "2026-05-19T12:00", vec![card.clone()]).unwrap();
        assert_eq!(agg.cards[0].last_checked_age, LastCheckedAgeClass::Stale);

        card.last_checked = Some("2026-05-15T12:00".to_owned());
        let agg =
            ServiceHealthAggregator::build("agg", "2026-05-19T12:00", vec![card.clone()]).unwrap();
        assert_eq!(
            agg.cards[0].last_checked_age,
            LastCheckedAgeClass::VeryStale
        );

        card.last_checked = None;
        let agg = ServiceHealthAggregator::build("agg", "2026-05-19T12:00", vec![card]).unwrap();
        assert_eq!(
            agg.cards[0].last_checked_age,
            LastCheckedAgeClass::NeverChecked
        );
    }

    #[test]
    fn cards_are_sorted_worst_state_first() {
        let mut ready = ready_reading("card:a", ServiceFamilyClass::AiAssist);
        ready.service_family = ServiceFamilyClass::LanguageServices;

        let mut unavailable = ready_reading("card:b", ServiceFamilyClass::AiAssist);
        unavailable.boundary_class = BoundaryClass::VendorProvider;
        unavailable.contract_state = ServiceContractStateClass::Unavailable;
        unavailable.state_explanation =
            "AI provider returned the wrong schema; treat results as unavailable.".into();
        unavailable.affected_workflows = vec![AffectedWorkflowClass::AiCompletion];

        let agg =
            ServiceHealthAggregator::build("agg", "2026-05-19T12:01", vec![ready, unavailable])
                .unwrap();
        assert_eq!(agg.cards[0].card_id, "card:b");
        assert_eq!(
            agg.cards[0].contract_state,
            ServiceContractStateClass::Unavailable
        );
    }

    #[test]
    fn plaintext_includes_envelope_and_each_card() {
        let agg = ServiceHealthAggregator::build(
            "agg:test:plain",
            "2026-05-19T12:01",
            vec![
                ready_reading("card:language", ServiceFamilyClass::LanguageServices),
                ready_reading("card:ai", ServiceFamilyClass::AiAssist),
            ],
        )
        .unwrap();
        let text = agg.render_plaintext();
        assert!(text.contains("Service-health aggregator"));
        assert!(text.contains("Aggregator: agg:test:plain"));
        assert!(text.contains("card:language"));
        assert!(text.contains("card:ai"));
    }
}
