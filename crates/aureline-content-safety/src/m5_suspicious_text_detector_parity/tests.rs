use super::*;

fn packet() -> M5SuspiciousTextParityPacket {
    frozen_m5_suspicious_text_parity_packet()
}

#[test]
fn frozen_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn covers_every_m5_surface_once() {
    let packet = packet();
    assert!(packet.covers_all_m5_surfaces());
    let present: BTreeSet<_> = packet.surfaces.iter().map(|s| s.surface).collect();
    for surface in M5SuspiciousTextSurface::ALL {
        assert!(present.contains(&surface), "missing {}", surface.as_str());
    }
}

#[test]
fn all_surfaces_share_one_detector_and_threat_vocabulary() {
    let packet = packet();
    assert!(packet.all_surfaces_share_content_classes());
    assert!(packet.all_surfaces_share_threat_classes());
    assert_eq!(
        packet.finding_classes,
        vec![
            "bidi_control".to_owned(),
            "invisible_formatting".to_owned(),
            "mixed_script_confusable".to_owned(),
        ]
    );
    assert_eq!(
        packet.threat_classes,
        vec![
            "hidden_codepoint_smuggling".to_owned(),
            "identity_confusable_spoof".to_owned(),
            "text_reordering_spoof".to_owned(),
        ]
    );
    assert!(packet.surfaces.iter().all(|s| s.warnings.len() == 3));
}

#[test]
fn raw_inspection_reachable_on_every_surface() {
    let packet = packet();
    assert!(packet.raw_inspection_reachable_where_required());
    for surface in &packet.surfaces {
        assert!(surface.raw_inspection_reachable);
        for warning in &surface.warnings {
            assert!(warning.raw_inspection_reachable());
            assert!(warning.threat_cue.materially_affects_trust);
            assert!(!warning.raw_snippet.is_empty());
            assert!(!warning.escaped_snippet.is_empty());
        }
        assert!(surface.offers_labeled_raw_and_escaped_copy());
    }
}

#[test]
fn strong_decision_surfaces_render_strict_identity() {
    let packet = packet();
    assert!(packet.strong_decision_surfaces_use_strict_display());
    for surface in &packet.surfaces {
        if surface.surface.is_strong_decision_surface() {
            assert!(
                surface.display_mode.is_strict(),
                "{} must be strict",
                surface.surface.as_str()
            );
        } else {
            assert_eq!(
                surface.display_mode,
                M5SuspiciousTextDisplayMode::OrdinaryBrowsing
            );
        }
    }
}

#[test]
fn support_admin_export_preserves_cues_without_a_pane() {
    let packet = packet();
    let export = &packet.support_admin_export;
    assert!(export.preserves_cues_without_pane);
    let present: BTreeSet<_> = export
        .threat_cue_summaries
        .iter()
        .map(|s| s.threat_class_token.as_str())
        .collect();
    let expected: BTreeSet<_> = packet.threat_classes.iter().map(String::as_str).collect();
    assert_eq!(present, expected);
    for summary in &export.threat_cue_summaries {
        assert!(summary.warning_count >= M5SuspiciousTextSurface::ALL.len());
        assert!(!summary.surfaces.is_empty());
        assert!(!summary.continuity_refs.is_empty());
    }
    assert_eq!(
        export.surfaces_covered.len(),
        M5SuspiciousTextSurface::ALL.len()
    );
}

#[test]
fn support_admin_export_never_leaks_raw_suspicious_bytes() {
    let packet = packet();
    for summary in &packet.support_admin_export.threat_cue_summaries {
        assert!(
            !has_suspicious_content(&summary.escaped_exemplar),
            "escaped exemplar leaked raw bytes: {}",
            summary.escaped_exemplar
        );
    }
}

#[test]
fn bytes_are_never_normalized_away() {
    let packet = packet();
    assert!(!packet.normalization_applied);
    // Every matched codepoint survives in the raw snippet — nothing stripped.
    for surface in &packet.surfaces {
        for warning in &surface.warnings {
            for cp in &warning.matched_codepoints {
                let ch = char::from_u32(*cp).expect("codepoint");
                assert!(
                    warning.raw_snippet.contains(ch),
                    "codepoint U+{cp:04X} dropped from raw snippet"
                );
            }
        }
    }
}

