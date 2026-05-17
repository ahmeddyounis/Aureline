//! Help / About / provenance / service-health skeleton with client-scope
//! badges.
//!
//! This is the initial seed for the canonical product self-description surface a
//! user opens to answer "what build is running, how was it installed, what
//! client scope am I in, where do I take issues, and is anything visibly
//! degraded?" without scanning logs or chasing a marketing page.
//!
//! ## Why one truth model, not seven
//!
//! Help / About, the command-palette About entry, the settings About pane,
//! diagnostics export, support bundles, and release-evidence packets all need
//! the same answer when a reviewer or a user asks "what build is this?".
//! Forking a private layout per surface lets one entry drift its vocabulary
//! while another lags — for example, a help screen claiming "verified"
//! provenance while the support bundle quotes "unverified" from the same
//! input. This module mints one [`HelpAboutSurface`] record that every entry
//! point projects from, joining the canonical
//! [`aureline_build_info::BuildIdentityRecord`], the resolved
//! [`aureline_runtime::ExecutionContext`], the shared
//! [`crate::badges::target_origin::TargetOriginBadge`] vocabulary, and (when
//! present) the embedded docs/help boundary card's
//! [`crate::embedded::boundary_card::SourceTruthRecord`].
//!
//! ## What the seed surface carries
//!
//! - **Build identity** — every field comes verbatim from the build-info
//!   record minted at compile time; the seed never re-derives versions.
//! - **Install mode** — derived from the release-channel-class token plus the
//!   tree-state inferred from the build-info `dirty` bit; the surface labels
//!   the row honestly when the channel token is unrecognized.
//! - **Client scope** — projects [`TargetOriginBadge`] so the chip vocabulary
//!   (`Local`, `Remote`, `Managed`, `Local desktop → managed plane`, ...)
//!   stays joined to the terminal, task, debug-prep, and provider/auth
//!   surfaces.
//! - **Docs/help truth** — projects the source / version / freshness rows
//!   already minted by the docs/help boundary card so the help shell never
//!   forks a private freshness ladder.
//! - **Service health** — typed placeholder rows for the runtime, auth,
//!   docs/help, and update-channel subsystems. The release-truth attachment
//!   adds the current manifest-derived Help / About card without pretending
//!   the placeholder subsystem rows are a live service monitor.
//! - **Provenance** — typed placeholder rows for signature / attestation /
//!   checksum / SBOM / advisory-open state. The full machine-readable
//!   provenance contract lives in the about_card schema; this seed renders
//!   the row scaffold so support exports and the chrome agree on stable row
//!   ids before that verifier lands.
//! - **Community handoff** — frozen route classes (public issue tracker,
//!   public RFC forum, private security channel, private support channel)
//!   with stable disclosure copy, destination trust classes, auth
//!   expectations, data-exit boundaries, and issue-template refs.
//!
//! ## Failure-drill posture
//!
//! When the upstream [`SourceTruthRecord`] reports a degraded freshness or a
//! drifted version match, the surface lights `honesty_marker_present` and
//! flags the corresponding row instead of rendering a stale "verified"
//! label. When the resolved execution context carries a degraded field or a
//! pending trust posture, the client-scope chip mirrors the boundary cue
//! coming off [`TargetOriginBadge`] so the lane never claims "Local — All
//! clear" while trust is unresolved. The fixtures under
//! [`/fixtures/help/about_cases/*.json`] exercise the protected walk on a
//! trusted local dev seed, the failure drill where the docs/help source
//! freshness goes stale, and the managed-workspace drill where the
//! client-scope chip lights `local_to_managed`.

use serde::{Deserialize, Serialize};

use aureline_build_info::BuildIdentityRecord;
use aureline_runtime::{
    DegradedFieldReason, DegradedFieldRecord, ExecutionContext, IdentityMode, TargetClass,
    TrustState,
};

use crate::about::HelpAboutReleaseTruthCard;
use crate::badges::target_origin::{
    BadgeEntryPoint, HostBoundaryCue, OriginBadgeClass, TargetBadgeClass, TargetOriginBadge,
};
use crate::embedded::boundary_card::{
    FreshnessClass, SourceClass, SourceTruthRecord, VersionMatchState,
};

/// Stable record-kind tag carried in serialized help/about payloads.
pub const HELP_ABOUT_SURFACE_RECORD_KIND: &str = "help_about_surface_record";

/// Schema version for the [`HelpAboutSurface`] payload shape.
pub const HELP_ABOUT_SURFACE_SCHEMA_VERSION: u32 = 1;

/// Seed notice rendered on the surface so a reviewer can see the lane's scope
/// without inferring it from the row vocabulary alone.
pub const HELP_ABOUT_SEED_SCOPE_NOTICE: &str =
    "Help / About seed: live rows quote the exact-build identity, derived install mode, and \
     shared client-scope chip. Service-health and provenance rows remain visible placeholders; \
     the release-truth card attaches the current claim-manifest rows, compatibility refs, and \
     community handoff route disclosures.";

/// Stable section ids the seed surface renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HelpAboutSectionId {
    BuildIdentity,
    InstallMode,
    ClientScope,
    DocsHelpTruth,
    ServiceHealth,
    Provenance,
    CommunityHandoff,
}

impl HelpAboutSectionId {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuildIdentity => "build_identity",
            Self::InstallMode => "install_mode",
            Self::ClientScope => "client_scope",
            Self::DocsHelpTruth => "docs_help_truth",
            Self::ServiceHealth => "service_health",
            Self::Provenance => "provenance",
            Self::CommunityHandoff => "community_handoff",
        }
    }

    /// Human-readable section heading.
    pub const fn heading(self) -> &'static str {
        match self {
            Self::BuildIdentity => "Build identity",
            Self::InstallMode => "Install mode",
            Self::ClientScope => "Client scope",
            Self::DocsHelpTruth => "Docs and help truth",
            Self::ServiceHealth => "Service health",
            Self::Provenance => "Provenance",
            Self::CommunityHandoff => "Community handoff",
        }
    }
}

