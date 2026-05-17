//! Integration tests for the Support Center beta shell surface.
//!
//! Loads every protected fixture from
//! `fixtures/ux/m3/support_center/`, replays the
//! [`SupportCenterBetaEvaluator`], and re-asserts the after-edit
//! refusals declared in the corpus README:
//!
//! - Surface MUST cover safe-mode, doctor, bisect, repair preview, and
//!   an export action.
//! - Local-only recovery lanes MUST remain accountless and never
//!   require a hosted service.
//! - Every launch action MUST preserve `user_authored_files`.
//! - Every export route MUST name a local-first path and refuse an
//!   upload-first first action.
//! - The support packet projection MUST exclude raw private material
//!   and ambient authority.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_shell::support_center::{
    load_surface_from_yaml, SupportCenterBetaEvaluator, SupportCenterBetaLaneClass,
    SupportCenterBetaSurface, SupportCenterDegradedTruthClass, SupportCenterExportRouteRow,
    SupportCenterLaunchActionClass, SupportCenterLaunchActionRow,
    SupportCenterPreservedStateClass, SupportCenterServiceDependencyClass,
    SUPPORT_CENTER_BETA_SCHEMA_VERSION, SUPPORT_CENTER_BETA_SURFACE_RECORD_KIND,
};

const FIXTURE_FILES: &[&str] = &[
    "local_only_baseline.yaml",
    "post_crash_loop_degraded.yaml",
    "managed_workspace_export_only.yaml",
];

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("repo root")
        .join("fixtures")
        .join("ux")
        .join("m3")
        .join("support_center")
}

fn load_fixture(name: &str) -> SupportCenterBetaSurface {
    let path = fixtures_root().join(name);
    let yaml = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display()));
    load_surface_from_yaml(&yaml)
        .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

#[test]
fn every_protected_fixture_loads_and_validates() {
    for name in FIXTURE_FILES {
        let surface = load_fixture(name);
        assert_eq!(
            surface.schema_version, SUPPORT_CENTER_BETA_SCHEMA_VERSION,
            "{name}: schema_version must be frozen at {}",
            SUPPORT_CENTER_BETA_SCHEMA_VERSION
        );
        assert_eq!(
            surface.record_kind, SUPPORT_CENTER_BETA_SURFACE_RECORD_KIND,
            "{name}: record_kind must be the surface record"
        );
        let violations = SupportCenterBetaEvaluator::validate(&surface);
        assert!(
            violations.is_empty(),
            "{name}: surface failed validation: {:?}",
            violations
        );
    }
}

#[test]
fn manifest_lists_every_protected_fixture_on_disk() {
    let manifest_path = fixtures_root().join("manifest.yaml");
    let manifest =
        fs::read_to_string(&manifest_path).expect("read fixtures/ux/m3/support_center/manifest.yaml");
    for name in FIXTURE_FILES {
        assert!(
            manifest.contains(name),
            "manifest.yaml must list scenario file {name}"
        );
    }
}

#[test]
fn every_surface_covers_the_five_required_launch_actions() {
    let required = [
        SupportCenterLaunchActionClass::EnterSafeMode,
        SupportCenterLaunchActionClass::OpenProjectDoctor,
        SupportCenterLaunchActionClass::StartExtensionBisect,
        SupportCenterLaunchActionClass::OpenRepairPreview,
    ];
    for name in FIXTURE_FILES {
        let surface = load_fixture(name);
        let classes: BTreeSet<_> = surface
            .launch_action_rows
            .iter()
            .map(|r| r.launch_action_class)
            .collect();
        for class in &required {
            assert!(
                classes.contains(class),
                "{name}: missing required launch action {}",
                class.as_str()
            );
        }
        let has_export = surface
            .launch_action_rows
            .iter()
            .any(|r| r.launch_action_class.is_export_action());
        assert!(has_export, "{name}: missing export or preview action");
    }
}

#[test]
fn local_only_recovery_lanes_remain_accountless_and_local_only() {
    for name in FIXTURE_FILES {
        let surface = load_fixture(name);
        for row in &surface.launch_action_rows {
            if SupportCenterBetaEvaluator::is_local_only_recovery_lane(row.beta_lane_class) {
                assert!(
                    !row.service_dependency_class.requires_hosted_service(),
                    "{name}: local-only lane {} on row {} must not require a hosted service",
                    row.beta_lane_class.as_str(),
                    row.row_id
                );
            }
        }
    }
}

#[test]
fn every_export_route_is_local_first_and_refuses_upload_first() {
    for name in FIXTURE_FILES {
        let surface = load_fixture(name);
        for row in &surface.export_route_rows {
            assert!(
                row.local_first_path_named,
                "{name}: export route {} must pin local_first_path_named=true",
                row.row_id
            );
            assert!(
                !row.upload_required_for_first_action,
                "{name}: export route {} must pin upload_required_for_first_action=false",
                row.row_id
            );
        }
    }
}

#[test]
fn every_launch_action_preserves_user_authored_files() {
    for name in FIXTURE_FILES {
        let surface = load_fixture(name);
        for row in &surface.launch_action_rows {
            assert!(
                row.preserved_state_classes
                    .contains(&SupportCenterPreservedStateClass::UserAuthoredFiles),
                "{name}: launch action {} must preserve user_authored_files",
                row.row_id
            );
        }
    }
}

