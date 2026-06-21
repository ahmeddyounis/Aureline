use super::seed::{seeded_voice_provider_routing_packet, HOSTED_APPROVED_ID, LOCAL_DEFAULT_ID};
use super::*;

fn packet() -> VoiceProviderRoutingPacket {
    seeded_voice_provider_routing_packet()
}

#[test]
fn seeded_packet_validates() {
    let violations = packet().validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn seeded_packet_covers_every_decision_class() {
    let decisions = packet().represented_decisions();
    for required in [
        VoiceRoutingDecision::RoutedAsRequested,
        VoiceRoutingDecision::RoutedLocalFirstDefault,
        VoiceRoutingDecision::LanguageProfileDowngraded,
        VoiceRoutingDecision::DowngradedToMorePrivate,
        VoiceRoutingDecision::BlockedExplicit,
    ] {
        assert!(
            decisions.contains(&required),
            "missing decision {}",
            required.as_str()
        );
    }
}

#[test]
fn local_first_default_routes_on_device_with_no_change() {
    let row = packet()
        .row("voice-routing:local-first-default:0001")
        .expect("local-first row")
        .clone();
    assert_eq!(
        row.outcome.decision,
        VoiceRoutingDecision::RoutedLocalFirstDefault
    );
    assert_eq!(
        row.outcome.active_locality,
        ProcessingLocalityCue::LocalOnDevice
    );
    assert!(row.outcome.denial_reason.is_none());
    assert!(!row.outcome.retention_changed_from_request);
    assert!(!row.outcome.export_changed_from_request);
    // The active provider and locality are inspectable (acceptance criterion 1).
    assert_eq!(
        row.outcome.active_provider_id.as_deref(),
        Some(LOCAL_DEFAULT_ID)
    );
}

#[test]
fn hosted_switch_surfaces_retention_and_export_change() {
    let row = packet()
        .row("voice-routing:hosted-opt-in-disclosed:0001")
        .expect("hosted row")
        .clone();
    assert_eq!(
        row.outcome.decision,
        VoiceRoutingDecision::RoutedAsRequested
    );
    assert_eq!(
        row.outcome.active_locality,
        ProcessingLocalityCue::HostedRemoteDisclosed
    );
    // Switching to hosted must not hide the retention/export change (criterion 2).
    assert!(row.outcome.retention_changed_from_request);
    assert!(row.outcome.export_changed_from_request);
}

#[test]
fn language_pack_unavailable_falls_back_to_baseline() {
    let row = packet()
        .row("voice-routing:language-pack-unavailable:0001")
        .expect("language row")
        .clone();
    assert_eq!(
        row.outcome.decision,
        VoiceRoutingDecision::LanguageProfileDowngraded
    );
    assert_eq!(
        row.outcome.denial_reason,
        Some(VoiceRoutingDenialReason::LanguagePackUnavailable)
    );
    assert!(row.outcome.language_changed_from_request);
    assert_eq!(
        row.outcome
            .active_language_profile
            .as_ref()
            .map(|p| p.language_tag.as_str()),
        Some("en-US")
    );
    // Still on-device; the fallback never widens privacy.
    assert_eq!(
        row.outcome.active_locality,
        ProcessingLocalityCue::LocalOnDevice
    );
}

#[test]
fn policy_blocks_voice_explicitly() {
    let row = packet()
        .row("voice-routing:policy-blocks-voice:0001")
        .expect("policy-block row")
        .clone();
    assert_eq!(row.outcome.decision, VoiceRoutingDecision::BlockedExplicit);
    assert_eq!(
        row.outcome.denial_reason,
        Some(VoiceRoutingDenialReason::PolicyBlocksVoice)
    );
    assert!(row.outcome.active_provider_id.is_none());
    assert_eq!(
        row.outcome.active_locality,
        ProcessingLocalityCue::ProcessingUnavailable
    );
    assert!(row.outcome.keyboard_fallback_available);
}

#[test]
fn policy_local_only_downgrades_to_more_private() {
    let row = packet()
        .row("voice-routing:policy-requires-local-only:0001")
        .expect("policy-local row")
        .clone();
    assert_eq!(
        row.outcome.decision,
        VoiceRoutingDecision::DowngradedToMorePrivate
    );
    assert_eq!(
        row.outcome.denial_reason,
        Some(VoiceRoutingDenialReason::PolicyRequiresLocalOnly)
    );
    // Held at a strictly more private engine, never widened (criterion 3).
    assert_eq!(
        row.outcome.active_locality,
        ProcessingLocalityCue::LocalOnDevice
    );
    assert!(row.outcome.denial_never_widens(&row.request));
}

#[test]
fn entitlement_denials_never_widen_to_less_private() {
    let p = packet();
    let downgrade = p
        .row("voice-routing:entitlement-upgrade-held-local:0001")
        .expect("entitlement downgrade row");
    assert_eq!(
        downgrade.outcome.decision,
        VoiceRoutingDecision::DowngradedToMorePrivate
    );
    assert_eq!(
        downgrade.outcome.active_locality,
        ProcessingLocalityCue::LocalOnDevice
    );

    let blocked = p
        .row("voice-routing:entitlement-revoked-blocked:0001")
        .expect("entitlement blocked row");
    assert_eq!(
        blocked.outcome.decision,
        VoiceRoutingDecision::BlockedExplicit
    );
    assert_eq!(
        blocked.outcome.denial_reason,
        Some(VoiceRoutingDenialReason::EntitlementRevoked)
    );
    // No silent fallback to a broader/less private provider.
    assert!(blocked.outcome.active_provider_id.is_none());
}

#[test]
fn provider_unavailable_holds_local() {
    let row = packet()
        .row("voice-routing:provider-unavailable-held-local:0001")
        .expect("provider-unavailable row")
        .clone();
    assert_eq!(
        row.outcome.decision,
        VoiceRoutingDecision::DowngradedToMorePrivate
    );
    assert_eq!(
        row.outcome.denial_reason,
        Some(VoiceRoutingDenialReason::RequestedProviderUnavailable)
    );
    assert_eq!(
        row.outcome.active_locality,
        ProcessingLocalityCue::LocalOnDevice
    );
}

#[test]
fn resolver_never_routes_to_less_private_on_denial() {
    // A direct resolver probe: hosted requested, hosted denied by policy, only a
    // local default available — the active locality must be local, never hosted.
    let request = VoiceProviderRoutingRequest {
        requested_provider_id: Some(HOSTED_APPROVED_ID.to_owned()),
        requested_locality: ProcessingLocalityCue::HostedRemoteDisclosed,
        requested_language_profile: VoiceLanguageProfile {
            language_tag: "en-US".to_owned(),
            acoustic_profile_class: VoiceAcousticProfileClass::DefaultAcoustic,
            pack_availability: VoiceLanguagePackAvailability::BundledLocal,
            profile_label: "English (US)".to_owned(),
        },
        requested_retention_export: VoiceRetentionExportControls::new(
            RetentionMode::NoAudioNoTranscriptRetained,
            AudioRetentionClass::NoAudioRetained,
            TranscriptExportPosture::NoTranscriptExport,
            true,
            "local only",
        ),
        policy_state: VoicePolicyState::EnterprisePolicyManaged,
        hosted_permitted_by_policy: false,
        entitlement_state: VoiceEntitlementState::Granted,
    };
    let local = seeded_voice_provider_routing_packet()
        .row("voice-routing:local-first-default:0001")
        .expect("local row")
        .candidates
        .clone();
    let outcome = resolve_voice_routing(&request, &local);
    assert!(outcome.denial_never_widens(&request));
    assert!(outcome.privacy_never_widened(&request));
    assert_ne!(
        outcome.active_locality,
        ProcessingLocalityCue::HostedRemoteDisclosed
    );
}

#[test]
fn tampered_outcome_is_rejected() {
    let mut p = packet();
    let row = p
        .rows
        .iter_mut()
        .find(|r| r.scenario_id == "voice-routing:policy-blocks-voice:0001")
        .expect("policy block row");
    // Pretend the block actually routed to the hosted provider.
    row.outcome.decision = VoiceRoutingDecision::RoutedAsRequested;
    row.outcome.active_provider_id = Some(HOSTED_APPROVED_ID.to_owned());
    row.outcome.active_locality = ProcessingLocalityCue::HostedRemoteDisclosed;
    let violations = p.validate();
    assert!(violations.contains(&VoiceRoutingViolation::OutcomeDoesNotMatchResolver));
}

#[test]
fn raw_transcript_retention_is_rejected() {
    let mut p = packet();
    let row = p
        .rows
        .iter_mut()
        .find(|r| r.scenario_id == "voice-routing:local-first-default:0001")
        .expect("local row");
    row.candidates[0]
        .retention_export
        .raw_transcripts_excluded_by_default = false;
    let violations = p.validate();
    assert!(violations.contains(&VoiceRoutingViolation::RawTranscriptRetainedByDefault));
}

#[test]
fn generic_disclosure_label_is_rejected() {
    let mut p = packet();
    let row = p
        .rows
        .iter_mut()
        .find(|r| r.scenario_id == "voice-routing:policy-blocks-voice:0001")
        .expect("policy block row");
    row.outcome.disclosure_label = "error".to_owned();
    let violations = p.validate();
    assert!(violations.contains(&VoiceRoutingViolation::DisclosureLabelGeneric));
}

#[test]
fn fingerprint_must_be_independent_of_id() {
    let mut p = packet();
    let row = p
        .rows
        .iter_mut()
        .find(|r| r.scenario_id == "voice-routing:local-first-default:0001")
        .expect("local row");
    row.fingerprint_token = row.scenario_id.clone();
    let violations = p.validate();
    assert!(violations.contains(&VoiceRoutingViolation::FingerprintSubstitutesIdentity));
}

#[test]
fn missing_source_contract_is_rejected() {
    let mut p = packet();
    p.source_contract_refs
        .retain(|r| r != VOICE_RETENTION_EXPORT_SCHEMA_REF);
    assert!(p
        .validate()
        .contains(&VoiceRoutingViolation::MissingSourceContracts));
}

#[test]
fn wrong_record_kind_is_rejected() {
    let mut p = packet();
    p.record_kind = "something_else".to_owned();
    assert!(p
        .validate()
        .contains(&VoiceRoutingViolation::WrongRecordKind));
}

#[test]
fn retention_export_controls_round_trip() {
    let controls = local_controls_probe();
    assert!(controls.is_well_formed());
    assert!(controls.is_local_only());
    let json = controls.export_safe_json();
    let parsed: VoiceRetentionExportControls = serde_json::from_str(&json).expect("round-trips");
    assert_eq!(parsed, controls);
}

fn local_controls_probe() -> VoiceRetentionExportControls {
    VoiceRetentionExportControls::new(
        RetentionMode::NoAudioNoTranscriptRetained,
        AudioRetentionClass::NoAudioRetained,
        TranscriptExportPosture::NoTranscriptExport,
        true,
        "On-device only — no audio or transcript leaves this device",
    )
}

#[test]
fn export_safe_json_round_trips() {
    let p = packet();
    let json = p.export_safe_json();
    let parsed: VoiceProviderRoutingPacket = serde_json::from_str(&json).expect("round-trips");
    assert_eq!(parsed, p);
}

#[test]
fn markdown_and_compact_name_scenarios() {
    let p = packet();
    let markdown = p.render_markdown();
    assert!(markdown.contains("Voice Provider Routing"));
    assert!(markdown.contains("local-first-default"));
    let compact = p.compact_lines();
    assert_eq!(compact.len(), p.rows.len() + 1);
    assert!(compact[0].contains("invariants_ok=true"));
}

#[test]
fn checked_support_export_matches_builder() {
    let checked =
        current_voice_provider_routing_export().expect("checked voice routing export validates");
    assert_eq!(checked, packet());
}
