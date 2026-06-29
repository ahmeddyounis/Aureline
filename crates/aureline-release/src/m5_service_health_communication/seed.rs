//! Canonical seed builders for the service-health communication packet.
//!
//! These builders are the single producer of the checked-in packet inventory, the release-grade
//! stale-release-data parity proof (and its Markdown report), the machine-readable per-card CSV
//! export, and the per-state drill fixtures. The headless emitter and the inline tests both call them
//! so the in-code packet, the artifacts, and the fixtures never drift.
//!
//! The canonical packet describes an all-healthy posture: every boundary is operational and serving
//! live data, every admin note is acknowledged and live, so every consumer reads "live trusted" and
//! local editing is safe. The drills perturb the data and let the derivation recompute each consumer's
//! readiness and continuity:
//!
//! - the **vendor-outage** drill puts the optional vendor-hosted boundary into an outage with
//!   unavailable data (blocked), carrying a continuation / recovery path, so consumers reading it read
//!   "no live data" *while local editing stays explicitly safe*;
//! - the **mirror-note** drill downgrades the vendor boundary's data to a labelled mirror and adds an
//!   unacknowledged mirror-change admin note (narrowed), so consumers show downgraded data with
//!   source-age truth; and
//! - the **local-only** drill takes every remote boundary offline (blocked) while the local machine
//!   stays operational and live, rendered under a `local_only_no_live_data` data state, so the panel
//!   proves a managed / vendor outage never implies local editing or recovery is unsafe.

use super::*;

/// Stable packet id for the canonical service-health communication packet.
pub const M5_SERVICE_HEALTH_COMMUNICATION_PACKET_ID: &str =
    "m5-service-health-communication:stable:0001";

/// Evaluation / mint timestamp for the canonical packet.
const SEED_EVALUATED_AT: &str = "2026-07-06T00:00:00Z";

fn both_profiles() -> Vec<DeploymentProfile> {
    vec![DeploymentProfile::Managed, DeploymentProfile::SelfHosted]
}

fn tier_evidence(tier: ServiceTier) -> Vec<String> {
    vec![format!(
        "artifacts/release/m5-stale-release-data-proof/tier.{}.evidence",
        tier.as_str()
    )]
}

fn note_evidence(kind: AdminNoteKind) -> Vec<String> {
    vec![format!(
        "artifacts/release/m5-stale-release-data-proof/note.{}.evidence",
        kind.as_str()
    )]
}

/// Builds a tier card. The continuation's local-safe flag is derived the same way the card derives it,
/// and a troubled card always carries an active recovery path with backing refs.
fn tier_card(
    tier: ServiceTier,
    health: HealthState,
    data: ReleaseDataState,
    source_age: SourceAge,
    recovery: RecoveryAction,
) -> ServiceTierHealthCard {
    let token = tier.as_str();
    let local_safe = if tier.affects_local_editing() {
        health != HealthState::Outage
    } else {
        true
    };
    let troubled = health != HealthState::Operational || data.is_downgraded();
    let guidance = if troubled {
        let refs = [format!(
            "artifacts/release/m5-stale-release-data-proof/{}.guidance",
            token
        )];
        let ref_slice: Vec<&str> = refs.iter().map(String::as_str).collect();
        RecoveryGuidance::new(token, recovery, &ref_slice)
    } else {
        RecoveryGuidance::none(token)
    };
    ServiceTierHealthCard::new(ServiceTierHealthCardInput {
        tier,
        health_state: health,
        release_data_state: data,
        source_age,
        continuation: ContinuationStatement::new(token, local_safe, guidance),
        profiles: both_profiles(),
        evidence_refs: tier_evidence(tier),
    })
}

/// Builds an admin-note card. A note that downgrades data always carries an active recovery path; a
/// downgraded note recovers by showing the labelled mirror copy, a live note needs no recovery.
fn note_card(
    kind: AdminNoteKind,
    affected_tier: ServiceTier,
    affected_channel: Option<ChannelScope>,
    data: ReleaseDataState,
    effective_from: Option<&str>,
    source_age: SourceAge,
    acknowledged: bool,
) -> AdminNoteCard {
    let token = kind.as_str();
    let troubled = data.is_downgraded();
    let guidance = if troubled {
        let refs = [format!(
            "artifacts/release/m5-stale-release-data-proof/note.{}.guidance",
            token
        )];
        let ref_slice: Vec<&str> = refs.iter().map(String::as_str).collect();
        RecoveryGuidance::new(
            &format!("note.{token}"),
            RecoveryAction::UseMirrorCopy,
            &ref_slice,
        )
    } else {
        RecoveryGuidance::none(&format!("note.{token}"))
    };
    AdminNoteCard::new(AdminNoteCardInput {
        kind,
        affected_tier,
        affected_channel,
        release_data_state: data,
        effective_from: effective_from.map(str::to_owned),
        source_age,
        acknowledged,
        continuation: ContinuationStatement::new(&format!("note.{token}"), true, guidance),
        evidence_refs: note_evidence(kind),
    })
}

