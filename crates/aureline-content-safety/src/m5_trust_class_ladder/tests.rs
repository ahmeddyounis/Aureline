use super::*;

fn packet() -> M5TrustClassLadderPacket {
    frozen_m5_trust_class_ladder_packet()
}

fn surface(
    packet: &M5TrustClassLadderPacket,
    surface: M5TrustLadderSurface,
) -> &M5TrustLadderSurfaceProjection {
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
    for s in M5TrustLadderSurface::ALL {
        assert!(present.contains(&s), "missing {}", s.as_str());
    }
}

#[test]
fn downgrade_rule_catalog_covers_every_trigger() {
    let packet = packet();
    assert!(packet.downgrade_rules_cover_every_trigger());
    let present: BTreeSet<_> = packet.downgrade_rules.iter().map(|r| r.trigger).collect();
    for trigger in M5DowngradeTrigger::ALL {
        assert!(present.contains(&trigger), "missing {}", trigger.as_str());
    }
    // Every rule's redundant token mirrors its enum value.
    for rule in &packet.downgrade_rules {
        assert_eq!(rule.trigger_token, rule.trigger.as_str());
        assert_eq!(rule.target_fallback_token, rule.target_fallback.as_str());
        assert!(!rule.rule_id.trim().is_empty());
        assert!(!rule.rationale.trim().is_empty());
    }
}

#[test]
fn honored_active_classes_execute_within_declared_class() {
    let packet = packet();
    let notebook = surface(&packet, M5TrustLadderSurface::NotebookRichOutput);
    assert!(notebook.is_honored());
    assert_eq!(
        notebook.effective_trust_class,
        M5TrustClass::TrustedLocalActive
    );
    assert_eq!(
        notebook.active_content_posture,
        M5ActiveContentPosture::TrustedLocalExecution
    );

    let docs = surface(&packet, M5TrustLadderSurface::DocsBrowserPanel);
    assert!(docs.is_honored());
    assert_eq!(
        docs.effective_trust_class,
        M5TrustClass::IsolatedRemoteActive
    );
    assert_eq!(
        docs.active_content_posture,
        M5ActiveContentPosture::IsolatedRemoteExecution
    );
}

#[test]
fn divergence_and_missing_preview_fall_back_to_compare_only() {
    let packet = packet();
    let ai = surface(&packet, M5TrustLadderSurface::AiEvidenceViewer);
    assert_eq!(ai.effective_trust_class, M5TrustClass::CompareOnly);
    assert_eq!(ai.fallback_mode, M5FallbackMode::CompareOnly);
    assert!(ai
        .fired_triggers
        .contains(&M5DowngradeTrigger::RawRenderedDivergenceUnresolved));

    let pipeline = surface(&packet, M5TrustLadderSurface::PipelineArtifactBrowser);
    assert_eq!(pipeline.effective_trust_class, M5TrustClass::CompareOnly);
    assert!(pipeline
        .fired_triggers
        .contains(&M5DowngradeTrigger::SafePreviewUnavailable));
}

#[test]
fn isolation_loss_degrades_active_to_sanitized() {
    let packet = packet();
    let market = surface(&packet, M5TrustLadderSurface::MarketplaceInstallReview);
    assert_eq!(
        market.requested_trust_class,
        M5TrustClass::IsolatedRemoteActive
    );
    assert_eq!(market.effective_trust_class, M5TrustClass::SanitizedRich);
    assert_eq!(market.fallback_mode, M5FallbackMode::SanitizedVisibility);
    assert_eq!(
        market.active_content_posture,
        M5ActiveContentPosture::InertNeverExecutes
    );
    assert!(market
        .fired_triggers
        .contains(&M5DowngradeTrigger::IsolationRuntimeUnavailable));
}

#[test]
fn policy_block_blocks_to_metadata_only() {
    let packet = packet();
    let remote = surface(&packet, M5TrustLadderSurface::RemotePreviewTarget);
    assert_eq!(remote.effective_trust_class, M5TrustClass::Blocked);
    assert_eq!(remote.fallback_mode, M5FallbackMode::BlockedMetadataOnly);
    assert_eq!(
        remote.active_content_posture,
        M5ActiveContentPosture::BlockedPendingReview
    );
    assert!(remote
        .fired_triggers
        .contains(&M5DowngradeTrigger::PolicyBlocked));
}

#[test]
fn frozen_packet_exercises_every_fallback_mode() {
    let packet = packet();
    for mode in [
        M5FallbackMode::NoFallback,
        M5FallbackMode::SanitizedVisibility,
        M5FallbackMode::CompareOnly,
        M5FallbackMode::BlockedMetadataOnly,
    ] {
        assert!(
            packet.surfaces.iter().any(|s| s.fallback_mode == mode),
            "missing fallback mode {}",
            mode.as_str()
        );
    }
}

