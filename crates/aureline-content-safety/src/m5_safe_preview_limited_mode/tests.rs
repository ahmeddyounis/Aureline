use super::*;

fn packet() -> M5SafePreviewLimitedModePacket {
    frozen_m5_safe_preview_limited_mode_packet()
}

fn artifact(
    packet: &M5SafePreviewLimitedModePacket,
    family: M5LimitedModeArtifactFamily,
) -> &M5LimitedModeArtifactProjection {
    packet
        .artifacts
        .iter()
        .find(|a| a.family == family)
        .expect("family present")
}

#[test]
fn frozen_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn covers_every_family_once() {
    let packet = packet();
    assert!(packet.covers_all_families());
    let present: BTreeSet<_> = packet.artifacts.iter().map(|a| a.family).collect();
    for f in M5LimitedModeArtifactFamily::ALL {
        assert!(present.contains(&f), "missing {}", f.as_str());
    }
}

#[test]
fn oversized_or_generated_open_in_limited_mode_first() {
    let packet = packet();
    assert!(packet.oversized_or_generated_open_limited());
    for a in &packet.artifacts {
        if a.oversized || a.generated {
            assert!(
                a.open_mode.is_limited(),
                "{} did not open limited",
                a.family.as_str()
            );
        }
    }
}

#[test]
fn oversized_artifacts_carry_oversized_banner() {
    let packet = packet();
    let log = artifact(&packet, M5LimitedModeArtifactFamily::BuildLog);
    assert!(log.oversized);
    assert!(log
        .banners
        .iter()
        .any(|b| b.kind == M5LimitedModeBannerKind::Oversized));

    let bundle = artifact(&packet, M5LimitedModeArtifactFamily::DistributionBundle);
    assert!(bundle.oversized);
    assert!(bundle
        .banners
        .iter()
        .any(|b| b.kind == M5LimitedModeBannerKind::Oversized));
}

#[test]
fn generated_artifacts_name_their_canonical_source() {
    let packet = packet();
    for a in &packet.artifacts {
        if a.generated {
            let banner = a
                .banners
                .iter()
                .find(|b| b.kind == M5LimitedModeBannerKind::GeneratedArtifact)
                .expect("generated banner");
            assert!(
                banner.message.contains(&a.canonical_source_ref),
                "{} banner omits its canonical source",
                a.family.as_str()
            );
            assert!(a.has_open_canonical_source());
        }
    }
}

#[test]
fn no_silent_expensive_or_unsafe_render() {
    let packet = packet();
    assert!(packet.no_silent_expensive_render());
    for a in &packet.artifacts {
        for action in &a.actions {
            if action.render_cost.requires_opt_in() {
                assert!(
                    action.posture.is_gated(),
                    "{} action {} is silent",
                    a.family.as_str(),
                    action.kind.as_str()
                );
            }
        }
    }
}

#[test]
fn unsafe_bundle_expand_is_gated_unsafe() {
    let packet = packet();
    let bundle = artifact(&packet, M5LimitedModeArtifactFamily::DistributionBundle);
    let expand = bundle.expand_action().expect("expand action");
    assert_eq!(expand.render_cost, M5RenderCost::Unsafe);
    assert_eq!(expand.posture, M5ActionPosture::RequiresExplicitOptIn);
    assert!(bundle
        .banners
        .iter()
        .any(|b| b.kind == M5LimitedModeBannerKind::ActiveContentGuarded));
}

#[test]
fn expensive_lockfile_expand_is_gated_expensive() {
    let packet = packet();
    let lockfile = artifact(&packet, M5LimitedModeArtifactFamily::DependencyLockfile);
    let expand = lockfile.expand_action().expect("expand action");
    assert_eq!(expand.render_cost, M5RenderCost::Expensive);
    assert_eq!(expand.posture, M5ActionPosture::RequiresExplicitOptIn);
    assert!(lockfile.expensive_render_guarded);
}

#[test]
fn cheap_generated_artifact_expands_immediately() {
    let packet = packet();
    let snapshot = artifact(&packet, M5LimitedModeArtifactFamily::TestSnapshot);
    assert!(snapshot.is_limited());
    let expand = snapshot.expand_action().expect("expand action");
    assert_eq!(expand.render_cost, M5RenderCost::Cheap);
    assert_eq!(expand.posture, M5ActionPosture::AvailableImmediately);
    assert!(!snapshot.expensive_render_guarded);
}

