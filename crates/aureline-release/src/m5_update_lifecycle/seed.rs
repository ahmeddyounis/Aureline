//! Canonical seed builders for the M5 update / support-lifecycle governance matrix.
//!
//! These builders are the single producer of the checked-in governance packet, the published
//! inventory, the rendered governance document, the machine-readable matrix CSV, the release-grade
//! parity proof (and its Markdown report), and the stale / missing drill fixtures. The headless
//! emitter and the inline tests both call them so the in-code packet, the artifacts, and the
//! fixtures never drift. Every consumer's verdict is derived from the same governed facets: the
//! canonical packet certifies every facet current and every lifecycle state governed, so every
//! consumer stands fully certified at Stable; the drills perturb one facet's proof freshness and
//! let the derivation recompute each consumer's status, gate, effective qualification, and gaps.

use super::*;

/// Stable packet id for the canonical (all-current) governance packet.
pub const M5_UPDATE_LIFECYCLE_PACKET_ID: &str = "m5-update-lifecycle:stable:0001";

/// Evaluation / mint timestamp for the canonical packet — a date at which every facet's proof is
/// current.
const SEED_EVALUATED_AT: &str = "2026-07-06T00:00:00Z";

const REDACTION_CLASS: &str = "metadata_safe_default";

/// The facet the stale drill perturbs. It is read by the change-disclosure and diagnostic
/// consumers but not Help/About, docs, or companion, so the drill narrows exactly the consumers
/// that depend on the change-impact forecast.
const STALE_DRILL_FACET: LifecycleFacet = LifecycleFacet::ChangeImpact;

/// The facet the missing drill perturbs. It is read by every continuity consumer (release center,
/// update center, Help/About, diagnostics, support exports, companion handoffs), so the drill
/// blocks exactly the consumers that depend on the service-health banner.
const MISSING_DRILL_FACET: LifecycleFacet = LifecycleFacet::ServiceHealth;

/// The governed facet definitions: each facet's current canonical state, the artifact classes it
/// discloses, the claimed channels and profiles it scopes to, and its stale-data behavior. Every
/// baseline state is governed, so the canonical packet certifies every consumer at Stable.
#[allow(clippy::type_complexity)]
const FACET_DEFS: [(
    LifecycleFacet,
    CanonicalState,
    &[ArtifactClass],
    &[ChannelScope],
    &[DeploymentProfile],
    StaleDataBehavior,
); 8] = [
    (
        LifecycleFacet::UpdateAvailability,
        CanonicalState::Update(UpdateState::UpToDate),
        &[ArtifactClass::CoreRuntime, ArtifactClass::ExtensionPacks],
        &[
            ChannelScope::Stable,
            ChannelScope::Beta,
            ChannelScope::Preview,
            ChannelScope::Nightly,
            ChannelScope::Lts,
        ],
        &[DeploymentProfile::Managed, DeploymentProfile::SelfHosted],
        StaleDataBehavior::MirroredLabelled,
    ),
    (
        LifecycleFacet::ChangeImpact,
        CanonicalState::Readiness(ReadinessState::RestartRequired),
        &[
            ArtifactClass::CoreRuntime,
            ArtifactClass::ExtensionPacks,
            ArtifactClass::SchemaContracts,
            ArtifactClass::WorkspaceState,
            ArtifactClass::Configuration,
            ArtifactClass::LanguageRuntimes,
        ],
        &[
            ChannelScope::Stable,
            ChannelScope::Beta,
            ChannelScope::Preview,
            ChannelScope::Nightly,
        ],
        &[DeploymentProfile::Managed, DeploymentProfile::SelfHosted],
        StaleDataBehavior::StaleBannerShown,
    ),
    (
        LifecycleFacet::ReleaseNoteEvidence,
        CanonicalState::Readiness(ReadinessState::ReadyNoRestart),
        &[
            ArtifactClass::CoreRuntime,
            ArtifactClass::ExtensionPacks,
            ArtifactClass::DocsHelpContent,
        ],
        &[
            ChannelScope::Stable,
            ChannelScope::Beta,
            ChannelScope::Preview,
        ],
        &[DeploymentProfile::Managed, DeploymentProfile::SelfHosted],
        StaleDataBehavior::OfflineCached,
    ),
    (
        LifecycleFacet::MigrationAssistant,
        CanonicalState::Migration(MigrationState::AutomaticMigration),
        &[
            ArtifactClass::WorkspaceState,
            ArtifactClass::Configuration,
            ArtifactClass::SchemaContracts,
        ],
        &[ChannelScope::Stable, ChannelScope::Beta, ChannelScope::Lts],
        &[DeploymentProfile::Managed, DeploymentProfile::SelfHosted],
        StaleDataBehavior::LocalOnlyNoLiveData,
    ),
    (
        LifecycleFacet::ServiceHealth,
        CanonicalState::Readiness(ReadinessState::ReadyNoRestart),
        &[ArtifactClass::CoreRuntime, ArtifactClass::LanguageRuntimes],
        &[
            ChannelScope::Stable,
            ChannelScope::Beta,
            ChannelScope::Preview,
            ChannelScope::Nightly,
            ChannelScope::Lts,
        ],
        &[DeploymentProfile::Managed, DeploymentProfile::SelfHosted],
        StaleDataBehavior::LocalOnlyNoLiveData,
    ),
    (
        LifecycleFacet::SupportWindow,
        CanonicalState::SupportWindow(SupportWindowState::FullSupport),
        &[ArtifactClass::CoreRuntime],
        &[ChannelScope::Stable, ChannelScope::Lts],
        &[DeploymentProfile::Managed, DeploymentProfile::SelfHosted],
        StaleDataBehavior::MirroredLabelled,
    ),
    (
        LifecycleFacet::CompatibilityWindow,
        CanonicalState::SupportWindow(SupportWindowState::FullSupport),
        &[
            ArtifactClass::SchemaContracts,
            ArtifactClass::ExtensionPacks,
        ],
        &[ChannelScope::Stable, ChannelScope::Beta, ChannelScope::Lts],
        &[DeploymentProfile::Managed, DeploymentProfile::SelfHosted],
        StaleDataBehavior::OfflineCached,
    ),
    (
        LifecycleFacet::EndOfSupport,
        CanonicalState::EndOfSupport(EndOfSupportState::Supported),
        &[ArtifactClass::CoreRuntime, ArtifactClass::DocsHelpContent],
        &[ChannelScope::Stable, ChannelScope::Lts],
        &[DeploymentProfile::Managed, DeploymentProfile::SelfHosted],
        StaleDataBehavior::MirroredLabelled,
    ),
];

