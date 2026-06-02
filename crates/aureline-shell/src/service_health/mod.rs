//! Service-health beta projection over the canonical M3 governed claim
//! manifest.
//!
//! ## Why one beta truth projection, not seven
//!
//! Service health, Help / About, support exports, release packets, release
//! notes, and CLI / headless help all need the same answer when a reviewer or
//! end user asks "is this beta row current, downgraded, or stale, and what
//! support class did we actually ship it under?". Forking a private layout
//! per surface lets one entry drift its vocabulary while another lags — a
//! help screen claiming "supported" while support exports quote "limited"
//! from the same manifest row, or a service-health chip showing "warm"
//! freshness after the evidence date has slipped past the review window.
//!
//! This module mints a single [`ServiceHealthBetaSurface`] record by
//! projecting the generated M3 claim manifest at
//! `artifacts/release/m3/claim_manifest.json` into a deterministic,
//! per-row record carrying provenance, freshness, support, claim posture,
//! lifecycle, and client-scope badges. Every shell surface that displays
//! beta truth is meant to read this record verbatim instead of hand-editing
//! static copy.
//!
//! ## What the surface carries
//!
//! - **Manifest envelope** — `manifest_id`, `manifest_revision`,
//!   `manifest_state`, `release_channel_scope`, `as_of`, and `generated_at`
//!   quoted verbatim from the generated artifact. The chrome can quote any
//!   of these in a footer chip so the user can see which manifest cut they
//!   are reading without scanning a YAML seed.
//! - **Beta rows** — one [`ServiceHealthBetaRow`] per manifest row. Each
//!   row carries:
//!   - the canonical `row_id`, `row_kind`, `headline`, and `claim_family`;
//!   - `claim_row_refs` and `compatibility_row_refs`, quoted verbatim so
//!     downstream surfaces can prove which compatibility rows back the
//!     claim before treating it as current truth;
//!   - declared and effective `claim_posture` plus the active downgrade
//!     reasons (the row's failure vocabulary, quoted verbatim from the
//!     manifest);
//!   - declared and effective `support_class` plus the target classes the
//!     row is targeting at beta-exit and at stable promotion;
//!   - `lifecycle_label` (`preview` / `beta`);
//!   - `freshness_badge_class` joined to a derived
//!     [`FreshnessState`] that lights `review_due_soon`, `review_overdue`,
//!     or `evidence_expired` whenever the `evidence_date` slipped past
//!     `review_window_days` relative to a caller-supplied `as_of`;
//!   - `provenance_label` plus the evidence owner / handoff path;
//!   - per-channel projection metadata for the four surfaces this beta
//!     truth lane MUST stay consistent on — `help_about`,
//!     `service_health`, `support_export`, and `release_packet` — so the
//!     surface itself can warn when a required projection is missing or
//!     when a row binds with a different copy field than its peers.
//!   - typed booleans for the user-visible failure drills:
//!     `claim_posture_downgraded`, `support_downgraded`, `evidence_stale`,
//!     `evidence_expired`, `required_projection_missing`,
//!     `copy_field_drifts_between_help_about_and_service_health`, and
//!     `honesty_marker_present` — the single bit the chrome reads when
//!     deciding whether to light a yellow chip on the row.
//! - **Summary counters** — `total_row_count`,
//!   `downgraded_claim_row_count`, `downgraded_support_row_count`,
//!   `evidence_stale_row_count`, `evidence_expired_row_count`,
//!   `required_projection_missing_row_count`, and
//!   `copy_field_drift_row_count`. The chrome reads these so it can paint
//!   a service-health overview chip without recomputing.
//! - **Client-scope chip** — projected from the manifest's
//!   `release_channel_scope` so Help / About, service health, and the
//!   support export agree on which channel scope the rows belong to.
//! - **Honesty marker** — true when any row carries a downgraded posture,
//!   downgraded support, stale or expired evidence, a missing required
//!   projection, or copy-field drift between the help-about and
//!   service-health channels. The shell chrome MUST surface this chip
//!   verbatim instead of fabricating an "all green" badge before the
//!   manifest is current.
//!
//! ## Failure-drill posture
//!
//! Three named failure drills are exercised in
//! [`/fixtures/release/beta_truth_cases/*.json`]:
//!
//! - `protected_walk_current_manifest_snapshot.json` — a synthetic snapshot
//!   where every row binds with a current evidence date inside its review
//!   window, posture is `claim_bearing`, and the surface does not light
//!   `honesty_marker_present`.
//! - `failure_drill_stale_evidence.json` — a snapshot whose
//!   `as_of_for_freshness_evaluation` is far enough past every row's
//!   `evidence_date` that each row flips to `evidence_expired` and the
//!   surface lights `honesty_marker_present`.
//! - `failure_drill_downgraded_claim_row.json` — a snapshot whose first
//!   row carries `claim_posture.declared = claim_bearing` and
//!   `effective = limited` with explicit downgrade reasons. The
//!   `downgraded_claim_row_count` increments to 1 and the row's chip
//!   surfaces the reason vocabulary verbatim.
//!
//! ## Out of scope
//!
//! The projection is read-only and does not:
//!
//! - mutate the manifest, the seed matrix, or any upstream evidence;
//! - reach out to publish / promote / rollback / yank command rails — those
//!   are owned by the release-center lane and stay reserved here;
//! - duplicate the schema validation already performed by
//!   `ci/check_m3_claim_manifest.py` against
//!   `schemas/release/m3_claim_manifest.schema.json`.

use std::path::Path;

use serde::{Deserialize, Serialize};

pub mod aggregator;
pub mod continuity_corpus;
pub mod seed;

pub use aggregator::{
    AffectedWorkflowClass, AggregatorBuildError, BoundaryClass, LastCheckedAgeClass,
    LocalContinuityClass, ServiceContractStateClass, ServiceFamilyClass, ServiceHealthAggregator,
    ServiceHealthAggregatorSummary, ServiceHealthCard, ServiceHealthProbeReading,
    SERVICE_HEALTH_AGGREGATOR_NOTICE, SERVICE_HEALTH_AGGREGATOR_RECORD_KIND,
    SERVICE_HEALTH_AGGREGATOR_SCHEMA_VERSION, SERVICE_HEALTH_CARD_RECORD_KIND,
    SERVICE_HEALTH_CARD_SCHEMA_VERSION,
};

/// Stable record-kind tag carried in serialized service-health-beta
/// payloads.
pub const SERVICE_HEALTH_BETA_SURFACE_RECORD_KIND: &str = "service_health_beta_surface_record";

/// Schema version for the [`ServiceHealthBetaSurface`] payload shape.
pub const SERVICE_HEALTH_BETA_SURFACE_SCHEMA_VERSION: u32 = 1;

