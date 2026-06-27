//! Inline tests for the M5 surface-qualification packet.

use super::*;

use crate::m5_evidence_pack::{seeded_m5_evidence_pack_stale_narrowed, M5EvidenceClaimGate};
use crate::m5_foundation_package::seeded_m5_foundation_package;

fn canonical() -> M5SurfaceQualificationPacket {
    seeded_m5_surface_qualification_packet()
}

#[test]
fn canonical_packet_validates() {
    let packet = canonical();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_SURFACE_QUALIFICATION_PACKET_ID);
    assert_eq!(packet.record_kind, M5_SURFACE_QUALIFICATION_RECORD_KIND);
}

#[test]
fn canonical_is_all_qualified_green() {
    let packet = canonical();
    assert!(!packet.surfaces.is_empty());
    for surface in &packet.surfaces {
        assert_eq!(
            surface.status,
            M5QualificationStatus::Qualified,
            "surface {} not qualified",
            surface.surface_id
        );
        assert_eq!(surface.signal, M5QualificationSignal::Green);
        assert!(surface.is_qualified());
        assert_eq!(surface.effective_class, M5DesignSystemClaimClass::Stable);
        assert!(surface.gaps.is_empty());
        for binding in &surface.lane_bindings {
            assert_eq!(binding.conformance, M5LaneConformance::Conformant);
        }
    }
    assert!(!packet.blocks_stable_promotion());
    let dashboard = packet.dashboard();
    assert_eq!(dashboard.green_count, packet.surfaces.len() as u32);
    assert_eq!(dashboard.yellow_count, 0);
    assert_eq!(dashboard.red_count, 0);
}

#[test]
fn every_surface_binds_all_four_lanes_and_names_components() {
    let packet = canonical();
    for surface in &packet.surfaces {
        assert!(!surface.bound_component_kinds.is_empty());
        let lanes: std::collections::BTreeSet<M5QualificationLane> =
            surface.lane_bindings.iter().map(|b| b.lane).collect();
        for lane in M5QualificationLane::ALL {
            assert!(
                lanes.contains(&lane),
                "surface {} missing lane {}",
                surface.surface_id,
                lane.as_str()
            );
        }
    }
    // Every launch-critical component family is rendered by at least one claimed surface.
    let rendered: std::collections::BTreeSet<M5ComponentKind> = packet
        .surfaces
        .iter()
        .flat_map(|s| s.bound_component_kinds.iter().copied())
        .collect();
    for kind in M5ComponentKind::ALL {
        assert!(
            rendered.contains(&kind),
            "component family {} unrendered",
            kind.as_str()
        );
    }
}

#[test]
fn every_bound_manifest_token_resolves_in_foundation() {
    // Guards the canonical green status: the two lanes read from one shared source.
    let foundation = seeded_m5_foundation_package();
    let tokens = resolvable_foundation_tokens(&foundation);
    let manifests = crate::m5_component_manifest::seeded_m5_component_manifest_package();
    for manifest in &manifests.manifests {
        for dep in &manifest.token_dependencies {
            assert!(
                tokens.contains(dep.as_str()),
                "manifest {} references unpublished token {}",
                manifest.component_id,
                dep
            );
        }
    }
}

#[test]
fn stale_drill_auto_narrows_without_blocking() {
    let packet = seeded_m5_surface_qualification_packet_stale_narrowed();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    // Stale proof narrows but never blocks.
    assert!(!packet.blocks_stable_promotion());

    let evidence = seeded_m5_evidence_pack_stale_narrowed();
    let mut narrowed_any = false;
    let mut qualified_any = false;
    for surface in &packet.surfaces {
        let expects_narrow = surface.bound_component_kinds.iter().any(|kind| {
            evidence.component(*kind).map(|c| c.claim_gate) != Some(M5EvidenceClaimGate::Certified)
        });
        if expects_narrow {
            narrowed_any = true;
            assert_eq!(
                surface.status,
                M5QualificationStatus::Provisional,
                "surface {} should be provisional",
                surface.surface_id
            );
            assert!(surface.is_auto_narrowed());
            assert_eq!(surface.effective_class, M5DesignSystemClaimClass::Beta);
            assert!(surface
                .gaps
                .iter()
                .any(|g| g.gap_kind == M5QualificationGapKind::EvidenceStale));
        } else {
            qualified_any = true;
            assert_eq!(
                surface.status,
                M5QualificationStatus::Qualified,
                "surface {} should stay qualified",
                surface.surface_id
            );
        }
    }
    assert!(narrowed_any, "expected at least one narrowed surface");
    assert!(
        qualified_any,
        "expected at least one still-qualified surface"
    );
}