/// The claimed consumer surfaces and the facets each reads. Together the reads cover every facet.
const CONSUMER_DEFS: [(LifecycleConsumer, QualificationClass, &[LifecycleFacet]); 8] = [
    (
        LifecycleConsumer::ReleaseCenter,
        QualificationClass::Stable,
        &[
            LifecycleFacet::UpdateAvailability,
            LifecycleFacet::ChangeImpact,
            LifecycleFacet::ReleaseNoteEvidence,
            LifecycleFacet::MigrationAssistant,
            LifecycleFacet::ServiceHealth,
            LifecycleFacet::SupportWindow,
            LifecycleFacet::CompatibilityWindow,
            LifecycleFacet::EndOfSupport,
        ],
    ),
    (
        LifecycleConsumer::UpdateCenter,
        QualificationClass::Stable,
        &[
            LifecycleFacet::UpdateAvailability,
            LifecycleFacet::ChangeImpact,
            LifecycleFacet::ReleaseNoteEvidence,
            LifecycleFacet::MigrationAssistant,
            LifecycleFacet::ServiceHealth,
        ],
    ),
    (
        LifecycleConsumer::HelpAbout,
        QualificationClass::Stable,
        &[
            LifecycleFacet::UpdateAvailability,
            LifecycleFacet::ReleaseNoteEvidence,
            LifecycleFacet::ServiceHealth,
            LifecycleFacet::SupportWindow,
            LifecycleFacet::EndOfSupport,
        ],
    ),
    (
        LifecycleConsumer::DocsHelp,
        QualificationClass::Stable,
        &[
            LifecycleFacet::ReleaseNoteEvidence,
            LifecycleFacet::MigrationAssistant,
            LifecycleFacet::SupportWindow,
            LifecycleFacet::CompatibilityWindow,
            LifecycleFacet::EndOfSupport,
        ],
    ),
    (
        LifecycleConsumer::Diagnostics,
        QualificationClass::Stable,
        &[
            LifecycleFacet::UpdateAvailability,
            LifecycleFacet::ChangeImpact,
            LifecycleFacet::ServiceHealth,
            LifecycleFacet::CompatibilityWindow,
        ],
    ),
    (
        LifecycleConsumer::SupportExport,
        QualificationClass::Stable,
        &[
            LifecycleFacet::ChangeImpact,
            LifecycleFacet::MigrationAssistant,
            LifecycleFacet::ServiceHealth,
            LifecycleFacet::SupportWindow,
            LifecycleFacet::CompatibilityWindow,
            LifecycleFacet::EndOfSupport,
        ],
    ),
    (
        LifecycleConsumer::Shiproom,
        QualificationClass::Stable,
        &[
            LifecycleFacet::UpdateAvailability,
            LifecycleFacet::ChangeImpact,
            LifecycleFacet::ReleaseNoteEvidence,
            LifecycleFacet::SupportWindow,
            LifecycleFacet::CompatibilityWindow,
            LifecycleFacet::EndOfSupport,
        ],
    ),
    (
        LifecycleConsumer::CompanionHandoff,
        QualificationClass::Stable,
        &[
            LifecycleFacet::UpdateAvailability,
            LifecycleFacet::ServiceHealth,
            LifecycleFacet::EndOfSupport,
        ],
    ),
];

