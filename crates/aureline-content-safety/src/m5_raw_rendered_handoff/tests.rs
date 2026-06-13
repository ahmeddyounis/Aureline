use super::*;

fn packet() -> M5RawRenderedHandoffPacket {
    frozen_m5_raw_rendered_handoff_packet()
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
    for surface in M5RawRenderedSurface::ALL {
        assert!(present.contains(&surface), "missing {}", surface.as_str());
    }
}

#[test]
fn every_surface_materially_diverges_in_frozen_packet() {
    let packet = packet();
    assert_eq!(
        packet.diverging_surface_count,
        M5RawRenderedSurface::ALL.len()
    );
    assert!(packet.surfaces.iter().all(|s| s.materially_diverges));
}

#[test]
fn diverging_surfaces_expose_raw_and_rendered_labels_and_actions() {
    let packet = packet();
    assert!(packet.diverging_surfaces_expose_raw_and_rendered());
    for surface in &packet.surfaces {
        assert!(surface.exposes_raw_and_rendered_labels());
        assert!(surface.offers_raw_rendered_and_export_actions());
        assert!(surface.raw_copy_reachable);
        // Exactly one canonical-bytes label and one rendered label.
        assert!(surface
            .representation_labels
            .iter()
            .any(|l| l.is_canonical_bytes));
        assert!(surface
            .representation_labels
            .iter()
            .any(|l| l.representation_class == "rendered"));
    }
}

#[test]
fn rendered_copy_and_export_never_imply_raw() {
    let packet = packet();
    assert!(packet.rendered_copy_never_implies_raw());
    for surface in &packet.surfaces {
        for action in &surface.copy_export_actions {
            if action.representation_class != "raw" {
                assert!(
                    !action.implies_byte_identical_raw,
                    "{} action implies raw bytes",
                    action.action_id
                );
                assert!(!action.preserves_canonical_bytes);
            }
        }
        // A raw-copy path that preserves canonical bytes is always present.
        assert!(surface
            .copy_export_actions
            .iter()
            .any(|a| a.action_id == "copy_raw"
                && a.preserves_canonical_bytes
                && a.implies_byte_identical_raw));
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
                M5RawRenderedDisplayMode::OrdinaryBrowsing
            );
        }
    }
}

#[test]
fn handoff_preserves_representation_across_every_carrier() {
    let packet = packet();
    assert!(packet.handoff_preserves_representation());
    let block = &packet.handoff_preservation;
    assert!(block.preserves_divergence_warning);
    let present: BTreeSet<_> = block.carriers.iter().map(|c| c.carrier).collect();
    for carrier in M5HandoffCarrier::ALL {
        assert!(present.contains(&carrier), "missing {}", carrier.as_str());
    }
    for carrier in &block.carriers {
        assert!(carrier.declares_rendered_not_raw);
        assert!(carrier.preserves_divergence_note);
        assert!(carrier.preserved_labels.iter().any(|l| l == "rendered"));
        if carrier.carrier.requires_raw_label() {
            assert!(carrier.preserved_labels.iter().any(|l| l == "raw"));
        }
        assert!(!carrier.surfaces_covered.is_empty());
        assert!(!carrier.preserved_render_transforms.is_empty());
    }
}

#[test]
fn handoff_never_leaks_raw_suspicious_bytes() {
    let packet = packet();
    for carrier in &packet.handoff_preservation.carriers {
        assert!(
            !has_suspicious_content(&carrier.escaped_exemplar),
            "carrier exemplar leaked raw bytes: {}",
            carrier.escaped_exemplar
        );
    }
}

#[test]
fn bytes_are_never_normalized_away() {
    let packet = packet();
    assert!(!packet.normalization_applied);
}

#[test]
fn divergence_kinds_cover_each_transform() {
    let packet = packet();
    for kind in [
        "rendered_reflows_layout",
        "rendered_applies_styling",
        "rendered_summarizes_content",
        "rendered_normalizes_for_comparison",
    ] {
        assert!(
            packet.divergence_kinds.iter().any(|k| k == kind),
            "missing divergence kind {kind}"
        );
    }
}

#[test]
fn missing_surface_fails_validation() {
    let mut packet = packet();
    packet
        .surfaces
        .retain(|s| s.surface != M5RawRenderedSurface::PolicyReviewOverlay);
    let violations = packet.validate();
    assert!(violations.contains(&M5RawRenderedHandoffViolation::SurfaceMissing));
}