#[test]
fn missing_surface_fails_validation() {
    let mut packet = packet();
    packet
        .surfaces
        .retain(|s| s.surface != M5SuspiciousTextSurface::ProviderPolicyOverlay);
    assert!(packet
        .validate()
        .contains(&M5SuspiciousTextParityViolation::SurfaceMissing));
}

#[test]
fn weak_strong_decision_display_fails_validation() {
    let mut packet = packet();
    let surface = packet
        .surfaces
        .iter_mut()
        .find(|s| s.surface == M5SuspiciousTextSurface::MarketplaceInstallUpdate)
        .expect("marketplace surface present");
    surface.display_mode = M5SuspiciousTextDisplayMode::OrdinaryBrowsing;
    assert!(packet
        .validate()
        .contains(&M5SuspiciousTextParityViolation::StrongDecisionDisplayTooWeak));
}

#[test]
fn unreachable_raw_inspection_fails_validation() {
    let mut packet = packet();
    let warning = packet.surfaces[0]
        .warnings
        .first_mut()
        .expect("a warning present");
    warning
        .available_actions
        .retain(|a| *a != SuspiciousTextWarningAction::CopyRaw);
    assert!(packet
        .validate()
        .contains(&M5SuspiciousTextParityViolation::RawInspectionUnreachable));
}

#[test]
fn divergent_surfaces_fail_validation() {
    let mut packet = packet();
    packet.surfaces[1].warnings.clear();
    assert!(packet
        .validate()
        .contains(&M5SuspiciousTextParityViolation::SurfacesDisagreeOnClasses));
}

#[test]
fn support_export_dropping_cues_fails_validation() {
    let mut packet = packet();
    packet.support_admin_export.threat_cue_summaries.clear();
    assert!(packet
        .validate()
        .contains(&M5SuspiciousTextParityViolation::SupportExportDropsCues));
}

#[test]
fn support_export_leaking_raw_bytes_fails_validation() {
    let mut packet = packet();
    if let Some(summary) = packet.support_admin_export.threat_cue_summaries.first_mut() {
        // Inject a raw bidi control to simulate a leak.
        summary.escaped_exemplar = "admin\u{202E}".to_owned();
    }
    assert!(packet
        .validate()
        .contains(&M5SuspiciousTextParityViolation::SupportExportLeaksRawBytes));
}

#[test]
fn normalization_flag_fails_validation() {
    let mut packet = packet();
    packet.normalization_applied = true;
    assert!(packet
        .validate()
        .contains(&M5SuspiciousTextParityViolation::NormalizationApplied));
}

#[test]
fn markdown_summary_lists_every_surface_and_threat_class() {
    let summary = packet().render_markdown_summary();
    for surface in M5SuspiciousTextSurface::ALL {
        assert!(
            summary.contains(surface.as_str()),
            "missing {}",
            surface.as_str()
        );
    }
    for threat in [
        M5SuspiciousTextThreatClass::TextReorderingSpoof,
        M5SuspiciousTextThreatClass::HiddenCodepointSmuggling,
        M5SuspiciousTextThreatClass::IdentityConfusableSpoof,
    ] {
        assert!(
            summary.contains(threat.as_str()),
            "missing {}",
            threat.as_str()
        );
    }
}

#[test]
fn packet_round_trips_via_serde() {
    let packet = packet();
    let json = packet.export_safe_json();
    let back: M5SuspiciousTextParityPacket = serde_json::from_str(&json).expect("parse");
    assert_eq!(back, packet);
    assert_eq!(back.record_kind, M5_SUSPICIOUS_TEXT_PARITY_RECORD_KIND);
}

#[test]
fn checked_support_export_matches_frozen_packet() {
    let checked = current_m5_suspicious_text_parity_export()
        .expect("checked M5 suspicious-text parity export validates");
    assert_eq!(checked.packet_id, M5_SUSPICIOUS_TEXT_PARITY_PACKET_ID);
    assert_eq!(
        checked,
        frozen_m5_suspicious_text_parity_packet(),
        "checked-in support export drifted from the frozen in-code packet; regenerate with the bin"
    );
}

#[test]
fn checked_clean_fixture_has_no_warnings() {
    let raw = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/security/m5/m5_suspicious_text_detector_parity/clean_content_no_warnings.json"
    ));
    let packet: M5SuspiciousTextParityPacket =
        serde_json::from_str(raw).expect("fixture parses as parity packet");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.finding_classes.is_empty());
    assert!(packet.surfaces.iter().all(|s| s.warnings.is_empty()));
}