#[test]
fn default_view_is_never_expensive() {
    let packet = packet();
    assert!(packet.default_view_never_expensive());
    for a in &packet.artifacts {
        assert_eq!(a.default_render_cost, M5RenderCost::Cheap);
    }
}

#[test]
fn open_raw_reachable_immediately_everywhere() {
    let packet = packet();
    assert!(packet.raw_always_reachable());
    for a in &packet.artifacts {
        assert!(a.raw_inspection_reachable && a.raw_copy_reachable);
        assert!(a.has_immediate_open_raw());
    }
}

#[test]
fn canonical_relationship_preserved_everywhere() {
    let packet = packet();
    assert!(packet.canonical_relationship_preserved());
    for a in &packet.artifacts {
        assert!(!a.canonical_source_ref.trim().is_empty());
        assert!(a.has_open_canonical_source());
    }
}

#[test]
fn limited_mode_artifacts_carry_visible_cue() {
    let packet = packet();
    assert!(packet.limited_mode_has_visible_cue());
    for a in &packet.artifacts {
        if a.is_limited() {
            assert!(a.visible_representation_cue);
            assert!(!a.banners.is_empty());
            assert!(a
                .banners
                .iter()
                .any(|b| b.kind == M5LimitedModeBannerKind::LimitedPreview));
        }
    }
}

#[test]
fn frozen_packet_exercises_every_banner_kind() {
    let packet = packet();
    for kind in [
        M5LimitedModeBannerKind::Oversized,
        M5LimitedModeBannerKind::GeneratedArtifact,
        M5LimitedModeBannerKind::LimitedPreview,
        M5LimitedModeBannerKind::ExpensiveRenderGuarded,
        M5LimitedModeBannerKind::ActiveContentGuarded,
    ] {
        assert!(
            packet
                .artifacts
                .iter()
                .any(|a| a.banners.iter().any(|b| b.kind == kind)),
            "missing banner kind {}",
            kind.as_str()
        );
    }
}

#[test]
fn counts_match_projections() {
    let packet = packet();
    assert_eq!(
        packet.limited_mode_artifact_count,
        packet.artifacts.iter().filter(|a| a.is_limited()).count()
    );
    assert_eq!(
        packet.guarded_render_count,
        packet
            .artifacts
            .iter()
            .filter(|a| a.expensive_render_guarded)
            .count()
    );
    assert_eq!(
        packet.oversized_count,
        packet.artifacts.iter().filter(|a| a.oversized).count()
    );
    assert_eq!(
        packet.generated_count,
        packet.artifacts.iter().filter(|a| a.generated).count()
    );
}

#[test]
fn oversized_or_generated_not_limited_fails_validation() {
    let mut packet = packet();
    let log = packet
        .artifacts
        .iter_mut()
        .find(|a| a.family == M5LimitedModeArtifactFamily::BuildLog)
        .expect("build log");
    log.open_mode = M5OpenMode::FullRenderInline;
    log.open_mode_token = M5OpenMode::FullRenderInline.as_str().to_owned();
    assert!(packet
        .validate()
        .contains(&M5SafePreviewLimitedModeViolation::OversizedOrGeneratedNotLimited));
}

#[test]
fn silent_expensive_render_fails_validation() {
    let mut packet = packet();
    let lockfile = packet
        .artifacts
        .iter_mut()
        .find(|a| a.family == M5LimitedModeArtifactFamily::DependencyLockfile)
        .expect("lockfile");
    let expand = lockfile
        .actions
        .iter_mut()
        .find(|a| a.kind == M5LimitedModeActionKind::ExpandFullRender)
        .expect("expand");
    expand.posture = M5ActionPosture::AvailableImmediately;
    expand.posture_token = M5ActionPosture::AvailableImmediately.as_str().to_owned();
    assert!(packet
        .validate()
        .contains(&M5SafePreviewLimitedModeViolation::SilentExpensiveRender));
}

