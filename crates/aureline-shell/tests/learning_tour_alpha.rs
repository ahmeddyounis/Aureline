use std::collections::BTreeSet;
use std::path::Path;

use aureline_commands::registry::seeded_registry;
use aureline_shell::learning_tour_alpha::{
    build_learning_tour_alpha_surface_projection, current_learning_tour_alpha_manifest,
    ActionSafetyClass, PackageDegradationClass, PackageInstallState, ProgressStateClass,
    ScopeWideningClass, SyncPostureClass,
};

fn repo_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crate is under crates/aureline-shell")
}

#[test]
fn manifest_loads_and_validates_against_command_registry() {
    let manifest = current_learning_tour_alpha_manifest().expect("manifest parses");
    manifest
        .validate_against_registry(seeded_registry())
        .expect("manifest validates");
    manifest
        .validate_fixture_refs(repo_root())
        .expect("protected fixture refs resolve");

    let install_states = manifest
        .packages
        .iter()
        .map(|package| package.install_state)
        .collect::<BTreeSet<_>>();
    assert!(install_states.contains(&PackageInstallState::LocalOnly));
    assert!(install_states.contains(&PackageInstallState::CachedCurrent));
    assert!(install_states.contains(&PackageInstallState::MirrorOnly));
    assert!(install_states.contains(&PackageInstallState::NotInstalled));

    assert!(!manifest.support_export.raw_pack_body_exported);
    assert!(manifest
        .support_export_rows()
        .iter()
        .all(|row| !row.raw_body_exported && !row.citation_refs.is_empty()));
}

#[test]
fn contextual_surface_preserves_explain_before_act_and_registry_command() {
    let projection =
        build_learning_tour_alpha_surface_projection().expect("projection builds from manifest");
    let surface = projection
        .contextual_teaching_surfaces
        .iter()
        .find(|surface| {
            surface.surface_id == "surface:learning_tour_alpha:contextual_tip.import-preview"
        })
        .expect("import-preview contextual surface");

    assert_eq!(surface.command_id, "cmd:workspace.import_profile");
    assert!(surface.explain_and_do_separate);
    assert!(surface.mutation_uses_command_registry);
    assert_eq!(
        surface.package_degradation_class,
        PackageDegradationClass::CachedDisclosed
    );
    assert!(surface.exact_reopen_ref.contains("learning-pack-rev"));

    let rail = projection
        .exercise_rails
        .iter()
        .find(|rail| rail.current_step_ref == surface.current_step_ref)
        .expect("rail for current step");
    assert!(rail.explanation_before_action_required);
    assert!(rail.reversible_or_sandboxed);
    assert_eq!(
        rail.reset_action.command_id.as_deref(),
        Some("cmd:editor.undo")
    );

    let manifest = current_learning_tour_alpha_manifest().expect("manifest parses");
    let step = manifest
        .step(&surface.step_ref)
        .expect("surface step resolves");
    assert_eq!(
        step.scope_widening_class,
        ScopeWideningClass::DisclosedReviewScopeWidening
    );
    assert_eq!(
        step.primary_action.action_safety_class,
        ActionSafetyClass::MutationRequiresApproval
    );
    assert_eq!(
        step.primary_action.command_id.as_deref(),
        Some("cmd:workspace.import_profile")
    );
    assert!(step.primary_action.preview_sheet_ref.is_some());
    assert!(step.primary_action.approval_path_ref.is_some());
    assert!(step.primary_action.evidence_packet_rule_ref.is_some());
}

#[test]
fn progress_snapshot_is_user_owned_and_portable_without_hidden_reads() {
    let manifest = current_learning_tour_alpha_manifest().expect("manifest parses");
    let snapshot = manifest
        .progress_snapshot("snapshot:learning.default_individual:2026.05.14")
        .expect("progress snapshot resolves");

    assert_eq!(
        snapshot.profile_state.data_ownership_class,
        "user_owned_portable_profile"
    );
    assert!(!snapshot.profile_state.trust_boundary_change_allowed);
    assert_eq!(
        snapshot.profile_state.mutation_guardrail_class.as_str(),
        "approval_required"
    );
    assert!(snapshot.profile_state.bookmarks_enabled);
    assert!(snapshot.profile_state.dismissals_enabled);

    let progress_states = snapshot
        .progress_entries
        .iter()
        .map(|entry| entry.progress_state_class)
        .collect::<BTreeSet<_>>();
    for required in [
        ProgressStateClass::Completed,
        ProgressStateClass::Dismissed,
        ProgressStateClass::Resumed,
        ProgressStateClass::Deferred,
    ] {
        assert!(progress_states.contains(&required));
    }

    let sync_postures = snapshot
        .progress_entries
        .iter()
        .map(|entry| entry.sync_posture_class)
        .collect::<BTreeSet<_>>();
    assert!(sync_postures.contains(&SyncPostureClass::LocalOnly));
    assert!(sync_postures.contains(&SyncPostureClass::PortableProfile));
    assert!(snapshot.progress_entries.iter().all(|entry| {
        entry.local_only_or_sync_posture_explicit
            && !entry.repo_pack_read_default
            && !entry.classroom_read_default
            && !entry.telemetry_read_default
            && entry.preserves_command_help_anchor
    }));
    assert!(!snapshot.export_posture.raw_step_body_exported);
    assert!(!snapshot.export_posture.raw_pack_body_exported);
    assert!(!snapshot.support_export_projection.raw_profile_body_exported);
}

#[test]
fn not_installed_package_degrades_to_inactive_placeholder() {
    let manifest = current_learning_tour_alpha_manifest().expect("manifest parses");
    let package = manifest
        .package("learning-pack:aureline.deep-dive.missing")
        .expect("not-installed package");
    assert_eq!(package.install_state, PackageInstallState::NotInstalled);
    assert_eq!(
        package.degradation_copy_class,
        PackageDegradationClass::NotInstalledPlaceholder
    );
    assert!(!package.citation_refs.is_empty());
    assert!(package.exact_reopen_ref.contains("learning-pack-rev"));

    let step = manifest
        .step("step:aureline.deep-dive.placeholder")
        .expect("placeholder step");
    assert!(!step.active);
    assert_eq!(
        step.degradation_copy_class,
        PackageDegradationClass::NotInstalledPlaceholder
    );
    assert_eq!(
        step.primary_action.action_safety_class,
        ActionSafetyClass::BlockedNotInstalled
    );
    assert!(step.exact_reopen_ref.contains("learning-pack-rev"));
}
