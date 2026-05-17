//! Fixture replay for beta guided tours and learning mode.
//!
//! The checked-in files under `fixtures/help/m3/guided_tours/` are generated
//! by `aureline_shell_learning_mode_beta`, then replayed here so runtime,
//! docs, and support-export evidence stay aligned.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_commands::registry::seeded_registry;
use aureline_shell::learning_mode::{
    seeded_learning_mode_beta_manifest, seeded_learning_mode_beta_support_export,
    seeded_learning_mode_beta_surface_projection, validate_seeded_learning_mode_beta,
    LearningModeBetaManifest, LearningModeBetaSupportExport, LearningModeBetaSurfaceProjection,
    LEARNING_MODE_BETA_MANIFEST_ID, LEARNING_MODE_BETA_SCHEMA_VERSION,
    LEARNING_MODE_BETA_VERSION_REF,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/help/m3/guided_tours")
}

fn load_json<T: serde::de::DeserializeOwned>(file: &str) -> T {
    let path = fixtures_root().join(file);
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

fn load_yaml(file: &str) -> serde_yaml::Value {
    let path = fixtures_root().join(file);
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_yaml::from_str(&body)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[test]
fn manifest_fixture_matches_seeded_builder() {
    let from_file: LearningModeBetaManifest = load_json("manifest.json");
    let from_code = seeded_learning_mode_beta_manifest();

    assert_eq!(from_file, from_code);
    assert_eq!(from_file.schema_version, LEARNING_MODE_BETA_SCHEMA_VERSION);
    assert_eq!(from_file.manifest_id, LEARNING_MODE_BETA_MANIFEST_ID);
    assert_eq!(
        from_file.manifest_version_ref,
        LEARNING_MODE_BETA_VERSION_REF
    );
    assert!(from_file.downgrade_policy.independent_from_onboarding_core);
}

#[test]
fn generated_records_validate() {
    validate_seeded_learning_mode_beta(seeded_registry())
        .expect("seeded beta learning-mode records validate");

    let manifest: LearningModeBetaManifest = load_json("manifest.json");
    manifest
        .validate_against_registry(seeded_registry())
        .expect("fixture manifest validates");
}

#[test]
fn surface_projection_fixture_matches_seeded_builder() {
    let from_file: LearningModeBetaSurfaceProjection = load_json("surface_projection.json");
    assert_eq!(from_file, seeded_learning_mode_beta_surface_projection());

    assert!(from_file
        .rows
        .iter()
        .any(|row| row.release_label == "beta" && row.preview_required));
    assert!(from_file
        .rows
        .iter()
        .any(|row| row.release_label == "preview"
            && row.degradation_state == "graph_unavailable_placeholder"));
    assert!(from_file
        .rows
        .iter()
        .all(|row| row.explain_and_apply_separate && !row.citation_refs.is_empty()));
    assert!(from_file
        .profile_controls
        .iter()
        .all(|control| control.user_visible && !control.silent_write_allowed));
}

#[test]
fn support_export_fixture_matches_seeded_builder() {
    let from_file: LearningModeBetaSupportExport = load_json("support_export.json");
    let from_code = seeded_learning_mode_beta_support_export();
    let manifest = seeded_learning_mode_beta_manifest();

    assert_eq!(from_file, from_code);
    from_file
        .validate_against_manifest(&manifest)
        .expect("support export fixture validates");
    assert!(!from_file.raw_bodies_exported);
    assert!(from_file
        .omitted_material_classes
        .iter()
        .any(|class| class == "raw_step_body"));
}

#[test]
fn mutation_preview_path_uses_registry_preview_approval_and_rollback() {
    let manifest = seeded_learning_mode_beta_manifest();
    let fixture = load_yaml("mutation_preview_path.yaml");
    let expected = &fixture["expected"];
    let step_ref = expected["step_ref"].as_str().expect("step ref");
    let step = manifest.step(step_ref).expect("step exists");
    let action = step
        .actions
        .iter()
        .find(|action| {
            action.action_id
                == expected["mutation_action_ref"]
                    .as_str()
                    .expect("mutation action ref")
        })
        .expect("mutation action exists");
    let rail = manifest
        .exercise_rails
        .iter()
        .find(|rail| rail.current_step_ref == step_ref)
        .expect("rail exists");

    assert_eq!(
        action.command_id.as_deref(),
        Some(expected["command_id"].as_str().expect("command id"))
    );
    assert_eq!(
        action.command_metadata_source,
        expected["command_metadata_source"].as_str().unwrap()
    );
    assert_eq!(
        action.action_safety_class,
        expected["action_safety_class"].as_str().unwrap()
    );
    assert_eq!(
        action.preview_sheet_ref.as_deref(),
        Some(expected["preview_sheet_ref"].as_str().unwrap())
    );
    assert_eq!(
        action.approval_path_ref.as_deref(),
        Some(expected["approval_path_ref"].as_str().unwrap())
    );
    assert_eq!(
        action.rollback_semantics_ref.as_deref(),
        Some(expected["rollback_semantics_ref"].as_str().unwrap())
    );
    assert_eq!(
        action.evidence_packet_rule_ref.as_deref(),
        Some(expected["evidence_packet_rule_ref"].as_str().unwrap())
    );
    assert!(action.explain_and_apply_separate);
    assert!(action.mutates_workspace);
    assert!(step.actions.iter().any(|action| {
        action.action_id == expected["explain_action_ref"].as_str().unwrap()
            && !action.mutates_workspace
    }));
    assert_eq!(
        rail.hint_reveal_state.rate_limit_ref,
        expected["rate_limit_ref"].as_str().unwrap()
    );
    assert_eq!(
        rail.hint_reveal_state.rate_limit_window_seconds,
        expected["rate_limit_window_seconds"].as_u64().unwrap() as u32
    );
    assert_eq!(
        rail.hint_reveal_state.max_reveals_per_window,
        expected["max_reveals_per_window"].as_u64().unwrap() as u32
    );
    assert_eq!(
        rail.restart_safe,
        expected["restart_safe"].as_bool().unwrap()
    );
    assert!(!expected["hidden_mutation_allowed"].as_bool().unwrap());
}

#[test]
fn degraded_sources_stay_labeled_and_downgradable() {
    let manifest = seeded_learning_mode_beta_manifest();
    let fixture = load_yaml("degraded_sources.yaml");
    let expected = &fixture["expected"];

    assert_eq!(
        manifest.downgrade_policy.independent_from_onboarding_core,
        expected["downgrade_policy"]["independent_from_onboarding_core"]
            .as_bool()
            .unwrap()
    );
    assert_eq!(
        manifest
            .downgrade_policy
            .stale_evidence_can_suppress_learning,
        expected["downgrade_policy"]["stale_evidence_can_suppress_learning"]
            .as_bool()
            .unwrap()
    );

    for expected_package in expected["packages"].as_sequence().expect("packages") {
        let package_ref = expected_package["package_ref"].as_str().unwrap();
        let package = manifest.package(package_ref).expect("package exists");
        assert_eq!(
            package.release_label,
            expected_package["release_label"].as_str().unwrap()
        );
        assert_eq!(
            package.availability_state,
            expected_package["availability_state"].as_str().unwrap()
        );
        assert_eq!(
            package.degradation_state,
            expected_package["degradation_state"].as_str().unwrap()
        );
        assert_eq!(
            package.freshness_class,
            expected_package["freshness_class"].as_str().unwrap()
        );
        assert!(package.downgrade_allowed);
        assert!(!package.citation_refs.is_empty());

        if let Some(step_ref) = expected_package["active_step_ref"].as_str() {
            let step = manifest.step(step_ref).expect("active step exists");
            assert!(step.active);
            assert!(!step.exact_reopen_ref.is_empty());
        }
        if let Some(step_ref) = expected_package["inactive_step_ref"].as_str() {
            let step = manifest.step(step_ref).expect("inactive step exists");
            assert!(!step.active);
            assert!(!step.exact_reopen_ref.is_empty());
        }
    }
}

#[test]
fn profile_controls_and_progress_are_user_owned() {
    let manifest = seeded_learning_mode_beta_manifest();
    let fixture = load_yaml("progress_user_owned.yaml");
    let expected = &fixture["expected"];
    let profile = manifest
        .profile(expected["profile_ref"].as_str().expect("profile ref"))
        .expect("profile exists");
    let snapshot = manifest
        .progress_snapshot(expected["snapshot_ref"].as_str().expect("snapshot ref"))
        .expect("snapshot exists");

    assert_eq!(
        profile.data_ownership_class,
        expected["data_ownership_class"].as_str().unwrap()
    );
    assert_eq!(
        profile.optional_sync_posture,
        expected["optional_sync_posture"].as_str().unwrap()
    );
    assert!(!profile.authority_boundary_change_allowed);
    assert!(!profile.blocking_onboarding_allowed);

    let controls = profile
        .controls
        .iter()
        .map(|control| control.control_class.as_str())
        .collect::<BTreeSet<_>>();
    for required in ["enable", "pause", "snooze", "reset", "resume"] {
        assert!(controls.contains(required), "missing {required}");
    }

    assert_eq!(
        snapshot.export_posture.sync_requires_user_action,
        expected["sync_requires_user_action"].as_bool().unwrap()
    );
    assert_eq!(
        snapshot.export_posture.user_can_reset,
        expected["user_can_reset"].as_bool().unwrap()
    );
    assert_eq!(
        snapshot.export_posture.user_can_export_metadata,
        expected["user_can_export_metadata"].as_bool().unwrap()
    );
    assert_eq!(
        snapshot.export_posture.raw_step_body_exported,
        expected["raw_step_body_exported"].as_bool().unwrap()
    );
    assert_eq!(
        snapshot.export_posture.raw_pack_body_exported,
        expected["raw_pack_body_exported"].as_bool().unwrap()
    );
    assert_eq!(
        snapshot.support_projection.raw_profile_body_exported,
        expected["raw_profile_body_exported"].as_bool().unwrap()
    );

    assert!(snapshot.progress_entries.iter().all(|entry| {
        entry.repo_pack_read_default
            == expected["hidden_reads"]["repo_pack_read_default"]
                .as_bool()
                .unwrap()
            && entry.collaborator_read_default
                == expected["hidden_reads"]["collaborator_read_default"]
                    .as_bool()
                    .unwrap()
            && entry.telemetry_read_default
                == expected["hidden_reads"]["telemetry_read_default"]
                    .as_bool()
                    .unwrap()
            && entry.inspectable_by_user
    }));

    let states = snapshot
        .progress_entries
        .iter()
        .map(|entry| entry.progress_state_class.as_str())
        .collect::<BTreeSet<_>>();
    for state in expected["progress_states"].as_sequence().unwrap() {
        assert!(states.contains(state.as_str().unwrap()));
    }

    let hint = snapshot
        .hint_reveal_entries
        .iter()
        .find(|entry| entry.state_ref == expected["hint_reveal"]["state_ref"].as_str().unwrap())
        .expect("hint progress exists");
    assert_eq!(
        hint.persisted_across_restart,
        expected["hint_reveal"]["persisted_across_restart"]
            .as_bool()
            .unwrap()
    );
    assert_eq!(
        hint.rate_limit_reset_ref,
        expected["hint_reveal"]["rate_limit_reset_ref"]
            .as_str()
            .unwrap()
    );
}