/// Reviewer-facing notice rendered on every beta-truth surface so the lane's
/// scope is not overstated.
pub const SERVICE_HEALTH_BETA_NOTICE: &str =
    "Service-health beta surface: rows are projected verbatim from the generated M3 governed \
     claim manifest. The shell never invents support, freshness, provenance, or claim-posture \
     truth of its own; refresh the manifest to update what users see in-product.";

/// Stable string token for the manifest-driven channel ids the beta truth
/// surface MUST stay consistent on. Mirrors the `channel_id` strings written
/// by `ci/check_m3_claim_manifest.py`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestChannelId {
    DocsSite,
    MigrationNotes,
    HelpAbout,
    ServiceHealth,
    SupportExport,
    ReleasePacket,
    ReleaseNotes,
    CliHelp,
    EvaluationArtifact,
    MarketplaceDiscovery,
    PublicProofPacket,
}

impl ManifestChannelId {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsSite => "docs_site",
            Self::MigrationNotes => "migration_notes",
            Self::HelpAbout => "help_about",
            Self::ServiceHealth => "service_health",
            Self::SupportExport => "support_export",
            Self::ReleasePacket => "release_packet",
            Self::ReleaseNotes => "release_notes",
            Self::CliHelp => "cli_help",
            Self::EvaluationArtifact => "evaluation_artifact",
            Self::MarketplaceDiscovery => "marketplace_discovery",
            Self::PublicProofPacket => "public_proof_packet",
        }
    }

    /// Best-effort parse from a manifest channel token. Returns `None` for
    /// unknown channels so the caller can hold the row honest about a token
    /// it does not recognise.
    pub fn from_token(token: &str) -> Option<Self> {
        Some(match token {
            "docs_site" => Self::DocsSite,
            "migration_notes" => Self::MigrationNotes,
            "help_about" => Self::HelpAbout,
            "service_health" => Self::ServiceHealth,
            "support_export" => Self::SupportExport,
            "release_packet" => Self::ReleasePacket,
            "release_notes" => Self::ReleaseNotes,
            "cli_help" => Self::CliHelp,
            "evaluation_artifact" => Self::EvaluationArtifact,
            "marketplace_discovery" => Self::MarketplaceDiscovery,
            "public_proof_packet" => Self::PublicProofPacket,
            _ => return None,
        })
    }
}

/// Closed claim-posture vocabulary mirroring
/// `schemas/release/m3_claim_manifest.schema.json#claim_posture`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimPostureClass {
    ClaimBearing,
    Experimental,
    Limited,
    PolicyDisabled,
    ReplacementGrade,
    SeedOnly,
    Withdrawn,
    /// Token did not match the closed vocabulary; the row is held honest
    /// rather than silently folding into `claim_bearing`.
    UnknownClaimPosture,
}

impl ClaimPostureClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimBearing => "claim_bearing",
            Self::Experimental => "experimental",
            Self::Limited => "limited",
            Self::PolicyDisabled => "policy_disabled",
            Self::ReplacementGrade => "replacement_grade",
            Self::SeedOnly => "seed_only",
            Self::Withdrawn => "withdrawn",
            Self::UnknownClaimPosture => "unknown_claim_posture",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ClaimBearing => "Claim-bearing",
            Self::Experimental => "Experimental",
            Self::Limited => "Limited",
            Self::PolicyDisabled => "Policy-disabled",
            Self::ReplacementGrade => "Replacement-grade",
            Self::SeedOnly => "Seed-only",
            Self::Withdrawn => "Withdrawn",
            Self::UnknownClaimPosture => "Unknown claim posture",
        }
    }

    /// Parse from the manifest's enum token.
    pub fn from_token(token: &str) -> Self {
        match token {
            "claim_bearing" => Self::ClaimBearing,
            "experimental" => Self::Experimental,
            "limited" => Self::Limited,
            "policy_disabled" => Self::PolicyDisabled,
            "replacement_grade" => Self::ReplacementGrade,
            "seed_only" => Self::SeedOnly,
            "withdrawn" => Self::Withdrawn,
            _ => Self::UnknownClaimPosture,
        }
    }
}

/// Closed support-class vocabulary mirroring
/// `schemas/release/m3_claim_manifest.schema.json#support_class`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClassClass {
    Certified,
    Supported,
    Limited,
    Experimental,
    Community,
    RetestPending,
    EvidenceStale,
    Unsupported,
    /// Token did not match the closed vocabulary; held honest.
    UnknownSupportClass,
}

impl SupportClassClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Supported => "supported",
            Self::Limited => "limited",
            Self::Experimental => "experimental",
            Self::Community => "community",
            Self::RetestPending => "retest_pending",
            Self::EvidenceStale => "evidence_stale",
            Self::Unsupported => "unsupported",
            Self::UnknownSupportClass => "unknown_support_class",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Certified => "Certified",
            Self::Supported => "Supported",
            Self::Limited => "Limited",
            Self::Experimental => "Experimental",
            Self::Community => "Community",
            Self::RetestPending => "Retest pending",
            Self::EvidenceStale => "Evidence stale",
            Self::Unsupported => "Unsupported",
            Self::UnknownSupportClass => "Unknown support class",
        }
    }

    /// Parse from the manifest's enum token.
    pub fn from_token(token: &str) -> Self {
        match token {
            "certified" => Self::Certified,
            "supported" => Self::Supported,
            "limited" => Self::Limited,
            "experimental" => Self::Experimental,
            "community" => Self::Community,
            "retest_pending" => Self::RetestPending,
            "evidence_stale" => Self::EvidenceStale,
            "unsupported" => Self::Unsupported,
            _ => Self::UnknownSupportClass,
        }
    }
}

/// Lifecycle label vocabulary mirroring
/// `schemas/release/m3_claim_manifest.schema.json#lifecycle_label`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleLabelClass {
    Preview,
    Beta,
    UnknownLifecycleLabel,
}

impl LifecycleLabelClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preview => "preview",
            Self::Beta => "beta",
            Self::UnknownLifecycleLabel => "unknown_lifecycle_label",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Preview => "Preview",
            Self::Beta => "Beta",
            Self::UnknownLifecycleLabel => "Unknown lifecycle label",
        }
    }

    /// Parse from the manifest's enum token.
    pub fn from_token(token: &str) -> Self {
        match token {
            "preview" => Self::Preview,
            "beta" => Self::Beta,
            _ => Self::UnknownLifecycleLabel,
        }
    }
}

/// Freshness badge vocabulary mirroring
/// `schemas/release/m3_claim_manifest.schema.json#freshness_badge_class`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessBadgeClass {
    AuthoritativeLive,
    WarmCached,
    DegradedCached,
    Stale,
    Unverified,
    UnknownFreshnessBadge,
}

