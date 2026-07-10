use super::*;

const PACKET_ID: &str = "media-compare-controls:stable:0001";

const ARTIFACT_SNAPSHOT: &str = "artifact:design/home-snapshot.png";
const ARTIFACT_REPORT: &str = "artifact:report/coverage.html";
const ARTIFACT_CAPTURE: &str = "artifact:media/session-capture.mp4";

fn rendered_compare_viewers() -> Vec<RenderedCompareViewer> {
    vec![
        RenderedCompareViewer {
            component: M5ArtifactComponent::RenderedCompareViewer,
            viewer_id: "viewer:snapshot".to_owned(),
            artifact_ref: ARTIFACT_SNAPSHOT.to_owned(),
            artifact_class_label: "design snapshot".to_owned(),
            trust_class: RenderTrustClass::SandboxedTrusted,
            scale_or_dimension_metadata: "1440x900 @2x, 100% scale".to_owned(),
            alt_text_fallback: "Home screen with sidebar collapsed and hero banner".to_owned(),
            sandbox_note: "Rendered inside an isolated preview sandbox".to_owned(),
            untrusted_render_note: String::new(),
            raw_fallback_label: String::new(),
            redaction_note: String::new(),
            available_actions: vec![
                RenderedViewerAction::OpenRaw,
                RenderedViewerAction::Export,
                RenderedViewerAction::ToggleScale,
                RenderedViewerAction::ViewTextFallback,
                RenderedViewerAction::CompareSideBySide,
            ],
            schema_fidelity: M5ArtifactFidelityState::StructuredFaithful,
            raw_context_action: "Open the raw snapshot bytes".to_owned(),
            rollback_posture: M5ArtifactComponentRollbackPosture::CompareOnlyNoWriteBack,
            fields_shown: vec![
                "trust_class".to_owned(),
                "scale_or_dimension_metadata".to_owned(),
                "alt_text_fallback".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_RENDERED_VIEWER_CONTRACT_REF.to_owned()
            ],
        },
        RenderedCompareViewer {
            component: M5ArtifactComponent::RenderedCompareViewer,
            viewer_id: "viewer:report".to_owned(),
            artifact_ref: ARTIFACT_REPORT.to_owned(),
            artifact_class_label: "rendered coverage report".to_owned(),
            trust_class: RenderTrustClass::SandboxedUntrusted,
            scale_or_dimension_metadata: "980px wide, fit-to-width".to_owned(),
            alt_text_fallback: "Coverage report: 82% lines, 74% branches covered".to_owned(),
            sandbox_note: "Rendered inside a sandbox with scripting disabled".to_owned(),
            untrusted_render_note: "The report is untrusted HTML; rendered read-only in a sandbox"
                .to_owned(),
            raw_fallback_label: String::new(),
            redaction_note: String::new(),
            available_actions: vec![
                RenderedViewerAction::OpenRaw,
                RenderedViewerAction::Export,
                RenderedViewerAction::ViewTextFallback,
            ],
            schema_fidelity: M5ArtifactFidelityState::RenderUntrusted,
            raw_context_action: "Open the raw report source".to_owned(),
            rollback_posture: M5ArtifactComponentRollbackPosture::ReadOnlyNoMutation,
            fields_shown: vec![
                "trust_class".to_owned(),
                "untrusted_render_note".to_owned(),
                "alt_text_fallback".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_RENDERED_VIEWER_CONTRACT_REF.to_owned()
            ],
        },
        RenderedCompareViewer {
            component: M5ArtifactComponent::RenderedCompareViewer,
            viewer_id: "viewer:capture".to_owned(),
            artifact_ref: ARTIFACT_CAPTURE.to_owned(),
            artifact_class_label: "session video capture".to_owned(),
            trust_class: RenderTrustClass::RawTextFallback,
            scale_or_dimension_metadata: "1920x1080, 30fps".to_owned(),
            alt_text_fallback: "12s screen capture of the checkout flow".to_owned(),
            sandbox_note: String::new(),
            untrusted_render_note: String::new(),
            raw_fallback_label:
                "No trusted inline render; showing an explicitly labeled raw/text fallback"
                    .to_owned(),
            redaction_note: String::new(),
            available_actions: vec![
                RenderedViewerAction::OpenRaw,
                RenderedViewerAction::Export,
                RenderedViewerAction::ViewTextFallback,
            ],
            schema_fidelity: M5ArtifactFidelityState::RawFallback,
            raw_context_action: "Open the raw capture file".to_owned(),
            rollback_posture: M5ArtifactComponentRollbackPosture::ReadOnlyNoMutation,
            fields_shown: vec![
                "trust_class".to_owned(),
                "raw_fallback_label".to_owned(),
                "alt_text_fallback".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_RENDERED_VIEWER_CONTRACT_REF.to_owned()
            ],
        },
    ]
}