#[test]
fn token_drift_drill_narrows_the_consuming_surface() {
    let packet = seeded_m5_surface_qualification_packet_token_drift_narrowed();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    // Failing token/state conformance narrows but never blocks.
    assert!(!packet.blocks_stable_promotion());
    let data_grid = packet
        .surface("design-system-surface:data_grid")
        .expect("data-grid surface present");
    assert_eq!(data_grid.status, M5QualificationStatus::Provisional);
    assert!(data_grid.is_auto_narrowed());
    assert_eq!(data_grid.effective_class, M5DesignSystemClaimClass::Beta);
    let gap = data_grid
        .gaps
        .iter()
        .find(|g| g.gap_kind == M5QualificationGapKind::FoundationTokenUnresolved)
        .expect("token-unresolved gap named");
    assert_eq!(gap.lane, M5QualificationLane::Foundation);
    let foundation_binding = data_grid
        .lane_bindings
        .iter()
        .find(|b| b.lane == M5QualificationLane::Foundation)
        .expect("foundation binding present");
    assert_eq!(
        foundation_binding.conformance,
        M5LaneConformance::Nonconformant
    );
}

#[test]
fn missing_manifest_drill_blocks_and_names_the_gap() {
    let packet = seeded_m5_surface_qualification_packet_missing_manifest_blocked();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.blocks_stable_promotion());
    let data_grid = packet
        .surface("design-system-surface:data_grid")
        .expect("data-grid surface present");
    assert_eq!(data_grid.status, M5QualificationStatus::Disqualified);
    assert_eq!(data_grid.signal, M5QualificationSignal::Red);
    assert!(data_grid.is_blocked());
    assert_eq!(data_grid.effective_class, M5DesignSystemClaimClass::Held);
    assert_eq!(
        packet.blocked_surface_ids(),
        vec![data_grid.surface_id.as_str()]
    );
    let gap = data_grid
        .gaps
        .iter()
        .find(|g| g.gap_kind == M5QualificationGapKind::ComponentManifestMissing)
        .expect("missing-manifest gap named");
    assert!(!gap.waived);
    let binding = data_grid
        .lane_bindings
        .iter()
        .find(|b| b.lane == M5QualificationLane::ComponentContract)
        .expect("component binding present");
    assert_eq!(binding.conformance, M5LaneConformance::Missing);
}

#[test]
fn waiver_drill_ships_narrowed_but_stays_red() {
    let packet = seeded_m5_surface_qualification_packet_waived_narrowed();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    // The waived gap no longer blocks promotion, but the true status stays disqualified.
    assert!(!packet.blocks_stable_promotion());
    let data_grid = packet
        .surface("design-system-surface:data_grid")
        .expect("data-grid surface present");
    assert_eq!(data_grid.status, M5QualificationStatus::Disqualified);
    assert_eq!(data_grid.signal, M5QualificationSignal::Red);
    assert!(data_grid.is_auto_narrowed());
    assert_eq!(data_grid.effective_class, M5DesignSystemClaimClass::Preview);
    assert_eq!(data_grid.waivers.len(), 1);
    let waiver = &data_grid.waivers[0];
    assert_eq!(
        waiver.gap_kind,
        M5QualificationGapKind::ComponentManifestMissing
    );
    assert_eq!(waiver.narrowed_to, M5DesignSystemClaimClass::Preview);
    assert!(!waiver.expires_at.trim().is_empty());
    let gap = data_grid
        .gaps
        .iter()
        .find(|g| g.gap_kind == M5QualificationGapKind::ComponentManifestMissing)
        .expect("gap named");
    assert!(gap.waived);
    let dashboard = packet.dashboard();
    assert!(dashboard.active_waiver_ids.contains(&waiver.waiver_id));
    assert!(dashboard.waived_surface_ids.contains(&data_grid.surface_id));
}

