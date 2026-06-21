//! The single mint-from-truth source for the voice provider routing packet, the
//! checked-in support-export artifact, and the routing fixtures.
//!
//! Every id, ref, and label is stable so the artifact and fixtures stay
//! byte-aligned with the in-crate builder. Each row records the resolver's
//! output for its inputs, so the recorded outcome can never drift from
//! [`super::resolve_voice_routing`].

use std::fs;
use std::io;
use std::path::Path;

use super::{
    fixture_json, resolve_voice_routing, AudioRetentionClass, ProcessingLocalityCue, RetentionMode,
    TranscriptExportPosture, VoiceAcousticProfileClass, VoiceClaimPosture, VoiceEntitlementState,
    VoiceLanguagePackAvailability, VoiceLanguageProfile, VoicePolicyState, VoiceProviderClass,
    VoiceProviderRoutingCandidate, VoiceProviderRoutingConsumerProjection,
    VoiceProviderRoutingGuardrails, VoiceProviderRoutingPacket, VoiceProviderRoutingPacketInput,
    VoiceProviderRoutingRequest, VoiceProviderRoutingRow, VoiceRetentionExportControls,
    VoiceTransportClass, VOICE_PROCESSING_AND_RETENTION_DOC_REF,
    VOICE_PROVIDER_DESCRIPTOR_SCHEMA_REF, VOICE_PROVIDER_ROUTING_ARTIFACT_REF,
    VOICE_PROVIDER_ROUTING_FIXTURES_DIR_REF, VOICE_RETENTION_EXPORT_SCHEMA_REF,
    VOICE_SESSION_STATE_SCHEMA_REF,
};

/// Stable packet id minted by [`seeded_voice_provider_routing_packet`].
pub const SEED_VOICE_PROVIDER_ROUTING_PACKET_ID: &str = "voice-provider-routing:stable:0001";

/// Mint timestamp used by [`seeded_voice_provider_routing_packet`].
pub const SEED_VOICE_PROVIDER_ROUTING_MINTED_AT: &str = "2026-06-20T00:00:00Z";

/// Durable id of the seeded on-device local-first default provider.
pub const LOCAL_DEFAULT_ID: &str = "voice.provider.on_device_default";
/// Durable id of the seeded approved hosted provider.
pub const HOSTED_APPROVED_ID: &str = "voice.provider.approved_remote";

fn local_controls() -> VoiceRetentionExportControls {
    VoiceRetentionExportControls::new(
        RetentionMode::NoAudioNoTranscriptRetained,
        AudioRetentionClass::NoAudioRetained,
        TranscriptExportPosture::NoTranscriptExport,
        true,
        "On-device only — no audio or transcript leaves this device",
    )
}

fn hosted_controls() -> VoiceRetentionExportControls {
    VoiceRetentionExportControls::new(
        RetentionMode::TranscriptRetainedRedactedInSupportBundle,
        AudioRetentionClass::EphemeralAudioLocalOnly,
        TranscriptExportPosture::ExplicitUserExportRedacted,
        true,
        "Hosted processing — audio is ephemeral, transcripts are retained redacted and exportable only on explicit, redacted request",
    )
}

fn en_us_bundled() -> VoiceLanguageProfile {
    VoiceLanguageProfile {
        language_tag: "en-US".to_owned(),
        acoustic_profile_class: VoiceAcousticProfileClass::DefaultAcoustic,
        pack_availability: VoiceLanguagePackAvailability::BundledLocal,
        profile_label: "English (US) — on-device".to_owned(),
    }
}

fn ja_jp_download_required() -> VoiceLanguageProfile {
    VoiceLanguageProfile {
        language_tag: "ja-JP".to_owned(),
        acoustic_profile_class: VoiceAcousticProfileClass::DefaultAcoustic,
        pack_availability: VoiceLanguagePackAvailability::AvailableForDownload,
        profile_label: "Japanese — download required".to_owned(),
    }
}

fn local_default_candidate() -> VoiceProviderRoutingCandidate {
    VoiceProviderRoutingCandidate {
        candidate_id: LOCAL_DEFAULT_ID.to_owned(),
        candidate_label: "on-device speech engine".to_owned(),
        fingerprint_token: format!("fp:candidate:{LOCAL_DEFAULT_ID}"),
        provider_class: VoiceProviderClass::OnDeviceLocal,
        processing_locality: ProcessingLocalityCue::LocalOnDevice,
        transport_class: VoiceTransportClass::LocalInProcessOnly,
        retention_export: local_controls(),
        supported_language_tags: vec!["en-US".to_owned(), "de-DE".to_owned(), "ja-JP".to_owned()],
        baseline_language_tag: "en-US".to_owned(),
        requires_entitlement: false,
        requires_hosted_policy: false,
        is_local_first_default: true,
        available: true,
        keyboard_fallback_available: true,
    }
}

