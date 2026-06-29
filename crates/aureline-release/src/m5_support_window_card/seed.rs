//! Canonical seed builders for the support-window card set.
//!
//! These builders are the single producer of the checked-in card-set packet, the published
//! inventory, the release-grade channel-lifecycle parity proof (and its Markdown report), the
//! machine-readable per-card CSV export, and the per-state drill fixtures. The headless emitter and
//! the inline tests both call them so the in-code packet, the artifacts, and the fixtures never drift.
//!
//! The canonical packet describes a healthy lifecycle: every channel is fully supported and every
//! compatibility subject is within its window, so every consumer reads "supported". The drills perturb
//! one card and let the derivation recompute each consumer's readiness and gaps:
//!
//! - the **deprecation** drill puts one channel into a grace-window / deprecated posture (narrowed),
//!   carrying replacement, overlap, and recovery guidance, so consumers reading it plan a migration;
//! - the **end-of-support** drill puts one channel out of support / removed (blocked), carrying an
//!   upgrade path, so consumers reading it require an action; and
//! - the **subject-compatibility** drill deprecates one subject and pushes it toward its ceiling
//!   (narrowed), so the compatibility report and the other consumers plan a migration.

use super::*;

/// Stable packet id for the canonical support-window card set.
pub const M5_SUPPORT_WINDOW_CARD_SET_PACKET_ID: &str = "m5-support-window-cards:stable:0001";

/// Evaluation / mint timestamp for the canonical packet.
const SEED_EVALUATED_AT: &str = "2026-07-06T00:00:00Z";

/// The channel the deprecation drill puts into a grace-window / deprecated posture.
const DEPRECATION_DRILL_CHANNEL: ChannelScope = ChannelScope::Preview;

/// The channel the end-of-support drill puts out of support / removed.
const END_OF_SUPPORT_DRILL_CHANNEL: ChannelScope = ChannelScope::Preview;

/// The subject the compatibility drill deprecates and pushes toward its ceiling.
const SUBJECT_DRILL_SUBJECT: CompatibilitySubject = CompatibilitySubject::ExtensionManifest;

fn both_profiles() -> Vec<DeploymentProfile> {
    vec![DeploymentProfile::Managed, DeploymentProfile::SelfHosted]
}

fn channel_evidence(channel: ChannelScope) -> Vec<String> {
    vec![format!(
        "artifacts/release/m5-channel-lifecycle-proof/channel.{}.evidence",
        channel.as_str()
    )]
}

fn subject_evidence(subject: CompatibilitySubject) -> Vec<String> {
    vec![format!(
        "artifacts/release/m5-channel-lifecycle-proof/subject.{}.evidence",
        subject.as_str()
    )]
}

/// Builds a fully-supported channel card with an open overlap window for the prior version.
fn supported_channel(channel: ChannelScope) -> ChannelSupportCard {
    let token = channel.as_str();
    ChannelSupportCard::new(ChannelSupportCardInput {
        channel,
        support_window_state: SupportWindowState::FullSupport,
        end_of_support_state: EndOfSupportState::Supported,
        support_window: SupportWindowDates::committed("2027-01-01", "2027-07-01"),
        overlap_window: OverlapWindow::overlapping(token, "1.7.0", "2026-10-01"),
        deprecation_horizon: DeprecationHorizon::none(),
        pin_postpone: PinPostponeGuidance::stay(token),
        compatibility_caveats: Vec::new(),
        profiles: both_profiles(),
        evidence_refs: channel_evidence(channel),
    })
}

/// The canonical, all-supported channel cards.
fn canonical_channels() -> Vec<ChannelSupportCard> {
    ChannelScope::ALL
        .iter()
        .map(|&c| supported_channel(c))
        .collect()
}

/// Builds a fully-supported subject card whose current version sits inside its window.
fn supported_subject(subject: CompatibilitySubject) -> CompatibilitySubjectCard {
    let token = subject.as_str();
    CompatibilitySubjectCard::new(CompatibilitySubjectCardInput {
        subject,
        end_of_support_state: EndOfSupportState::Supported,
        compatibility_window: CompatibilityWindow {
            floor_version: Some("1.0".to_owned()),
            current_version: Some("1.4".to_owned()),
            ceiling_version: Some("2.0".to_owned()),
            posture: CompatibilityWindowPosture::WithinWindow,
        },
        successor_message_id: None,
        pin_postpone: PinPostponeGuidance::stay(token),
        compatibility_caveats: Vec::new(),
        profiles: both_profiles(),
        evidence_refs: subject_evidence(subject),
    })
}

