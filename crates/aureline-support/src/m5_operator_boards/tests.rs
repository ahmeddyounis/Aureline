//! Unit tests for the operator-board builder, the no-silent-green tile rule,
//! saved-view application, and export parity.

use super::*;

#[test]
fn set_validates_and_all_invariants_hold() {
    let set = operator_board_set();
    set.validate().expect("canonical board set validates");
    assert!(set.all_invariants_hold());
    assert!(!set.invariants.is_empty());
}

#[test]
fn set_is_deterministic() {
    assert_eq!(operator_board_set(), operator_board_set());
}

#[test]
fn set_is_support_export_safe() {
    let set = operator_board_set();
    assert!(set.raw_payload_excluded);
    assert!(set.is_support_export_safe());
}

#[test]
fn every_board_family_is_present_once() {
    let set = operator_board_set();
    assert_eq!(set.boards.len(), BoardClass::ALL.len());
    for class in BoardClass::ALL {
        let board = set.board(class).expect("board present");
        assert_eq!(board.board_id, class.board_id());
        assert!(!board.tiles.is_empty());
        assert!(!board.saved_views.is_empty());
    }
}

#[test]
fn boards_bind_a_canonical_matrix_surface() {
    let set = operator_board_set();
    let matrix = crate::m5_operator_surfaces::operator_surface_matrix();
    for board in &set.boards {
        assert_eq!(board.surface_id, board.surface.surface_id());
        assert!(
            matrix.surface(board.surface).is_some(),
            "board {} must bind a matrix surface",
            board.board.as_str()
        );
    }
}

#[test]
fn tiles_point_at_canonical_objects_not_dashboard_ids() {
    let set = operator_board_set();
    for board in &set.boards {
        for tile in &board.tiles {
            assert!(
                tile.object_ref.starts_with("aureline://"),
                "{} must carry a canonical object handle",
                tile.tile_id
            );
            assert_eq!(
                tile.open_detail_ref, tile.object_ref,
                "{} open-detail must route to its canonical object",
                tile.tile_id
            );
        }
    }
}

#[test]
fn no_silent_green_is_computed() {
    // A stale clear tile downgrades to unconfirmed.
    assert_eq!(
        compute_effective_state(
            OperatorStateClass::Clear,
            FreshnessClass::Stale,
            BlockerWaiverClass::None
        ),
        OperatorStateClass::Unconfirmed
    );
    // A fresh clear tile stays clear.
    assert_eq!(
        compute_effective_state(
            OperatorStateClass::Clear,
            FreshnessClass::Fresh,
            BlockerWaiverClass::None
        ),
        OperatorStateClass::Clear
    );
    // A waived tile is never green.
    assert_eq!(
        compute_effective_state(
            OperatorStateClass::Clear,
            FreshnessClass::Fresh,
            BlockerWaiverClass::Waived
        ),
        OperatorStateClass::Attention
    );
    // A blocked tile is blocked.
    assert_eq!(
        compute_effective_state(
            OperatorStateClass::Attention,
            FreshnessClass::Fresh,
            BlockerWaiverClass::Blocked
        ),
        OperatorStateClass::Blocked
    );
    // An expired waiver re-asserts the blocker.
    assert_eq!(
        compute_effective_state(
            OperatorStateClass::Clear,
            FreshnessClass::Fresh,
            BlockerWaiverClass::WaiverExpired
        ),
        OperatorStateClass::Blocked
    );
}

#[test]
fn every_tile_effective_state_matches_the_computed_rule() {
    let set = operator_board_set();
    for board in &set.boards {
        for tile in &board.tiles {
            assert_eq!(
                tile.effective_state,
                compute_effective_state(tile.displayed_state, tile.freshness, tile.blocker_waiver),
                "{} effective state must be the computed no-silent-green state",
                tile.tile_id
            );
        }
    }
}

#[test]
fn owner_and_blocker_reason_are_first_class() {
    let set = operator_board_set();
    for board in &set.boards {
        for tile in &board.tiles {
            assert!(!tile.owner.is_empty());
            assert!(!tile.decision_right.is_empty());
            if tile.blocker_waiver.requires_reason() {
                assert!(
                    !tile.blocker_reason.is_empty(),
                    "{} must show a visible blocker/waiver reason",
                    tile.tile_id
                );
            }
        }
    }
}