fn media_metadata_rails() -> Vec<MediaMetadataRail> {
    vec![
        MediaMetadataRail {
            component: M5ArtifactComponent::MediaMetadataRail,
            rail_id: "rail:snapshot".to_owned(),
            artifact_ref: ARTIFACT_SNAPSHOT.to_owned(),
            artifact_kind: MediaArtifactKind::DesignSnapshot,
            format_label: "PNG (sRGB)".to_owned(),
            measure_kind: MediaMeasureKind::Dimensions,
            measure_value: "1440x900 px".to_owned(),
            hidden_content_state: HiddenContentState::NoEmbeddedSensitiveContent,
            hidden_content_note: String::new(),
            safety_posture: MediaSafetyPosture::ExportSafe,
            share_scope: MediaShareScope::TeamShare,
            share_guidance: "Export-safe: shareable with the team and in support packets"
                .to_owned(),
            schema_fidelity: M5ArtifactFidelityState::StructuredFaithful,
            raw_context_action: "Open the raw snapshot bytes".to_owned(),
            rollback_posture: M5ArtifactComponentRollbackPosture::ReadOnlyNoMutation,
            fields_shown: vec![
                "format_label".to_owned(),
                "measure_value".to_owned(),
                "safety_posture".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_MEDIA_RAIL_CONTRACT_REF.to_owned()
            ],
        },
        MediaMetadataRail {
            component: M5ArtifactComponent::MediaMetadataRail,
            rail_id: "rail:report".to_owned(),
            artifact_ref: ARTIFACT_REPORT.to_owned(),
            artifact_kind: MediaArtifactKind::RenderedDocument,
            format_label: "HTML report".to_owned(),
            measure_kind: MediaMeasureKind::ByteSize,
            measure_value: "412 KB".to_owned(),
            hidden_content_state: HiddenContentState::EmbeddedContentScanUnknown,
            hidden_content_note:
                "This report was not scanned for embedded content; treat share with care".to_owned(),
            safety_posture: MediaSafetyPosture::Sandboxed,
            share_scope: MediaShareScope::LocalOnly,
            share_guidance: "Local-only until the embedded-content scan completes".to_owned(),
            schema_fidelity: M5ArtifactFidelityState::RenderUntrusted,
            raw_context_action: "Open the raw report source".to_owned(),
            rollback_posture: M5ArtifactComponentRollbackPosture::ReadOnlyNoMutation,
            fields_shown: vec![
                "format_label".to_owned(),
                "measure_value".to_owned(),
                "hidden_content_state".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_MEDIA_RAIL_CONTRACT_REF.to_owned()
            ],
        },
        MediaMetadataRail {
            component: M5ArtifactComponent::MediaMetadataRail,
            rail_id: "rail:capture".to_owned(),
            artifact_ref: ARTIFACT_CAPTURE.to_owned(),
            artifact_kind: MediaArtifactKind::Video,
            format_label: "MP4 (H.264)".to_owned(),
            measure_kind: MediaMeasureKind::Duration,
            measure_value: "00:00:12".to_owned(),
            hidden_content_state: HiddenContentState::EmbeddedSensitiveContentPresent,
            hidden_content_note:
                "Capture contains on-screen account details; keep local until sanitized".to_owned(),
            safety_posture: MediaSafetyPosture::RawUnsanitized,
            share_scope: MediaShareScope::LocalOnly,
            share_guidance: "Local-only: sanitize before sharing or exporting".to_owned(),
            schema_fidelity: M5ArtifactFidelityState::RawFallback,
            raw_context_action: "Open the raw capture file".to_owned(),
            rollback_posture: M5ArtifactComponentRollbackPosture::ReadOnlyNoMutation,
            fields_shown: vec![
                "format_label".to_owned(),
                "measure_value".to_owned(),
                "hidden_content_state".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_MEDIA_RAIL_CONTRACT_REF.to_owned()
            ],
        },
    ]
}