/// Closed install-mode vocabulary derived from the running build's
/// release-channel-class token plus its tree-state. Mirrors the
/// `channel_class` enum frozen in `schemas/about/about_card.schema.json` so
/// the seed and the about-card hardening agree on row tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallModeClass {
    /// Local development build (e.g. `cargo run`); typically dirty tree.
    DevLocalBuiltFromSource,
    /// Nightly channel install.
    NightlyLocalInstall,
    /// Preview channel install.
    PreviewLocalInstall,
    /// Beta channel install.
    BetaLocalInstall,
    /// Stable channel install.
    StableLocalInstall,
    /// LTS channel install.
    LtsLocalInstall,
    /// Hotfix channel install.
    HotfixLocalInstall,
    /// Channel token did not match a known class; the row is labeled honestly
    /// so the surface never silently renders an unknown channel as `Stable`.
    UnknownInstallMode,
}

impl InstallModeClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DevLocalBuiltFromSource => "dev_local_built_from_source",
            Self::NightlyLocalInstall => "nightly_local_install",
            Self::PreviewLocalInstall => "preview_local_install",
            Self::BetaLocalInstall => "beta_local_install",
            Self::StableLocalInstall => "stable_local_install",
            Self::LtsLocalInstall => "lts_local_install",
            Self::HotfixLocalInstall => "hotfix_local_install",
            Self::UnknownInstallMode => "unknown_install_mode",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DevLocalBuiltFromSource => "Dev (built from source)",
            Self::NightlyLocalInstall => "Nightly install",
            Self::PreviewLocalInstall => "Preview install",
            Self::BetaLocalInstall => "Beta install",
            Self::StableLocalInstall => "Stable install",
            Self::LtsLocalInstall => "LTS install",
            Self::HotfixLocalInstall => "Hotfix install",
            Self::UnknownInstallMode => "Unknown install mode",
        }
    }

    /// Map a release-channel-class token (as minted by
    /// [`aureline_build_info::release_channel_class`]) onto a stable install
    /// mode. Tokens that do not match the channel vocabulary settle on
    /// [`InstallModeClass::UnknownInstallMode`] so the surface is forced to
    /// surface an honesty marker rather than silently labeling unknown
    /// channels as the default.
    pub fn from_channel_token(token: &str) -> Self {
        match token {
            "dev_local" => Self::DevLocalBuiltFromSource,
            "nightly" => Self::NightlyLocalInstall,
            "preview" => Self::PreviewLocalInstall,
            "beta" => Self::BetaLocalInstall,
            "stable" => Self::StableLocalInstall,
            "lts" => Self::LtsLocalInstall,
            "hotfix" => Self::HotfixLocalInstall,
            _ => Self::UnknownInstallMode,
        }
    }
}

/// Stable tree-state vocabulary mirrored from the about_card schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreeStateClass {
    CleanCheckout,
    DirtyLocal,
}

impl TreeStateClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CleanCheckout => "clean_checkout",
            Self::DirtyLocal => "dirty_local",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::CleanCheckout => "Clean checkout",
            Self::DirtyLocal => "Dirty local tree",
        }
    }
}

/// Frozen seed-action vocabulary for the help/about surface.
///
/// Live actions cover surfacing the shared execution-context inspector and
/// preparing a support-export copy. Routes that need machine-readable
/// provenance, advisory history, release-packet linkage, or community-handoff
/// lane wiring are reserved for the milestone that owns those contracts; the
/// seed labels the rows so the user can see the lanes exist without claiming
/// depth this seed does not own.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HelpAboutActionClass {
    /// Open the shared execution-context inspector. Live in the seed.
    OpenExecutionContextInspector,
    /// Copy the surface payload (build identity + client-scope chip + rows)
    /// to the clipboard so a support packet can quote it. Live in the seed.
    CopyContextForSupportExport,
    /// Open the release-evidence packet for the running build. Reserved.
    OpenReleasePacket,
    /// View the full provenance details (signatures, attestations, SBOM
    /// bodies). Reserved.
    ViewProvenanceDetails,
    /// Open the advisory history index for the running build. Reserved.
    OpenAdvisoryHistory,
    /// Hand the user off to the matching community-handoff route based on
    /// the issue class. Live once release-truth routing is attached.
    ReportIssueViaCommunityHandoff,
}

impl HelpAboutActionClass {
    /// Stable string token recorded on the action row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenExecutionContextInspector => "open_execution_context_inspector",
            Self::CopyContextForSupportExport => "copy_context_for_support_export",
            Self::OpenReleasePacket => "open_release_packet",
            Self::ViewProvenanceDetails => "view_provenance_details",
            Self::OpenAdvisoryHistory => "open_advisory_history",
            Self::ReportIssueViaCommunityHandoff => "report_issue_via_community_handoff",
        }
    }

    /// Human-readable label for the action.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OpenExecutionContextInspector => "Open execution-context inspector",
            Self::CopyContextForSupportExport => "Copy About card for support export",
            Self::OpenReleasePacket => "Open release packet",
            Self::ViewProvenanceDetails => "View provenance details",
            Self::OpenAdvisoryHistory => "Open advisory history",
            Self::ReportIssueViaCommunityHandoff => "Report issue via community handoff",
        }
    }

    const fn default_availability(self) -> HelpAboutActionAvailability {
        match self {
            Self::OpenExecutionContextInspector | Self::CopyContextForSupportExport => {
                HelpAboutActionAvailability::Live
            }
            Self::OpenReleasePacket
            | Self::ViewProvenanceDetails
            | Self::OpenAdvisoryHistory
            | Self::ReportIssueViaCommunityHandoff => {
                HelpAboutActionAvailability::ReservedForLaterMilestone
            }
        }
    }
}

/// Availability class rendered on every action row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HelpAboutActionAvailability {
    /// Live within the seed.
    Live,
    /// Reserved for a later milestone; the surface labels the row so the
    /// user can see the lane exists but cannot run it yet.
    ReservedForLaterMilestone,
    /// Resolved execution-context carries a degraded field that prevents
    /// safe action; the surface holds the row visible but disabled.
    BlockedByDegradedContext,
    /// Workspace trust posture is unresolved; live work is withheld until
    /// the trust prompt is settled.
    BlockedByPendingTrust,
}