fn hosted_candidate(available: bool) -> VoiceProviderRoutingCandidate {
    VoiceProviderRoutingCandidate {
        candidate_id: HOSTED_APPROVED_ID.to_owned(),
        candidate_label: "approved hosted speech provider".to_owned(),
        fingerprint_token: format!("fp:candidate:{HOSTED_APPROVED_ID}"),
        provider_class: VoiceProviderClass::ApprovedRemoteDisclosed,
        processing_locality: ProcessingLocalityCue::HostedRemoteDisclosed,
        transport_class: VoiceTransportClass::ExplicitOptInDisclosedEndpoint,
        retention_export: hosted_controls(),
        supported_language_tags: vec!["en-US".to_owned(), "de-DE".to_owned(), "ja-JP".to_owned()],
        baseline_language_tag: "en-US".to_owned(),
        requires_entitlement: true,
        requires_hosted_policy: true,
        is_local_first_default: false,
        available,
        keyboard_fallback_available: true,
    }
}

/// Inline constructor input for one seeded routing row.
struct Scenario {
    scenario_id: &'static str,
    label: &'static str,
    claim_posture: VoiceClaimPosture,
    request: VoiceProviderRoutingRequest,
    candidates: Vec<VoiceProviderRoutingCandidate>,
}

fn row(scenario: Scenario) -> VoiceProviderRoutingRow {
    let outcome = resolve_voice_routing(&scenario.request, &scenario.candidates);
    VoiceProviderRoutingRow {
        scenario_id: scenario.scenario_id.to_owned(),
        scenario_label: scenario.label.to_owned(),
        fingerprint_token: format!("fp:scenario:{}", scenario.scenario_id),
        claim_posture: scenario.claim_posture,
        request: scenario.request,
        candidates: scenario.candidates,
        outcome,
        source_contract_refs: vec![VOICE_PROCESSING_AND_RETENTION_DOC_REF.to_owned()],
    }
}

/// Voice defaults to the on-device engine with no specific request.
fn local_first_default_row() -> VoiceProviderRoutingRow {
    row(Scenario {
        scenario_id: "voice-routing:local-first-default:0001",
        label: "With no provider requested, voice routes to the on-device default and discloses local-only processing",
        claim_posture: VoiceClaimPosture::ClaimedBeta,
        request: VoiceProviderRoutingRequest {
            requested_provider_id: None,
            requested_locality: ProcessingLocalityCue::LocalOnDevice,
            requested_language_profile: en_us_bundled(),
            requested_retention_export: local_controls(),
            policy_state: VoicePolicyState::UserControlled,
            hosted_permitted_by_policy: false,
            entitlement_state: VoiceEntitlementState::NotRequired,
        },
        candidates: vec![local_default_candidate(), hosted_candidate(true)],
    })
}

/// The user opts in to a disclosed hosted provider; the retention/export change
/// is surfaced rather than hidden.
fn hosted_opt_in_disclosed_row() -> VoiceProviderRoutingRow {
    row(Scenario {
        scenario_id: "voice-routing:hosted-opt-in-disclosed:0001",
        label: "Opting in to the approved hosted provider surfaces the retention and export change before capture",
        claim_posture: VoiceClaimPosture::ClaimedPreview,
        request: VoiceProviderRoutingRequest {
            requested_provider_id: Some(HOSTED_APPROVED_ID.to_owned()),
            requested_locality: ProcessingLocalityCue::HostedRemoteDisclosed,
            requested_language_profile: en_us_bundled(),
            requested_retention_export: local_controls(),
            policy_state: VoicePolicyState::EnterprisePolicyManaged,
            hosted_permitted_by_policy: true,
            entitlement_state: VoiceEntitlementState::Granted,
        },
        candidates: vec![local_default_candidate(), hosted_candidate(true)],
    })
}

/// A requested language pack is unavailable; voice continues on the same provider
/// with the baseline language profile.
fn language_pack_unavailable_row() -> VoiceProviderRoutingRow {
    row(Scenario {
        scenario_id: "voice-routing:language-pack-unavailable:0001",
        label: "A requested language pack that is not downloaded falls back to the on-device baseline profile, disclosed explicitly",
        claim_posture: VoiceClaimPosture::ClaimedBeta,
        request: VoiceProviderRoutingRequest {
            requested_provider_id: Some(LOCAL_DEFAULT_ID.to_owned()),
            requested_locality: ProcessingLocalityCue::LocalOnDevice,
            requested_language_profile: ja_jp_download_required(),
            requested_retention_export: local_controls(),
            policy_state: VoicePolicyState::UserControlled,
            hosted_permitted_by_policy: false,
            entitlement_state: VoiceEntitlementState::NotRequired,
        },
        candidates: vec![local_default_candidate()],
    })
}

