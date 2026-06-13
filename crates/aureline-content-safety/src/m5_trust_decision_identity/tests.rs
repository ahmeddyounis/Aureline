use super::*;

fn packet() -> M5TrustDecisionIdentityPacket {
    frozen_m5_trust_decision_identity_packet()
}

fn surface(
    packet: &M5TrustDecisionIdentityPacket,
    surface: M5TrustDecisionSurface,
) -> &M5TrustDecisionIdentityProjection {
    packet
        .surfaces
        .iter()
        .find(|s| s.surface == surface)
        .expect("surface present")
}

#[test]
fn frozen_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn covers_every_surface_once() {
    let packet = packet();
    assert!(packet.covers_all_surfaces());
    let present: BTreeSet<_> = packet.surfaces.iter().map(|s| s.surface).collect();
    for s in M5TrustDecisionSurface::ALL {
        assert!(present.contains(&s), "missing {}", s.as_str());
    }
}

#[test]
fn every_surface_uses_strong_decision_mode() {
    let packet = packet();
    assert!(packet.all_stronger_than_ordinary_browsing());
    for s in &packet.surfaces {
        assert_eq!(s.render_mode, M5IdentityRenderMode::StrongDecision);
        assert!(s.render_strength_rank > M5_ORDINARY_BROWSING_STRENGTH_RANK);
        assert!(s.stronger_than_ordinary_browsing());
    }
}

#[test]
fn strong_decision_is_strictly_stronger_than_browsing() {
    let packet = packet();
    assert_eq!(
        packet.ordinary_browsing_strength_rank,
        M5_ORDINARY_BROWSING_STRENGTH_RANK
    );
    assert_eq!(
        packet.strong_decision_strength_rank,
        M5_STRONG_DECISION_STRENGTH_RANK
    );
    assert!(packet.strong_decision_strength_rank > packet.ordinary_browsing_strength_rank);
}

#[test]
fn decision_affordance_uses_distinct_verb() {
    let packet = packet();
    for s in &packet.surfaces {
        assert!(s.decision_affordance_distinct);
        assert!(!s.decision_verb.trim().is_empty());
        // The decision verb is never a plain browsing "Open".
        assert_ne!(s.decision_verb, "Open");
        assert_eq!(s.decision_verb, s.decision_action.verb());
    }
}

#[test]
fn confusable_publisher_is_flagged() {
    let packet = packet();
    let publisher = surface(&packet, M5TrustDecisionSurface::PublisherPackageName);
    assert!(publisher.has_suspicious_cues);
    assert!(publisher
        .threat_classes
        .iter()
        .any(|c| c == "mixed_script_confusable"));
    assert!(publisher
        .warnings
        .iter()
        .any(|w| w.kind == M5IdentityWarningKind::MixedScriptConfusable));
    assert!(publisher.has_inspect_codepoints());
}

#[test]
fn invisible_host_is_flagged() {
    let packet = packet();
    let host = surface(&packet, M5TrustDecisionSurface::RemoteHostLabel);
    assert!(host.has_suspicious_cues);
    assert!(host
        .threat_classes
        .iter()
        .any(|c| c == "invisible_formatting"));
    assert!(host
        .warnings
        .iter()
        .any(|w| w.kind == M5IdentityWarningKind::BidiOrInvisibleBytes));
    // The escaped inspection form makes the hidden codepoint visible.
    assert!(host.identity_inspection_escaped.contains("\\u{200B}"));
}

#[test]
fn policy_review_bidi_is_flagged() {
    let packet = packet();
    let policy = surface(&packet, M5TrustDecisionSurface::PolicyReview);
    assert!(policy.has_suspicious_cues);
    assert!(policy.threat_classes.iter().any(|c| c == "bidi_control"));
    assert!(policy.identity_inspection_escaped.contains("\\u{202E}"));
}

#[test]
fn clean_identities_carry_no_warning() {
    let packet = packet();
    for kind in [
        M5TrustDecisionSurface::CollaboratorIdentity,
        M5TrustDecisionSurface::RouteShare,
    ] {
        let s = surface(&packet, kind);
        assert!(
            !s.has_suspicious_cues,
            "{} flagged unexpectedly",
            kind.as_str()
        );
        assert!(s.warnings.is_empty());
        assert!(s.threat_classes.is_empty());
        assert!(!s.has_inspect_codepoints());
        // Raw inspection still stays reachable even when nothing is flagged.
        assert!(s.has_open_raw_identity());
        assert!(s.has_copy_escaped_identity());
    }
}

#[test]
fn suspicious_surface_count_matches_projections() {
    let packet = packet();
    assert_eq!(packet.suspicious_surface_count, 3);
    assert_eq!(
        packet.suspicious_surface_count,
        packet
            .surfaces
            .iter()
            .filter(|s| s.has_suspicious_cues)
            .count()
    );
    assert_eq!(packet.surface_count, packet.surfaces.len());
}

#[test]
fn raw_inspection_reachable_everywhere() {
    let packet = packet();
    assert!(packet.raw_inspection_reachable_everywhere());
    for s in &packet.surfaces {
        assert!(s.raw_inspection_reachable && s.escaped_copy_reachable);
        assert!(s.has_open_raw_identity());
        assert!(s.has_copy_escaped_identity());
        assert!(!s.raw_identity_ref.trim().is_empty());
    }
}

#[test]
fn full_identity_shown_without_truncation() {
    let packet = packet();
    assert!(packet.full_identity_shown_everywhere());
    for s in &packet.surfaces {
        assert!(s.full_identity_shown);
        assert_eq!(s.displayed_char_len, s.identity_char_len);
    }
}