/// The canonical, all-supported subject cards.
fn canonical_subjects() -> Vec<CompatibilitySubjectCard> {
    CompatibilitySubject::ALL
        .iter()
        .map(|&s| supported_subject(s))
        .collect()
}

/// The claimed consumer rows. Help/About, docs/help, the update center, the compatibility report,
/// support export, the admin console, and the release center all read every channel and every subject,
/// so the same support-window data appears on each surface.
fn consumer_rows() -> Vec<SupportConsumerRow> {
    SupportConsumer::ALL
        .iter()
        .map(|&c| SupportConsumerRow::new(c, &ChannelScope::ALL, &CompatibilitySubject::ALL))
        .collect()
}

fn with_channel(
    mut channels: Vec<ChannelSupportCard>,
    channel: ChannelScope,
    replacement: ChannelSupportCard,
) -> Vec<ChannelSupportCard> {
    for card in &mut channels {
        if card.channel == channel {
            *card = replacement.clone();
        }
    }
    channels
}

fn with_subject(
    mut subjects: Vec<CompatibilitySubjectCard>,
    subject: CompatibilitySubject,
    replacement: CompatibilitySubjectCard,
) -> Vec<CompatibilitySubjectCard> {
    for card in &mut subjects {
        if card.subject == subject {
            *card = replacement.clone();
        }
    }
    subjects
}

/// Assembles a packet from the given cards.
fn assemble_packet(
    packet_id: &str,
    report_label: &str,
    data_state: StaleDataBehavior,
    channels: Vec<ChannelSupportCard>,
    subjects: Vec<CompatibilitySubjectCard>,
) -> SupportWindowCardSet {
    SupportWindowCardSet::new(SupportWindowCardSetInput {
        packet_id: packet_id.to_owned(),
        report_label: report_label.to_owned(),
        evaluated_at: SEED_EVALUATED_AT.to_owned(),
        data_state,
        channels,
        subjects,
        consumers: consumer_rows(),
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_EVALUATED_AT.to_owned(),
    })
}

/// The canonical, all-supported support-window card set: every channel is fully supported and every
/// subject is within its window, so every consumer reads "supported".
pub fn seeded_m5_support_window_card_set() -> SupportWindowCardSet {
    assemble_packet(
        M5_SUPPORT_WINDOW_CARD_SET_PACKET_ID,
        "Aureline M5 support-window cards",
        StaleDataBehavior::LiveVerified,
        canonical_channels(),
        canonical_subjects(),
    )
}

/// Drill: one channel is deprecated and in its grace window (narrowed), carrying replacement, overlap,
/// and recovery guidance, so the consumers that read it plan a migration without an action being
/// forced.
pub fn seeded_m5_support_window_card_set_deprecation() -> SupportWindowCardSet {
    let token = DEPRECATION_DRILL_CHANNEL.as_str();
    let card = ChannelSupportCard::new(ChannelSupportCardInput {
        channel: DEPRECATION_DRILL_CHANNEL,
        support_window_state: SupportWindowState::GraceWindow,
        end_of_support_state: EndOfSupportState::Deprecated,
        support_window: SupportWindowDates::committed("2026-08-01", "2026-12-01"),
        overlap_window: OverlapWindow::overlapping(token, "1.6.0", "2026-12-01"),
        deprecation_horizon: DeprecationHorizon {
            successor_channel: Some(ChannelScope::Stable),
            deprecation_on: Some("2026-06-01".to_owned()),
            removal_target_version: Some("2.0.0".to_owned()),
            removal_on: Some("2026-12-01".to_owned()),
            replacement_message_id: Some(format!(
                "{}channel.{}.replacement",
                M5_SUPPORT_WINDOW_MESSAGE_ID_PREFIX, token
            )),
        },
        pin_postpone: PinPostponeGuidance::new(
            token,
            PinPostponeChoice::MoveToSuccessorChannel,
            &["artifacts/release/m5-channel-lifecycle-proof/preview_migration.guidance"],
        ),
        compatibility_caveats: vec![CompatibilityCaveat::new(
            token,
            "manifest_format",
            ArtifactClass::ExtensionPacks,
            &["artifacts/release/m5-channel-lifecycle-proof/preview_caveat.evidence"],
        )],
        profiles: both_profiles(),
        evidence_refs: channel_evidence(DEPRECATION_DRILL_CHANNEL),
    });
    assemble_packet(
        "m5-support-window-cards:drill-deprecation:0001",
        "Aureline M5 support-window cards — deprecation drill",
        StaleDataBehavior::LiveVerified,
        with_channel(canonical_channels(), DEPRECATION_DRILL_CHANNEL, card),
        canonical_subjects(),
    )
}