impl FreshnessBadgeClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeLive => "authoritative_live",
            Self::WarmCached => "warm_cached",
            Self::DegradedCached => "degraded_cached",
            Self::Stale => "stale",
            Self::Unverified => "unverified",
            Self::UnknownFreshnessBadge => "unknown_freshness_badge",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AuthoritativeLive => "Authoritative — live",
            Self::WarmCached => "Warm — cached",
            Self::DegradedCached => "Degraded — cached",
            Self::Stale => "Stale",
            Self::Unverified => "Unverified",
            Self::UnknownFreshnessBadge => "Unknown freshness badge",
        }
    }

    /// Parse from the manifest's enum token.
    pub fn from_token(token: &str) -> Self {
        match token {
            "authoritative_live" => Self::AuthoritativeLive,
            "warm_cached" => Self::WarmCached,
            "degraded_cached" => Self::DegradedCached,
            "stale" => Self::Stale,
            "unverified" => Self::Unverified,
            _ => Self::UnknownFreshnessBadge,
        }
    }
}

/// Provenance label vocabulary mirroring
/// `schemas/release/m3_claim_manifest.schema.json#provenance_label`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceLabelClass {
    ProjectDocs,
    GeneratedReference,
    MirroredOfficialDocs,
    CuratedKnowledgePack,
    DerivedExplanation,
    VendorProviderDocs,
    SupportRunbook,
    ExternalStatusFeed,
    UnknownProvenanceLabel,
}

impl ProvenanceLabelClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectDocs => "project_docs",
            Self::GeneratedReference => "generated_reference",
            Self::MirroredOfficialDocs => "mirrored_official_docs",
            Self::CuratedKnowledgePack => "curated_knowledge_pack",
            Self::DerivedExplanation => "derived_explanation",
            Self::VendorProviderDocs => "vendor_provider_docs",
            Self::SupportRunbook => "support_runbook",
            Self::ExternalStatusFeed => "external_status_feed",
            Self::UnknownProvenanceLabel => "unknown_provenance_label",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ProjectDocs => "Project docs",
            Self::GeneratedReference => "Generated reference",
            Self::MirroredOfficialDocs => "Mirrored official docs",
            Self::CuratedKnowledgePack => "Curated knowledge pack",
            Self::DerivedExplanation => "Derived explanation",
            Self::VendorProviderDocs => "Vendor / provider docs",
            Self::SupportRunbook => "Support runbook",
            Self::ExternalStatusFeed => "External status feed",
            Self::UnknownProvenanceLabel => "Unknown provenance label",
        }
    }

    /// Parse from the manifest's enum token.
    pub fn from_token(token: &str) -> Self {
        match token {
            "project_docs" => Self::ProjectDocs,
            "generated_reference" => Self::GeneratedReference,
            "mirrored_official_docs" => Self::MirroredOfficialDocs,
            "curated_knowledge_pack" => Self::CuratedKnowledgePack,
            "derived_explanation" => Self::DerivedExplanation,
            "vendor_provider_docs" => Self::VendorProviderDocs,
            "support_runbook" => Self::SupportRunbook,
            "external_status_feed" => Self::ExternalStatusFeed,
            _ => Self::UnknownProvenanceLabel,
        }
    }
}

/// Closed row-kind vocabulary mirroring
/// `schemas/release/m3_claim_manifest.schema.json#row_kind`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BetaRowKindClass {
    CanonicalClaimFamily,
    BetaSurfaceBinding,
    BetaArchetypeBinding,
    UnknownRowKind,
}

impl BetaRowKindClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalClaimFamily => "canonical_claim_family",
            Self::BetaSurfaceBinding => "beta_surface_binding",
            Self::BetaArchetypeBinding => "beta_archetype_binding",
            Self::UnknownRowKind => "unknown_row_kind",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::CanonicalClaimFamily => "Canonical claim family",
            Self::BetaSurfaceBinding => "Beta surface binding",
            Self::BetaArchetypeBinding => "Beta archetype binding",
            Self::UnknownRowKind => "Unknown row kind",
        }
    }

    /// Parse from the manifest's enum token.
    pub fn from_token(token: &str) -> Self {
        match token {
            "canonical_claim_family" => Self::CanonicalClaimFamily,
            "beta_surface_binding" => Self::BetaSurfaceBinding,
            "beta_archetype_binding" => Self::BetaArchetypeBinding,
            _ => Self::UnknownRowKind,
        }
    }
}

/// Closed manifest-state vocabulary mirroring
/// `schemas/release/m3_claim_manifest.schema.json#manifest_state`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestStateClass {
    Draft,
    Ratified,
    Frozen,
    Retired,
    UnknownManifestState,
}

impl ManifestStateClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Ratified => "ratified",
            Self::Frozen => "frozen",
            Self::Retired => "retired",
            Self::UnknownManifestState => "unknown_manifest_state",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Draft => "Draft",
            Self::Ratified => "Ratified",
            Self::Frozen => "Frozen",
            Self::Retired => "Retired",
            Self::UnknownManifestState => "Unknown manifest state",
        }
    }

    /// Parse from the manifest's enum token.
    pub fn from_token(token: &str) -> Self {
        match token {
            "draft" => Self::Draft,
            "ratified" => Self::Ratified,
            "frozen" => Self::Frozen,
            "retired" => Self::Retired,
            _ => Self::UnknownManifestState,
        }
    }
}

/// Release-channel-scope chip projected from
/// `release_channel_scope` on the manifest. Mirrors the manifest's enum so
/// Help / About, service health, and support exports read the same chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseChannelScopeClass {
    Nightly,
    Preview,
    Beta,
    Stable,
    Lts,
    UnknownReleaseChannelScope,
}

impl ReleaseChannelScopeClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Nightly => "nightly",
            Self::Preview => "preview",
            Self::Beta => "beta",
            Self::Stable => "stable",
            Self::Lts => "lts",
            Self::UnknownReleaseChannelScope => "unknown_release_channel_scope",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Nightly => "Nightly channel",
            Self::Preview => "Preview channel",
            Self::Beta => "Beta channel",
            Self::Stable => "Stable channel",
            Self::Lts => "LTS channel",
            Self::UnknownReleaseChannelScope => "Unknown release channel",
        }
    }

    /// Parse from the manifest's enum token.
    pub fn from_token(token: &str) -> Self {
        match token {
            "nightly" => Self::Nightly,
            "preview" => Self::Preview,
            "beta" => Self::Beta,
            "stable" => Self::Stable,
            "lts" => Self::Lts,
            _ => Self::UnknownReleaseChannelScope,
        }
    }
}