impl HelpAboutActionAvailability {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::ReservedForLaterMilestone => "reserved_for_later_milestone",
            Self::BlockedByDegradedContext => "blocked_by_degraded_context",
            Self::BlockedByPendingTrust => "blocked_by_pending_trust",
        }
    }

    /// Human-readable label for the chip.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Live => "Live",
            Self::ReservedForLaterMilestone => "Reserved for a later milestone",
            Self::BlockedByDegradedContext => "Blocked: degraded context",
            Self::BlockedByPendingTrust => "Blocked: trust pending",
        }
    }
}

/// Service-health row classes rendered as seed placeholders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceHealthRowClass {
    /// Local runtime-execution lane (PTY host, runtime resolver, capsule
    /// drift detector).
    LocalRuntimeHealth,
    /// Auth subsystem (system-browser callback, credential state).
    AuthSubsystemHealth,
    /// Docs / help subsystem freshness aggregator.
    DocsHelpSubsystemHealth,
    /// Update / release-channel feed reachability.
    UpdateChannelHealth,
}

impl ServiceHealthRowClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalRuntimeHealth => "local_runtime_health",
            Self::AuthSubsystemHealth => "auth_subsystem_health",
            Self::DocsHelpSubsystemHealth => "docs_help_subsystem_health",
            Self::UpdateChannelHealth => "update_channel_health",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalRuntimeHealth => "Local runtime",
            Self::AuthSubsystemHealth => "Auth subsystem",
            Self::DocsHelpSubsystemHealth => "Docs and help",
            Self::UpdateChannelHealth => "Update channel",
        }
    }
}

/// Service-health state vocabulary. Every row in the initial seed defaults to
/// [`ServiceHealthState::SeedPlaceholderAwaitingWiring`]; the wider state
/// vocabulary is reserved so support exports and the chrome agree on stable
/// tokens before the live aggregator lands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceHealthState {
    SeedPlaceholderAwaitingWiring,
    Healthy,
    Degraded,
    Unavailable,
    StaleSnapshot,
}

impl ServiceHealthState {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SeedPlaceholderAwaitingWiring => "seed_placeholder_awaiting_wiring",
            Self::Healthy => "healthy",
            Self::Degraded => "degraded",
            Self::Unavailable => "unavailable",
            Self::StaleSnapshot => "stale_snapshot",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SeedPlaceholderAwaitingWiring => "Seed placeholder (wiring pending)",
            Self::Healthy => "Healthy",
            Self::Degraded => "Degraded",
            Self::Unavailable => "Unavailable",
            Self::StaleSnapshot => "Stale snapshot",
        }
    }
}

/// Provenance row classes seeded as placeholders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceRowClass {
    SignatureState,
    AttestationState,
    ChecksumState,
    SbomState,
    AdvisoryOpenState,
}

impl ProvenanceRowClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignatureState => "signature_state",
            Self::AttestationState => "attestation_state",
            Self::ChecksumState => "checksum_state",
            Self::SbomState => "sbom_state",
            Self::AdvisoryOpenState => "advisory_open_state",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SignatureState => "Signature",
            Self::AttestationState => "Attestation",
            Self::ChecksumState => "Checksum",
            Self::SbomState => "SBOM",
            Self::AdvisoryOpenState => "Open advisories",
        }
    }
}

/// Provenance row state vocabulary. The initial seed renders every row with
/// [`ProvenanceRowState::SeedPlaceholderAwaitingWiring`]; the broader
/// vocabulary is reserved for about-card hardening so support exports and
/// the chrome agree on stable tokens before the live verifier lands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceRowState {
    SeedPlaceholderAwaitingWiring,
    SignedVerified,
    AttestationVerified,
    ChecksumVerified,
    SbomAttachedVerified,
    NoOpenAdvisories,
    NotVerifiedThisSeed,
}

impl ProvenanceRowState {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SeedPlaceholderAwaitingWiring => "seed_placeholder_awaiting_wiring",
            Self::SignedVerified => "signed_verified",
            Self::AttestationVerified => "attestation_verified",
            Self::ChecksumVerified => "checksum_verified",
            Self::SbomAttachedVerified => "sbom_attached_verified",
            Self::NoOpenAdvisories => "no_open_advisories",
            Self::NotVerifiedThisSeed => "not_verified_this_seed",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SeedPlaceholderAwaitingWiring => "Seed placeholder (wiring pending)",
            Self::SignedVerified => "Signed and verified",
            Self::AttestationVerified => "Attestation verified",
            Self::ChecksumVerified => "Checksum verified",
            Self::SbomAttachedVerified => "SBOM attached and verified",
            Self::NoOpenAdvisories => "No open advisories",
            Self::NotVerifiedThisSeed => "Not verified by this seed",
        }
    }
}

/// Frozen community-handoff route vocabulary. Mirrors the route classes in
/// `schemas/about/about_card.schema.json#community_handoff_route_class` so
/// the seed and the about-card contract render the same row ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommunityHandoffRouteClass {
    PublicIssueTracker,
    PublicRfcForum,
    PrivateSecurityChannel,
    PrivateSupportChannel,
}

