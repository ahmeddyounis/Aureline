//! Fixture replay and invariant tests for stable shell-slot zoning.

use std::collections::BTreeSet;

use aureline_shell::stabilize_shell_zoning_and_responsive_fallback::{
    canonical_shell_zoning_packet, canonical_slot_registry, placeholder_hydration_cases,
    responsive_fallback_ladders, stable_surface_claims, AdaptiveClass, FallbackPlacement,
    ShellSlotId, ShellZoningPacket, ZoneId,
};

const FIXTURE_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ux/m4/stabilize-shell-zoning-and-responsive-fallback/shell_zoning_responsive_fallback_packet.json",
);

fn load_packet() -> ShellZoningPacket {
    let body = std::fs::read_to_string(FIXTURE_PATH)
        .unwrap_or_else(|err| panic!("failed to read {FIXTURE_PATH}: {err}"));
    serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("failed to parse {FIXTURE_PATH}: {err}"))
}

#[test]
fn fixture_matches_canonical_packet() {
    assert_eq!(
        load_packet(),
        canonical_shell_zoning_packet(),
        "fixture drifted; re-emit with `cargo run -q -p aureline-shell --bin aureline_shell_stabilize_shell_zoning_and_responsive_fallback -- emit-fixtures fixtures/ux/m4/stabilize-shell-zoning-and-responsive-fallback`",
    );
}

#[test]
fn declared_registry_covers_required_zones_and_slots() {
    let slots = canonical_slot_registry();
    assert!(slots.iter().all(|slot| slot.validates()));

    let zones: BTreeSet<&'static str> = slots.iter().map(|slot| slot.shell_zone.as_str()).collect();
    for required in [
        ZoneId::TitleContextBar,
        ZoneId::ActivityRail,
        ZoneId::LeftSidebar,
        ZoneId::MainWorkspace,
        ZoneId::RightInspector,
        ZoneId::BottomPanel,
        ZoneId::StatusBar,
        ZoneId::TransientOverlay,
    ] {
        assert!(
            zones.contains(required.as_str()),
            "missing zone {required:?}"
        );
    }

    for slot in &slots {
        assert!(
            slot.fallback_order
                .contains(&FallbackPlacement::Placeholder),
            "{} lacks placeholder fallback",
            slot.slot_id.as_str(),
        );
    }
}

#[test]
fn stable_surface_claims_are_declared_and_do_not_mint_private_chrome() {
    let registry = canonical_slot_registry();
    for claim in stable_surface_claims() {
        assert!(
            claim.is_admitted_by(&registry),
            "{} is not admitted by its declared slot",
            claim.surface_id,
        );
        assert!(!claim.has_private_top_level_chrome);
        assert!(!claim.creates_duplicate_sidebar);
        assert!(!claim.creates_floating_global_button);
    }
}

#[test]
fn responsive_ladders_preserve_truth_for_every_slot_and_class() {
    let ladders = responsive_fallback_ladders();
    let mut coverage = BTreeSet::new();

    for ladder in &ladders {
        assert!(
            ladder.protects_truth_cues(),
            "{} {:?} does not preserve stable truth",
            ladder.slot_id.as_str(),
            ladder.adaptive_class,
        );
        coverage.insert((ladder.slot_id.as_str(), ladder.adaptive_class.as_str()));
    }

    for slot in canonical_slot_registry() {
        for class in [
            AdaptiveClass::CompactDesktop,
            AdaptiveClass::StandardDesktop,
            AdaptiveClass::ExpandedDesktop,
        ] {
            assert!(
                coverage.contains(&(slot.slot_id.as_str(), class.as_str())),
                "{} missing {:?} fallback ladder",
                slot.slot_id.as_str(),
                class,
            );
        }
    }
}

#[test]
fn placeholder_hydration_preserves_declared_slot_and_adjacent_layout() {
    for case in placeholder_hydration_cases() {
        assert!(
            case.preserves_layout_truth(),
            "{} does not preserve placeholder-in-place truth",
            case.scenario_id,
        );
        assert_eq!(case.shell_zone, case.slot_id.zone());
    }
}

#[test]
fn packet_audit_passes_and_names_core_truth_routes() {
    let packet = canonical_shell_zoning_packet();
    assert!(packet.audit.passes());

    let slots: BTreeSet<ShellSlotId> = packet
        .stable_surface_claims
        .iter()
        .map(|claim| claim.declared_slot_id)
        .collect();
    for expected in [
        ShellSlotId::TitleContextBarIdentity,
        ShellSlotId::MainWorkspaceWorkingSet,
        ShellSlotId::RightInspectorContextualDetail,
        ShellSlotId::BottomPanelToolPanels,
        ShellSlotId::StatusBarRecoveryPrimary,
    ] {
        assert!(slots.contains(&expected), "missing claim for {expected:?}");
    }
}
