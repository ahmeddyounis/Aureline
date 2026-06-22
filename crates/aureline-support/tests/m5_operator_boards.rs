//! Freeze gate for the M5 operator-board set.
//!
//! The checked-in fixture
//! `fixtures/ops/m5-operator-boards/canonical_boards.json` is the published board
//! set. This gate rebuilds the set in code and asserts it equals the fixture
//! after a serialize round-trip, so the overview-board contract cannot drift from
//! the published artifact without failing CI. It also re-proves support-export
//! safety, full board coverage, canonical object identity, the no-silent-green
//! tile rule, export parity, and every frozen invariant.

use std::path::{Path, PathBuf};

use aureline_support::m5_operator_boards::{
    export_board_view, operator_board_lines, operator_board_set, BlockerWaiverClass, BoardClass,
    OverviewBoardSet, M5_OPERATOR_BOARDS_MATRIX_REF, M5_OPERATOR_BOARDS_RECORD_KIND,
    M5_OPERATOR_BOARDS_SCHEMA_REF,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/ops/m5-operator-boards/canonical_boards.json")
}

fn load_fixture() -> OverviewBoardSet {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_set_matches_checked_in_fixture() {
    let built = operator_board_set();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code operator-board set drifted from the checked-in fixture; \
         regenerate it with `cargo run -p aureline-support --example dump_m5_operator_boards`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_OPERATOR_BOARDS_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_OPERATOR_BOARDS_SCHEMA_REF);
    assert_eq!(fixture.matrix_ref, M5_OPERATOR_BOARDS_MATRIX_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: OverviewBoardSet =
        serde_json::from_str(&serde_json::to_string(&fixture).expect("serializes"))
            .expect("round-trips");
    assert_eq!(roundtrip, fixture);
}

#[test]
fn every_frozen_invariant_holds() {
    let fixture = load_fixture();
    assert!(!fixture.invariants.is_empty());
    for invariant in &fixture.invariants {
        assert!(
            invariant.holds,
            "frozen invariant must hold: {}",
            invariant.invariant_id
        );
    }
    assert!(fixture.all_invariants_hold());
}

#[test]
fn set_covers_every_first_real_board() {
    let fixture = load_fixture();
    assert_eq!(fixture.boards.len(), BoardClass::ALL.len());
    for board in BoardClass::ALL {
        let entry = fixture.board(board).expect("board present");
        assert!(!entry.tiles.is_empty());
        assert!(!entry.saved_views.is_empty());
        assert_eq!(entry.surface_id, board.surface().surface_id());
    }
}

#[test]
fn fixture_tiles_carry_canonical_objects_and_visible_owners() {
    let fixture = load_fixture();
    for board in &fixture.boards {
        for tile in &board.tiles {
            assert!(tile.object_ref.starts_with("aureline://"));
            assert_eq!(tile.open_detail_ref, tile.object_ref);
            assert!(!tile.owner.is_empty());
            assert!(!tile.decision_right.is_empty());
            if tile.blocker_waiver.requires_reason() {
                assert!(!tile.blocker_reason.is_empty());
            }
        }
    }
}

#[test]
fn fixture_has_no_silent_green_tiles() {
    use aureline_support::m5_operator_boards::{compute_effective_state, FreshnessClass};
    use aureline_support::m5_operator_surfaces::OperatorStateClass;
    let fixture = load_fixture();
    for board in &fixture.boards {
        for tile in &board.tiles {
            assert_eq!(
                tile.effective_state,
                compute_effective_state(tile.displayed_state, tile.freshness, tile.blocker_waiver)
            );
            // A stale or waived tile is never reported clear.
            let stale = !matches!(
                tile.freshness,
                FreshnessClass::Fresh | FreshnessClass::Recent
            );
            let waived = tile.blocker_waiver != BlockerWaiverClass::None;
            if stale || waived {
                assert_ne!(
                    tile.effective_state,
                    OperatorStateClass::Clear,
                    "{} must not show clear when stale or waived",
                    tile.tile_id
                );
            }
        }
    }
}

#[test]
fn fixture_export_parity_holds() {
    let fixture = load_fixture();
    for board in &fixture.boards {
        let recomputed = export_board_view(board, &board.default_view).expect("default exports");
        assert_eq!(
            recomputed,
            board.export,
            "{} frozen export must equal re-applying its default view",
            board.board.as_str()
        );
    }
}

#[test]
fn human_readable_projection_renders_for_support() {
    let fixture = load_fixture();
    let lines = operator_board_lines(&fixture);
    assert!(lines
        .iter()
        .any(|line| line.contains("Operator overview boards")));
    for board in BoardClass::ALL {
        assert!(
            lines.iter().any(|line| line.contains(board.as_str())),
            "projection must mention board {}",
            board.as_str()
        );
    }
}