fn redaction_trust_badge_sets() -> Vec<RedactionOrTrustBadgeSet> {
    vec![
        RedactionOrTrustBadgeSet {
            component: M5ArtifactComponent::RedactionOrTrustBadgeSet,
            badge_set_id: "badges:snapshot".to_owned(),
            artifact_ref: ARTIFACT_SNAPSHOT.to_owned(),
            redaction_state: RedactionState::NotRedacted,
            trust_level: TrustLevel::Trusted,
            available_badges: vec![
                TrustBadge::Trusted,
                TrustBadge::ExportSafe,
                TrustBadge::Sanitized,
            ],
            redaction_note: String::new(),
            untrusted_note: String::new(),
            share_guidance: "Trusted and export-safe: shareable with the team".to_owned(),
            export_posture_preserved: true,
            schema_fidelity: M5ArtifactFidelityState::StructuredFaithful,
            raw_context_action: "Open the raw snapshot bytes".to_owned(),
            rollback_posture: M5ArtifactComponentRollbackPosture::ReadOnlyNoMutation,
            fields_shown: vec![
                "redaction_state".to_owned(),
                "trust_level".to_owned(),
                "available_badges".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_REDACTION_BADGE_CONTRACT_REF.to_owned()
            ],
        },
        RedactionOrTrustBadgeSet {
            component: M5ArtifactComponent::RedactionOrTrustBadgeSet,
            badge_set_id: "badges:report".to_owned(),
            artifact_ref: ARTIFACT_REPORT.to_owned(),
            redaction_state: RedactionState::PartiallyRedacted,
            trust_level: TrustLevel::SandboxedOnly,
            available_badges: vec![
                TrustBadge::Redacted,
                TrustBadge::Sandboxed,
                TrustBadge::LocalOnly,
            ],
            redaction_note: "Account identifiers are redacted from the rendered report".to_owned(),
            untrusted_note: String::new(),
            share_guidance: "Sandboxed-only until the scan clears; redaction preserved on export"
                .to_owned(),
            export_posture_preserved: true,
            schema_fidelity: M5ArtifactFidelityState::RenderUntrusted,
            raw_context_action: "Open the raw report source".to_owned(),
            rollback_posture: M5ArtifactComponentRollbackPosture::ReadOnlyNoMutation,
            fields_shown: vec![
                "redaction_state".to_owned(),
                "trust_level".to_owned(),
                "redaction_note".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_REDACTION_BADGE_CONTRACT_REF.to_owned()
            ],
        },
        RedactionOrTrustBadgeSet {
            component: M5ArtifactComponent::RedactionOrTrustBadgeSet,
            badge_set_id: "badges:capture".to_owned(),
            artifact_ref: ARTIFACT_CAPTURE.to_owned(),
            redaction_state: RedactionState::FullyRedacted,
            trust_level: TrustLevel::Untrusted,
            available_badges: vec![
                TrustBadge::Redacted,
                TrustBadge::Untrusted,
                TrustBadge::LocalOnly,
            ],
            redaction_note: "The capture is fully redacted; only metadata is shareable".to_owned(),
            untrusted_note: "Untrusted media source; keep local until reviewed".to_owned(),
            share_guidance: "Local-only: redaction and trust posture preserved on any export"
                .to_owned(),
            export_posture_preserved: true,
            schema_fidelity: M5ArtifactFidelityState::RedactedOrWithheld,
            raw_context_action: "Open the raw capture file".to_owned(),
            rollback_posture: M5ArtifactComponentRollbackPosture::ReadOnlyNoMutation,
            fields_shown: vec![
                "redaction_state".to_owned(),
                "trust_level".to_owned(),
                "untrusted_note".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_REDACTION_BADGE_CONTRACT_REF.to_owned()
            ],
        },
    ]
}