#[test]
fn active_content_never_executes_outside_declared_class() {
    let packet = packet();
    assert!(packet.active_content_confined_to_declared_class());
    for s in &packet.surfaces {
        if s.active_content_posture.executes() {
            assert!(
                s.effective_trust_class.is_active(),
                "{} executes without an active effective class",
                s.surface.as_str()
            );
        }
    }
}

#[test]
fn embedded_review_surfaces_never_execute() {
    let packet = packet();
    assert!(packet.embedded_review_surfaces_never_execute());
    for s in &packet.surfaces {
        if s.surface.is_embedded_review_surface() {
            assert!(
                !s.active_content_posture.executes(),
                "{} must not execute",
                s.surface.as_str()
            );
        }
    }
}

#[test]
fn strong_decision_surfaces_render_strict_identity() {
    let packet = packet();
    assert!(packet.strong_decision_surfaces_use_strict_display());
    for s in &packet.surfaces {
        if s.surface.is_strong_decision_surface() {
            assert!(s.display_mode.is_strict(), "{}", s.surface.as_str());
        } else {
            assert_eq!(s.display_mode, M5DisplayMode::OrdinaryBrowsing);
        }
    }
}

#[test]
fn resolution_never_escalates_above_request() {
    let packet = packet();
    assert!(packet.never_escalates_above_request());
}

#[test]
fn raw_inspection_and_copy_reachable_everywhere() {
    let packet = packet();
    assert!(packet.raw_always_reachable());
    for s in &packet.surfaces {
        assert!(s.raw_inspection_reachable && s.raw_copy_reachable);
        assert!(!s.trust_class_ladder.is_empty());
        assert_eq!(s.trust_class_ladder[0], M5TrustClass::RawText);
    }
}

#[test]
fn downgrade_narrows_without_opaque_failure() {
    let packet = packet();
    assert!(packet.downgrade_narrows_without_opaque_failure());
    for s in &packet.surfaces {
        if s.downgraded {
            assert_ne!(s.fallback_mode, M5FallbackMode::NoFallback);
            assert!(!s.rationale.trim().is_empty());
            assert!(!s.applied_downgrade_rules.is_empty());
        } else {
            assert_eq!(s.fallback_mode, M5FallbackMode::NoFallback);
        }
    }
}

#[test]
fn compare_only_fallback_reachable_for_renderable_surfaces() {
    let packet = packet();
    for s in &packet.surfaces {
        if s.effective_trust_class != M5TrustClass::Blocked {
            assert!(
                s.compare_only_fallback_available,
                "{} lost its compare-only floor",
                s.surface.as_str()
            );
        }
    }
}

#[test]
fn suspicious_content_is_annotated_not_normalized() {
    let packet = packet();
    let provider = surface(&packet, M5TrustLadderSurface::ProviderOverlay);
    assert!(provider.suspicious_annotated);
    // Sanitized request + suspicious content stays sanitized with raw reachable;
    // it is not silently rewritten away.
    assert_eq!(provider.effective_trust_class, M5TrustClass::SanitizedRich);
    assert!(provider.raw_inspection_reachable);
    assert!(!packet.normalization_applied);
}

#[test]
fn downgraded_count_matches_projections() {
    let packet = packet();
    let counted = packet.surfaces.iter().filter(|s| s.downgraded).count();
    assert_eq!(packet.downgraded_surface_count, counted);
}

#[test]
fn missing_surface_fails_validation() {
    let mut packet = packet();
    packet
        .surfaces
        .retain(|s| s.surface != M5TrustLadderSurface::StructuredCompareView);
    assert!(packet
        .validate()
        .contains(&M5TrustClassLadderViolation::SurfaceMissing));
}

#[test]
fn missing_downgrade_rule_fails_validation() {
    let mut packet = packet();
    packet
        .downgrade_rules
        .retain(|r| r.trigger != M5DowngradeTrigger::ProofStale);
    assert!(packet
        .validate()
        .contains(&M5TrustClassLadderViolation::DowngradeRuleCoverageIncomplete));
}

#[test]
fn executing_review_surface_fails_validation() {
    let mut packet = packet();
    let ai = packet
        .surfaces
        .iter_mut()
        .find(|s| s.surface == M5TrustLadderSurface::AiEvidenceViewer)
        .expect("ai surface");
    ai.effective_trust_class = M5TrustClass::TrustedLocalActive;
    ai.active_content_posture = M5ActiveContentPosture::TrustedLocalExecution;
    let violations = packet.validate();
    assert!(violations.contains(&M5TrustClassLadderViolation::ActiveContentInReviewSurface));
}