impl CommunityHandoffRouteClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicIssueTracker => "public_issue_tracker",
            Self::PublicRfcForum => "public_rfc_forum",
            Self::PrivateSecurityChannel => "private_security_channel",
            Self::PrivateSupportChannel => "private_support_channel",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::PublicIssueTracker => "Public issue tracker",
            Self::PublicRfcForum => "Public RFC forum",
            Self::PrivateSecurityChannel => "Private security channel",
            Self::PrivateSupportChannel => "Private support channel",
        }
    }

    /// Stable disclosure copy describing the lane.
    pub const fn disclosure(self) -> &'static str {
        match self {
            Self::PublicIssueTracker => {
                "Public issue tracker for OSS bugs, performance regressions, docs-truth defects, \
                 compatibility regressions, and accessibility defects."
            }
            Self::PublicRfcForum => {
                "Public RFC pull-request lane for design proposals and design-review threads."
            }
            Self::PrivateSecurityChannel => {
                "Private security intake under the published PGP key. Public advisory follows \
                 after fix-and-disclosure cadence; raw exploit payloads are only allowed on this \
                 lane."
            }
            Self::PrivateSupportChannel => {
                "Private support intake for live-device, account, or workspace content. \
                 Sanitised summaries may surface publicly after fix."
            }
        }
    }

    /// Destination trust class token disclosed before navigation.
    pub const fn destination_trust_class_token(self) -> &'static str {
        match self {
            Self::PublicIssueTracker => "official_public",
            Self::PublicRfcForum => "community",
            Self::PrivateSecurityChannel | Self::PrivateSupportChannel => "official_authenticated",
        }
    }

    /// Authentication expectation disclosed before navigation.
    pub const fn auth_expectation(self) -> &'static str {
        match self {
            Self::PublicIssueTracker => {
                "no sign-in required to view; sign-in may be required to comment"
            }
            Self::PublicRfcForum => "public forum account may be required",
            Self::PrivateSecurityChannel => "security intake identity required",
            Self::PrivateSupportChannel => "support identity required",
        }
    }

    /// Data-exit boundary disclosed before navigation.
    pub const fn data_exit_boundary(self) -> &'static str {
        match self {
            Self::PublicIssueTracker => {
                "metadata-safe object refs may leave the product after review"
            }
            Self::PublicRfcForum => {
                "proposal refs only; local diagnostics are not attached automatically"
            }
            Self::PrivateSecurityChannel => {
                "security payloads leave only through the private security lane"
            }
            Self::PrivateSupportChannel => {
                "redacted support packet leaves only after local preview"
            }
        }
    }

    /// Issue template ref attached to the handoff route.
    pub const fn issue_template_ref(self) -> &'static str {
        match self {
            Self::PublicIssueTracker => "issue-template:docs-or-compatibility-public",
            Self::PublicRfcForum => "issue-template:public-rfc-proposal",
            Self::PrivateSecurityChannel => "issue-template:private-security-intake",
            Self::PrivateSupportChannel => "issue-template:private-support-intake",
        }
    }
}

/// Inputs the surface needs to project one record. Every field comes from an
/// upstream truth source the help/about lane reuses; the projection never
/// invents build, target, or freshness truth of its own.
#[derive(Debug, Clone)]
pub struct HelpAboutInputs<'a> {
    /// Build-identity record minted at compile time by the build-info crate.
    pub build_identity: &'a BuildIdentityRecord,
    /// Stable release-channel-class token (e.g. `dev_local`, `nightly`,
    /// `stable`). Comes from
    /// [`aureline_build_info::release_channel_class`].
    pub release_channel_class_token: &'a str,
    /// Optional resolved execution context. When present, the client-scope
    /// section projects a [`TargetOriginBadge`] from it; when absent, the
    /// section degrades to a typed honesty marker rather than fabricating a
    /// "Local — All clear" chip.
    pub execution_context: Option<&'a ExecutionContext>,
    /// Optional docs/help source-truth row. When present, the docs/help
    /// section quotes its source class, version match, freshness, and
    /// snapshot-age label. When absent, the section flags the seed
    /// placeholder honestly.
    pub docs_source_truth: Option<&'a SourceTruthRecord>,
}

/// Build-identity section.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpAboutBuildIdentitySection {
    pub product_name_class: String,
    pub workspace_version: String,
    pub release_channel_class_token: String,
    pub exact_build_identity_ref: String,
    pub commit: String,
    pub commit_short: String,
    pub tree_state_class: TreeStateClass,
    pub tree_state_class_token: String,
    pub tree_state_label: String,
    pub host_triple: String,
    pub target_triple: String,
    pub profile: String,
    pub rustc_version: String,
    pub cargo_version: String,
    pub toolchain_channel: String,
    pub source_date_epoch: i64,
    pub build_timestamp_utc: String,
}

/// Install-mode section.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpAboutInstallModeSection {
    pub install_mode_class: InstallModeClass,
    pub install_mode_token: String,
    pub install_mode_label: String,
    pub channel_class_token: String,
    pub tree_state_class: TreeStateClass,
    pub tree_state_token: String,
    pub tree_state_label: String,
    /// True when [`Self::install_mode_class`] is
    /// [`InstallModeClass::UnknownInstallMode`]; the chrome MUST surface a
    /// visible honesty chip when this is true.
    pub honesty_marker_present: bool,
}

/// Client-scope section. Reuses the shared
/// [`crate::badges::target_origin::TargetOriginBadge`] vocabulary so the
/// help/about surface, the terminal pane, the task seed, and the debug-prep
/// seed all read the same target/origin tokens.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpAboutClientScopeSection {
    /// Projected target/origin badge when an execution context is present.
    /// `None` when the lane is rendered before workspace bootstrap settles a
    /// context and the row is left as a typed honesty marker instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub badge: Option<TargetOriginBadge>,
    pub target_class: TargetBadgeClass,
    pub target_class_token: String,
    pub target_label: String,
    pub origin_class: OriginBadgeClass,
    pub origin_class_token: String,
    pub origin_label: String,
    pub boundary_cue: HostBoundaryCue,
    pub boundary_cue_token: String,
    pub boundary_cue_label: String,
    pub boundary_cue_visible: bool,
    pub trust_state: TrustState,
    pub trust_state_token: String,
    pub identity_mode: IdentityMode,
    pub identity_mode_token: String,
    pub honesty_marker_present: bool,
    /// True when no execution context was wired into the seed and the row
    /// degraded to its typed placeholder.
    pub context_missing: bool,
}

/// Docs/help truth section. Projects the source / version / freshness
/// vocabulary already minted by the embedded docs/help boundary card so the
/// help shell never forks a private freshness ladder.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpAboutDocsHelpTruthSection {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_class: Option<SourceClass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_match_state: Option<VersionMatchState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub freshness_class: Option<FreshnessClass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_age_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub running_build_identity_ref: Option<String>,
    pub source_class_token: String,
    pub version_match_token: String,
    pub freshness_class_token: String,
    pub honesty_marker_present: bool,
    pub source_missing: bool,
}

/// One service-health row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpAboutServiceHealthRow {
    pub row_class: ServiceHealthRowClass,
    pub row_class_token: String,
    pub label: String,
    pub state: ServiceHealthState,
    pub state_token: String,
    pub state_label: String,
}

