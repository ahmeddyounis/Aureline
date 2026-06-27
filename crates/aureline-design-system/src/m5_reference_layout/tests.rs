//! Inline tests for the M5 design-system reference-layout package.

use super::*;

fn canonical() -> M5ReferenceLayoutPackage {
    seeded_m5_reference_layout_package()
}

#[test]
fn canonical_package_validates() {
    let package = canonical();
    assert!(package.validate().is_empty(), "{:?}", package.validate());
    assert_eq!(package.record_kind, M5_REFERENCE_LAYOUT_PACKAGE_RECORD_KIND);
    assert_eq!(package.package_id, M5_REFERENCE_LAYOUT_PACKAGE_ID);
    assert_eq!(package.package_version, M5_REFERENCE_LAYOUT_PACKAGE_VERSION);
}

#[test]
fn package_publishes_every_workspace_kind() {
    let package = canonical();
    for kind in M5WorkspaceKind::ALL {
        let layout = package
            .layout(kind)
            .unwrap_or_else(|| panic!("missing {}", kind.as_str()));
        assert_eq!(layout.workspace_kind, kind);
        assert_eq!(
            layout.workspace_id,
            format!("design-system:reference-layout:{}", kind.as_str())
        );
    }
    assert_eq!(package.layouts.len(), M5WorkspaceKind::ALL.len());
}

#[test]
fn every_layout_claims_a_required_main_work_surface() {
    let package = canonical();
    for layout in &package.layouts {
        let main = layout
            .zone(M5ShellZone::MainWorkspace)
            .unwrap_or_else(|| panic!("{} has no main workspace", layout.workspace_id));
        assert!(
            main.required,
            "{} main workspace must be required",
            layout.workspace_id
        );
        assert!(!layout.required_zones().is_empty());
        // The status strip is always claimed so status truth persists.
        assert!(layout.zone(M5ShellZone::StatusBar).is_some());
    }
}

#[test]
fn collapse_rules_cover_every_adaptive_class_and_never_collapse_persistent_zones() {
    let package = canonical();
    for layout in &package.layouts {
        for class in M5AdaptiveClass::ALL {
            let rule = layout
                .collapse_rule(class)
                .unwrap_or_else(|| panic!("{} missing {}", layout.workspace_id, class.as_str()));
            let occupied: std::collections::BTreeSet<M5ShellZone> =
                layout.occupied_zones().into_iter().collect();
            for zone in &rule.collapsed_zones {
                assert!(occupied.contains(zone), "collapses unoccupied zone");
                assert!(!zone.is_persistent(), "collapses persistent zone");
            }
        }
        // The widest class never collapses anything.
        let expanded = layout
            .collapse_rule(M5AdaptiveClass::ExpandedDesktop)
            .unwrap();
        assert!(expanded.collapsed_zones.is_empty());
        assert_eq!(expanded.placement, M5FallbackPlacement::Docked);
    }
}

#[test]
fn every_layout_offers_a_reopen_and_a_reset_route() {
    let package = canonical();
    for layout in &package.layouts {
        assert!(
            !layout.routes_of_kind(M5LayoutRouteKind::Reopen).is_empty(),
            "{} has no reopen route",
            layout.workspace_id
        );
        let resets = layout.routes_of_kind(M5LayoutRouteKind::Reset);
        assert_eq!(resets.len(), 1, "{} reset routes", layout.workspace_id);
        assert_eq!(resets[0].keys, "Ctrl+Alt+0");
    }
}

#[test]
fn missing_dependency_rules_reference_occupied_zones_and_governed_messages() {
    let package = canonical();
    for layout in &package.layouts {
        assert!(!layout.missing_dependency_rules.is_empty());
        let occupied: std::collections::BTreeSet<M5ShellZone> =
            layout.occupied_zones().into_iter().collect();
        for rule in &layout.missing_dependency_rules {
            assert!(occupied.contains(&rule.affected_zone));
            assert!(rule
                .placeholder_message_id
                .starts_with(M5_REFERENCE_LAYOUT_MESSAGE_ID_PREFIX));
        }
    }
}

