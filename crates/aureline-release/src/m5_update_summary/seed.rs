//! Canonical seed builders for the update-center summary objects.
//!
//! These builders are the single producer of the checked-in summary packet, the published
//! inventory, the release-grade parity proof (and its Markdown report), the machine-readable delta
//! CSV, and the stale / not-provided drill fixtures. The headless emitter and the inline tests both
//! call them so the in-code packet, the artifacts, and the fixtures never drift. The canonical packet
//! summarizes every artifact family with verified, live (or honestly labeled mirrored / offline)
//! data, so every consumer stands certified at Stable; the drills perturb one family's release data
//! and let the derivation recompute each consumer's verdict and gaps.

use super::*;

/// Stable packet id for the canonical update-center summary packet.
pub const M5_UPDATE_CENTER_SUMMARY_PACKET_ID: &str = "m5-update-center-summary:stable:0001";

/// Evaluation / mint timestamp for the canonical packet.
const SEED_EVALUATED_AT: &str = "2026-07-06T00:00:00Z";

/// The family whose backing data the stale drill ages out. Read by the release center and update
/// center but not Help/About, so the drill narrows exactly those two consumers.
const STALE_DRILL_FAMILY: ArtifactFamily = ArtifactFamily::DocsPack;

/// The family whose backing data the not-provided drill drops. Read by the release center and update
/// center but not Help/About, so the drill blocks exactly those two consumers.
const NOT_PROVIDED_DRILL_FAMILY: ArtifactFamily = ArtifactFamily::FrameworkPack;

/// Builds the canonical, all-governed update summary entries: every family verified, with live or
/// honestly labeled mirrored / offline data, so every consumer certifies at Stable.
fn canonical_entries() -> Vec<UpdateSummaryEntry> {
    vec![
        // Desktop app: up to date, true rollback, verified, live.
        UpdateSummaryEntry::new(
            ArtifactFamily::DesktopApp,
            ChannelScope::Stable,
            &[DeploymentProfile::Managed, DeploymentProfile::SelfHosted],
            "1.8.0",
            "1.8.0",
            UpdatePosture::Applied,
            RollbackAvailability::RollbackSupported,
            vec![
                ArtifactDeltaRow::new(
                    ArtifactFamily::DesktopApp,
                    ArtifactClass::CoreRuntime,
                    DeltaChangeKind::Unchanged,
                    Some("1.8.0"),
                    Some("1.8.0"),
                    VerificationState::Verified,
                    RestartImpact::RestartApp,
                    ReleaseDataState::Live,
                ),
                ArtifactDeltaRow::new(
                    ArtifactFamily::DesktopApp,
                    ArtifactClass::Configuration,
                    DeltaChangeKind::Unchanged,
                    Some("1.8.0"),
                    Some("1.8.0"),
                    VerificationState::Verified,
                    RestartImpact::None,
                    ReleaseDataState::Live,
                ),
            ],
        ),
        // Extension: update available, side-by-side fallback (not a true rollback), mirrored data.
        UpdateSummaryEntry::new(
            ArtifactFamily::Extension,
            ChannelScope::Stable,
            &[DeploymentProfile::Managed, DeploymentProfile::SelfHosted],
            "3.2.1",
            "3.4.0",
            UpdatePosture::Staged,
            RollbackAvailability::SideBySideFallback,
            vec![ArtifactDeltaRow::new(
                ArtifactFamily::Extension,
                ArtifactClass::ExtensionPacks,
                DeltaChangeKind::Updated,
                Some("3.2.1"),
                Some("3.4.0"),
                VerificationState::Verified,
                RestartImpact::ReloadWindow,
                ReleaseDataState::Mirrored,
            )],
        ),
        // Docs pack: update available, true rollback, offline-cached data (labeled, still usable).
        UpdateSummaryEntry::new(
            ArtifactFamily::DocsPack,
            ChannelScope::Stable,
            &[DeploymentProfile::Managed, DeploymentProfile::SelfHosted],
            "2025.6",
            "2025.7",
            UpdatePosture::Downloaded,
            RollbackAvailability::RollbackSupported,
            vec![ArtifactDeltaRow::new(
                ArtifactFamily::DocsPack,
                ArtifactClass::DocsHelpContent,
                DeltaChangeKind::Updated,
                Some("2025.6"),
                Some("2025.7"),
                VerificationState::Verified,
                RestartImpact::None,
                ReleaseDataState::Offline,
            )],
        ),
        // Policy bundle: managed-only, update available, true rollback, live data.
        UpdateSummaryEntry::new(
            ArtifactFamily::PolicyBundle,
            ChannelScope::Stable,
            &[DeploymentProfile::Managed],
            "2.0.0",
            "2.1.0",
            UpdatePosture::Staged,
            RollbackAvailability::RollbackSupported,
            vec![
                ArtifactDeltaRow::new(
                    ArtifactFamily::PolicyBundle,
                    ArtifactClass::Configuration,
                    DeltaChangeKind::Updated,
                    Some("2.0.0"),
                    Some("2.1.0"),
                    VerificationState::Verified,
                    RestartImpact::None,
                    ReleaseDataState::Live,
                ),
                ArtifactDeltaRow::new(
                    ArtifactFamily::PolicyBundle,
                    ArtifactClass::SchemaContracts,
                    DeltaChangeKind::Updated,
                    Some("12"),
                    Some("13"),
                    VerificationState::Verified,
                    RestartImpact::ReloadWindow,
                    ReleaseDataState::Live,
                ),
            ],
        ),
        // Framework pack: beta channel, update available, reinstall-only (not a true rollback).
        UpdateSummaryEntry::new(
            ArtifactFamily::FrameworkPack,
            ChannelScope::Beta,
            &[DeploymentProfile::Managed, DeploymentProfile::SelfHosted],
            "0.9.0",
            "0.10.0",
            UpdatePosture::Downloaded,
            RollbackAvailability::ReinstallOnly,
            vec![
                ArtifactDeltaRow::new(
                    ArtifactFamily::FrameworkPack,
                    ArtifactClass::SchemaContracts,
                    DeltaChangeKind::Updated,
                    Some("0.9.0"),
                    Some("0.10.0"),
                    VerificationState::Verified,
                    RestartImpact::RestartApp,
                    ReleaseDataState::Live,
                ),
                ArtifactDeltaRow::new(
                    ArtifactFamily::FrameworkPack,
                    ArtifactClass::ExtensionPacks,
                    DeltaChangeKind::Added,
                    None,
                    Some("0.10.0"),
                    VerificationState::Verified,
                    RestartImpact::RestartApp,
                    ReleaseDataState::Live,
                ),
                ArtifactDeltaRow::new(
                    ArtifactFamily::FrameworkPack,
                    ArtifactClass::WorkspaceState,
                    DeltaChangeKind::Updated,
                    Some("5"),
                    Some("6"),
                    VerificationState::Verified,
                    RestartImpact::None,
                    ReleaseDataState::Live,
                ),
            ],
        ),
        // Runtime / toolchain: update available, reinstall-only, restart-app, live.
        UpdateSummaryEntry::new(
            ArtifactFamily::RuntimeToolchain,
            ChannelScope::Stable,
            &[DeploymentProfile::Managed, DeploymentProfile::SelfHosted],
            "1.84.0",
            "1.85.0",
            UpdatePosture::Downloaded,
            RollbackAvailability::ReinstallOnly,
            vec![ArtifactDeltaRow::new(
                ArtifactFamily::RuntimeToolchain,
                ArtifactClass::LanguageRuntimes,
                DeltaChangeKind::Updated,
                Some("1.84.0"),
                Some("1.85.0"),
                VerificationState::Verified,
                RestartImpact::RestartApp,
                ReleaseDataState::Live,
            )],
        ),
    ]
}