/// The canonical, all-healthy tier cards: every boundary operational and serving live data.
fn canonical_tiers() -> Vec<ServiceTierHealthCard> {
    ServiceTier::ALL
        .iter()
        .map(|&t| {
            tier_card(
                t,
                HealthState::Operational,
                ReleaseDataState::LiveVerified,
                SourceAge::live(SEED_EVALUATED_AT),
                RecoveryAction::NotApplicable,
            )
        })
        .collect()
}

/// The canonical, all-acknowledged admin notes: each note is live and acknowledged, so none carries
/// pressure.
fn canonical_notes() -> Vec<AdminNoteCard> {
    vec![
        note_card(
            AdminNoteKind::ChannelChange,
            ServiceTier::VendorHostedService,
            Some(ChannelScope::Stable),
            ReleaseDataState::LiveVerified,
            Some("2026-06-01"),
            SourceAge::live(SEED_EVALUATED_AT),
            true,
        ),
        note_card(
            AdminNoteKind::MirrorChange,
            ServiceTier::VendorHostedService,
            None,
            ReleaseDataState::LiveVerified,
            Some("2026-06-01"),
            SourceAge::live(SEED_EVALUATED_AT),
            true,
        ),
        note_card(
            AdminNoteKind::DeploymentChange,
            ServiceTier::EnterpriseControlPlane,
            None,
            ReleaseDataState::LiveVerified,
            Some("2026-06-01"),
            SourceAge::live(SEED_EVALUATED_AT),
            true,
        ),
    ]
}

/// The claimed consumer rows. The service-health panel, Help/About, docs/help, support export, the
/// admin console, and the release center all read every tier and every admin note, so the same
/// service-health data and admin notes appear on each surface.
fn consumer_rows() -> Vec<HealthConsumerRow> {
    HealthConsumer::ALL
        .iter()
        .map(|&c| HealthConsumerRow::new(c, &ServiceTier::ALL, &AdminNoteKind::ALL))
        .collect()
}

fn with_tier(
    mut tiers: Vec<ServiceTierHealthCard>,
    tier: ServiceTier,
    replacement: ServiceTierHealthCard,
) -> Vec<ServiceTierHealthCard> {
    for card in &mut tiers {
        if card.tier == tier {
            *card = replacement.clone();
        }
    }
    tiers
}

fn with_note(
    mut notes: Vec<AdminNoteCard>,
    kind: AdminNoteKind,
    replacement: AdminNoteCard,
) -> Vec<AdminNoteCard> {
    for card in &mut notes {
        if card.kind == kind {
            *card = replacement.clone();
        }
    }
    notes
}

/// Assembles a packet from the given cards.
fn assemble_packet(
    packet_id: &str,
    report_label: &str,
    data_state: StaleDataBehavior,
    tiers: Vec<ServiceTierHealthCard>,
    notes: Vec<AdminNoteCard>,
) -> ServiceHealthCommunication {
    ServiceHealthCommunication::new(ServiceHealthCommunicationInput {
        packet_id: packet_id.to_owned(),
        report_label: report_label.to_owned(),
        evaluated_at: SEED_EVALUATED_AT.to_owned(),
        data_state,
        tiers,
        notes,
        consumers: consumer_rows(),
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_EVALUATED_AT.to_owned(),
    })
}

/// The canonical, all-healthy service-health packet: every boundary operational and live, every admin
/// note acknowledged, so every consumer reads "live trusted" and local editing is safe.
pub fn seeded_m5_service_health_communication() -> ServiceHealthCommunication {
    assemble_packet(
        M5_SERVICE_HEALTH_COMMUNICATION_PACKET_ID,
        "Aureline M5 service-health communication",
        StaleDataBehavior::LiveVerified,
        canonical_tiers(),
        canonical_notes(),
    )
}