/// Policy permits on-device processing only; a hosted request is held at the
/// on-device default rather than routed to a less private provider.
fn policy_requires_local_only_row() -> VoiceProviderRoutingRow {
    row(Scenario {
        scenario_id: "voice-routing:policy-requires-local-only:0001",
        label: "Policy permits on-device processing only, so a hosted request is held at the on-device default instead of widening",
        claim_posture: VoiceClaimPosture::ClaimedBeta,
        request: VoiceProviderRoutingRequest {
            requested_provider_id: Some(HOSTED_APPROVED_ID.to_owned()),
            requested_locality: ProcessingLocalityCue::HostedRemoteDisclosed,
            requested_language_profile: en_us_bundled(),
            requested_retention_export: local_controls(),
            policy_state: VoicePolicyState::EnterprisePolicyManaged,
            hosted_permitted_by_policy: false,
            entitlement_state: VoiceEntitlementState::Granted,
        },
        candidates: vec![local_default_candidate(), hosted_candidate(true)],
    })
}

/// Policy blocks voice entirely; the outcome is an explicit block with the
/// keyboard path intact.
fn policy_blocks_voice_row() -> VoiceProviderRoutingRow {
    row(Scenario {
        scenario_id: "voice-routing:policy-blocks-voice:0001",
        label: "Policy blocks voice in this context, producing an explicit blocked state with the keyboard path intact",
        claim_posture: VoiceClaimPosture::ClaimedBeta,
        request: VoiceProviderRoutingRequest {
            requested_provider_id: None,
            requested_locality: ProcessingLocalityCue::LocalOnDevice,
            requested_language_profile: en_us_bundled(),
            requested_retention_export: local_controls(),
            policy_state: VoicePolicyState::PolicyBlocked,
            hosted_permitted_by_policy: false,
            entitlement_state: VoiceEntitlementState::NotRequired,
        },
        candidates: vec![local_default_candidate(), hosted_candidate(true)],
    })
}

/// An entitlement upgrade is required; voice is held at the more-private
/// on-device default rather than routed to the gated hosted provider.
fn entitlement_upgrade_held_local_row() -> VoiceProviderRoutingRow {
    row(Scenario {
        scenario_id: "voice-routing:entitlement-upgrade-held-local:0001",
        label: "A hosted provider needing an entitlement upgrade is denied; voice is held at the on-device default, not widened",
        claim_posture: VoiceClaimPosture::ClaimedPreview,
        request: VoiceProviderRoutingRequest {
            requested_provider_id: Some(HOSTED_APPROVED_ID.to_owned()),
            requested_locality: ProcessingLocalityCue::HostedRemoteDisclosed,
            requested_language_profile: en_us_bundled(),
            requested_retention_export: local_controls(),
            policy_state: VoicePolicyState::UserControlled,
            hosted_permitted_by_policy: true,
            entitlement_state: VoiceEntitlementState::RequiresUpgrade,
        },
        candidates: vec![local_default_candidate(), hosted_candidate(true)],
    })
}

/// An entitlement was revoked and no on-device fallback exists; the outcome is an
/// explicit block, never a silent widening.
fn entitlement_revoked_blocked_row() -> VoiceProviderRoutingRow {
    row(Scenario {
        scenario_id: "voice-routing:entitlement-revoked-blocked:0001",
        label: "A revoked entitlement with no on-device fallback produces an explicit blocked state rather than a silent fallback",
        claim_posture: VoiceClaimPosture::ClaimedPreview,
        request: VoiceProviderRoutingRequest {
            requested_provider_id: Some(HOSTED_APPROVED_ID.to_owned()),
            requested_locality: ProcessingLocalityCue::HostedRemoteDisclosed,
            requested_language_profile: en_us_bundled(),
            requested_retention_export: hosted_controls(),
            policy_state: VoicePolicyState::EnterprisePolicyManaged,
            hosted_permitted_by_policy: true,
            entitlement_state: VoiceEntitlementState::Revoked,
        },
        candidates: vec![hosted_candidate(true)],
    })
}