#[test]
fn governed_zone_tokens_match_the_canonical_shell_vocabulary() {
    // The zone tokens this lane publishes must match the canonical shell zone tokens so shell code
    // consumes the same identities the descriptors name.
    assert_eq!(
        M5ShellZone::ALL.map(|z| z.as_str()),
        [
            "title_context_bar",
            "activity_rail",
            "left_sidebar",
            "main_workspace",
            "right_inspector",
            "bottom_panel",
            "status_bar",
            "transient_overlay",
        ]
    );
    assert_eq!(
        M5AdaptiveClass::ALL.map(|c| c.as_str()),
        ["compact_desktop", "standard_desktop", "expanded_desktop"]
    );
    assert_eq!(M5FallbackPlacement::Sheet.as_str(), "sheet");
    assert_eq!(
        M5PlaceholderClass::MissingProvider.as_str(),
        "missing_provider"
    );
}

#[test]
fn conformance_packet_resolves_collapses_and_missing_deps_to_slot_ids() {
    let package = canonical();
    let conformance = package.conformance_packet();
    assert_eq!(
        conformance.total_workspaces,
        M5WorkspaceKind::ALL.len() as u32
    );
    assert_eq!(
        conformance.record_kind,
        M5_SHELL_SLOT_CONFORMANCE_RECORD_KIND
    );

    let notebook = conformance.workspace(M5WorkspaceKind::Notebook).unwrap();
    // The main work surface is a required slot expectation a feature test can assert against.
    let main = notebook.slot(M5ShellZone::MainWorkspace).unwrap();
    assert_eq!(main.slot_id, "slot.main_workspace.working_set");
    assert!(main.required);

    // The compact-desktop collapse is resolved to the exact slots that collapse, not just zones.
    let compact = notebook
        .collapse_expectations
        .iter()
        .find(|c| c.adaptive_class == M5AdaptiveClass::CompactDesktop)
        .unwrap();
    assert_eq!(compact.placement, M5FallbackPlacement::Sheet);
    assert!(compact
        .collapsed_slot_ids
        .contains(&"slot.right_inspector.contextual_detail".to_owned()));
    assert!(compact
        .collapsed_slot_ids
        .contains(&"slot.sidebar.section_surface".to_owned()));

    // Missing-dependency expectations resolve the affected zone to its occupied slot.
    let kernel = notebook
        .missing_dependency_expectations
        .iter()
        .find(|m| m.dependency_id == "notebook.kernel_runtime")
        .unwrap();
    assert_eq!(
        kernel.affected_slot_id.as_deref(),
        Some("slot.bottom_panel.tool_panels")
    );
    assert_eq!(
        kernel.placeholder_class,
        M5PlaceholderClass::MissingProvider
    );

    // The total slot expectations equals the sum of occupied zones across workspaces.
    let expected: u32 = package
        .layouts
        .iter()
        .map(|l| l.zone_occupancy.len() as u32)
        .sum();
    assert_eq!(conformance.total_slot_expectations, expected);
}

#[test]
fn release_packet_summarizes_every_layout() {
    let package = canonical();
    let release = package.release_packet();
    assert_eq!(release.total_layouts, M5WorkspaceKind::ALL.len() as u32);
    assert_eq!(release.layout_summaries.len(), M5WorkspaceKind::ALL.len());
    for summary in &release.layout_summaries {
        assert!(summary.zone_count >= 1);
        assert!(summary.required_zone_count >= 1);
        assert_eq!(summary.collapse_rule_count, 3);
        assert!(summary.missing_dependency_count >= 1);
        assert!(summary.reopen_route_count >= 2);
    }
}

#[test]
fn export_import_round_trips_and_revalidates() {
    let package = canonical();
    let json = package.export_safe_json();
    let imported = M5ReferenceLayoutPackage::from_json(&json).expect("imports");
    assert_eq!(imported, package);
    assert!(imported.validate().is_empty());
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = canonical().export_safe_json().to_lowercase();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("password"));
    assert!(!json.contains("authorization"));
    assert!(!json.contains("bearer "));
}

#[test]
fn validation_rejects_main_workspace_not_required() {
    let mut package = canonical();
    let layout = &mut package.layouts[0];
    let main = layout
        .zone_occupancy
        .iter_mut()
        .find(|o| o.zone == M5ShellZone::MainWorkspace)
        .unwrap();
    main.required = false;
    assert!(package
        .validate()
        .contains(&M5ReferenceLayoutViolation::ZoneOccupancyIncomplete));
}

#[test]
fn validation_rejects_collapsing_a_persistent_zone() {
    let mut package = canonical();
    let layout = &mut package.layouts[0];
    let rule = layout
        .responsive_collapse
        .iter_mut()
        .find(|r| r.adaptive_class == M5AdaptiveClass::CompactDesktop)
        .unwrap();
    rule.collapsed_zones.push(M5ShellZone::MainWorkspace);
    assert!(package
        .validate()
        .contains(&M5ReferenceLayoutViolation::CollapseRulesIncomplete));
}