#[test]
fn rendering_preserved_across_carriers() {
    let packet = packet();
    assert!(packet.preserved_everywhere());
    for s in &packet.surfaces {
        assert!(s.preserved_in_product);
        assert!(s.preserved_in_exported_review_packet);
        assert!(s.preserved_in_support_handoff);
    }
}

#[test]
fn suspicious_surfaces_warn_and_inspect() {
    let packet = packet();
    assert!(packet.suspicious_surfaces_warn_and_inspect());
    for s in &packet.surfaces {
        if s.has_suspicious_cues {
            assert!(!s.warnings.is_empty());
            assert!(!s.threat_classes.is_empty());
            assert!(s.has_inspect_codepoints());
        }
    }
}

#[test]
fn not_stronger_than_browsing_fails_validation() {
    let mut packet = packet();
    packet.surfaces[0].render_mode = M5IdentityRenderMode::OrdinaryBrowsing;
    packet.surfaces[0].render_mode_token =
        M5IdentityRenderMode::OrdinaryBrowsing.as_str().to_owned();
    packet.surfaces[0].render_strength_rank = M5_ORDINARY_BROWSING_STRENGTH_RANK;
    assert!(packet
        .validate()
        .contains(&M5TrustDecisionIdentityViolation::NotStrongerThanBrowsing));
}

#[test]
fn truncated_identity_fails_validation() {
    let mut packet = packet();
    packet.surfaces[0].displayed_char_len = packet.surfaces[0].identity_char_len.saturating_sub(1);
    packet.surfaces[0].full_identity_shown = false;
    assert!(packet
        .validate()
        .contains(&M5TrustDecisionIdentityViolation::IdentityTruncated));
}

#[test]
fn unreachable_raw_fails_validation() {
    let mut packet = packet();
    packet.surfaces[0]
        .actions
        .retain(|a| a.kind != M5IdentityInspectionActionKind::OpenRawIdentity);
    assert!(packet
        .validate()
        .contains(&M5TrustDecisionIdentityViolation::RawInspectionUnreachable));
}

#[test]
fn suspicious_surface_without_warning_fails_validation() {
    let mut packet = packet();
    let publisher = packet
        .surfaces
        .iter_mut()
        .find(|s| s.surface == M5TrustDecisionSurface::PublisherPackageName)
        .expect("publisher");
    publisher.warnings.clear();
    assert!(packet
        .validate()
        .contains(&M5TrustDecisionIdentityViolation::SuspiciousSurfaceMissingWarning));
}

#[test]
fn lost_preservation_fails_validation() {
    let mut packet = packet();
    packet.surfaces[0].preserved_in_support_handoff = false;
    assert!(packet
        .validate()
        .contains(&M5TrustDecisionIdentityViolation::RenderingNotPreserved));
}

#[test]
fn missing_surface_fails_validation() {
    let mut packet = packet();
    packet
        .surfaces
        .retain(|s| s.surface != M5TrustDecisionSurface::PolicyReview);
    packet.surface_count = packet.surfaces.len();
    assert!(packet
        .validate()
        .contains(&M5TrustDecisionIdentityViolation::SurfaceMissing));
}

#[test]
fn normalization_flag_fails_validation() {
    let mut packet = packet();
    packet.normalization_applied = true;
    assert!(packet
        .validate()
        .contains(&M5TrustDecisionIdentityViolation::NormalizationApplied));
}

#[test]
fn strength_rank_mismatch_fails_validation() {
    let mut packet = packet();
    packet.strong_decision_strength_rank = packet.ordinary_browsing_strength_rank;
    assert!(packet
        .validate()
        .contains(&M5TrustDecisionIdentityViolation::StrengthRankMismatch));
}

#[test]
fn incomplete_review_fails_validation() {
    let mut packet = packet();
    packet.review.raw_identity_inspection_reachable_everywhere = false;
    assert!(packet
        .validate()
        .contains(&M5TrustDecisionIdentityViolation::ReviewIncomplete));
}

#[test]
fn markdown_summary_lists_every_surface() {
    let summary = packet().render_markdown_summary();
    for s in M5TrustDecisionSurface::ALL {
        assert!(summary.contains(s.as_str()), "missing {}", s.as_str());
    }
}

#[test]
fn packet_round_trips_via_serde() {
    let packet = packet();
    let json = packet.export_safe_json();
    let back: M5TrustDecisionIdentityPacket = serde_json::from_str(&json).expect("parse");
    assert_eq!(back, packet);
    assert_eq!(back.record_kind, M5_TRUST_DECISION_IDENTITY_RECORD_KIND);
}

#[test]
fn checked_support_export_matches_frozen_packet() {
    let checked = current_m5_trust_decision_identity_export()
        .expect("checked M5 trust-decision identity export validates");
    assert_eq!(checked.packet_id, M5_TRUST_DECISION_IDENTITY_PACKET_ID);
    assert_eq!(
        checked,
        frozen_m5_trust_decision_identity_packet(),
        "checked-in support export drifted from the frozen in-code packet; regenerate with the bin"
    );
}

#[test]
fn checked_clean_fixture_has_no_suspicious_surface() {
    let raw = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/security/m5/m5_trust_decision_identity/all_clean_identities.json"
    ));
    let packet: M5TrustDecisionIdentityPacket =
        serde_json::from_str(raw).expect("fixture parses as trust-decision identity packet");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.suspicious_surface_count, 0);
    assert!(packet.surfaces.iter().all(|s| !s.has_suspicious_cues));
    assert!(packet.surfaces.iter().all(|s| s.warnings.is_empty()));
    // Strong-decision rendering and raw inspection hold even with nothing flagged.
    assert!(packet.all_stronger_than_ordinary_browsing());
    assert!(packet.raw_inspection_reachable_everywhere());
}