#[test]
fn active_content_outside_declared_class_fails_validation() {
    let mut packet = packet();
    let docs = packet
        .surfaces
        .iter_mut()
        .find(|s| s.surface == M5TrustLadderSurface::DocsBrowserPanel)
        .expect("docs surface");
    // Claim isolated-remote execution while the effective class is only sanitized.
    docs.effective_trust_class = M5TrustClass::SanitizedRich;
    docs.active_content_posture = M5ActiveContentPosture::IsolatedRemoteExecution;
    assert!(packet
        .validate()
        .contains(&M5TrustClassLadderViolation::ActiveContentOutsideDeclaredClass));
}

#[test]
fn weak_strong_decision_display_fails_validation() {
    let mut packet = packet();
    let market = packet
        .surfaces
        .iter_mut()
        .find(|s| s.surface == M5TrustLadderSurface::MarketplaceInstallReview)
        .expect("marketplace surface");
    market.display_mode = M5DisplayMode::OrdinaryBrowsing;
    assert!(packet
        .validate()
        .contains(&M5TrustClassLadderViolation::StrongDecisionDisplayTooWeak));
}

#[test]
fn escalation_above_request_fails_validation() {
    let mut packet = packet();
    let provider = packet
        .surfaces
        .iter_mut()
        .find(|s| s.surface == M5TrustLadderSurface::ProviderOverlay)
        .expect("provider surface");
    // Requested sanitized but claim isolated-remote-active effective.
    provider.effective_trust_class = M5TrustClass::IsolatedRemoteActive;
    provider.active_content_posture = M5ActiveContentPosture::IsolatedRemoteExecution;
    assert!(packet
        .validate()
        .contains(&M5TrustClassLadderViolation::EscalatedAboveRequest));
}

#[test]
fn unreachable_raw_fails_validation() {
    let mut packet = packet();
    packet.surfaces[0].raw_copy_reachable = false;
    assert!(packet
        .validate()
        .contains(&M5TrustClassLadderViolation::RawInspectionUnreachable));
}

#[test]
fn missing_compare_only_fallback_fails_validation() {
    let mut packet = packet();
    let ai = packet
        .surfaces
        .iter_mut()
        .find(|s| s.surface == M5TrustLadderSurface::AiEvidenceViewer)
        .expect("ai surface");
    ai.compare_only_fallback_available = false;
    assert!(packet
        .validate()
        .contains(&M5TrustClassLadderViolation::CompareOnlyFallbackMissing));
}

#[test]
fn normalization_flag_fails_validation() {
    let mut packet = packet();
    packet.normalization_applied = true;
    assert!(packet
        .validate()
        .contains(&M5TrustClassLadderViolation::NormalizationApplied));
}

#[test]
fn incomplete_trust_review_fails_validation() {
    let mut packet = packet();
    packet.trust_review.downgrade_narrows_instead_of_hiding = false;
    assert!(packet
        .validate()
        .contains(&M5TrustClassLadderViolation::TrustReviewIncomplete));
}

#[test]
fn markdown_summary_lists_every_surface_and_rule() {
    let summary = packet().render_markdown_summary();
    for s in M5TrustLadderSurface::ALL {
        assert!(summary.contains(s.as_str()), "missing {}", s.as_str());
    }
    for rule in &packet().downgrade_rules {
        assert!(summary.contains(&rule.rule_id), "missing {}", rule.rule_id);
    }
}

#[test]
fn packet_round_trips_via_serde() {
    let packet = packet();
    let json = packet.export_safe_json();
    let back: M5TrustClassLadderPacket = serde_json::from_str(&json).expect("parse");
    assert_eq!(back, packet);
    assert_eq!(back.record_kind, M5_TRUST_CLASS_LADDER_RECORD_KIND);
}

#[test]
fn checked_support_export_matches_frozen_packet() {
    let checked = current_m5_trust_class_ladder_export()
        .expect("checked M5 trust-class ladder export validates");
    assert_eq!(checked.packet_id, M5_TRUST_CLASS_LADDER_PACKET_ID);
    assert_eq!(
        checked,
        frozen_m5_trust_class_ladder_packet(),
        "checked-in support export drifted from the frozen in-code packet; regenerate with the bin"
    );
}

#[test]
fn checked_clean_fixture_has_no_downgrade() {
    let raw = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/security/m5/m5_trust_class_ladder/all_trusted_no_downgrade.json"
    ));
    let packet: M5TrustClassLadderPacket =
        serde_json::from_str(raw).expect("fixture parses as trust-class ladder packet");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.downgraded_surface_count, 0);
    assert!(packet.surfaces.iter().all(|s| !s.downgraded));
    assert!(packet
        .surfaces
        .iter()
        .all(|s| s.fallback_mode == M5FallbackMode::NoFallback));
}