/// Freshness-state classification derived by joining `freshness_badge_class`
/// with the row's `evidence_date`, `review_window_days`, and a caller-
/// supplied `as_of` date. The state is what the chrome reads on the row —
/// the raw badge stays available too, but the state is what fires the
/// honesty marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessStateClass {
    /// Evidence is current — inside its review window and badged as
    /// authoritative-live or warm-cached.
    Current,
    /// Evidence is within its review window but the badge is degraded.
    DegradedButCurrent,
    /// Evidence is within its review window but the badge is `unverified`.
    UnverifiedButCurrent,
    /// More than half of the review window has elapsed.
    ReviewDueSoon,
    /// Evidence has slipped past the review window.
    ReviewOverdue,
    /// Evidence is more than twice its review window old.
    EvidenceExpired,
    /// `as_of` came in before the evidence date, or the date string could
    /// not be parsed — the seed labels the row honestly rather than
    /// fabricating a `current` state.
    UnableToEvaluateFreshness,
}

impl FreshnessStateClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::DegradedButCurrent => "degraded_but_current",
            Self::UnverifiedButCurrent => "unverified_but_current",
            Self::ReviewDueSoon => "review_due_soon",
            Self::ReviewOverdue => "review_overdue",
            Self::EvidenceExpired => "evidence_expired",
            Self::UnableToEvaluateFreshness => "unable_to_evaluate_freshness",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Current => "Current",
            Self::DegradedButCurrent => "Degraded — but still current",
            Self::UnverifiedButCurrent => "Unverified — but still current",
            Self::ReviewDueSoon => "Review due soon",
            Self::ReviewOverdue => "Review overdue",
            Self::EvidenceExpired => "Evidence expired",
            Self::UnableToEvaluateFreshness => "Unable to evaluate freshness",
        }
    }

    /// True when the state should light the row's honesty marker.
    pub const fn is_honest_warning(self) -> bool {
        matches!(
            self,
            Self::ReviewDueSoon
                | Self::ReviewOverdue
                | Self::EvidenceExpired
                | Self::UnableToEvaluateFreshness
        )
    }

    /// True when the state should be counted as `evidence_stale` in the
    /// summary counters.
    pub const fn is_stale(self) -> bool {
        matches!(self, Self::ReviewDueSoon | Self::ReviewOverdue)
    }

    /// True when the state should be counted as `evidence_expired` in the
    /// summary counters.
    pub const fn is_expired(self) -> bool {
        matches!(self, Self::EvidenceExpired)
    }
}

/// Parsed, validated record of the manifest's freshness section, joined to
/// the derived freshness state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceHealthFreshness {
    pub badge_class: FreshnessBadgeClass,
    pub badge_token: String,
    pub badge_label: String,
    pub evidence_date: String,
    pub review_window_days: u32,
    pub freshness_derivation: String,
    pub state: FreshnessStateClass,
    pub state_token: String,
    pub state_label: String,
    /// `Some(days_overdue)` when the evidence date has slipped past
    /// `evidence_date + review_window_days`. `None` otherwise (including
    /// `unable_to_evaluate_freshness`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overdue_days: Option<i64>,
    /// `Some(days_remaining)` when the evidence is still inside its review
    /// window. `None` otherwise.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remaining_days: Option<i64>,
}

/// Parsed record of the manifest's claim-posture summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceHealthClaimPosture {
    pub declared: ClaimPostureClass,
    pub declared_token: String,
    pub effective: ClaimPostureClass,
    pub effective_token: String,
    pub active_downgrade_reasons: Vec<String>,
    /// True when declared and effective postures differ.
    pub downgraded: bool,
}

/// Parsed record of the manifest's support summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceHealthSupport {
    pub declared: SupportClassClass,
    pub declared_token: String,
    pub effective: SupportClassClass,
    pub effective_token: String,
    pub downgrade_triggers_fired: Vec<String>,
    pub open_waiver_count: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_at_beta_exit: Option<SupportClassClass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_at_beta_exit_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_at_stable: Option<SupportClassClass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_at_stable_token: Option<String>,
    /// True when declared and effective support classes differ.
    pub downgraded: bool,
}

/// Parsed record of the manifest's provenance summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceHealthProvenance {
    pub label: ProvenanceLabelClass,
    pub label_token: String,
    pub evidence_owner: String,
    pub intake_owner: String,
    pub triage_owner: String,
    pub release_owner: String,
    pub escalation_ref: String,
}

/// Parsed record of one channel-projection row on a manifest row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceHealthChannelProjection {
    pub channel_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_class: Option<ManifestChannelId>,
    pub binding_status: String,
    pub projection_kind: String,
    pub copy_field: String,
    pub surface_ref: String,
}

/// One service-health beta row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceHealthBetaRow {
    /// Claim row refs quoted from the upstream manifest row.
    pub claim_row_refs: Vec<String>,
    /// Compatibility row refs that must resolve in the current
    /// compatibility report before a surface may treat this row as
    /// claim-bearing truth.
    pub compatibility_row_refs: Vec<String>,
    pub row_id: String,
    pub row_kind: BetaRowKindClass,
    pub row_kind_token: String,
    pub headline: String,
    pub claim_family: String,
    pub requirement_ids: Vec<String>,
    pub claim_posture: ServiceHealthClaimPosture,
    pub support: ServiceHealthSupport,
    pub lifecycle_label: LifecycleLabelClass,
    pub lifecycle_label_token: String,
    pub freshness: ServiceHealthFreshness,
    pub provenance: ServiceHealthProvenance,
    pub channel_projections: Vec<ServiceHealthChannelProjection>,
    /// Selection of the four channels the beta-truth lane MUST stay
    /// consistent on. `None` when the row does not project to that channel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help_about_projection: Option<ServiceHealthChannelProjection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_health_projection: Option<ServiceHealthChannelProjection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_export_projection: Option<ServiceHealthChannelProjection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release_packet_projection: Option<ServiceHealthChannelProjection>,
    /// True when at least one of the four required beta-truth channels is
    /// absent or has `binding_status = "not_applicable"` while the row is
    /// not `withdrawn`.
    pub required_projection_missing: bool,
    /// True when the help_about and service_health channels resolve to
    /// different `copy_field` values for this row — the chrome MUST hold
    /// the row honest because the two surfaces would otherwise quote
    /// different beta-row copy.
    pub copy_field_drifts_between_help_about_and_service_health: bool,
    /// Aggregate honest-warning flag for the row.
    pub honesty_marker_present: bool,
}