/// Service-health section: typed seed-placeholder rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpAboutServiceHealthSection {
    pub rows: Vec<HelpAboutServiceHealthRow>,
    pub honesty_marker_present: bool,
}

/// One provenance row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpAboutProvenanceRow {
    pub row_class: ProvenanceRowClass,
    pub row_class_token: String,
    pub label: String,
    pub state: ProvenanceRowState,
    pub state_token: String,
    pub state_label: String,
}

/// Provenance section: typed seed-placeholder rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpAboutProvenanceSection {
    pub rows: Vec<HelpAboutProvenanceRow>,
    pub honesty_marker_present: bool,
}

/// One community-handoff row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpAboutCommunityHandoffRow {
    pub route_class: CommunityHandoffRouteClass,
    pub route_class_token: String,
    pub label: String,
    pub disclosure: String,
    pub destination_trust_class_token: String,
    pub auth_expectation: String,
    pub data_exit_boundary: String,
    pub issue_template_ref: String,
}

/// Community-handoff section: stable route vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpAboutCommunityHandoffSection {
    pub rows: Vec<HelpAboutCommunityHandoffRow>,
}

/// One action row on the seed surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpAboutAction {
    pub action_class: HelpAboutActionClass,
    pub action_class_token: String,
    pub label: String,
    pub availability: HelpAboutActionAvailability,
    pub availability_token: String,
    pub availability_label: String,
}

impl HelpAboutAction {
    fn build(
        action_class: HelpAboutActionClass,
        availability: HelpAboutActionAvailability,
    ) -> Self {
        Self {
            action_class,
            action_class_token: action_class.as_str().to_owned(),
            label: action_class.label().to_owned(),
            availability,
            availability_token: availability.as_str().to_owned(),
            availability_label: availability.label().to_owned(),
        }
    }
}

/// Help / About / provenance / service-health seed surface record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpAboutSurface {
    pub record_kind: String,
    pub schema_version: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_ref: Option<String>,
    pub build_identity: HelpAboutBuildIdentitySection,
    pub install_mode: HelpAboutInstallModeSection,
    pub client_scope: HelpAboutClientScopeSection,
    pub docs_help_truth: HelpAboutDocsHelpTruthSection,
    pub service_health: HelpAboutServiceHealthSection,
    pub provenance: HelpAboutProvenanceSection,
    pub community_handoff: HelpAboutCommunityHandoffSection,
    pub actions: Vec<HelpAboutAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release_truth_card: Option<HelpAboutReleaseTruthCard>,
    pub seed_scope_notice: String,
    pub honesty_marker_present: bool,
}

impl HelpAboutSurface {
    /// Project a help/about surface from the named upstream inputs.
    pub fn project(inputs: HelpAboutInputs<'_>) -> Self {
        let HelpAboutInputs {
            build_identity,
            release_channel_class_token,
            execution_context,
            docs_source_truth,
        } = inputs;

        let build_identity_section = project_build_identity(build_identity);
        let install_mode_section =
            project_install_mode(release_channel_class_token, build_identity);
        let client_scope_section = project_client_scope(execution_context);
        let docs_help_truth_section = project_docs_help_truth(docs_source_truth);
        let service_health_section = project_service_health();
        let provenance_section = project_provenance();
        let community_handoff_section = project_community_handoff();

        let trust_pending = execution_context
            .map(|context| {
                matches!(
                    context.policy_and_trust.trust_state,
                    TrustState::PendingEvaluation
                )
            })
            .unwrap_or(false);
        let context_degraded = execution_context
            .map(|context| !context.degraded_fields.is_empty())
            .unwrap_or(false);

        let actions = build_actions(trust_pending, context_degraded);

        let workspace_id =
            execution_context.map(|context| context.invocation_subject.workspace_id.clone());
        let execution_context_ref =
            execution_context.map(|context| context.execution_context_id.clone());

        let honesty_marker_present = install_mode_section.honesty_marker_present
            || client_scope_section.honesty_marker_present
            || docs_help_truth_section.honesty_marker_present
            || service_health_section.honesty_marker_present
            || provenance_section.honesty_marker_present;

        Self {
            record_kind: HELP_ABOUT_SURFACE_RECORD_KIND.to_owned(),
            schema_version: HELP_ABOUT_SURFACE_SCHEMA_VERSION,
            workspace_id,
            execution_context_ref,
            build_identity: build_identity_section,
            install_mode: install_mode_section,
            client_scope: client_scope_section,
            docs_help_truth: docs_help_truth_section,
            service_health: service_health_section,
            provenance: provenance_section,
            community_handoff: community_handoff_section,
            actions,
            release_truth_card: None,
            seed_scope_notice: HELP_ABOUT_SEED_SCOPE_NOTICE.to_owned(),
            honesty_marker_present,
        }
    }

    /// Project a help/about surface and attach the current release-truth card.
    pub fn project_with_release_truth(
        inputs: HelpAboutInputs<'_>,
        release_truth_card: HelpAboutReleaseTruthCard,
    ) -> Self {
        Self::project(inputs).with_release_truth_card(release_truth_card)
    }

    /// Attach the release-truth card and activate community-handoff routing.
    pub fn with_release_truth_card(
        mut self,
        release_truth_card: HelpAboutReleaseTruthCard,
    ) -> Self {
        self.honesty_marker_present |= release_truth_card.honesty_marker_present;
        for action in &mut self.actions {
            if action.action_class == HelpAboutActionClass::ReportIssueViaCommunityHandoff {
                action.availability = HelpAboutActionAvailability::Live;
                action.availability_token = action.availability.as_str().to_owned();
                action.availability_label = action.availability.label().to_owned();
            }
        }
        self.release_truth_card = Some(release_truth_card);
        self
    }