fn trust_review() -> MediaCompareControlsTrustReview {
    MediaCompareControlsTrustReview {
        render_trust_always_explicit: true,
        accessibility_fallback_always_present: true,
        scale_or_dimension_metadata_present: true,
        hidden_content_state_disclosed: true,
        metadata_visibility_explicit: true,
        sanitized_or_export_safe_posture_explicit: true,
        share_guidance_explicit: true,
        redaction_posture_preserved_on_export: true,
        raw_export_safe_fallback_explicit: true,
        raw_context_always_reachable: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> MediaCompareControlsConsumerProjection {
    MediaCompareControlsConsumerProjection {
        rendered_viewer_shows_render_trust_and_scale: true,
        rendered_viewer_shows_alt_text_fallback: true,
        media_rail_shows_format_and_measure: true,
        media_rail_shows_hidden_content_and_posture: true,
        badge_set_shows_redaction_and_trust: true,
        share_export_preserves_posture: true,
        raw_context_reachable_from_all: true,
        cli_headless_shows_truth: true,
        support_export_shows_truth: true,
        help_about_shows_truth: true,
    }
}

fn proof_freshness() -> MediaCompareControlsProofFreshness {
    MediaCompareControlsProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-07-08T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<MediaCompareControlsDowngradeTrigger> {
    vec![
        MediaCompareControlsDowngradeTrigger::ProofStale,
        MediaCompareControlsDowngradeTrigger::RenderUntrusted,
        MediaCompareControlsDowngradeTrigger::HiddenContentDetected,
        MediaCompareControlsDowngradeTrigger::RedactionApplied,
        MediaCompareControlsDowngradeTrigger::UpstreamDependencyNarrowed,
    ]
}

fn consumer_surfaces() -> Vec<MediaCompareControlsConsumerSurface> {
    vec![
        MediaCompareControlsConsumerSurface::DiffCompareView,
        MediaCompareControlsConsumerSurface::DesignSnapshotReview,
        MediaCompareControlsConsumerSurface::ArtifactBrowser,
        MediaCompareControlsConsumerSurface::CliHeadless,
        MediaCompareControlsConsumerSurface::SupportExport,
    ]
}

fn source_contract_refs() -> Vec<String> {
    vec![
        MEDIA_COMPARE_CONTROLS_SCHEMA_REF.to_owned(),
        MEDIA_COMPARE_CONTROLS_DOC_REF.to_owned(),
        M5_ARTIFACT_COMPONENT_MATRIX_SCHEMA_REF.to_owned(),
        M5_ARTIFACT_COMPONENT_MATRIX_RENDERED_VIEWER_CONTRACT_REF.to_owned(),
        M5_ARTIFACT_COMPONENT_MATRIX_MEDIA_RAIL_CONTRACT_REF.to_owned(),
        M5_ARTIFACT_COMPONENT_MATRIX_REDACTION_BADGE_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> MediaCompareControlsPacket {
    MediaCompareControlsPacket::new(MediaCompareControlsPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label: "Rendered compare viewers, media rails, and trust badge sets".to_owned(),
        rendered_compare_viewers: rendered_compare_viewers(),
        media_metadata_rails: media_metadata_rails(),
        redaction_trust_badge_sets: redaction_trust_badge_sets(),
        downgrade_triggers: downgrade_triggers(),
        consumer_surfaces: consumer_surfaces(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-08T00:00:00Z".to_owned(),
    })
}

#[test]
fn media_compare_controls_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn rendered_viewer_resolver_derives_from_trust_class() {
    let trusted = resolve_rendered_viewer_disclosure(RenderTrustClass::SandboxedTrusted);
    assert!(trusted.needs_sandbox_note);
    assert!(trusted.render_directly_trusted);
    assert!(!trusted.needs_untrusted_render_note);

    let untrusted = resolve_rendered_viewer_disclosure(RenderTrustClass::SandboxedUntrusted);
    assert!(untrusted.needs_sandbox_note);
    assert!(untrusted.needs_untrusted_render_note);
    assert!(!untrusted.render_directly_trusted);

    let raw = resolve_rendered_viewer_disclosure(RenderTrustClass::RawTextFallback);
    assert!(!raw.needs_sandbox_note);
    assert!(raw.needs_raw_fallback_label);

    let redacted = resolve_rendered_viewer_disclosure(RenderTrustClass::RedactedWithheld);
    assert!(redacted.needs_redaction_note);
    assert!(!redacted.render_directly_trusted);
}

#[test]
fn media_rail_resolver_derives_from_state() {
    let none = resolve_media_rail_disclosure(HiddenContentState::NoEmbeddedSensitiveContent);
    assert!(!none.needs_hidden_content_note);

    let present =
        resolve_media_rail_disclosure(HiddenContentState::EmbeddedSensitiveContentPresent);
    assert!(present.needs_hidden_content_note);
    assert!(!present.needs_unknown_scan_note);

    let unknown = resolve_media_rail_disclosure(HiddenContentState::EmbeddedContentScanUnknown);
    assert!(unknown.needs_hidden_content_note);
    assert!(unknown.needs_unknown_scan_note);
}

#[test]
fn badge_set_resolver_derives_from_state() {
    let clean = resolve_badge_set_disclosure(RedactionState::NotRedacted, TrustLevel::Trusted);
    assert!(!clean.needs_redaction_note);
    assert!(!clean.needs_untrusted_note);

    let redacted =
        resolve_badge_set_disclosure(RedactionState::PartiallyRedacted, TrustLevel::Untrusted);
    assert!(redacted.needs_redaction_note);
    assert!(redacted.needs_untrusted_note);
}

#[test]
fn scale_or_dimension_metadata_missing_fails() {
    let mut packet = packet();
    packet.rendered_compare_viewers[0].scale_or_dimension_metadata = String::new();
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::ScaleOrDimensionMetadataMissing));
}

#[test]
fn alt_text_fallback_missing_fails() {
    let mut packet = packet();
    packet.rendered_compare_viewers[0].alt_text_fallback = String::new();
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::AltTextFallbackMissing));
}

#[test]
fn sandbox_note_missing_fails() {
    let mut packet = packet();
    packet.rendered_compare_viewers[0].sandbox_note = String::new();
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::SandboxNoteMissing));
}

