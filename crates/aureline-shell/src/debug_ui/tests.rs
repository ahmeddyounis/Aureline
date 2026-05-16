use super::*;

#[test]
fn protected_walk_seeds_six_surfaces_with_two_narrowed() {
    let projection = seeded_protected_walk_projection();
    assert_eq!(
        projection.record_kind, DEBUG_UI_BETA_PROJECTION_RECORD_KIND,
        "record kind matches the beta lane"
    );
    assert_eq!(projection.surfaces.len(), 6, "all six surfaces present");
    assert_eq!(projection.shared_contract_ref, DEBUG_UI_BETA_SHARED_CONTRACT_REF);
    assert!(!projection.active_binding.is_empty());
    let narrowed: Vec<&str> = projection
        .surfaces
        .iter()
        .filter(|r| {
            matches!(
                r.availability_class,
                DebugUiAvailabilityClass::NarrowedByDroppedCapability
            )
        })
        .map(|r| r.surface_class_token.as_str())
        .collect();
    // log_points was dropped during negotiation; breakpoints + console
    // depend on it and must narrow, the other four stay available.
    assert!(narrowed.contains(&"breakpoints"), "breakpoints narrows");
    assert!(narrowed.contains(&"debug_console"), "debug_console narrows");
    assert_eq!(narrowed.len(), 2, "only two surfaces narrow");
    assert!(projection.honesty_marker_present, "narrowed surfaces light the marker");
    validate_debug_ui_projection(&projection).expect("seeded protected walk validates");
}

#[test]
fn reconnect_drill_drops_all_content_and_surfaces() {
    let projection = seeded_reconnect_drill_projection();
    for row in &projection.surfaces {
        assert!(
            matches!(
                row.availability_class,
                DebugUiAvailabilityClass::UnavailableDuringReconnect
            ),
            "surface {} must mark unavailable_during_reconnect",
            row.surface_class_token
        );
        assert!(
            matches!(
                row.focus_return_class,
                DebugUiFocusReturnClass::SessionCard
            ),
            "surface {} must park focus on the session card",
            row.surface_class_token
        );
        assert!(matches!(
            row.source_jump_stability_class,
            SourceJumpStabilityClass::UnstableDuringReconnect
        ));
        assert_eq!(row.content_row_count, 0, "reconnect drops content");
    }
    assert!(projection.breakpoints.is_empty());
    assert!(projection.call_stack_frames.is_empty());
    assert!(projection.variable_scopes.is_empty());
    assert!(projection.watch_expressions.is_empty());
    assert!(projection.evaluate_requests.is_empty());
    assert!(projection.console_lines.is_empty());
    assert!(projection.honesty_marker_present, "reconnect lights the marker");
    validate_debug_ui_projection(&projection).expect("reconnect drill validates");
}

#[test]
fn no_session_drill_routes_focus_to_session_card_and_refuses_jumps() {
    let projection = seeded_no_session_drill_projection();
    assert!(projection.active_binding.is_empty());
    for row in &projection.surfaces {
        assert!(matches!(
            row.availability_class,
            DebugUiAvailabilityClass::UnavailableNoActiveSession
        ));
        assert!(matches!(
            row.source_jump_stability_class,
            SourceJumpStabilityClass::NotOfferedNoSession
        ));
    }
    assert!(!projection.honesty_marker_present);
    validate_debug_ui_projection(&projection).expect("no-session drill validates");
}

#[test]
fn validator_flags_cross_session_content_leak() {
    let mut projection = seeded_protected_walk_projection();
    // Re-key a breakpoint onto a different session and target.
    let leaked = projection.breakpoints[0].clone();
    let mut leaked = leaked;
    leaked.session_id = "session:rogue".into();
    leaked.canonical_target_id = "target:rogue".into();
    projection.breakpoints[0] = leaked;
    let defects = validate_debug_ui_projection(&projection)
        .expect_err("validator must flag cross-session leak");
    assert!(defects.iter().any(|d| matches!(
        d.defect_kind,
        DebugUiDefectKind::ContentRowNotBoundToActiveSession
    )));
}

#[test]
fn validator_flags_hidden_capability_downgrade() {
    let mut projection = seeded_protected_walk_projection();
    // Flip the narrowed breakpoints row to claim full availability
    // without clearing the dropped-capability list. The validator must
    // detect the hidden downgrade.
    for row in projection.surfaces.iter_mut() {
        if row.surface_class_token == "breakpoints" {
            row.availability_class = DebugUiAvailabilityClass::Available;
            row.availability_class_token =
                DebugUiAvailabilityClass::Available.as_str().to_owned();
        }
    }
    let defects = validate_debug_ui_projection(&projection)
        .expect_err("validator must flag hidden downgrade");
    assert!(defects.iter().any(|d| matches!(
        d.defect_kind,
        DebugUiDefectKind::HiddenCapabilityDowngrade
    )));
}

#[test]
fn validator_flags_unsafe_source_jump_during_reconnect() {
    let mut projection = seeded_reconnect_drill_projection();
    for row in projection.surfaces.iter_mut() {
        if row.surface_class_token == "call_stack" {
            row.source_jump_stability_class = SourceJumpStabilityClass::Stable;
            row.source_jump_stability_class_token =
                SourceJumpStabilityClass::Stable.as_str().to_owned();
            row.focus_return_class = DebugUiFocusReturnClass::InvokingSurface;
            row.focus_return_class_token =
                DebugUiFocusReturnClass::InvokingSurface.as_str().to_owned();
        }
    }
    let defects = validate_debug_ui_projection(&projection)
        .expect_err("validator must flag unsafe posture");
    assert!(defects.iter().any(|d| matches!(
        d.defect_kind,
        DebugUiDefectKind::UnsafeSourceJumpDuringNonSteadyState
    )));
    assert!(defects.iter().any(|d| matches!(
        d.defect_kind,
        DebugUiDefectKind::UnsafeFocusReturnDuringNonSteadyState
    )));
}

#[test]
fn support_export_counts_surface_availabilities() {
    let projection = seeded_protected_walk_projection();
    let export = DebugUiSupportExport::from_projection(
        "support-export:debug-ui-beta:001",
        "2026-05-15T00:00:11Z",
        projection,
    );
    assert!(export.raw_private_material_excluded);
    let available = export.availability_counts.get("available").copied().unwrap_or_default();
    let narrowed = export
        .availability_counts
        .get("narrowed_by_dropped_capability")
        .copied()
        .unwrap_or_default();
    assert_eq!(available, 4, "four surfaces stay available");
    assert_eq!(narrowed, 2, "two surfaces narrow");
}

#[test]
fn projection_round_trips_through_serde() {
    let projection = seeded_protected_walk_projection();
    let json = serde_json::to_string(&projection).expect("serialize");
    let parsed: DebugUiProjection = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(parsed, projection);
}