    /// Render a deterministic plaintext block for the copy-context action and
    /// support exports. The block is stable across runs for the same input
    /// snapshot.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str("Help / About surface\n");
        if let Some(workspace_id) = &self.workspace_id {
            out.push_str(&format!("Workspace: {workspace_id}\n"));
        }
        if let Some(context_ref) = &self.execution_context_ref {
            out.push_str(&format!("Execution context: {context_ref}\n"));
        }
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
            "[{}]\n  Product: {}\n  Version: {}\n  Channel: {}\n  Exact build: {}\n  Commit: {} (full: {})\n  Tree state: {} ({})\n  Host: {}\n  Target: {}\n  Profile: {}\n  Toolchain: rustc {} on channel {} (cargo {})\n  Built: {} (source-date-epoch {})\n\n",
            HelpAboutSectionId::BuildIdentity.heading(),
            self.build_identity.product_name_class,
            self.build_identity.workspace_version,
            self.build_identity.release_channel_class_token,
            self.build_identity.exact_build_identity_ref,
            self.build_identity.commit_short,
            self.build_identity.commit,
            self.build_identity.tree_state_label,
            self.build_identity.tree_state_class_token,
            self.build_identity.host_triple,
            self.build_identity.target_triple,
            self.build_identity.profile,
            self.build_identity.rustc_version,
            self.build_identity.toolchain_channel,
            self.build_identity.cargo_version,
            self.build_identity.build_timestamp_utc,
            self.build_identity.source_date_epoch,
        ));

        out.push_str(&format!(
            "[{}]\n  Mode: {} ({})\n  Channel token: {}\n  Tree state: {} ({})\n\n",
            HelpAboutSectionId::InstallMode.heading(),
            self.install_mode.install_mode_label,
            self.install_mode.install_mode_token,
            self.install_mode.channel_class_token,
            self.install_mode.tree_state_label,
            self.install_mode.tree_state_token,
        ));

        out.push_str(&format!(
            "[{}]\n  Target: {} ({})\n  Origin: {} ({})\n  Boundary cue: {} (visible: {})\n  Trust: {}\n  Identity mode: {}\n  Context wired: {}\n\n",
            HelpAboutSectionId::ClientScope.heading(),
            self.client_scope.target_label,
            self.client_scope.target_class_token,
            self.client_scope.origin_label,
            self.client_scope.origin_class_token,
            self.client_scope.boundary_cue_label,
            self.client_scope.boundary_cue_visible,
            self.client_scope.trust_state_token,
            self.client_scope.identity_mode_token,
            !self.client_scope.context_missing,
        ));

        out.push_str(&format!(
            "[{}]\n",
            HelpAboutSectionId::DocsHelpTruth.heading()
        ));
        if self.docs_help_truth.source_missing {
            out.push_str("  (no upstream docs-help source wired; seed placeholder)\n");
        } else {
            out.push_str(&format!(
                "  Source class: {}\n  Version match: {}\n  Freshness: {}\n",
                self.docs_help_truth.source_class_token,
                self.docs_help_truth.version_match_token,
                self.docs_help_truth.freshness_class_token,
            ));
            if let Some(label) = &self.docs_help_truth.snapshot_age_label {
                out.push_str(&format!("  Snapshot age: {label}\n"));
            }
            if let Some(refed) = &self.docs_help_truth.running_build_identity_ref {
                out.push_str(&format!("  Running build identity ref: {refed}\n"));
            }
        }
        out.push('\n');

        out.push_str(&format!(
            "[{}]\n",
            HelpAboutSectionId::ServiceHealth.heading()
        ));
        for row in &self.service_health.rows {
            out.push_str(&format!(
                "  - {}: {} [{}]\n",
                row.row_class_token, row.label, row.state_token,
            ));
        }
        out.push('\n');

        if let Some(card) = &self.release_truth_card {
            out.push_str("[Release truth]\n");
            for line in card.render_plaintext().lines() {
                out.push_str("  ");
                out.push_str(line);
                out.push('\n');
            }
            out.push('\n');
        }

        out.push_str(&format!("[{}]\n", HelpAboutSectionId::Provenance.heading()));
        for row in &self.provenance.rows {
            out.push_str(&format!(
                "  - {}: {} [{}]\n",
                row.row_class_token, row.label, row.state_token,
            ));
        }
        out.push('\n');

        out.push_str(&format!(
            "[{}]\n",
            HelpAboutSectionId::CommunityHandoff.heading()
        ));
        for row in &self.community_handoff.rows {
            out.push_str(&format!(
                "  - {}: {}\n      {}\n      trust={} auth={} boundary={} template={}\n",
                row.route_class_token,
                row.label,
                row.disclosure,
                row.destination_trust_class_token,
                row.auth_expectation,
                row.data_exit_boundary,
                row.issue_template_ref,
            ));
        }
        out.push('\n');

        out.push_str("Actions:\n");
        for action in &self.actions {
            out.push_str(&format!(
                "  - {}: {} [{}]\n",
                action.action_class_token, action.label, action.availability_token,
            ));
        }
        out.push('\n');
        out.push_str(&format!("Notice: {}\n", self.seed_scope_notice));
        out
    }
}

fn project_build_identity(record: &BuildIdentityRecord) -> HelpAboutBuildIdentitySection {
    let tree_state = if record.dirty {
        TreeStateClass::DirtyLocal
    } else {
        TreeStateClass::CleanCheckout
    };
    let exact_build_identity_ref = aureline_build_info::exact_build_identity_ref();
    let release_channel_class_token = aureline_build_info::release_channel_class().to_owned();

    HelpAboutBuildIdentitySection {
        product_name_class: "aureline".to_owned(),
        workspace_version: record.workspace_version.clone(),
        release_channel_class_token,
        exact_build_identity_ref,
        commit: record.commit.clone(),
        commit_short: record.commit_short.clone(),
        tree_state_class: tree_state,
        tree_state_class_token: tree_state.as_str().to_owned(),
        tree_state_label: tree_state.label().to_owned(),
        host_triple: record.host_triple.clone(),
        target_triple: record.target_triple.clone(),
        profile: record.profile.clone(),
        rustc_version: record.rustc_version.clone(),
        cargo_version: record.cargo_version.clone(),
        toolchain_channel: record.toolchain_channel.clone(),
        source_date_epoch: record.source_date_epoch,
        build_timestamp_utc: record.build_timestamp_utc.clone(),
    }
}