/// Drill: one channel is out of support / removed (blocked), carrying an upgrade path, so the
/// consumers that read it require a migration action.
pub fn seeded_m5_support_window_card_set_end_of_support() -> SupportWindowCardSet {
    let token = END_OF_SUPPORT_DRILL_CHANNEL.as_str();
    let card = ChannelSupportCard::new(ChannelSupportCardInput {
        channel: END_OF_SUPPORT_DRILL_CHANNEL,
        support_window_state: SupportWindowState::OutOfSupport,
        end_of_support_state: EndOfSupportState::Removed,
        support_window: SupportWindowDates::committed("2026-03-01", "2026-06-01"),
        overlap_window: OverlapWindow::none(token),
        deprecation_horizon: DeprecationHorizon {
            successor_channel: Some(ChannelScope::Stable),
            deprecation_on: Some("2026-03-01".to_owned()),
            removal_target_version: Some("1.9.0".to_owned()),
            removal_on: Some("2026-06-01".to_owned()),
            replacement_message_id: Some(format!(
                "{}channel.{}.replacement",
                M5_SUPPORT_WINDOW_MESSAGE_ID_PREFIX, token
            )),
        },
        pin_postpone: PinPostponeGuidance::new(
            token,
            PinPostponeChoice::UpgradeRequired,
            &["artifacts/release/m5-channel-lifecycle-proof/preview_upgrade.guidance"],
        ),
        compatibility_caveats: Vec::new(),
        profiles: both_profiles(),
        evidence_refs: channel_evidence(END_OF_SUPPORT_DRILL_CHANNEL),
    });
    assemble_packet(
        "m5-support-window-cards:drill-end-of-support:0001",
        "Aureline M5 support-window cards — end-of-support drill",
        StaleDataBehavior::LiveVerified,
        with_channel(canonical_channels(), END_OF_SUPPORT_DRILL_CHANNEL, card),
        canonical_subjects(),
    )
}

/// Drill: one compatibility subject is deprecated and nearing its ceiling (narrowed), carrying
/// replacement and recovery guidance, so the compatibility report and the other consumers plan a
/// migration.
pub fn seeded_m5_support_window_card_set_subject_compat() -> SupportWindowCardSet {
    let token = SUBJECT_DRILL_SUBJECT.as_str();
    let card = CompatibilitySubjectCard::new(CompatibilitySubjectCardInput {
        subject: SUBJECT_DRILL_SUBJECT,
        end_of_support_state: EndOfSupportState::Deprecated,
        compatibility_window: CompatibilityWindow {
            floor_version: Some("1.0".to_owned()),
            current_version: Some("1.9".to_owned()),
            ceiling_version: Some("2.0".to_owned()),
            posture: CompatibilityWindowPosture::NearingCeiling,
        },
        successor_message_id: Some(format!(
            "{}subject.{}.replacement",
            M5_SUPPORT_WINDOW_MESSAGE_ID_PREFIX, token
        )),
        pin_postpone: PinPostponeGuidance::new(
            token,
            PinPostponeChoice::MoveToSuccessorChannel,
            &["artifacts/release/m5-channel-lifecycle-proof/manifest_migration.guidance"],
        ),
        compatibility_caveats: vec![CompatibilityCaveat::new(
            token,
            "manifest_v1_deprecated",
            ArtifactClass::ExtensionPacks,
            &["artifacts/release/m5-channel-lifecycle-proof/manifest_caveat.evidence"],
        )],
        profiles: both_profiles(),
        evidence_refs: subject_evidence(SUBJECT_DRILL_SUBJECT),
    });
    assemble_packet(
        "m5-support-window-cards:drill-subject-compat:0001",
        "Aureline M5 support-window cards — subject-compatibility drill",
        StaleDataBehavior::LiveVerified,
        canonical_channels(),
        with_subject(canonical_subjects(), SUBJECT_DRILL_SUBJECT, card),
    )
}