#[test]
fn validation_rejects_missing_an_adaptive_class() {
    let mut package = canonical();
    package.layouts[0]
        .responsive_collapse
        .retain(|r| r.adaptive_class != M5AdaptiveClass::StandardDesktop);
    assert!(package
        .validate()
        .contains(&M5ReferenceLayoutViolation::CollapseRulesIncomplete));
}

#[test]
fn validation_rejects_missing_dependency_on_unoccupied_zone() {
    let mut package = canonical();
    // Docs does not occupy the bottom panel; pointing a missing-dependency rule at it is invalid.
    let docs = package
        .layouts
        .iter_mut()
        .find(|l| l.workspace_kind == M5WorkspaceKind::Docs)
        .unwrap();
    assert!(docs.zone(M5ShellZone::BottomPanel).is_none());
    docs.missing_dependency_rules[0].affected_zone = M5ShellZone::BottomPanel;
    assert!(package
        .validate()
        .contains(&M5ReferenceLayoutViolation::MissingDependencyRulesIncomplete));
}

#[test]
fn validation_rejects_layout_without_reset_route() {
    let mut package = canonical();
    package.layouts[0]
        .reopen_routes
        .retain(|r| r.route_kind != M5LayoutRouteKind::Reset);
    assert!(package
        .validate()
        .contains(&M5ReferenceLayoutViolation::ReopenRoutesIncomplete));
}

#[test]
fn validation_rejects_bad_package_version() {
    let mut package = canonical();
    package.package_version = "1.0".to_owned();
    assert!(package
        .validate()
        .contains(&M5ReferenceLayoutViolation::BadPackageVersion));
}

#[test]
fn validation_rejects_duplicate_workspace_kind() {
    let mut package = canonical();
    let extra = package.layouts[0].clone();
    package.layouts.push(extra);
    let violations = package.validate();
    assert!(violations.contains(&M5ReferenceLayoutViolation::DuplicateWorkspaceKind));
}

#[test]
fn checked_package_fixture_matches_seed_and_validates() {
    let from_disk =
        current_stable_m5_reference_layout_package().expect("checked package validates");
    assert_eq!(
        from_disk,
        canonical(),
        "checked reference layout package drifted from the seed builder"
    );
}

#[test]
fn checked_release_packet_matches_computed() {
    let raw = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-design-system-proof/reference-layout-release.json"
    ));
    let from_disk: M5ReferenceLayoutReleasePacket =
        serde_json::from_str(raw).expect("release packet parses");
    assert_eq!(
        from_disk,
        seeded_m5_reference_layout_package().release_packet(),
        "checked release packet drifted from the computed release packet"
    );
}

#[test]
fn checked_conformance_packet_matches_computed() {
    let raw = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-design-system-proof/reference-layout-conformance.json"
    ));
    let from_disk: M5ShellSlotConformancePacket =
        serde_json::from_str(raw).expect("conformance packet parses");
    assert_eq!(
        from_disk,
        seeded_m5_reference_layout_package().conformance_packet(),
        "checked conformance packet drifted from the computed conformance packet"
    );
}

#[test]
fn checked_per_workspace_fixtures_match_seed() {
    // One checked-in fixture per workspace, matching the layout the seed builds.
    macro_rules! check_workspace {
        ($kind:expr, $file:literal) => {{
            let raw = include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/ui/m5-reference-layout/",
                $file
            ));
            let from_disk: M5WorkspaceReferenceLayout =
                serde_json::from_str(raw).expect("workspace fixture parses");
            let expected = seeded_m5_reference_layout_package()
                .layout($kind)
                .expect("layout present")
                .clone();
            assert_eq!(from_disk, expected, "{} fixture drifted", $file);
        }};
    }
    check_workspace!(M5WorkspaceKind::Notebook, "workspace-notebook.json");
    check_workspace!(M5WorkspaceKind::DataGrid, "workspace-data_grid.json");
    check_workspace!(M5WorkspaceKind::Profiler, "workspace-profiler.json");
    check_workspace!(M5WorkspaceKind::Pipeline, "workspace-pipeline.json");
    check_workspace!(M5WorkspaceKind::Docs, "workspace-docs.json");
    check_workspace!(M5WorkspaceKind::Preview, "workspace-preview.json");
    check_workspace!(M5WorkspaceKind::Incident, "workspace-incident.json");
    check_workspace!(M5WorkspaceKind::Companion, "workspace-companion.json");
}