#[test]
fn support_packet_excludes_raw_private_material_and_ambient_authority() {
    for name in FIXTURE_FILES {
        let surface = load_fixture(name);
        let packet =
            surface.support_packet(format!("packet:{}", surface.surface_id), &surface.emitted_at);
        assert!(packet.raw_private_material_excluded, "{name}: packet leaked raw private material");
        assert!(packet.ambient_authority_excluded, "{name}: packet leaked ambient authority");
        assert_eq!(packet.surface_ref, surface.surface_id);
        assert_eq!(
            packet.launch_action_row_refs.len(),
            surface.launch_action_rows.len()
        );
    }
}

#[test]
fn post_crash_loop_surface_names_explicit_exit_command() {
    let surface = load_fixture("post_crash_loop_degraded.yaml");
    let active_rows: Vec<_> = surface
        .degraded_truth_rows
        .iter()
        .filter(|r| r.degraded_truth_class != SupportCenterDegradedTruthClass::NoneDegraded)
        .collect();
    assert!(!active_rows.is_empty(), "post-crash-loop surface must quote at least one active degraded row");
    for row in active_rows {
        assert!(
            row.active,
            "active degraded row {} must have active=true",
            row.row_id
        );
        let exit = row
            .exit_command_id
            .as_ref()
            .expect("active degraded row must name an exit_command_id");
        assert!(
            exit.starts_with("cmd:"),
            "exit_command_id must be a cmd:* id, got {}",
            exit
        );
    }
}

#[test]
fn after_edit_destructive_reset_is_refused() {
    let mut surface = load_fixture("local_only_baseline.yaml");
    // Remove user_authored_files from one row; the evaluator MUST
    // refuse the surface as a stand-in for a launch action that
    // would delete user-owned state.
    if let Some(row) = surface.launch_action_rows.first_mut() {
        row.preserved_state_classes
            .retain(|c| *c != SupportCenterPreservedStateClass::UserAuthoredFiles);
    }
    let violations = SupportCenterBetaEvaluator::validate(&surface);
    assert!(
        violations
            .iter()
            .any(|v| v.check_id == "launch_action.preserved_state.user_authored_files_missing"),
        "destructive launch action must be refused: {:?}",
        violations
    );
}

#[test]
fn after_edit_hidden_service_dependency_is_refused() {
    let mut surface = load_fixture("local_only_baseline.yaml");
    if let Some(row) = surface
        .launch_action_rows
        .iter_mut()
        .find(|r| r.beta_lane_class == SupportCenterBetaLaneClass::SafeMode)
    {
        row.service_dependency_class = SupportCenterServiceDependencyClass::RequiresHostedIntake;
    }
    let violations = SupportCenterBetaEvaluator::validate(&surface);
    assert!(
        violations
            .iter()
            .any(|v| v.check_id == "launch_action.local_only_lane_hosted_service"),
        "hidden hosted-service dependency must be refused"
    );
}

#[test]
fn after_edit_upload_first_export_route_is_refused() {
    let mut surface = load_fixture("local_only_baseline.yaml");
    if let Some(row) = surface
        .export_route_rows
        .iter_mut()
        .find(|r| r.local_first_path_named)
    {
        row.upload_required_for_first_action = true;
    }
    let violations = SupportCenterBetaEvaluator::validate(&surface);
    assert!(
        violations
            .iter()
            .any(|v| v.check_id == "export_route.upload_required_for_first_action"),
        "upload-first first action must be refused"
    );
}

#[test]
fn after_edit_missing_required_action_is_refused() {
    let mut surface = load_fixture("local_only_baseline.yaml");
    surface.launch_action_rows.retain(|r| {
        r.launch_action_class != SupportCenterLaunchActionClass::OpenRepairPreview
    });
    let violations = SupportCenterBetaEvaluator::validate(&surface);
    assert!(
        violations
            .iter()
            .any(|v| v.check_id == "surface.required_launch_action_missing"),
        "missing repair preview launch action must be refused"
    );
}

#[test]
fn after_edit_none_degraded_row_must_not_carry_exit_command() {
    let mut surface = load_fixture("local_only_baseline.yaml");
    if let Some(row) = surface
        .degraded_truth_rows
        .iter_mut()
        .find(|r| r.degraded_truth_class == SupportCenterDegradedTruthClass::NoneDegraded)
    {
        row.exit_command_id = Some("cmd:bogus.exit".into());
    }
    let violations = SupportCenterBetaEvaluator::validate(&surface);
    assert!(
        violations
            .iter()
            .any(|v| v.check_id == "degraded_truth.none_exit_command_present"),
        "none_degraded row carrying an exit command must be refused"
    );
}

#[test]
fn fixture_unique_action_command_ids() {
    for name in FIXTURE_FILES {
        let surface = load_fixture(name);
        ensure_no_blank_commands(name, &surface.launch_action_rows);
        ensure_no_blank_export_destinations(name, &surface.export_route_rows);
    }
}

fn ensure_no_blank_commands(scenario: &str, rows: &[SupportCenterLaunchActionRow]) {
    for row in rows {
        assert!(
            row.command_id.starts_with("cmd:"),
            "{scenario}: row {} has non-cmd command_id {}",
            row.row_id,
            row.command_id
        );
        assert!(
            !row.label.is_empty(),
            "{scenario}: row {} has empty label",
            row.row_id
        );
    }
}

fn ensure_no_blank_export_destinations(scenario: &str, rows: &[SupportCenterExportRouteRow]) {
    for row in rows {
        assert!(
            !row.destination_schema_ref.is_empty(),
            "{scenario}: export route {} has empty destination_schema_ref",
            row.row_id
        );
    }
}