impl ServiceHealthBetaRow {
    /// Convenience: returns true when this row's claim posture downgraded.
    pub fn claim_posture_downgraded(&self) -> bool {
        self.claim_posture.downgraded
    }

    /// Convenience: returns true when this row's support class downgraded.
    pub fn support_downgraded(&self) -> bool {
        self.support.downgraded
    }

    /// Convenience: returns true when this row's evidence is stale (inside
    /// or past its review window in a way that should warn the user).
    pub fn evidence_stale(&self) -> bool {
        self.freshness.state.is_stale()
    }

    /// Convenience: returns true when this row's evidence is expired.
    pub fn evidence_expired(&self) -> bool {
        self.freshness.state.is_expired()
    }
}

/// Aggregated counters across all rows on the surface.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ServiceHealthBetaSummary {
    pub total_row_count: u32,
    pub downgraded_claim_row_count: u32,
    pub downgraded_support_row_count: u32,
    pub evidence_stale_row_count: u32,
    pub evidence_expired_row_count: u32,
    pub required_projection_missing_row_count: u32,
    pub copy_field_drift_row_count: u32,
    pub canonical_claim_family_row_count: u32,
    pub beta_surface_binding_row_count: u32,
    pub beta_archetype_binding_row_count: u32,
}

/// Top-level beta truth projection record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceHealthBetaSurface {
    pub record_kind: String,
    pub schema_version: u32,
    pub notice: String,
    pub manifest_id: String,
    pub manifest_revision: u32,
    pub manifest_state: ManifestStateClass,
    pub manifest_state_token: String,
    pub release_channel_scope: ReleaseChannelScopeClass,
    pub release_channel_scope_token: String,
    pub release_channel_scope_label: String,
    pub milestone_id: String,
    pub as_of: String,
    pub generated_at: String,
    pub owner: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup_owner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup_waiver: Option<String>,
    pub consuming_surfaces: Vec<String>,
    /// `as_of_for_freshness_evaluation` that the surface was projected
    /// against. Quoted verbatim into the record so support exports can
    /// reproduce the freshness state without guessing at the chrome's
    /// clock.
    pub as_of_for_freshness_evaluation: String,
    pub rows: Vec<ServiceHealthBetaRow>,
    pub summary: ServiceHealthBetaSummary,
    /// True when any row carries a downgraded posture, downgraded support,
    /// stale or expired evidence, a missing required projection, or
    /// copy-field drift between the help_about and service_health
    /// channels.
    pub honesty_marker_present: bool,
}

impl ServiceHealthBetaSurface {
    /// Project a beta truth surface from a parsed manifest snapshot. The
    /// `as_of_for_freshness_evaluation` is the date the chrome reads
    /// "now" as for the purpose of evaluating evidence freshness. Callers
    /// usually pass today's date; tests pass a fixed date so the surface
    /// is deterministic.
    pub fn project(
        manifest: &M3ClaimManifestSnapshot,
        as_of_for_freshness_evaluation: &str,
    ) -> Self {
        let rows: Vec<ServiceHealthBetaRow> = manifest
            .rows
            .iter()
            .map(|row| project_row(row, as_of_for_freshness_evaluation))
            .collect();

        let summary = compute_summary(&rows);

        let honesty_marker_present = summary.downgraded_claim_row_count > 0
            || summary.downgraded_support_row_count > 0
            || summary.evidence_stale_row_count > 0
            || summary.evidence_expired_row_count > 0
            || summary.required_projection_missing_row_count > 0
            || summary.copy_field_drift_row_count > 0;

        let manifest_state = ManifestStateClass::from_token(&manifest.manifest_state);
        let release_channel_scope =
            ReleaseChannelScopeClass::from_token(&manifest.release_channel_scope);

        Self {
            record_kind: SERVICE_HEALTH_BETA_SURFACE_RECORD_KIND.to_owned(),
            schema_version: SERVICE_HEALTH_BETA_SURFACE_SCHEMA_VERSION,
            notice: SERVICE_HEALTH_BETA_NOTICE.to_owned(),
            manifest_id: manifest.manifest_id.clone(),
            manifest_revision: manifest.manifest_revision,
            manifest_state,
            manifest_state_token: manifest_state.as_str().to_owned(),
            release_channel_scope,
            release_channel_scope_token: release_channel_scope.as_str().to_owned(),
            release_channel_scope_label: release_channel_scope.label().to_owned(),
            milestone_id: manifest.milestone_id.clone(),
            as_of: manifest.as_of.clone(),
            generated_at: manifest.generated_at.clone(),
            owner: manifest.owner.clone(),
            backup_owner: manifest.backup_owner.clone(),
            backup_waiver: manifest.backup_waiver.clone(),
            consuming_surfaces: manifest.consuming_surfaces.clone(),
            as_of_for_freshness_evaluation: as_of_for_freshness_evaluation.to_owned(),
            rows,
            summary,
            honesty_marker_present,
        }
    }

    /// Project against the manifest's own `as_of` date (i.e. evaluate
    /// freshness as of when the manifest itself was generated). The
    /// resulting record is identical to the one a reviewer would see at
    /// the manifest's check-in commit.
    pub fn project_at_manifest_as_of(manifest: &M3ClaimManifestSnapshot) -> Self {
        let as_of = manifest.as_of.clone();
        Self::project(manifest, &as_of)
    }

    /// Render a deterministic plaintext block for support-export and
    /// reviewer-facing previews. Stable for the same input snapshot.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str("Service-health beta surface\n");
        out.push_str(&format!(
            "Manifest: {} (rev {})\n",
            self.manifest_id, self.manifest_revision
        ));
        out.push_str(&format!(
            "Manifest state: {} ({})\n",
            self.manifest_state.label(),
            self.manifest_state_token,
        ));
        out.push_str(&format!(
            "Release channel: {} ({})\n",
            self.release_channel_scope_label, self.release_channel_scope_token,
        ));
        out.push_str(&format!("Milestone: {}\n", self.milestone_id));
        out.push_str(&format!("As of: {}\n", self.as_of));
        out.push_str(&format!(
            "Evaluated at: {}\n",
            self.as_of_for_freshness_evaluation,
        ));
        out.push_str(&format!(
            "Honesty marker: {}\n",
            if self.honesty_marker_present {
                "present"
            } else {
                "none"
            },
        ));
        out.push('\n');
        out.push_str(&format!(
            "Summary: total={}, downgraded_claim={}, downgraded_support={}, evidence_stale={}, evidence_expired={}, projection_missing={}, copy_field_drift={}\n\n",
            self.summary.total_row_count,
            self.summary.downgraded_claim_row_count,
            self.summary.downgraded_support_row_count,
            self.summary.evidence_stale_row_count,
            self.summary.evidence_expired_row_count,
            self.summary.required_projection_missing_row_count,
            self.summary.copy_field_drift_row_count,
        ));

