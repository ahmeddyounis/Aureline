//! Inline tests for the M5 design-system contract matrix.

use super::*;

fn canonical() -> M5DesignSystemContractMatrix {
    seeded_m5_design_system_contract_matrix()
}

#[test]
fn canonical_matrix_validates() {
    let matrix = canonical();
    assert!(matrix.validate().is_empty(), "{:?}", matrix.validate());
    assert_eq!(matrix.matrix_id, M5_DESIGN_SYSTEM_CONTRACT_MATRIX_ID);
    assert_eq!(
        matrix.record_kind,
        M5_DESIGN_SYSTEM_CONTRACT_MATRIX_RECORD_KIND
    );
}

#[test]
fn canonical_publishes_every_governed_object_kind() {
    let matrix = canonical();
    let present: std::collections::BTreeSet<M5ContractObjectKind> = matrix
        .contract_objects
        .iter()
        .map(|o| o.object_kind)
        .collect();
    for kind in M5ContractObjectKind::ALL {
        assert!(present.contains(&kind), "missing kind {}", kind.as_str());
    }
    // Every governed object names an owner, a first consumer, a canonical artifact, a proof
    // lane, and a release packet.
    for object in &matrix.contract_objects {
        assert!(!object.owner_role.trim().is_empty());
        assert!(!object.canonical_artifact_ref.trim().is_empty());
        assert!(!object.proof_lane_ref.trim().is_empty());
        assert!(!object.release_packet_ref.trim().is_empty());
        assert_eq!(object.schema_ref, object.object_kind.canonical_schema_ref());
    }
}

#[test]
fn canonical_is_all_conformant_green() {
    let matrix = canonical();
    assert!(!matrix.surfaces.is_empty());
    for surface in &matrix.surfaces {
        assert_eq!(
            surface.coverage_status,
            M5CoverageStatus::Conformant,
            "surface {} not conformant",
            surface.surface_id
        );
        assert_eq!(surface.signal, M5CoverageSignal::Green);
        assert!(surface.is_conformant());
        assert_eq!(surface.effective_class, M5DesignSystemClaimClass::Stable);
        assert!(surface.gaps.is_empty());
    }
    assert!(!matrix.blocks_stable_promotion());
    let dashboard = matrix.dashboard();
    assert_eq!(dashboard.green_count, matrix.surfaces.len() as u32);
    assert_eq!(dashboard.yellow_count, 0);
    assert_eq!(dashboard.red_count, 0);
}

#[test]
fn every_surface_maps_required_objects_present_in_inventory() {
    let matrix = canonical();
    for surface in &matrix.surfaces {
        assert!(!surface.required_objects.is_empty());
        for required in &surface.required_objects {
            assert!(
                matrix.object(&required.object_id).is_some(),
                "surface {} maps unmapped object {}",
                surface.surface_id,
                required.object_id
            );
        }
    }
}

#[test]
fn missing_object_drill_blocks_stable_promotion_and_names_the_gap() {
    let matrix = seeded_m5_design_system_contract_matrix_missing_object();
    assert!(matrix.validate().is_empty(), "{:?}", matrix.validate());
    assert!(matrix.blocks_stable_promotion());
    let shell = matrix
        .surfaces
        .iter()
        .find(|s| s.surface_class == LaunchSurfaceClass::ShellChrome)
        .expect("shell surface present");
    assert_eq!(shell.coverage_status, M5CoverageStatus::Uncovered);
    assert_eq!(shell.signal, M5CoverageSignal::Red);
    assert!(shell.is_blocked());
    assert_eq!(shell.effective_class, M5DesignSystemClaimClass::Held);
    assert_eq!(
        matrix.blocked_surface_ids(),
        vec![shell.surface_id.as_str()]
    );
    let gap = shell
        .gaps
        .iter()
        .find(|g| g.object_id == "design-system:component:diff-viewer")
        .expect("unmapped-object gap named");
    assert_eq!(gap.gap_kind, M5ContractGapKind::UnmappedObject);
    assert!(!gap.waived);
}