fn project_install_mode(
    channel_token: &str,
    record: &BuildIdentityRecord,
) -> HelpAboutInstallModeSection {
    let install_mode_class = InstallModeClass::from_channel_token(channel_token);
    let tree_state = if record.dirty {
        TreeStateClass::DirtyLocal
    } else {
        TreeStateClass::CleanCheckout
    };
    let honesty_marker_present = matches!(install_mode_class, InstallModeClass::UnknownInstallMode);

    HelpAboutInstallModeSection {
        install_mode_class,
        install_mode_token: install_mode_class.as_str().to_owned(),
        install_mode_label: install_mode_class.label().to_owned(),
        channel_class_token: channel_token.to_owned(),
        tree_state_class: tree_state,
        tree_state_token: tree_state.as_str().to_owned(),
        tree_state_label: tree_state.label().to_owned(),
        honesty_marker_present,
    }
}

fn project_client_scope(context: Option<&ExecutionContext>) -> HelpAboutClientScopeSection {
    match context {
        Some(context) => {
            let badge = TargetOriginBadge::project(BadgeEntryPoint::Terminal, context);
            HelpAboutClientScopeSection {
                target_class: badge.target_class,
                target_class_token: badge.target_class_token.clone(),
                target_label: badge.target_label.clone(),
                origin_class: badge.origin_class,
                origin_class_token: badge.origin_class_token.clone(),
                origin_label: badge.origin_label.clone(),
                boundary_cue: badge.boundary_cue,
                boundary_cue_token: badge.boundary_cue_token.clone(),
                boundary_cue_label: badge.boundary_cue_label.clone(),
                boundary_cue_visible: badge.boundary_cue_visible,
                trust_state: context.policy_and_trust.trust_state,
                trust_state_token: trust_token(context.policy_and_trust.trust_state).to_owned(),
                identity_mode: context.policy_and_trust.identity_mode,
                identity_mode_token: context.policy_and_trust.identity_mode.as_str().to_owned(),
                honesty_marker_present: badge.honesty_marker_present
                    || matches!(
                        context.policy_and_trust.trust_state,
                        TrustState::PendingEvaluation
                    )
                    || context_has_non_trust_degraded(context),
                context_missing: false,
                badge: Some(badge),
            }
        }
        None => {
            // The lane has no resolved execution context (e.g. opened from
            // the start center before workspace bootstrap). Render typed
            // honesty placeholders rather than fabricating a "Local — All
            // clear" chip.
            HelpAboutClientScopeSection {
                badge: None,
                target_class: TargetBadgeClass::LocalDesktop,
                target_class_token: TargetBadgeClass::LocalDesktop.as_str().to_owned(),
                target_label: "Local desktop (seed default)".to_owned(),
                origin_class: OriginBadgeClass::AccountFreeLocal,
                origin_class_token: OriginBadgeClass::AccountFreeLocal.as_str().to_owned(),
                origin_label: "Local only (seed default)".to_owned(),
                boundary_cue: HostBoundaryCue::Unknown,
                boundary_cue_token: HostBoundaryCue::Unknown.as_str().to_owned(),
                boundary_cue_label: HostBoundaryCue::Unknown.label().to_owned(),
                boundary_cue_visible: HostBoundaryCue::Unknown.is_visible(),
                trust_state: TrustState::PendingEvaluation,
                trust_state_token: trust_token(TrustState::PendingEvaluation).to_owned(),
                identity_mode: IdentityMode::AccountFreeLocal,
                identity_mode_token: IdentityMode::AccountFreeLocal.as_str().to_owned(),
                honesty_marker_present: true,
                context_missing: true,
            }
        }
    }
}

fn context_has_non_trust_degraded(context: &ExecutionContext) -> bool {
    context
        .degraded_fields
        .iter()
        .any(|record| !matches!(record.reason, DegradedFieldReason::TrustStateUnresolved))
}

fn project_docs_help_truth(source: Option<&SourceTruthRecord>) -> HelpAboutDocsHelpTruthSection {
    match source {
        Some(truth) => {
            let honesty_marker_present = !matches!(
                truth.freshness_class,
                FreshnessClass::AuthoritativeLive | FreshnessClass::WarmCached
            ) || !matches!(
                truth.version_match_state,
                VersionMatchState::ExactBuildMatch | VersionMatchState::CompatibleMinorDrift
            );
            HelpAboutDocsHelpTruthSection {
                source_class: Some(truth.source_class),
                version_match_state: Some(truth.version_match_state),
                freshness_class: Some(truth.freshness_class),
                snapshot_age_label: truth.snapshot_age_label.clone(),
                running_build_identity_ref: Some(truth.running_build_identity_ref.clone()),
                source_class_token: source_class_token(truth.source_class).to_owned(),
                version_match_token: version_match_token(truth.version_match_state).to_owned(),
                freshness_class_token: freshness_class_token(truth.freshness_class).to_owned(),
                honesty_marker_present,
                source_missing: false,
            }
        }
        None => HelpAboutDocsHelpTruthSection {
            source_class: None,
            version_match_state: None,
            freshness_class: None,
            snapshot_age_label: None,
            running_build_identity_ref: None,
            source_class_token: "seed_placeholder_awaiting_wiring".to_owned(),
            version_match_token: "seed_placeholder_awaiting_wiring".to_owned(),
            freshness_class_token: "seed_placeholder_awaiting_wiring".to_owned(),
            honesty_marker_present: true,
            source_missing: true,
        },
    }
}

fn project_service_health() -> HelpAboutServiceHealthSection {
    let row_classes = [
        ServiceHealthRowClass::LocalRuntimeHealth,
        ServiceHealthRowClass::AuthSubsystemHealth,
        ServiceHealthRowClass::DocsHelpSubsystemHealth,
        ServiceHealthRowClass::UpdateChannelHealth,
    ];
    let rows = row_classes
        .into_iter()
        .map(|class| HelpAboutServiceHealthRow {
            row_class: class,
            row_class_token: class.as_str().to_owned(),
            label: class.label().to_owned(),
            state: ServiceHealthState::SeedPlaceholderAwaitingWiring,
            state_token: ServiceHealthState::SeedPlaceholderAwaitingWiring
                .as_str()
                .to_owned(),
            state_label: ServiceHealthState::SeedPlaceholderAwaitingWiring
                .label()
                .to_owned(),
        })
        .collect();
    HelpAboutServiceHealthSection {
        rows,
        // Seed placeholders are in-spec here; they do not light
        // the global honesty marker on their own. The marker fires only when
        // an upstream truth source actively degrades.
        honesty_marker_present: false,
    }
}