/// The requested hosted provider is unavailable; voice is held at the on-device
/// default.
fn provider_unavailable_held_local_row() -> VoiceProviderRoutingRow {
    row(Scenario {
        scenario_id: "voice-routing:provider-unavailable-held-local:0001",
        label: "An unavailable hosted provider falls back to the on-device default with an explicit downgrade note",
        claim_posture: VoiceClaimPosture::ClaimedBeta,
        request: VoiceProviderRoutingRequest {
            requested_provider_id: Some(HOSTED_APPROVED_ID.to_owned()),
            requested_locality: ProcessingLocalityCue::HostedRemoteDisclosed,
            requested_language_profile: en_us_bundled(),
            requested_retention_export: local_controls(),
            policy_state: VoicePolicyState::UserControlled,
            hosted_permitted_by_policy: true,
            entitlement_state: VoiceEntitlementState::Granted,
        },
        candidates: vec![local_default_candidate(), hosted_candidate(false)],
    })
}

fn seeded_rows() -> Vec<VoiceProviderRoutingRow> {
    vec![
        local_first_default_row(),
        hosted_opt_in_disclosed_row(),
        language_pack_unavailable_row(),
        policy_requires_local_only_row(),
        policy_blocks_voice_row(),
        entitlement_upgrade_held_local_row(),
        entitlement_revoked_blocked_row(),
        provider_unavailable_held_local_row(),
    ]
}

/// The canonical, validating voice provider routing packet that the checked-in
/// support export, the routing fixtures, and the conformance tests all share.
///
/// The seed covers the local-first default, an opted-in hosted switch that
/// discloses the retention/export change, a language-pack-unavailable baseline
/// downgrade, a policy-local-only downgrade, a policy block, an entitlement
/// upgrade held at local, an entitlement-revoked block, and a hosted
/// provider-unavailable downgrade.
pub fn seeded_voice_provider_routing_packet() -> VoiceProviderRoutingPacket {
    VoiceProviderRoutingPacket::new(VoiceProviderRoutingPacketInput {
        packet_id: SEED_VOICE_PROVIDER_ROUTING_PACKET_ID.to_owned(),
        label: "Voice Provider Routing & Privacy-Gating".to_owned(),
        rows: seeded_rows(),
        guardrails: VoiceProviderRoutingGuardrails {
            provider_and_locality_inspectable: true,
            local_first_default_visible: true,
            switching_never_hides_retention_or_export: true,
            denials_block_instead_of_widening: true,
            no_silent_fallback_to_less_private: true,
            audio_transcript_never_leave_declared_model: true,
            keyboard_fallback_always_available: true,
        },
        consumer_projection: VoiceProviderRoutingConsumerProjection {
            settings_ingests_routing: true,
            admin_ingests_routing: true,
            diagnostics_ingests_routing: true,
            support_export_ingests_routing: true,
            active_provider_visible_without_settings_dive: true,
        },
        source_contract_refs: vec![
            VOICE_RETENTION_EXPORT_SCHEMA_REF.to_owned(),
            VOICE_PROVIDER_DESCRIPTOR_SCHEMA_REF.to_owned(),
            VOICE_SESSION_STATE_SCHEMA_REF.to_owned(),
            VOICE_PROCESSING_AND_RETENTION_DOC_REF.to_owned(),
            VOICE_PROVIDER_ROUTING_ARTIFACT_REF.to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_VOICE_PROVIDER_ROUTING_MINTED_AT.to_owned(),
    })
}

/// Stable fixture file name for a scenario row (derived from the scenario slug).
pub fn row_fixture_file_name(scenario_id: &str) -> String {
    let slug = scenario_id.split(':').nth(1).unwrap_or(scenario_id);
    format!("{slug}.json")
}

/// Writes the seeded packet, the per-scenario fixtures, and the compact summary
/// to `dir`. This is the single mint path the example dump and the equality test
/// share, so the checked-in fixtures can never drift silently.
pub fn write_fixtures(dir: &Path, packet: &VoiceProviderRoutingPacket) -> io::Result<()> {
    fs::create_dir_all(dir)?;

    let packet_json =
        fixture_json(packet).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    fs::write(dir.join("packet.json"), packet_json)?;

    for r in &packet.rows {
        let json = fixture_json(r).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        fs::write(dir.join(row_fixture_file_name(&r.scenario_id)), json)?;
    }

    let mut compact = packet.compact_lines().join("\n");
    compact.push('\n');
    fs::write(dir.join("compact.txt"), compact)?;

    Ok(())
}

/// Repo-relative fixtures dir, re-exported for the example dump and tests.
pub const FIXTURES_DIR_REF: &str = VOICE_PROVIDER_ROUTING_FIXTURES_DIR_REF;
