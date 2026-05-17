//! Protected tests for the safe-mode runtime profile beta evaluator.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use aureline_support::safe_mode::{
    load_safe_mode_profile, load_safe_mode_transition, FullerModeClass, PreservedCapabilityClass,
    PreservedStateClass, SafeModeDispositionClass, SafeModeEntryReasonClass, SafeModeEvaluator,
    SafeModeExitReasonClass, SafeModeProfile, SafeModeProfileClass, SafeModeReasonClass,
    SafeModeTransition, TransitionClass, SAFE_MODE_PROFILE_DOC_REF,
    SAFE_MODE_PROFILE_RECORD_KIND, SAFE_MODE_PROFILE_SCHEMA_REF, SAFE_MODE_SUPPORT_PACKET_RECORD_KIND,
    SAFE_MODE_TRANSITION_RECORD_KIND,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Manifest {
    profile_files: Vec<String>,
    transition_files: Vec<String>,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_dir() -> PathBuf {
    repo_root().join("fixtures/recovery/m3/safe_mode")
}

fn load_manifest() -> Manifest {
    let path = fixture_dir().join("manifest.yaml");
    let yaml = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_yaml::from_str(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn load_profiles() -> Vec<SafeModeProfile> {
    load_manifest()
        .profile_files
        .into_iter()
        .map(|file| {
            let path = fixture_dir().join(file);
            let yaml =
                std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
            load_safe_mode_profile(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
        })
        .collect()
}

fn load_transitions() -> Vec<SafeModeTransition> {
    load_manifest()
        .transition_files
        .into_iter()
        .map(|file| {
            let path = fixture_dir().join(file);
            let yaml =
                std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
            load_safe_mode_transition(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
        })
        .collect()
}

#[test]
fn safe_mode_profiles_declare_disabled_or_narrowed_hosts_services_and_surfaces_with_reasons() {
    let evaluator = SafeModeEvaluator::new();
    let profiles = load_profiles();
    assert_eq!(profiles.len(), 3);

    let mut covered_profile_classes = BTreeSet::new();
    for profile in &profiles {
        evaluator
            .validate_profile(profile)
            .unwrap_or_else(|err| panic!("{} failed: {err:?}", profile.profile_id));

        assert_eq!(profile.record_kind, SAFE_MODE_PROFILE_RECORD_KIND);
        assert!(!profile.declared_hosts.is_empty());
        assert!(!profile.declared_services.is_empty());
        for host in &profile.declared_hosts {
            assert!(matches!(
                host.disposition_class,
                SafeModeDispositionClass::Disabled
                    | SafeModeDispositionClass::NarrowedToReadOnly
                    | SafeModeDispositionClass::NarrowedToLocalOnly
                    | SafeModeDispositionClass::NarrowedToReviewerView
            ));
            assert!(!host.narrowing_summary.trim().is_empty());
            let _: SafeModeReasonClass = host.reason_class;
        }
        for service in &profile.declared_services {
            assert!(!service.narrowing_summary.trim().is_empty());
        }
        for surface in &profile.declared_surfaces {
            assert!(!surface.narrowing_summary.trim().is_empty());
        }
        assert!(!profile.destructive_resets_present);
        assert!(profile.doctor_finding_ref.starts_with("doctor.finding."));
        covered_profile_classes.insert(profile.profile_class);
    }

    assert_eq!(
        covered_profile_classes,
        [
            SafeModeProfileClass::PostCrashLoopProfile,
            SafeModeProfileClass::UserInvokedProfile,
            SafeModeProfileClass::PolicyForcedProfile,
        ]
        .into_iter()
        .collect::<BTreeSet<_>>()
    );
}

#[test]
fn safe_mode_profiles_preserve_local_editing_basic_navigation_and_diagnostics_export() {
    let evaluator = SafeModeEvaluator::new();
    for profile in &load_profiles() {
        evaluator
            .validate_profile(profile)
            .expect("profile validates");
        for required in [
            PreservedCapabilityClass::LocalEditing,
            PreservedCapabilityClass::BasicNavigation,
            PreservedCapabilityClass::LocalDiagnosticsExport,
            PreservedCapabilityClass::SupportBundlePreview,
            PreservedCapabilityClass::ProjectDoctorSurfaces,
            PreservedCapabilityClass::SafeModeExitAction,
        ] {
            assert!(
                profile.preserved_capabilities.contains(&required),
                "profile {} missing preserved capability {:?}",
                profile.profile_id,
                required
            );
        }
        for required in [
            PreservedStateClass::UserAuthoredFiles,
            PreservedStateClass::OpenBufferSelection,
            PreservedStateClass::WorkspaceTrustStore,
            PreservedStateClass::CredentialStore,
            PreservedStateClass::SessionRestoreStore,
            PreservedStateClass::SupportExportStore,
        ] {
            assert!(
                profile.preserved_state_classes.contains(&required),
                "profile {} missing preserved state class {:?}",
                profile.profile_id,
                required
            );
        }
    }
}

#[test]
fn safe_mode_transitions_preserve_user_owned_state_and_avoid_destructive_resets() {
    let evaluator = SafeModeEvaluator::new();
    let transitions = load_transitions();
    assert!(!transitions.is_empty());

    let mut enter_count = 0_usize;
    let mut exit_count = 0_usize;
    for transition in &transitions {
        evaluator
            .validate_transition(transition)
            .unwrap_or_else(|err| panic!("{} failed: {err:?}", transition.transition_id));
        assert_eq!(transition.record_kind, SAFE_MODE_TRANSITION_RECORD_KIND);
        assert!(!transition.user_owned_state_deleted);
        assert!(!transition.durable_state_deleted);
        assert!(transition
            .preserved_state_classes_observed
            .contains(&PreservedStateClass::UserAuthoredFiles));
        match transition.transition_class {
            TransitionClass::Enter => {
                enter_count += 1;
                assert!(transition.entry_reason_class.is_some());
                assert!(transition.exit_reason_class.is_none());
            }
            TransitionClass::Exit => {
                exit_count += 1;
                assert!(transition.exit_reason_class.is_some());
                assert!(transition.entry_reason_class.is_none());
            }
        }
    }
    assert!(enter_count >= 1);
    assert!(exit_count >= 1);
}

#[test]
fn safe_mode_support_packet_excludes_raw_private_content_and_ambient_authority() {
    let evaluator = SafeModeEvaluator::new();
    let profiles = load_profiles();
    let transitions = load_transitions();

    // Group transitions by profile_ref so each packet covers one profile lifecycle.
    let mut by_profile: BTreeMap<String, Vec<SafeModeTransition>> = BTreeMap::new();
    for transition in &transitions {
        by_profile
            .entry(transition.profile_ref.clone())
            .or_default()
            .push(transition.clone());
    }

    for profile in &profiles {
        let bound = by_profile
            .get(&profile.profile_id)
            .cloned()
            .unwrap_or_default();
        let packet = evaluator
            .support_packet(
                format!("support:safe-mode-beta:{}", profile.profile_id),
                "2026-05-15T12:00:00Z",
                profile,
                &bound,
            )
            .unwrap_or_else(|err| panic!("{} packet failed: {err:?}", profile.profile_id));

        assert_eq!(packet.record_kind, SAFE_MODE_SUPPORT_PACKET_RECORD_KIND);
        assert_eq!(packet.doc_ref, SAFE_MODE_PROFILE_DOC_REF);
        assert_eq!(packet.schema_ref, SAFE_MODE_PROFILE_SCHEMA_REF);
        assert!(packet.raw_private_material_excluded);
        assert!(packet.ambient_authority_excluded);
        assert!(!packet.destructive_resets_present);
        assert!(packet.is_export_safe());
        assert!(packet.doctor_finding_ref.starts_with("doctor.finding."));
        assert!(packet
            .preserved_capabilities
            .contains(&PreservedCapabilityClass::LocalEditing));
        assert!(packet
            .preserved_state_classes
            .contains(&PreservedStateClass::UserAuthoredFiles));
        assert_eq!(packet.transition_rows.len(), bound.len());
    }
}

#[test]
fn safe_mode_post_crash_loop_profile_pairs_enter_and_exit_transitions() {
    let evaluator = SafeModeEvaluator::new();
    let profile = load_profiles()
        .into_iter()
        .find(|profile| profile.profile_class == SafeModeProfileClass::PostCrashLoopProfile)
        .expect("post-crash-loop profile exists");
    let bound = load_transitions()
        .into_iter()
        .filter(|transition| transition.profile_ref == profile.profile_id)
        .collect::<Vec<_>>();
    assert_eq!(bound.len(), 2);
    let enter = bound
        .iter()
        .find(|transition| transition.transition_class == TransitionClass::Enter)
        .expect("enter transition");
    let exit = bound
        .iter()
        .find(|transition| transition.transition_class == TransitionClass::Exit)
        .expect("exit transition");
    assert_eq!(
        enter.entry_reason_class,
        Some(SafeModeEntryReasonClass::CrashLoopDetected)
    );
    assert_eq!(
        exit.exit_reason_class,
        Some(SafeModeExitReasonClass::DoctorFindingReviewed)
    );

    evaluator
        .validate_transition_against_profile(&profile, enter)
        .expect("enter validates against profile");
    evaluator
        .validate_transition_against_profile(&profile, exit)
        .expect("exit validates against profile");
    assert_eq!(profile.return_path.fuller_mode_class, FullerModeClass::FullMode);
    assert!(profile.return_path.return_action.requires_review);
}

#[test]
fn safe_mode_evaluator_refuses_destructive_or_mismatched_transitions() {
    let evaluator = SafeModeEvaluator::new();
    let profile = load_profiles()
        .into_iter()
        .find(|profile| profile.profile_class == SafeModeProfileClass::PostCrashLoopProfile)
        .expect("post-crash-loop profile exists");
    let enter = load_transitions()
        .into_iter()
        .find(|transition| {
            transition.profile_ref == profile.profile_id
                && transition.transition_class == TransitionClass::Enter
        })
        .expect("enter transition exists");

    // Refuses a transition that deletes user-owned state.
    let mut destructive = enter.clone();
    destructive.user_owned_state_deleted = true;
    let report = evaluator
        .validate_transition(&destructive)
        .expect_err("destructive transitions must be rejected");
    assert!(report.violations.iter().any(|violation| {
        violation.check_id == "safe_mode.transition_deletes_user_owned_state"
    }));

    // Refuses a transition that drops the user-authored-files preservation.
    let mut without_user_files = enter.clone();
    without_user_files
        .preserved_state_classes_observed
        .retain(|class| *class != PreservedStateClass::UserAuthoredFiles);
    let report = evaluator
        .validate_transition(&without_user_files)
        .expect_err("missing user-authored-files preservation must be rejected");
    assert!(report.violations.iter().any(|violation| {
        violation.check_id == "safe_mode.transition_must_preserve_user_authored_files"
    }));

    // Refuses a transition whose profile_ref does not match the bound profile.
    let mut mismatched = enter.clone();
    mismatched.profile_ref = "safe_mode_profile:does_not_exist".to_owned();
    let report = evaluator
        .validate_transition_against_profile(&profile, &mismatched)
        .expect_err("mismatched profile_ref must be rejected");
    assert!(report.violations.iter().any(|violation| {
        violation.check_id == "safe_mode.transition_profile_ref_mismatch"
    }));

    // Refuses an enter transition that names an exit_reason_class.
    let mut crossed = enter.clone();
    crossed.exit_reason_class = Some(SafeModeExitReasonClass::UserConfirmedFullMode);
    let report = evaluator
        .validate_transition(&crossed)
        .expect_err("enter with exit_reason_class must be rejected");
    assert!(report.violations.iter().any(|violation| {
        violation.check_id == "safe_mode.transition_enter_has_exit_reason"
    }));
}

#[test]
fn safe_mode_evaluator_refuses_profile_without_doctor_finding_or_local_editing() {
    let evaluator = SafeModeEvaluator::new();
    let mut profile = load_profiles()
        .into_iter()
        .find(|profile| profile.profile_class == SafeModeProfileClass::UserInvokedProfile)
        .expect("user-invoked profile exists");

    profile.doctor_finding_ref.clear();
    profile
        .preserved_capabilities
        .retain(|class| *class != PreservedCapabilityClass::LocalEditing);
    profile.destructive_resets_present = true;
    let report = evaluator
        .validate_profile(&profile)
        .expect_err("profile must fail validation");
    let check_ids = report
        .violations
        .iter()
        .map(|violation| violation.check_id.as_str())
        .collect::<BTreeSet<_>>();
    assert!(check_ids.contains("safe_mode.doctor_finding_ref_missing"));
    assert!(check_ids.contains("safe_mode.local_editing_must_be_preserved"));
    assert!(check_ids.contains("safe_mode.destructive_reset_declared"));
}