        for row in &self.rows {
            out.push_str(&format!("- {} [{}]\n", row.row_id, row.row_kind_token));
            out.push_str(&format!("    headline: {}\n", row.headline));
            out.push_str(&format!(
                "    posture: declared={} effective={} downgraded={}\n",
                row.claim_posture.declared_token,
                row.claim_posture.effective_token,
                row.claim_posture.downgraded,
            ));
            if !row.claim_posture.active_downgrade_reasons.is_empty() {
                out.push_str(&format!(
                    "    posture downgrade reasons: {}\n",
                    row.claim_posture.active_downgrade_reasons.join(", "),
                ));
            }
            out.push_str(&format!(
                "    support: declared={} effective={} downgraded={} waivers={}\n",
                row.support.declared_token,
                row.support.effective_token,
                row.support.downgraded,
                row.support.open_waiver_count,
            ));
            out.push_str(&format!(
                "    lifecycle: {} | freshness: {} (badge {}, evidence {}, window {} days)\n",
                row.lifecycle_label_token,
                row.freshness.state_token,
                row.freshness.badge_token,
                row.freshness.evidence_date,
                row.freshness.review_window_days,
            ));
            out.push_str(&format!(
                "    provenance: {} (owner: {})\n",
                row.provenance.label_token, row.provenance.evidence_owner,
            ));
            if !row.compatibility_row_refs.is_empty() {
                out.push_str(&format!(
                    "    compatibility rows: {}\n",
                    row.compatibility_row_refs.join(", "),
                ));
            }
            if row.required_projection_missing {
                out.push_str("    required projection missing: yes\n");
            }
            if row.copy_field_drifts_between_help_about_and_service_health {
                out.push_str("    copy field drift between help_about and service_health: yes\n");
            }
        }

        out
    }

    /// Returns the rows whose `help_about` channel projection is required.
    /// Help / About reads these so it does not render rows that the
    /// manifest excluded from the help_about channel.
    pub fn rows_for_help_about(&self) -> Vec<&ServiceHealthBetaRow> {
        self.rows
            .iter()
            .filter(|row| {
                row.help_about_projection
                    .as_ref()
                    .map(|p| p.binding_status == "required")
                    .unwrap_or(false)
            })
            .collect()
    }

    /// Returns the rows whose `service_health` channel projection is
    /// required.
    pub fn rows_for_service_health(&self) -> Vec<&ServiceHealthBetaRow> {
        self.rows
            .iter()
            .filter(|row| {
                row.service_health_projection
                    .as_ref()
                    .map(|p| p.binding_status == "required")
                    .unwrap_or(false)
            })
            .collect()
    }
}

fn project_row(
    row: &ManifestRowSnapshot,
    as_of_for_freshness_evaluation: &str,
) -> ServiceHealthBetaRow {
    let row_kind = BetaRowKindClass::from_token(&row.row_kind);

    let claim_posture = project_claim_posture(&row.claim_posture);
    let support = project_support(&row.support);
    let lifecycle_label = LifecycleLabelClass::from_token(&row.lifecycle.display_lifecycle_label);
    let freshness = project_freshness(&row.freshness, as_of_for_freshness_evaluation);
    let provenance = project_provenance(&row.provenance);

    let channel_projections: Vec<ServiceHealthChannelProjection> = row
        .channel_projections
        .iter()
        .map(|cp| ServiceHealthChannelProjection {
            channel_id: cp.channel_id.clone(),
            channel_class: ManifestChannelId::from_token(&cp.channel_id),
            binding_status: cp.binding_status.clone(),
            projection_kind: cp.projection_kind.clone(),
            copy_field: cp.copy_field.clone(),
            surface_ref: cp.surface_ref.clone(),
        })
        .collect();

    let pick = |id: ManifestChannelId| -> Option<ServiceHealthChannelProjection> {
        channel_projections
            .iter()
            .find(|p| p.channel_class == Some(id))
            .cloned()
    };

    let help_about_projection = pick(ManifestChannelId::HelpAbout);
    let service_health_projection = pick(ManifestChannelId::ServiceHealth);
    let support_export_projection = pick(ManifestChannelId::SupportExport);
    let release_packet_projection = pick(ManifestChannelId::ReleasePacket);

    let is_withdrawn = matches!(claim_posture.effective, ClaimPostureClass::Withdrawn);
    let required_projection_missing = !is_withdrawn
        && [
            &help_about_projection,
            &service_health_projection,
            &support_export_projection,
            &release_packet_projection,
        ]
        .into_iter()
        .any(|projection| match projection {
            Some(p) => p.binding_status == "not_applicable",
            None => true,
        });

    let copy_field_drifts_between_help_about_and_service_health = match (
        help_about_projection.as_ref(),
        service_health_projection.as_ref(),
    ) {
        (Some(a), Some(b)) => a.copy_field != b.copy_field,
        _ => false,
    };

    let honesty_marker_present = claim_posture.downgraded
        || support.downgraded
        || freshness.state.is_honest_warning()
        || required_projection_missing
        || copy_field_drifts_between_help_about_and_service_health;

    ServiceHealthBetaRow {
        claim_row_refs: row.claim_row_refs.clone(),
        compatibility_row_refs: row.compatibility_row_refs.clone(),
        row_id: row.row_id.clone(),
        row_kind,
        row_kind_token: row_kind.as_str().to_owned(),
        headline: row.headline.clone(),
        claim_family: row.claim_family.clone(),
        requirement_ids: row.requirement_ids.clone(),
        claim_posture,
        support,
        lifecycle_label,
        lifecycle_label_token: lifecycle_label.as_str().to_owned(),
        freshness,
        provenance,
        channel_projections,
        help_about_projection,
        service_health_projection,
        support_export_projection,
        release_packet_projection,
        required_projection_missing,
        copy_field_drifts_between_help_about_and_service_health,
        honesty_marker_present,
    }
}

fn project_claim_posture(snapshot: &ClaimPostureSnapshot) -> ServiceHealthClaimPosture {
    let declared = ClaimPostureClass::from_token(&snapshot.declared);
    let effective = ClaimPostureClass::from_token(&snapshot.effective);
    ServiceHealthClaimPosture {
        declared,
        declared_token: declared.as_str().to_owned(),
        effective,
        effective_token: effective.as_str().to_owned(),
        active_downgrade_reasons: snapshot.active_downgrade_reasons.clone(),
        downgraded: declared != effective,
    }
}