#[test]
fn expensive_default_view_fails_validation() {
    let mut packet = packet();
    packet.artifacts[0].default_render_cost = M5RenderCost::Expensive;
    packet.artifacts[0].default_render_cost_token = M5RenderCost::Expensive.as_str().to_owned();
    assert!(packet
        .validate()
        .contains(&M5SafePreviewLimitedModeViolation::DefaultViewExpensive));
}

#[test]
fn unreachable_raw_fails_validation() {
    let mut packet = packet();
    packet.artifacts[0]
        .actions
        .retain(|a| a.kind != M5LimitedModeActionKind::OpenRaw);
    assert!(packet
        .validate()
        .contains(&M5SafePreviewLimitedModeViolation::RawInspectionUnreachable));
}

#[test]
fn lost_canonical_relationship_fails_validation() {
    let mut packet = packet();
    packet.artifacts[0]
        .actions
        .retain(|a| a.kind != M5LimitedModeActionKind::OpenCanonicalSource);
    assert!(packet
        .validate()
        .contains(&M5SafePreviewLimitedModeViolation::CanonicalRelationshipLost));
}

#[test]
fn limited_mode_without_cue_fails_validation() {
    let mut packet = packet();
    packet.artifacts[0].banners.clear();
    assert!(packet
        .validate()
        .contains(&M5SafePreviewLimitedModeViolation::LimitedModeMissingCue));
}

#[test]
fn missing_family_fails_validation() {
    let mut packet = packet();
    packet
        .artifacts
        .retain(|a| a.family != M5LimitedModeArtifactFamily::GeneratedArtifact);
    assert!(packet
        .validate()
        .contains(&M5SafePreviewLimitedModeViolation::FamilyMissing));
}

#[test]
fn normalization_flag_fails_validation() {
    let mut packet = packet();
    packet.normalization_applied = true;
    assert!(packet
        .validate()
        .contains(&M5SafePreviewLimitedModeViolation::NormalizationApplied));
}

#[test]
fn incomplete_review_fails_validation() {
    let mut packet = packet();
    packet.review.no_silent_expensive_or_unsafe_render = false;
    assert!(packet
        .validate()
        .contains(&M5SafePreviewLimitedModeViolation::ReviewIncomplete));
}

#[test]
fn markdown_summary_lists_every_family() {
    let summary = packet().render_markdown_summary();
    for f in M5LimitedModeArtifactFamily::ALL {
        assert!(summary.contains(f.as_str()), "missing {}", f.as_str());
    }
}

#[test]
fn packet_round_trips_via_serde() {
    let packet = packet();
    let json = packet.export_safe_json();
    let back: M5SafePreviewLimitedModePacket = serde_json::from_str(&json).expect("parse");
    assert_eq!(back, packet);
    assert_eq!(back.record_kind, M5_SAFE_PREVIEW_LIMITED_RECORD_KIND);
}

#[test]
fn checked_support_export_matches_frozen_packet() {
    let checked = current_m5_safe_preview_limited_mode_export()
        .expect("checked M5 safe-preview limited-mode export validates");
    assert_eq!(checked.packet_id, M5_SAFE_PREVIEW_LIMITED_PACKET_ID);
    assert_eq!(
        checked,
        frozen_m5_safe_preview_limited_mode_packet(),
        "checked-in support export drifted from the frozen in-code packet; regenerate with the bin"
    );
}

#[test]
fn checked_clean_fixture_has_no_guarded_render() {
    let raw = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/security/m5/m5_safe_preview_limited_mode/all_small_no_guard.json"
    ));
    let packet: M5SafePreviewLimitedModePacket =
        serde_json::from_str(raw).expect("fixture parses as safe-preview limited-mode packet");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.guarded_render_count, 0);
    assert_eq!(packet.oversized_count, 0);
    assert!(packet.artifacts.iter().all(|a| !a.expensive_render_guarded));
    assert!(packet.artifacts.iter().all(|a| {
        a.actions
            .iter()
            .all(|action| action.render_cost == M5RenderCost::Cheap)
    }));
    // A non-generated, cheap, in-budget artifact opens fully inline.
    assert!(packet
        .artifacts
        .iter()
        .any(|a| a.open_mode == M5OpenMode::FullRenderInline));
}