#[test]
fn stale_proof_drill_auto_narrows_without_blocking() {
    let matrix = seeded_m5_design_system_contract_matrix_stale_proof_retest_pending();
    assert!(matrix.validate().is_empty(), "{:?}", matrix.validate());
    // Stale proof narrows but never blocks: the surface keeps shipping at a reduced claim.
    assert!(!matrix.blocks_stable_promotion());
    let shell = matrix
        .surfaces
        .iter()
        .find(|s| s.surface_class == LaunchSurfaceClass::ShellChrome)
        .expect("shell surface present");
    assert_eq!(shell.coverage_status, M5CoverageStatus::RetestPending);
    assert_eq!(shell.signal, M5CoverageSignal::Yellow);
    assert!(shell.is_auto_narrowed());
    assert_eq!(shell.effective_class, M5DesignSystemClaimClass::Beta);
    let gap = shell
        .gaps
        .iter()
        .find(|g| g.gap_kind == M5ContractGapKind::StaleProof)
        .expect("stale-proof gap named");
    assert!(!gap.waived);
    // The dashboard names the stale object.
    assert!(matrix
        .dashboard()
        .stale_object_ids
        .iter()
        .any(|id| id == &gap.object_id));
}

#[test]
fn waiver_drill_ships_narrowed_but_stays_red() {
    let matrix = seeded_m5_design_system_contract_matrix_waived_narrowed();
    assert!(matrix.validate().is_empty(), "{:?}", matrix.validate());
    // The waived gap no longer blocks promotion, but the true status stays uncovered.
    assert!(!matrix.blocks_stable_promotion());
    let shell = matrix
        .surfaces
        .iter()
        .find(|s| s.surface_class == LaunchSurfaceClass::ShellChrome)
        .expect("shell surface present");
    assert_eq!(shell.coverage_status, M5CoverageStatus::Uncovered);
    assert_eq!(shell.signal, M5CoverageSignal::Red);
    assert!(shell.is_auto_narrowed());
    assert_eq!(shell.effective_class, M5DesignSystemClaimClass::Preview);
    assert_eq!(shell.waivers.len(), 1);
    let waiver = &shell.waivers[0];
    assert_eq!(waiver.object_id, "design-system:component:diff-viewer");
    assert_eq!(waiver.narrowed_to, M5DesignSystemClaimClass::Preview);
    assert!(!waiver.expires_at.trim().is_empty());
    let gap = shell
        .gaps
        .iter()
        .find(|g| g.object_id == "design-system:component:diff-viewer")
        .expect("gap named");
    assert!(gap.waived);
    let dashboard = matrix.dashboard();
    assert!(dashboard.active_waiver_ids.contains(&waiver.waiver_id));
    assert!(dashboard.waived_surface_ids.contains(&shell.surface_id));
}

#[test]
fn dashboard_traffic_light_matches_rows() {
    for matrix in [
        seeded_m5_design_system_contract_matrix(),
        seeded_m5_design_system_contract_matrix_missing_object(),
        seeded_m5_design_system_contract_matrix_stale_proof_retest_pending(),
        seeded_m5_design_system_contract_matrix_waived_narrowed(),
    ] {
        let dashboard = matrix.dashboard();
        assert_eq!(dashboard.total_surfaces, matrix.surfaces.len() as u32);
        assert_eq!(
            dashboard.total_objects,
            matrix.contract_objects.len() as u32
        );
        assert_eq!(
            dashboard.green_count + dashboard.yellow_count + dashboard.red_count,
            dashboard.total_surfaces
        );
        assert_eq!(
            dashboard.record_kind,
            M5_DESIGN_SYSTEM_DASHBOARD_RECORD_KIND
        );
    }
}

#[test]
fn detects_tampered_derived_fields() {
    let mut matrix = canonical();
    // Flip a derived status without changing the required objects; the validator must catch
    // the inconsistency.
    matrix.surfaces[0].coverage_status = M5CoverageStatus::Uncovered;
    let violations = matrix.validate();
    assert!(violations.contains(&M5ContractMatrixViolation::DerivedRowInconsistent));
}