#[test]
fn untrusted_render_note_missing_fails() {
    let mut packet = packet();
    packet.rendered_compare_viewers[1].untrusted_render_note = String::new();
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::UntrustedRenderNoteMissing));
}

#[test]
fn raw_fallback_label_missing_fails() {
    let mut packet = packet();
    packet.rendered_compare_viewers[2].raw_fallback_label = String::new();
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::RawFallbackLabelMissing));
}

#[test]
fn render_redaction_note_missing_fails() {
    let mut packet = packet();
    // A withheld render must carry a redaction note.
    packet.rendered_compare_viewers[0].trust_class = RenderTrustClass::RedactedWithheld;
    packet.rendered_compare_viewers[0].redaction_note = String::new();
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::RenderRedactionNoteMissing));
}

#[test]
fn viewer_actions_missing_fails() {
    let mut packet = packet();
    packet.rendered_compare_viewers[0].available_actions.clear();
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::ViewerActionsMissing));
}

#[test]
fn open_raw_action_missing_fails() {
    let mut packet = packet();
    packet.rendered_compare_viewers[0].available_actions = vec![
        RenderedViewerAction::Export,
        RenderedViewerAction::ViewTextFallback,
    ];
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::OpenRawActionMissing));
}

#[test]
fn export_action_missing_fails() {
    let mut packet = packet();
    packet.rendered_compare_viewers[0].available_actions = vec![
        RenderedViewerAction::OpenRaw,
        RenderedViewerAction::ViewTextFallback,
    ];
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::ExportActionMissing));
}

#[test]
fn render_trust_class_coverage_missing_fails() {
    let mut packet = packet();
    packet
        .rendered_compare_viewers
        .retain(|viewer| viewer.trust_class != RenderTrustClass::RawTextFallback);
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::RenderTrustClassCoverageMissing));
}