#[test]
fn weak_strong_decision_display_fails_validation() {
    let mut packet = packet();
    let surface = packet
        .surfaces
        .iter_mut()
        .find(|s| s.surface == M5RawRenderedSurface::MarketplaceInstallReview)
        .expect("marketplace surface present");
    surface.display_mode = M5RawRenderedDisplayMode::OrdinaryBrowsing;
    assert!(packet
        .validate()
        .contains(&M5RawRenderedHandoffViolation::StrongDecisionDisplayTooWeak));
}

#[test]
fn diverging_surface_without_labels_fails_validation() {
    let mut packet = packet();
    packet.surfaces[0]
        .representation_labels
        .retain(|l| l.is_canonical_bytes);
    assert!(packet
        .validate()
        .contains(&M5RawRenderedHandoffViolation::DivergingSurfaceMissingRawRenderedLabels));
}

#[test]
fn diverging_surface_without_copy_actions_fails_validation() {
    let mut packet = packet();
    packet.surfaces[0]
        .copy_export_actions
        .retain(|a| a.action_id != "copy_rendered");
    assert!(packet
        .validate()
        .contains(&M5RawRenderedHandoffViolation::DivergingSurfaceMissingCopyExportActions));
}

#[test]
fn rendered_copy_implying_raw_fails_validation() {
    let mut packet = packet();
    let action = packet.surfaces[0]
        .copy_export_actions
        .iter_mut()
        .find(|a| a.action_id == "copy_rendered")
        .expect("rendered action present");
    action.implies_byte_identical_raw = true;
    assert!(packet
        .validate()
        .contains(&M5RawRenderedHandoffViolation::RenderedCopyImpliesRaw));
}

#[test]
fn handoff_dropping_warning_fails_validation() {
    let mut packet = packet();
    if let Some(carrier) = packet.handoff_preservation.carriers.first_mut() {
        carrier.declares_rendered_not_raw = false;
    }
    assert!(packet
        .validate()
        .contains(&M5RawRenderedHandoffViolation::HandoffDropsDivergenceWarning));
}

#[test]
fn handoff_missing_carrier_fails_validation() {
    let mut packet = packet();
    packet
        .handoff_preservation
        .carriers
        .retain(|c| c.carrier != M5HandoffCarrier::ScreenshotCaption);
    assert!(packet
        .validate()
        .contains(&M5RawRenderedHandoffViolation::HandoffCarrierMissing));
}

#[test]
fn handoff_leaking_raw_bytes_fails_validation() {
    let mut packet = packet();
    if let Some(carrier) = packet.handoff_preservation.carriers.first_mut() {
        carrier.escaped_exemplar = "admin\u{202E}".to_owned();
    }
    assert!(packet
        .validate()
        .contains(&M5RawRenderedHandoffViolation::HandoffLeaksRawBytes));
}

#[test]
fn normalization_flag_fails_validation() {
    let mut packet = packet();
    packet.normalization_applied = true;
    assert!(packet
        .validate()
        .contains(&M5RawRenderedHandoffViolation::NormalizationApplied));
}

#[test]
fn markdown_summary_lists_every_surface_and_carrier() {
    let summary = packet().render_markdown_summary();
    for surface in M5RawRenderedSurface::ALL {
        assert!(
            summary.contains(surface.as_str()),
            "missing {}",
            surface.as_str()
        );
    }
    for carrier in M5HandoffCarrier::ALL {
        assert!(
            summary.contains(carrier.as_str()),
            "missing {}",
            carrier.as_str()
        );
    }
}

#[test]
fn packet_round_trips_via_serde() {
    let packet = packet();
    let json = packet.export_safe_json();
    let back: M5RawRenderedHandoffPacket = serde_json::from_str(&json).expect("parse");
    assert_eq!(back, packet);
    assert_eq!(back.record_kind, M5_RAW_RENDERED_HANDOFF_RECORD_KIND);
}

#[test]
fn checked_support_export_matches_frozen_packet() {
    let checked = current_m5_raw_rendered_handoff_export()
        .expect("checked M5 raw-rendered handoff export validates");
    assert_eq!(checked.packet_id, M5_RAW_RENDERED_HANDOFF_PACKET_ID);
    assert_eq!(
        checked,
        frozen_m5_raw_rendered_handoff_packet(),
        "checked-in support export drifted from the frozen in-code packet; regenerate with the bin"
    );
}

#[test]
fn checked_clean_fixture_has_no_divergence() {
    let raw = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/security/m5/m5_raw_rendered_handoff/byte_identical_no_divergence.json"
    ));
    let packet: M5RawRenderedHandoffPacket =
        serde_json::from_str(raw).expect("fixture parses as handoff packet");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.diverging_surface_count, 0);
    assert!(packet.surfaces.iter().all(|s| !s.materially_diverges));
    assert!(!packet.handoff_preservation.preserves_divergence_warning);
}