#[test]
fn detects_vocabulary_drift() {
    let mut matrix = canonical();
    matrix.vocabulary_set.object_kinds.push("bogus".to_owned());
    assert!(matrix
        .validate()
        .contains(&M5ContractMatrixViolation::VocabularySetDrift));
}

#[test]
fn detects_object_schema_mismatch() {
    let mut matrix = canonical();
    matrix.contract_objects[0].schema_ref = "schemas/design-system/wrong.schema.json".to_owned();
    assert!(matrix
        .validate()
        .contains(&M5ContractMatrixViolation::ObjectSchemaMismatch));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk =
        current_stable_m5_design_system_contract_matrix().expect("checked matrix export validates");
    assert_eq!(
        from_disk,
        canonical(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn checked_dashboard_matches_seed() {
    let from_disk = current_stable_m5_design_system_dashboard().expect("checked dashboard parses");
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
            "/../../fixtures/ui/m5-design-system-contract-matrix/missing_object.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-design-system-contract-matrix/stale_proof_retest_pending.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-design-system-contract-matrix/waived_narrowed.json"
        )),
    ] {
        let matrix: M5DesignSystemContractMatrix =
            serde_json::from_str(raw).expect("fixture parses as contract matrix");
        assert!(
            matrix.validate().is_empty(),
            "fixture failed validation: {:?}",
            matrix.validate()
        );
    }
}

#[test]
fn gallery_foundations_fixture_matches_seed_and_validates() {
    let raw = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-component-gallery/foundations.json"
    ));
    let from_disk: M5FoundationsArtifact =
        serde_json::from_str(raw).expect("foundations fixture parses");
    assert!(
        from_disk.validate().is_empty(),
        "{:?}",
        from_disk.validate()
    );
    assert_eq!(from_disk, seeded_m5_foundations_artifact());
}

#[test]
fn gallery_reference_layout_fixture_matches_seed_and_validates() {
    let raw = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-component-gallery/reference-layout.json"
    ));
    let from_disk: M5ReferenceLayoutArtifact =
        serde_json::from_str(raw).expect("reference-layout fixture parses");
    assert!(
        from_disk.validate().is_empty(),
        "{:?}",
        from_disk.validate()
    );
    assert_eq!(from_disk, seeded_m5_reference_layout_artifact());
}

#[test]
fn gallery_component_contract_fixtures_match_seed_and_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-component-gallery/component-contract-shell_chrome.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-component-gallery/component-contract-command_palette.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-component-gallery/component-contract-trust_prompt.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-component-gallery/component-contract-notification_envelope.json"
        )),
    ] {
        let from_disk: M5ComponentContractArtifact =
            serde_json::from_str(raw).expect("component-contract fixture parses");
        assert!(
            from_disk.validate().is_empty(),
            "{:?}",
            from_disk.validate()
        );
        let seeded = seeded_m5_component_contract_gallery()
            .into_iter()
            .find(|c| c.component_id == from_disk.component_id)
            .expect("seeded component contract for fixture");
        assert_eq!(from_disk, seeded);
    }
}

#[test]
fn every_inventory_component_object_has_a_gallery_contract() {
    let matrix = canonical();
    let gallery = seeded_m5_component_contract_gallery();
    for object in matrix
        .contract_objects
        .iter()
        .filter(|o| o.object_kind == M5ContractObjectKind::ComponentContract)
    {
        assert!(
            gallery.iter().any(|c| c.component_id == object.object_id),
            "no gallery contract for component object {}",
            object.object_id
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
fn markdown_summary_names_objects_and_surfaces() {
    let summary =
        seeded_m5_design_system_contract_matrix_waived_narrowed().render_markdown_summary();
    assert!(summary.contains("Design-System Contract Matrix"));
    assert!(summary.contains("design-system:foundation:tokens"));
    assert!(summary.contains("design-system-surface:shell_chrome"));
    assert!(summary.contains("waiver:shell-chrome-diff-viewer"));
}