/// Drill: the optional vendor-hosted boundary is in an outage with unavailable data (blocked), carrying
/// a continuation / recovery path, so the consumers reading it read "no live data" while local editing
/// stays explicitly safe.
pub fn seeded_m5_service_health_communication_vendor_outage() -> ServiceHealthCommunication {
    let vendor = tier_card(
        ServiceTier::VendorHostedService,
        HealthState::Outage,
        ReleaseDataState::Unavailable,
        SourceAge::aged(
            Some("2026-07-05T18:00:00Z"),
            None,
            "no live data since 2026-07-05 18:00Z",
        ),
        RecoveryAction::RetryWhenReachable,
    );
    assemble_packet(
        "m5-service-health-communication:drill-vendor-outage:0001",
        "Aureline M5 service-health communication — vendor-outage drill",
        StaleDataBehavior::StaleBannerShown,
        with_tier(canonical_tiers(), ServiceTier::VendorHostedService, vendor),
        canonical_notes(),
    )
}

/// Drill: the vendor boundary's data is downgraded to a labelled mirror and an unacknowledged
/// mirror-change admin note is propagated (narrowed), so consumers show downgraded data with source-age
/// truth while local editing stays safe.
pub fn seeded_m5_service_health_communication_mirror_note() -> ServiceHealthCommunication {
    let vendor = tier_card(
        ServiceTier::VendorHostedService,
        HealthState::Degraded,
        ReleaseDataState::Mirrored,
        SourceAge::aged(
            Some("2026-07-06T00:00:00Z"),
            Some("2026-07-04T00:00:00Z"),
            "mirrored, 2 days old",
        ),
        RecoveryAction::UseMirrorCopy,
    );
    let note = note_card(
        AdminNoteKind::MirrorChange,
        ServiceTier::VendorHostedService,
        None,
        ReleaseDataState::Mirrored,
        Some("2026-07-04"),
        SourceAge::aged(
            Some("2026-07-06T00:00:00Z"),
            Some("2026-07-04T00:00:00Z"),
            "mirror switched 2026-07-04",
        ),
        false,
    );
    assemble_packet(
        "m5-service-health-communication:drill-mirror-note:0001",
        "Aureline M5 service-health communication — mirror-note drill",
        StaleDataBehavior::MirroredLabelled,
        with_tier(canonical_tiers(), ServiceTier::VendorHostedService, vendor),
        with_note(canonical_notes(), AdminNoteKind::MirrorChange, note),
    )
}

/// Drill: every remote boundary is offline (blocked) while the local machine stays operational and
/// live, rendered under a `local_only_no_live_data` data state, so the panel proves a managed / vendor
/// outage never implies local editing or recovery is unsafe.
pub fn seeded_m5_service_health_communication_local_only() -> ServiceHealthCommunication {
    let mut tiers = canonical_tiers();
    for tier in [
        ServiceTier::RemoteTarget,
        ServiceTier::EnterpriseControlPlane,
        ServiceTier::VendorHostedService,
    ] {
        let recovery = match tier {
            ServiceTier::RemoteTarget => RecoveryAction::ReconnectTarget,
            ServiceTier::EnterpriseControlPlane => RecoveryAction::ContactAdmin,
            _ => RecoveryAction::RetryWhenReachable,
        };
        let card = tier_card(
            tier,
            HealthState::Outage,
            ReleaseDataState::Unavailable,
            SourceAge::aged(
                Some("2026-07-05T12:00:00Z"),
                None,
                "no live data since 2026-07-05 12:00Z",
            ),
            recovery,
        );
        tiers = with_tier(tiers, tier, card);
    }
    // The local machine stays operational; its local install info is still live and local.
    let local = tier_card(
        ServiceTier::LocalMachine,
        HealthState::Operational,
        ReleaseDataState::LiveVerified,
        SourceAge::live(SEED_EVALUATED_AT),
        RecoveryAction::NotApplicable,
    );
    tiers = with_tier(tiers, ServiceTier::LocalMachine, local);
    assemble_packet(
        "m5-service-health-communication:drill-local-only:0001",
        "Aureline M5 service-health communication — local-only drill",
        StaleDataBehavior::LocalOnlyNoLiveData,
        tiers,
        canonical_notes(),
    )
}