#[test]
fn dashboard_traffic_light_matches_rows() {
    for packet in [
        seeded_m5_surface_qualification_packet(),
        seeded_m5_surface_qualification_packet_stale_narrowed(),
        seeded_m5_surface_qualification_packet_token_drift_narrowed(),
        seeded_m5_surface_qualification_packet_missing_manifest_blocked(),
        seeded_m5_surface_qualification_packet_waived_narrowed(),
    ] {
        let dashboard = packet.dashboard();
        assert_eq!(dashboard.total_surfaces, packet.surfaces.len() as u32);
        assert_eq!(
            dashboard.green_count + dashboard.yellow_count + dashboard.red_count,
            dashboard.total_surfaces
        );
        assert_eq!(
            dashboard.record_kind,
            M5_SURFACE_QUALIFICATION_DASHBOARD_RECORD_KIND
        );
    }
}

#[test]
fn detects_tampered_verdict() {
    let mut packet = canonical();
    packet.surfaces[0].status = M5QualificationStatus::Disqualified;
    assert!(packet
        .validate()
        .contains(&M5QualificationViolation::DerivedVerdictInconsistent));
}

#[test]
fn detects_tampered_lane_binding() {
    let mut packet = canonical();
    packet.surfaces[0].lane_bindings[0].conformance = M5LaneConformance::Missing;
    assert!(packet
        .validate()
        .contains(&M5QualificationViolation::LaneBindingInconsistent));
}

#[test]
fn detects_vocabulary_drift() {
    let mut packet = canonical();
    packet.vocabulary_set.lanes.push("bogus".to_owned());
    assert!(packet
        .validate()
        .contains(&M5QualificationViolation::VocabularySetDrift));
}

#[test]
fn detects_incomplete_lane_sources() {
    let mut packet = canonical();
    packet.lane_sources.pop();
    assert!(packet
        .validate()
        .contains(&M5QualificationViolation::LaneSourcesIncomplete));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_surface_qualification_packet()
        .expect("checked qualification export validates");
    assert_eq!(
        from_disk,
        canonical(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn checked_dashboard_matches_seed() {
    let from_disk =
        current_stable_m5_surface_qualification_dashboard().expect("checked dashboard parses");
    assert_eq!(
        from_disk,
        canonical().dashboard(),
        "checked dashboard drifted from the seed builder"
    );
}

#[test]
fn checked_drill_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-surface-qualification/stale_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-surface-qualification/token_drift_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-surface-qualification/missing_manifest_blocked.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-surface-qualification/waived_narrowed.json"
        )),
    ] {
        let packet: M5SurfaceQualificationPacket =
            serde_json::from_str(raw).expect("fixture parses as qualification packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn checked_drill_fixtures_match_seed() {
    let pairs: [(&str, M5SurfaceQualificationPacket); 4] = [
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/ui/m5-surface-qualification/stale_narrowed.json"
            )),
            seeded_m5_surface_qualification_packet_stale_narrowed(),
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/ui/m5-surface-qualification/token_drift_narrowed.json"
            )),
            seeded_m5_surface_qualification_packet_token_drift_narrowed(),
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/ui/m5-surface-qualification/missing_manifest_blocked.json"
            )),
            seeded_m5_surface_qualification_packet_missing_manifest_blocked(),
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/ui/m5-surface-qualification/waived_narrowed.json"
            )),
            seeded_m5_surface_qualification_packet_waived_narrowed(),
        ),
    ];
    for (raw, seed) in pairs {
        let from_disk: M5SurfaceQualificationPacket =
            serde_json::from_str(raw).expect("fixture parses");
        assert_eq!(
            from_disk, seed,
            "drill fixture drifted from the seed builder"
        );
    }
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = canonical().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
}

#[test]
fn markdown_summary_names_surfaces_and_lanes() {
    let summary =
        seeded_m5_surface_qualification_packet_waived_narrowed().render_markdown_summary();
    assert!(summary.contains("Surface-Qualification Packet"));
    assert!(summary.contains("design-system-surface:data_grid"));
    assert!(summary.contains("waiver:data-grid-form-control"));
    assert!(summary.contains("component_contract"));
}