#[test]
fn at_least_one_board_proves_each_downgrade_path() {
    let set = operator_board_set();
    let all_tiles: Vec<&BoardTile> = set.boards.iter().flat_map(|b| b.tiles.iter()).collect();
    // A stale would-be-green tile downgraded to unconfirmed exists.
    assert!(all_tiles.iter().any(|t| {
        t.displayed_state == OperatorStateClass::Clear
            && !t.freshness.green_eligible()
            && t.effective_state == OperatorStateClass::Unconfirmed
    }));
    // A waived tile rendered as attention (never green) exists.
    assert!(all_tiles
        .iter()
        .any(|t| t.blocker_waiver == BlockerWaiverClass::Waived
            && t.effective_state == OperatorStateClass::Attention));
    // A blocked tile exists.
    assert!(all_tiles
        .iter()
        .any(|t| t.blocker_waiver == BlockerWaiverClass::Blocked
            && t.effective_state == OperatorStateClass::Blocked));
}

#[test]
fn default_view_resolves_and_orders_are_named() {
    let set = operator_board_set();
    for board in &set.boards {
        assert!(
            board
                .saved_views
                .iter()
                .any(|v| v.token == board.default_view),
            "{} default view must be a saved view",
            board.board.as_str()
        );
        for view in &board.saved_views {
            assert!(
                !view.order.reason.is_empty(),
                "{} order must be named",
                view.view_id
            );
        }
    }
}

#[test]
fn saved_view_filters_use_the_shared_vocabulary() {
    let set = operator_board_set();
    for board in &set.boards {
        for view in &board.saved_views {
            for clause in &view.filters {
                let facet = set.facet(clause.facet).expect("facet defined");
                if facet.closed_vocabulary {
                    for value in &clause.include_tokens {
                        assert!(
                            facet.allowed_tokens.contains(value),
                            "value {value} must be in facet {}",
                            clause.facet.as_str()
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn export_parity_holds_for_every_board_default_view() {
    let set = operator_board_set();
    for board in &set.boards {
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
fn applying_a_filter_view_narrows_and_preserves_truth() {
    let set = operator_board_set();
    let incident = set.board(BoardClass::IncidentResponse).expect("board");
    let export = export_board_view(incident, "blocked_and_waived").expect("view exports");
    // Only the blocked and waived incident tiles survive the filter.
    assert!(export.row_count >= 1);
    assert!(export.rows.iter().all(|r| matches!(
        r.blocker_waiver,
        BlockerWaiverClass::Blocked
            | BlockerWaiverClass::Waived
            | BlockerWaiverClass::WaiverExpired
    )));
    // The blocking row sorts above the waived row under severity-descending order.
    let blocked_pos = export
        .rows
        .iter()
        .position(|r| r.blocker_waiver == BlockerWaiverClass::Blocked);
    let waived_pos = export
        .rows
        .iter()
        .position(|r| r.blocker_waiver == BlockerWaiverClass::Waived);
    if let (Some(b), Some(w)) = (blocked_pos, waived_pos) {
        assert!(b < w, "blocked must sort above waived");
    }
    // Export preserves the reason verbatim.
    for row in &export.rows {
        let tile = incident
            .tiles
            .iter()
            .find(|t| t.tile_id == row.tile_id)
            .expect("row maps to a tile");
        assert_eq!(row.blocker_reason, tile.blocker_reason);
        assert_eq!(row.open_detail_ref, tile.object_ref);
    }
}

#[test]
fn export_board_view_returns_none_for_unknown_view() {
    let set = operator_board_set();
    let board = set.board(BoardClass::SupportQueue).expect("board");
    assert!(export_board_view(board, "no_such_view").is_none());
}

#[test]
fn validate_rejects_a_raw_payload_flag_flip() {
    let mut set = operator_board_set();
    set.raw_payload_excluded = false;
    assert!(set.validate().is_err());
    assert!(!set.is_support_export_safe());
}

#[test]
fn validate_rejects_an_unsafe_object_ref() {
    let mut set = operator_board_set();
    set.boards[0].tiles[0].object_ref = "https://internal.example.com/incident".to_owned();
    assert!(!set.is_support_export_safe());
    assert!(set.validate().is_err());
}

#[test]
fn validate_rejects_a_silent_green_tile() {
    let mut set = operator_board_set();
    // Force a stale tile to claim clear: the computed rule must reject it.
    let tile = &mut set.boards[0].tiles[1];
    tile.freshness = FreshnessClass::VeryStale;
    tile.displayed_state = OperatorStateClass::Clear;
    tile.effective_state = OperatorStateClass::Clear;
    assert!(set.validate().is_err());
}

#[test]
fn human_readable_projection_renders() {
    let set = operator_board_set();
    let lines = operator_board_lines(&set);
    assert!(lines.iter().any(|l| l.contains("Operator overview boards")));
    for class in BoardClass::ALL {
        assert!(
            lines.iter().any(|l| l.contains(class.as_str())),
            "projection must mention board {}",
            class.as_str()
        );
    }
}