#[test]
fn wrong_rendered_component_class_fails() {
    let mut packet = packet();
    packet.rendered_compare_viewers[0].component = M5ArtifactComponent::MediaMetadataRail;
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::RenderedCompareViewerWrongComponentClass));
}

#[test]
fn rendered_artifact_class_missing_fails() {
    let mut packet = packet();
    packet.rendered_compare_viewers[0].artifact_class_label = String::new();
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::RenderedArtifactClassMissing));
}

#[test]
fn media_format_missing_fails() {
    let mut packet = packet();
    packet.media_metadata_rails[0].format_label = String::new();
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::MediaFormatMissing));
}

#[test]
fn media_measure_missing_fails() {
    let mut packet = packet();
    packet.media_metadata_rails[0].measure_value = String::new();
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::MediaMeasureMissing));
}

#[test]
fn hidden_content_note_missing_fails() {
    let mut packet = packet();
    // The capture rail has embedded sensitive content and must disclose it.
    packet.media_metadata_rails[2].hidden_content_note = String::new();
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::HiddenContentNoteMissing));
}

#[test]
fn media_share_guidance_missing_fails() {
    let mut packet = packet();
    packet.media_metadata_rails[0].share_guidance = String::new();
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::MediaShareGuidanceMissing));
}

#[test]
fn unsanitized_hidden_content_shareable_fails() {
    let mut packet = packet();
    // Sharing present hidden content beyond local without a safe posture.
    packet.media_metadata_rails[2].share_scope = MediaShareScope::TeamShare;
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::UnsanitizedHiddenContentShareable));
}

#[test]
fn hidden_content_state_coverage_missing_fails() {
    let mut packet = packet();
    packet.media_metadata_rails.retain(|rail| {
        rail.hidden_content_state != HiddenContentState::EmbeddedSensitiveContentPresent
    });
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::HiddenContentStateCoverageMissing));
}

#[test]
fn wrong_media_component_class_fails() {
    let mut packet = packet();
    packet.media_metadata_rails[0].component = M5ArtifactComponent::RenderedCompareViewer;
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::MediaMetadataRailWrongComponentClass));
}

#[test]
fn badge_redaction_note_missing_fails() {
    let mut packet = packet();
    // The report badge set is partially redacted and must explain the redaction.
    packet.redaction_trust_badge_sets[1].redaction_note = String::new();
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::BadgeRedactionNoteMissing));
}

#[test]
fn untrusted_badge_note_missing_fails() {
    let mut packet = packet();
    // The capture badge set is untrusted and must carry an untrusted note.
    packet.redaction_trust_badge_sets[2].untrusted_note = String::new();
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::UntrustedBadgeNoteMissing));
}

#[test]
fn badge_share_guidance_missing_fails() {
    let mut packet = packet();
    packet.redaction_trust_badge_sets[0].share_guidance = String::new();
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::BadgeShareGuidanceMissing));
}

#[test]
fn export_posture_not_preserved_fails() {
    let mut packet = packet();
    packet.redaction_trust_badge_sets[1].export_posture_preserved = false;
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::ExportPostureNotPreserved));
}

#[test]
fn trust_badges_missing_fails() {
    let mut packet = packet();
    packet.redaction_trust_badge_sets[0]
        .available_badges
        .clear();
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::TrustBadgesMissing));
}

#[test]
fn redaction_state_coverage_missing_fails() {
    let mut packet = packet();
    packet
        .redaction_trust_badge_sets
        .retain(|set| set.redaction_state != RedactionState::FullyRedacted);
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::RedactionStateCoverageMissing));
}

#[test]
fn wrong_badge_component_class_fails() {
    let mut packet = packet();
    packet.redaction_trust_badge_sets[0].component = M5ArtifactComponent::MediaMetadataRail;
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::RedactionOrTrustBadgeSetWrongComponentClass));
}

#[test]
fn trust_badge_set_missing_fails() {
    let mut packet = packet();
    // A viewer whose artifact has no accompanying badge set.
    packet.rendered_compare_viewers[0].artifact_ref = "artifact:design/orphan.png".to_owned();
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::TrustBadgeSetMissing));
}