fn project_support(snapshot: &SupportSnapshot) -> ServiceHealthSupport {
    let declared = SupportClassClass::from_token(&snapshot.declared);
    let effective = SupportClassClass::from_token(&snapshot.effective);
    let target_at_beta_exit = snapshot
        .target_at_beta_exit
        .as_deref()
        .map(SupportClassClass::from_token);
    let target_at_stable = snapshot
        .target_at_stable
        .as_deref()
        .map(SupportClassClass::from_token);

    ServiceHealthSupport {
        declared,
        declared_token: declared.as_str().to_owned(),
        effective,
        effective_token: effective.as_str().to_owned(),
        downgrade_triggers_fired: snapshot.downgrade_triggers_fired.clone(),
        open_waiver_count: snapshot.open_waiver_count,
        target_at_beta_exit,
        target_at_beta_exit_token: target_at_beta_exit.map(|c| c.as_str().to_owned()),
        target_at_stable,
        target_at_stable_token: target_at_stable.map(|c| c.as_str().to_owned()),
        downgraded: declared != effective,
    }
}

fn project_freshness(snapshot: &FreshnessSnapshot, as_of: &str) -> ServiceHealthFreshness {
    let badge = FreshnessBadgeClass::from_token(&snapshot.badge_class);
    let evaluation =
        evaluate_freshness(&snapshot.evidence_date, snapshot.review_window_days, as_of);

    // Map the joined badge + window position onto a freshness state.
    let state = match (badge, &evaluation) {
        (_, FreshnessEvaluation::UnparseableDate) => FreshnessStateClass::UnableToEvaluateFreshness,
        (FreshnessBadgeClass::UnknownFreshnessBadge, _) => {
            FreshnessStateClass::UnableToEvaluateFreshness
        }
        (FreshnessBadgeClass::Unverified, FreshnessEvaluation::InsideWindow { .. }) => {
            FreshnessStateClass::UnverifiedButCurrent
        }
        (FreshnessBadgeClass::DegradedCached, FreshnessEvaluation::InsideWindow { .. }) => {
            FreshnessStateClass::DegradedButCurrent
        }
        (FreshnessBadgeClass::Stale, FreshnessEvaluation::InsideWindow { .. }) => {
            FreshnessStateClass::ReviewDueSoon
        }
        (_, FreshnessEvaluation::InsideWindow { remaining_days }) => {
            if review_window_more_than_half_elapsed(snapshot.review_window_days, *remaining_days) {
                FreshnessStateClass::ReviewDueSoon
            } else {
                FreshnessStateClass::Current
            }
        }
        (
            _,
            FreshnessEvaluation::OutsideWindow {
                overdue_days,
                review_window_days,
            },
        ) => {
            if *overdue_days > i64::from(*review_window_days) {
                FreshnessStateClass::EvidenceExpired
            } else {
                FreshnessStateClass::ReviewOverdue
            }
        }
    };

    let (overdue_days, remaining_days) = match evaluation {
        FreshnessEvaluation::InsideWindow { remaining_days } => (None, Some(remaining_days)),
        FreshnessEvaluation::OutsideWindow { overdue_days, .. } => (Some(overdue_days), None),
        FreshnessEvaluation::UnparseableDate => (None, None),
    };

    ServiceHealthFreshness {
        badge_class: badge,
        badge_token: badge.as_str().to_owned(),
        badge_label: badge.label().to_owned(),
        evidence_date: snapshot.evidence_date.clone(),
        review_window_days: snapshot.review_window_days,
        freshness_derivation: snapshot.freshness_derivation.clone(),
        state,
        state_token: state.as_str().to_owned(),
        state_label: state.label().to_owned(),
        overdue_days,
        remaining_days,
    }
}

fn project_provenance(snapshot: &ProvenanceSnapshot) -> ServiceHealthProvenance {
    let label = ProvenanceLabelClass::from_token(&snapshot.label);
    ServiceHealthProvenance {
        label,
        label_token: label.as_str().to_owned(),
        evidence_owner: snapshot.evidence_owner.clone(),
        intake_owner: snapshot.owner_handoff_path.intake_owner.clone(),
        triage_owner: snapshot.owner_handoff_path.triage_owner.clone(),
        release_owner: snapshot.owner_handoff_path.release_owner.clone(),
        escalation_ref: snapshot.owner_handoff_path.escalation_ref.clone(),
    }
}

#[derive(Debug, Clone, Copy)]
enum FreshnessEvaluation {
    InsideWindow {
        remaining_days: i64,
    },
    OutsideWindow {
        overdue_days: i64,
        review_window_days: u32,
    },
    UnparseableDate,
}

fn evaluate_freshness(
    evidence_date: &str,
    review_window_days: u32,
    as_of: &str,
) -> FreshnessEvaluation {
    let evidence = match parse_iso_date(evidence_date) {
        Some(d) => d,
        None => return FreshnessEvaluation::UnparseableDate,
    };
    let now = match parse_iso_date(as_of) {
        Some(d) => d,
        None => return FreshnessEvaluation::UnparseableDate,
    };
    let days_since_evidence = days_between(evidence, now);
    let window = i64::from(review_window_days);
    if days_since_evidence <= window {
        FreshnessEvaluation::InsideWindow {
            remaining_days: window - days_since_evidence,
        }
    } else {
        FreshnessEvaluation::OutsideWindow {
            overdue_days: days_since_evidence - window,
            review_window_days,
        }
    }
}

fn review_window_more_than_half_elapsed(review_window_days: u32, remaining_days: i64) -> bool {
    if review_window_days == 0 {
        return false;
    }
    let half = i64::from(review_window_days) / 2;
    remaining_days <= half
}

fn parse_iso_date(input: &str) -> Option<(i64, u32, u32)> {
    let bytes = input.as_bytes();
    if bytes.len() < 10 {
        return None;
    }
    if bytes[4] != b'-' || bytes[7] != b'-' {
        return None;
    }
    let year: i64 = std::str::from_utf8(&bytes[0..4]).ok()?.parse().ok()?;
    let month: u32 = std::str::from_utf8(&bytes[5..7]).ok()?.parse().ok()?;
    let day: u32 = std::str::from_utf8(&bytes[8..10]).ok()?.parse().ok()?;
    if !(1..=12).contains(&month) {
        return None;
    }
    if !(1..=31).contains(&day) {
        return None;
    }
    Some((year, month, day))
}

/// Days from `a` to `b`, computed via a Gregorian day-number scheme that
/// matches calendar arithmetic on the inputs the manifest carries (ISO
/// `YYYY-MM-DD`). Negative when `b` precedes `a`.
fn days_between(a: (i64, u32, u32), b: (i64, u32, u32)) -> i64 {
    days_from_civil(b.0, b.1, b.2) - days_from_civil(a.0, a.1, a.2)
}