fn project_provenance() -> HelpAboutProvenanceSection {
    let row_classes = [
        ProvenanceRowClass::SignatureState,
        ProvenanceRowClass::AttestationState,
        ProvenanceRowClass::ChecksumState,
        ProvenanceRowClass::SbomState,
        ProvenanceRowClass::AdvisoryOpenState,
    ];
    let rows = row_classes
        .into_iter()
        .map(|class| HelpAboutProvenanceRow {
            row_class: class,
            row_class_token: class.as_str().to_owned(),
            label: class.label().to_owned(),
            state: ProvenanceRowState::SeedPlaceholderAwaitingWiring,
            state_token: ProvenanceRowState::SeedPlaceholderAwaitingWiring
                .as_str()
                .to_owned(),
            state_label: ProvenanceRowState::SeedPlaceholderAwaitingWiring
                .label()
                .to_owned(),
        })
        .collect();
    HelpAboutProvenanceSection {
        rows,
        honesty_marker_present: false,
    }
}

fn project_community_handoff() -> HelpAboutCommunityHandoffSection {
    let route_classes = [
        CommunityHandoffRouteClass::PublicIssueTracker,
        CommunityHandoffRouteClass::PublicRfcForum,
        CommunityHandoffRouteClass::PrivateSecurityChannel,
        CommunityHandoffRouteClass::PrivateSupportChannel,
    ];
    let rows = route_classes
        .into_iter()
        .map(|class| HelpAboutCommunityHandoffRow {
            route_class: class,
            route_class_token: class.as_str().to_owned(),
            label: class.label().to_owned(),
            disclosure: class.disclosure().to_owned(),
            destination_trust_class_token: class.destination_trust_class_token().to_owned(),
            auth_expectation: class.auth_expectation().to_owned(),
            data_exit_boundary: class.data_exit_boundary().to_owned(),
            issue_template_ref: class.issue_template_ref().to_owned(),
        })
        .collect();
    HelpAboutCommunityHandoffSection { rows }
}

fn build_actions(trust_pending: bool, context_degraded: bool) -> Vec<HelpAboutAction> {
    let action_classes = [
        HelpAboutActionClass::OpenExecutionContextInspector,
        HelpAboutActionClass::CopyContextForSupportExport,
        HelpAboutActionClass::OpenReleasePacket,
        HelpAboutActionClass::ViewProvenanceDetails,
        HelpAboutActionClass::OpenAdvisoryHistory,
        HelpAboutActionClass::ReportIssueViaCommunityHandoff,
    ];
    action_classes
        .into_iter()
        .map(|class| {
            let availability = adjust_availability(class, trust_pending, context_degraded);
            HelpAboutAction::build(class, availability)
        })
        .collect()
}

fn adjust_availability(
    class: HelpAboutActionClass,
    trust_pending: bool,
    context_degraded: bool,
) -> HelpAboutActionAvailability {
    let default = class.default_availability();
    if matches!(
        default,
        HelpAboutActionAvailability::ReservedForLaterMilestone
    ) {
        return default;
    }
    // Copying the seed surface payload for a support export must remain live
    // even when the resolved context is degraded — that is precisely when
    // support packets need the truth-source dump.
    if matches!(class, HelpAboutActionClass::CopyContextForSupportExport) {
        return HelpAboutActionAvailability::Live;
    }
    if matches!(class, HelpAboutActionClass::OpenExecutionContextInspector) {
        if trust_pending {
            return HelpAboutActionAvailability::BlockedByPendingTrust;
        }
        if context_degraded {
            return HelpAboutActionAvailability::BlockedByDegradedContext;
        }
    }
    HelpAboutActionAvailability::Live
}

const fn trust_token(state: TrustState) -> &'static str {
    match state {
        TrustState::Trusted => "trusted",
        TrustState::Restricted => "restricted",
        TrustState::PendingEvaluation => "pending_evaluation",
    }
}

const fn source_class_token(class: SourceClass) -> &'static str {
    match class {
        SourceClass::ProjectDocs => "project_docs",
        SourceClass::GeneratedReference => "generated_reference",
        SourceClass::MirroredOfficialDocs => "mirrored_official_docs",
        SourceClass::CuratedKnowledgePack => "curated_knowledge_pack",
        SourceClass::DerivedExplanation => "derived_explanation",
        SourceClass::VendorProviderDocs => "vendor_provider_docs",
        SourceClass::SupportRunbook => "support_runbook",
        SourceClass::ExternalStatusFeed => "external_status_feed",
    }
}

const fn version_match_token(state: VersionMatchState) -> &'static str {
    match state {
        VersionMatchState::ExactBuildMatch => "exact_build_match",
        VersionMatchState::CompatibleMinorDrift => "compatible_minor_drift",
        VersionMatchState::IncompatibleDriftDetected => "incompatible_drift_detected",
        VersionMatchState::PreReleaseUnverified => "pre_release_unverified",
        VersionMatchState::UnknownTargetBuild => "unknown_target_build",
    }
}

const fn freshness_class_token(class: FreshnessClass) -> &'static str {
    match class {
        FreshnessClass::AuthoritativeLive => "authoritative_live",
        FreshnessClass::WarmCached => "warm_cached",
        FreshnessClass::DegradedCached => "degraded_cached",
        FreshnessClass::Stale => "stale",
        FreshnessClass::Unverified => "unverified",
    }
}

#[allow(dead_code)]
const fn target_is_remote_or_managed(class: TargetClass) -> bool {
    !matches!(class, TargetClass::LocalHost)
}

#[allow(dead_code)]
fn degraded_record_is_trust(record: &DegradedFieldRecord) -> bool {
    matches!(record.reason, DegradedFieldReason::TrustStateUnresolved)
}

#[cfg(test)]
mod tests;