/// Builds the canonical governed facets with every proof current.
fn canonical_facets() -> Vec<LifecycleFacetRow> {
    FACET_DEFS
        .iter()
        .map(|(facet, state, classes, channels, profiles, stale)| {
            LifecycleFacetRow::new(
                *facet,
                *state,
                FreshnessState::Current,
                classes,
                channels,
                profiles,
                *stale,
            )
        })
        .collect()
}

/// Marks one facet's proof at the given freshness state.
fn with_facet_state(
    mut facets: Vec<LifecycleFacetRow>,
    facet: LifecycleFacet,
    state: FreshnessState,
) -> Vec<LifecycleFacetRow> {
    for row in &mut facets {
        if row.facet == facet {
            let def = FACET_DEFS
                .iter()
                .find(|(f, ..)| *f == facet)
                .expect("facet has a definition");
            *row = LifecycleFacetRow::new(facet, def.1, state, def.2, def.3, def.4, def.5);
        }
    }
    facets
}

/// Builds the claimed consumer rows; unions, gaps, and verdict are recomputed in the packet.
fn consumer_rows() -> Vec<LifecycleConsumerRow> {
    CONSUMER_DEFS
        .iter()
        .map(|(consumer, claimed, facets)| LifecycleConsumerRow::new(*consumer, *claimed, facets))
        .collect()
}

/// Assembles a packet from the given governed facets.
fn assemble_packet(
    packet_id: &str,
    report_label: &str,
    facets: Vec<LifecycleFacetRow>,
) -> M5UpdateLifecycleGovernance {
    M5UpdateLifecycleGovernance::new(M5UpdateLifecycleInput {
        packet_id: packet_id.to_owned(),
        report_label: report_label.to_owned(),
        evaluated_at: SEED_EVALUATED_AT.to_owned(),
        facets,
        consumers: consumer_rows(),
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_EVALUATED_AT.to_owned(),
    })
}

/// The canonical, all-current update / support-lifecycle governance packet: every facet governed at
/// a current proof and a governed lifecycle state, so every consumer stands fully certified at
/// Stable.
pub fn seeded_m5_update_lifecycle() -> M5UpdateLifecycleGovernance {
    assemble_packet(
        M5_UPDATE_LIFECYCLE_PACKET_ID,
        "M5 update / support-lifecycle governance matrix",
        canonical_facets(),
    )
}

/// Drill: one facet's proof is stale, so the consumers that read it auto-narrow below Stable.
pub fn seeded_m5_update_lifecycle_stale_proof_narrowed() -> M5UpdateLifecycleGovernance {
    let facets = with_facet_state(canonical_facets(), STALE_DRILL_FACET, FreshnessState::Stale);
    assemble_packet(
        "m5-update-lifecycle:drill-stale:0001",
        "M5 update / support-lifecycle governance — stale-proof drill",
        facets,
    )
}

/// Drill: one facet's proof is missing, so the consumers that read it are blocked from Stable
/// promotion.
pub fn seeded_m5_update_lifecycle_missing_proof_blocked() -> M5UpdateLifecycleGovernance {
    let facets = with_facet_state(
        canonical_facets(),
        MISSING_DRILL_FACET,
        FreshnessState::Missing,
    );
    assemble_packet(
        "m5-update-lifecycle:drill-missing:0001",
        "M5 update / support-lifecycle governance — missing-proof drill",
        facets,
    )
}