// Returns serial day number using the algorithm by Howard Hinnant; the
// origin is arbitrary but consistent across calls so subtraction yields the
// number of calendar days between two dates.
fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = (if y >= 0 { y } else { y - 399 }) / 400;
    let yoe = y - era * 400; // 0 ..= 399
    let m_i = m as i64;
    let doy = (153 * (if m_i > 2 { m_i - 3 } else { m_i + 9 }) + 2) / 5 + d as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

fn compute_summary(rows: &[ServiceHealthBetaRow]) -> ServiceHealthBetaSummary {
    let mut summary = ServiceHealthBetaSummary {
        total_row_count: rows.len() as u32,
        ..ServiceHealthBetaSummary::default()
    };
    for row in rows {
        if row.claim_posture.downgraded {
            summary.downgraded_claim_row_count += 1;
        }
        if row.support.downgraded {
            summary.downgraded_support_row_count += 1;
        }
        if row.freshness.state.is_stale() {
            summary.evidence_stale_row_count += 1;
        }
        if row.freshness.state.is_expired() {
            summary.evidence_expired_row_count += 1;
        }
        if row.required_projection_missing {
            summary.required_projection_missing_row_count += 1;
        }
        if row.copy_field_drifts_between_help_about_and_service_health {
            summary.copy_field_drift_row_count += 1;
        }
        match row.row_kind {
            BetaRowKindClass::CanonicalClaimFamily => summary.canonical_claim_family_row_count += 1,
            BetaRowKindClass::BetaSurfaceBinding => summary.beta_surface_binding_row_count += 1,
            BetaRowKindClass::BetaArchetypeBinding => summary.beta_archetype_binding_row_count += 1,
            BetaRowKindClass::UnknownRowKind => {}
        }
    }
    summary
}

// ---------------------------------------------------------------------------
// Manifest snapshot — minimal Serde mirror over the generated artifact
// ---------------------------------------------------------------------------

/// Minimal parsed view of `artifacts/release/m3/claim_manifest.json`. The
/// shell only owns the fields the beta truth projection needs; the
/// authoritative schema and validation live with the CI generator.
#[derive(Debug, Clone, Deserialize)]
pub struct M3ClaimManifestSnapshot {
    pub schema_version: u32,
    pub record_kind: String,
    pub manifest_id: String,
    pub manifest_revision: u32,
    pub milestone_id: String,
    pub release_channel_scope: String,
    pub manifest_state: String,
    pub as_of: String,
    pub generated_at: String,
    pub owner: String,
    #[serde(default)]
    pub backup_owner: Option<String>,
    #[serde(default)]
    pub backup_waiver: Option<String>,
    #[serde(default)]
    pub consuming_surfaces: Vec<String>,
    pub rows: Vec<ManifestRowSnapshot>,
}

/// Errors raised when loading or parsing a manifest snapshot from disk.
#[derive(Debug)]
pub enum ManifestLoadError {
    Io(std::io::Error),
    Parse(serde_json::Error),
    SchemaMismatch {
        expected_record_kind: &'static str,
        actual_record_kind: String,
    },
}

impl std::fmt::Display for ManifestLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io error reading m3 claim manifest: {e}"),
            Self::Parse(e) => write!(f, "parse error in m3 claim manifest: {e}"),
            Self::SchemaMismatch {
                expected_record_kind,
                actual_record_kind,
            } => write!(
                f,
                "m3 claim manifest record_kind mismatch: expected {expected_record_kind}, got {actual_record_kind}"
            ),
        }
    }
}

impl std::error::Error for ManifestLoadError {}

impl From<std::io::Error> for ManifestLoadError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<serde_json::Error> for ManifestLoadError {
    fn from(e: serde_json::Error) -> Self {
        Self::Parse(e)
    }
}

impl M3ClaimManifestSnapshot {
    /// Stable expected record-kind tag on the upstream artifact.
    pub const EXPECTED_RECORD_KIND: &'static str = "m3_claim_manifest";

    /// Load and parse the manifest from a path on disk.
    pub fn load_from_path(path: impl AsRef<Path>) -> Result<Self, ManifestLoadError> {
        let bytes = std::fs::read(path)?;
        Self::from_bytes(&bytes)
    }

    /// Parse the manifest from raw JSON bytes. Validates the
    /// `record_kind` so the projection is never fed an unrelated record.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ManifestLoadError> {
        let snapshot: Self = serde_json::from_slice(bytes)?;
        if snapshot.record_kind != Self::EXPECTED_RECORD_KIND {
            return Err(ManifestLoadError::SchemaMismatch {
                expected_record_kind: Self::EXPECTED_RECORD_KIND,
                actual_record_kind: snapshot.record_kind,
            });
        }
        Ok(snapshot)
    }
}

/// Manifest row snapshot.
#[derive(Debug, Clone, Deserialize)]
pub struct ManifestRowSnapshot {
    pub row_id: String,
    pub row_kind: String,
    pub headline: String,
    pub claim_family: String,
    #[serde(default)]
    pub claim_row_refs: Vec<String>,
    #[serde(default)]
    pub compatibility_row_refs: Vec<String>,
    #[serde(default)]
    pub requirement_ids: Vec<String>,
    pub claim_posture: ClaimPostureSnapshot,
    pub support: SupportSnapshot,
    pub lifecycle: LifecycleSnapshot,
    pub freshness: FreshnessSnapshot,
    pub provenance: ProvenanceSnapshot,
    #[serde(default)]
    pub channel_projections: Vec<ChannelProjectionSnapshot>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClaimPostureSnapshot {
    pub declared: String,
    pub effective: String,
    #[serde(default)]
    pub active_downgrade_reasons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SupportSnapshot {
    pub declared: String,
    pub effective: String,
    #[serde(default)]
    pub downgrade_triggers_fired: Vec<String>,
    pub open_waiver_count: u32,
    #[serde(default)]
    pub target_at_beta_exit: Option<String>,
    #[serde(default)]
    pub target_at_stable: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LifecycleSnapshot {
    pub display_lifecycle_label: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FreshnessSnapshot {
    pub badge_class: String,
    pub evidence_date: String,
    pub review_window_days: u32,
    pub freshness_derivation: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProvenanceSnapshot {
    pub label: String,
    pub evidence_owner: String,
    pub owner_handoff_path: OwnerHandoffPathSnapshot,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OwnerHandoffPathSnapshot {
    pub intake_owner: String,
    pub triage_owner: String,
    pub release_owner: String,
    pub escalation_ref: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChannelProjectionSnapshot {
    pub channel_id: String,
    pub binding_status: String,
    pub projection_kind: String,
    pub copy_field: String,
    pub surface_ref: String,
}

#[cfg(test)]
mod tests;