#[test]
fn raw_context_action_missing_fails() {
    let mut packet = packet();
    packet.rendered_compare_viewers[0].raw_context_action = String::new();
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::RawContextActionMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.accessibility_fallback_always_present = false;
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet.consumer_projection.share_export_preserves_posture = false;
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&MediaCompareControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_controls() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Rendered compare viewers"));
    assert!(summary.contains("## Media-metadata rails"));
    assert!(summary.contains("## Redaction / trust badge sets"));
    assert!(summary.contains("design snapshot"));
    assert!(summary.contains("sandboxed_untrusted"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_media_compare_controls_export()
        .expect("checked media compare controls export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-rendered-compare-media-trust-controls/untrusted_render_raw_fallback.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-rendered-compare-media-trust-controls/redacted_export_preserves_posture.json"
        )),
    ] {
        let packet: MediaCompareControlsPacket =
            serde_json::from_str(raw).expect("fixture parses as media compare controls packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

/// Regenerates the checked release artifacts and narrowed fixtures.
///
/// Guarded behind `GEN_MEDIA_COMPARE_CONTROLS_ARTIFACTS` so ordinary test runs
/// never touch the working tree.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_MEDIA_COMPARE_CONTROLS_ARTIFACTS").is_err() {
        return;
    }

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let repo_root = std::path::Path::new(manifest_dir).join("..").join("..");

    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    let proof_dir = repo_root
        .join("artifacts")
        .join("release")
        .join("m5-rendered-compare-media-trust-controls-proof");
    std::fs::create_dir_all(&proof_dir).expect("create proof dir");
    std::fs::write(
        proof_dir.join("support_export.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .expect("write support export");
    std::fs::write(
        proof_dir.join("summary.md"),
        packet.render_markdown_summary(),
    )
    .expect("write summary");

    let fixture_dir = repo_root
        .join("fixtures")
        .join("ui")
        .join("m5-rendered-compare-media-trust-controls");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");

    // Fixture 1: an untrusted render that falls back to an explicitly labeled raw
    // text view; the accessibility fallback and open-raw/export actions stay
    // present.
    let mut raw_fallback = packet.clone();
    raw_fallback.packet_id = "media-compare-controls:fixture:untrusted-raw-fallback".to_owned();
    if let Some(viewer) = raw_fallback
        .rendered_compare_viewers
        .iter_mut()
        .find(|viewer| viewer.artifact_ref == ARTIFACT_CAPTURE)
    {
        viewer.raw_fallback_label =
            "No trusted inline render for this capture; showing the labeled raw/text fallback"
                .to_owned();
        viewer.alt_text_fallback =
            "12s screen capture of the checkout flow (raw fallback shown)".to_owned();
    }
    assert!(
        raw_fallback.validate().is_empty(),
        "{:?}",
        raw_fallback.validate()
    );
    std::fs::write(
        fixture_dir.join("untrusted_render_raw_fallback.json"),
        format!("{}\n", raw_fallback.export_safe_json()),
    )
    .expect("write untrusted-raw-fallback fixture");

    // Fixture 2: a fully redacted capture whose redaction and trust posture is
    // preserved on export — never flattened into an ambiguous attachment.
    let mut redacted = packet.clone();
    redacted.packet_id = "media-compare-controls:fixture:redacted-export".to_owned();
    if let Some(set) = redacted
        .redaction_trust_badge_sets
        .iter_mut()
        .find(|set| set.artifact_ref == ARTIFACT_CAPTURE)
    {
        set.share_guidance =
            "Local-only: the full redaction and untrusted posture is preserved on any export"
                .to_owned();
        set.redaction_note =
            "The capture is fully redacted; only export-safe metadata leaves the boundary"
                .to_owned();
    }
    assert!(redacted.validate().is_empty(), "{:?}", redacted.validate());
    std::fs::write(
        fixture_dir.join("redacted_export_preserves_posture.json"),
        format!("{}\n", redacted.export_safe_json()),
    )
    .expect("write redacted-export fixture");
}