/// The claimed consumer rows. The release center and update center read every family; Help/About
/// reads the desktop app, extension, and runtime / toolchain it surfaces in the About panel.
fn consumer_rows() -> Vec<SummaryConsumerRow> {
    vec![
        SummaryConsumerRow::new(
            SummaryConsumer::ReleaseCenter,
            QualificationClass::Stable,
            &ArtifactFamily::ALL,
        ),
        SummaryConsumerRow::new(
            SummaryConsumer::UpdateCenter,
            QualificationClass::Stable,
            &ArtifactFamily::ALL,
        ),
        SummaryConsumerRow::new(
            SummaryConsumer::HelpAbout,
            QualificationClass::Stable,
            &[
                ArtifactFamily::DesktopApp,
                ArtifactFamily::Extension,
                ArtifactFamily::RuntimeToolchain,
            ],
        ),
    ]
}

/// Overrides every delta row's release-data state for one family, so the entry's rolled-up data
/// state — and therefore the consumers that read it — recompute against the perturbed state.
fn with_family_data(
    mut entries: Vec<UpdateSummaryEntry>,
    family: ArtifactFamily,
    state: ReleaseDataState,
) -> Vec<UpdateSummaryEntry> {
    for entry in &mut entries {
        if entry.family == family {
            for row in &mut entry.delta_rows {
                row.release_data_state = state;
            }
            entry.recompute();
        }
    }
    entries
}

/// Assembles a packet from the given entries.
fn assemble_packet(
    packet_id: &str,
    report_label: &str,
    entries: Vec<UpdateSummaryEntry>,
) -> M5UpdateCenterSummary {
    M5UpdateCenterSummary::new(M5UpdateCenterSummaryInput {
        packet_id: packet_id.to_owned(),
        report_label: report_label.to_owned(),
        evaluated_at: SEED_EVALUATED_AT.to_owned(),
        channel: ChannelScope::Stable,
        entries,
        consumers: consumer_rows(),
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_EVALUATED_AT.to_owned(),
    })
}

/// The canonical, all-governed update-center summary: every family verified with live or honestly
/// labeled mirrored / offline data, so every consumer stands certified at Stable.
pub fn seeded_m5_update_center_summary() -> M5UpdateCenterSummary {
    assemble_packet(
        M5_UPDATE_CENTER_SUMMARY_PACKET_ID,
        "Aureline M5 update-center summary",
        canonical_entries(),
    )
}

/// Drill: one family's release data has aged out (stale), so the consumers that read it auto-narrow
/// below Stable while the rest stay certified and the release gate stays a pass.
pub fn seeded_m5_update_center_summary_stale_data_narrowed() -> M5UpdateCenterSummary {
    let entries = with_family_data(
        canonical_entries(),
        STALE_DRILL_FAMILY,
        ReleaseDataState::Stale,
    );
    assemble_packet(
        "m5-update-center-summary:drill-stale:0001",
        "Aureline M5 update-center summary — stale-data drill",
        entries,
    )
}

/// Drill: one family has no live release data (not provided), so the consumers that read it are
/// blocked from Stable promotion while the rest stay certified.
pub fn seeded_m5_update_center_summary_not_provided_blocked() -> M5UpdateCenterSummary {
    let entries = with_family_data(
        canonical_entries(),
        NOT_PROVIDED_DRILL_FAMILY,
        ReleaseDataState::NotProvided,
    );
    assemble_packet(
        "m5-update-center-summary:drill-not-provided:0001",
        "Aureline M5 update-center summary — not-provided drill",
        entries,
    )
}
