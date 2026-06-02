//! Protected tests for the hardened safe-mode runtime profile evaluator.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_support::harden_the_safe_mode_runtime_profile_retained_capabilities::{
    load_hardened_safe_mode_profile, AccessibilityDimensionClass, HardenedPreservedStateClass,
    HardenedSafeModeDispositionClass, HardenedSafeModeEvaluator, HardenedSafeModeHostClass,
    HardenedSafeModeProfile, HardenedSafeModeProfileClass, HardenedSafeModeReasonClass,
    HardenedSafeModeServiceClass, HardenedSafeModeSupportClass, RecoveryLadderRungClass,
    RetainedCapabilityClass, HARDENED_SAFE_MODE_DOC_REF, HARDENED_SAFE_MODE_PROFILE_RECORD_KIND,
    HARDENED_SAFE_MODE_SCHEMA_REF, HARDENED_SAFE_MODE_SUPPORT_PACKET_RECORD_KIND,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Manifest {
    profile_files: Vec<String>,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_dir() -> PathBuf {
    repo_root()
        .join("fixtures/support/m4/harden_the_safe_mode_runtime_profile_retained_capabilities")
}

fn load_manifest() -> Manifest {
    let path = fixture_dir().join("manifest.yaml");
    let yaml = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_yaml::from_str(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn load_profiles() -> Vec<HardenedSafeModeProfile> {
    load_manifest()
        .profile_files
        .into_iter()
        .map(|file| {
            let path = fixture_dir().join(file);
            let yaml =
                std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
            load_hardened_safe_mode_profile(&yaml)
                .unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
        })
        .collect()
}

#[test]
fn hardened_safe_mode_profiles_declare_disabled_or_narrowed_hosts_services_and_surfaces_with_reasons(
) {
    let evaluator = HardenedSafeModeEvaluator::new();
    let profiles = load_profiles();
    assert_eq!(profiles.len(), 3);

    let mut covered_profile_classes = BTreeSet::new();
    for profile in &profiles {
        evaluator
            .validate_profile(profile)
            .unwrap_or_else(|err| panic!("{} failed: {err:?}", profile.profile_id));

        assert_eq!(profile.record_kind, HARDENED_SAFE_MODE_PROFILE_RECORD_KIND);
        assert!(!profile.declared_hosts.is_empty());
        assert!(!profile.declared_services.is_empty());
        for host in &profile.declared_hosts {
            assert!(matches!(
                host.disposition_class,
                HardenedSafeModeDispositionClass::Disabled
                    | HardenedSafeModeDispositionClass::NarrowedToReadOnly
                    | HardenedSafeModeDispositionClass::NarrowedToLocalOnly
                    | HardenedSafeModeDispositionClass::NarrowedToReviewerView
            ));
            assert!(!host.narrowing_summary.trim().is_empty());
            let _: HardenedSafeModeReasonClass = host.reason_class;
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
            HardenedSafeModeProfileClass::PostCrashLoopProfile,
            HardenedSafeModeProfileClass::UserInvokedProfile,
            HardenedSafeModeProfileClass::PolicyForcedProfile,
        ]
        .into_iter()
        .collect::<BTreeSet<_>>()
    );
}

#[test]
fn hardened_safe_mode_profiles_preserve_user_authored_files_and_required_state() {
    let evaluator = HardenedSafeModeEvaluator::new();
    for profile in &load_profiles() {
        evaluator
            .validate_profile(profile)
            .expect("profile validates");
        assert!(
            profile
                .preserved_state_classes
                .contains(&HardenedPreservedStateClass::UserAuthoredFiles),
            "profile {} must preserve user-authored files",
            profile.profile_id
        );
        for required in [
            HardenedPreservedStateClass::OpenBufferSelection,
            HardenedPreservedStateClass::WorkspaceTrustStore,
            HardenedPreservedStateClass::CredentialStore,
            HardenedPreservedStateClass::SessionRestoreStore,
            HardenedPreservedStateClass::SupportExportStore,
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
fn hardened_safe_mode_profiles_admit_all_required_retained_capabilities() {
    let evaluator = HardenedSafeModeEvaluator::new();
    let profiles = load_profiles();

    for profile in &profiles {
        evaluator
            .validate_profile(profile)
            .expect("profile validates");

        let admitted: BTreeSet<RetainedCapabilityClass> = profile
            .retained_capabilities
            .iter()
            .map(|r| r.capability_class)
            .collect();

        for required in RetainedCapabilityClass::REQUIRED {
            assert!(
                admitted.contains(&required),
                "profile {} missing retained capability {:?}",
                profile.profile_id,
                required
            );
        }
        assert_eq!(
            admitted.len(),
            RetainedCapabilityClass::REQUIRED.len(),
            "profile {} has duplicate or extra retained capabilities",
            profile.profile_id
        );

        for record in &profile.retained_capabilities {
            assert!(
                !record.rationale.trim().is_empty(),
                "profile {} retained capability {:?} has empty rationale",
                profile.profile_id,
                record.capability_class
            );
            assert!(
                !record.support_guidance.trim().is_empty(),
                "profile {} retained capability {:?} has empty support_guidance",
                profile.profile_id,
                record.capability_class
            );
            assert!(
                record.explicitly_tested,
                "profile {} retained capability {:?} must be explicitly_tested",
                profile.profile_id, record.capability_class
            );
        }
    }
}

#[test]
fn hardened_safe_mode_profiles_cover_all_accessibility_dimensions_for_touched_surfaces_and_capabilities(
) {
    let evaluator = HardenedSafeModeEvaluator::new();
    let profiles = load_profiles();

    for profile in &profiles {
        evaluator
            .validate_profile(profile)
            .expect("profile validates");

        let mut touched_surface_ids: BTreeSet<&str> = BTreeSet::new();
        for surface in &profile.declared_surfaces {
            touched_surface_ids.insert(surface.surface_id.as_str());
        }
        let mut touched_capability_ids: BTreeSet<&str> = BTreeSet::new();
        for record in &profile.retained_capabilities {
            touched_capability_ids.insert(record.capability_class.as_str());
        }

        let mut seen_surface_dimensions: BTreeSet<(&str, AccessibilityDimensionClass)> =
            BTreeSet::new();
        let mut seen_capability_dimensions: BTreeSet<(&str, AccessibilityDimensionClass)> =
            BTreeSet::new();
        for posture in &profile.accessibility_postures {
            assert!(
                !posture.explanation.trim().is_empty(),
                "profile {} has empty accessibility explanation",
                profile.profile_id
            );
            match posture.target_kind.as_str() {
                "surface" => {
                    seen_surface_dimensions.insert((posture.target_id.as_str(), posture.dimension));
                }
                "capability" => {
                    seen_capability_dimensions
                        .insert((posture.target_id.as_str(), posture.dimension));
                }
                other => panic!("unexpected target_kind: {other}"),
            }
        }

        for surface_id in &touched_surface_ids {
            for dimension in AccessibilityDimensionClass::REQUIRED {
                assert!(
                    seen_surface_dimensions.contains(&(surface_id, dimension)),
                    "profile {} missing accessibility posture for surface {} dimension {}",
                    profile.profile_id,
                    surface_id,
                    dimension.as_str()
                );
            }
        }
        for capability_id in &touched_capability_ids {
            for dimension in AccessibilityDimensionClass::REQUIRED {
                assert!(
                    seen_capability_dimensions.contains(&(capability_id, dimension)),
                    "profile {} missing accessibility posture for capability {} dimension {}",
                    profile.profile_id,
                    capability_id,
                    dimension.as_str()
                );
            }
        }
    }
}

#[test]
fn hardened_safe_mode_profiles_bind_all_recovery_ladder_rungs() {
    let evaluator = HardenedSafeModeEvaluator::new();
    let profiles = load_profiles();

    for profile in &profiles {
        evaluator
            .validate_profile(profile)
            .expect("profile validates");

        let bound: BTreeSet<RecoveryLadderRungClass> = profile
            .recovery_ladder_bindings
            .iter()
            .map(|b| b.rung_class)
            .collect();

        for required in RecoveryLadderRungClass::REQUIRED {
            assert!(
                bound.contains(&required),
                "profile {} missing recovery-ladder rung {:?}",
                profile.profile_id,
                required
            );
        }
        assert_eq!(
            bound.len(),
            RecoveryLadderRungClass::REQUIRED.len(),
            "profile {} has duplicate or extra recovery-ladder rungs",
            profile.profile_id
        );

        for binding in &profile.recovery_ladder_bindings {
            assert!(
                !binding.rung_summary.trim().is_empty(),
                "profile {} rung {:?} has empty summary",
                profile.profile_id,
                binding.rung_class
            );
            assert!(
                !binding.evidence_refs.is_empty(),
                "profile {} rung {:?} missing evidence refs",
                profile.profile_id,
                binding.rung_class
            );
        }
    }
}

#[test]
fn hardened_safe_mode_support_packet_excludes_raw_private_content_and_ambient_authority() {
    let evaluator = HardenedSafeModeEvaluator::new();
    let profiles = load_profiles();

    for profile in &profiles {
        let packet = evaluator
            .support_packet(
                format!("support:hardened-safe-mode:{}", profile.profile_id),
                "2026-05-15T12:00:00Z",
                profile,
            )
            .unwrap_or_else(|err| panic!("{} packet failed: {err:?}", profile.profile_id));

        assert_eq!(
            packet.record_kind,
            HARDENED_SAFE_MODE_SUPPORT_PACKET_RECORD_KIND
        );
        assert_eq!(packet.doc_ref, HARDENED_SAFE_MODE_DOC_REF);
        assert_eq!(packet.schema_ref, HARDENED_SAFE_MODE_SCHEMA_REF);
        assert!(packet.raw_private_material_excluded);
        assert!(packet.ambient_authority_excluded);
        assert!(!packet.destructive_resets_present);
        assert!(packet.is_export_safe());
        assert!(packet.doctor_finding_ref.starts_with("doctor.finding."));
        assert!(packet
            .preserved_state_classes
            .contains(&HardenedPreservedStateClass::UserAuthoredFiles));
        assert_eq!(
            packet.retained_capability_rows.len(),
            RetainedCapabilityClass::REQUIRED.len()
        );
        assert_eq!(
            packet.recovery_ladder_binding_rows.len(),
            RecoveryLadderRungClass::REQUIRED.len()
        );
        assert!(!packet.accessibility_posture_rows.is_empty());
    }
}

#[test]
fn hardened_safe_mode_evaluator_refuses_destructive_or_incomplete_profiles() {
    let evaluator = HardenedSafeModeEvaluator::new();
    let mut profile = load_profiles()
        .into_iter()
        .find(|profile| profile.profile_class == HardenedSafeModeProfileClass::PostCrashLoopProfile)
        .expect("post-crash-loop profile exists");

    // Refuses a profile that declares a destructive reset.
    profile.destructive_resets_present = true;
    let report = evaluator
        .validate_profile(&profile)
        .expect_err("destructive profile must be rejected");
    assert!(report.violations.iter().any(|violation| {
        violation.check_id == "hardened_safe_mode.destructive_reset_declared"
    }));

    // Refuses a profile that drops user-authored-files preservation.
    let mut profile = load_profiles()
        .into_iter()
        .find(|profile| profile.profile_class == HardenedSafeModeProfileClass::PostCrashLoopProfile)
        .expect("post-crash-loop profile exists");
    profile
        .preserved_state_classes
        .retain(|class| *class != HardenedPreservedStateClass::UserAuthoredFiles);
    let report = evaluator
        .validate_profile(&profile)
        .expect_err("missing user-authored-files must be rejected");
    assert!(report.violations.iter().any(|violation| {
        violation.check_id == "hardened_safe_mode.user_authored_files_must_be_preserved"
    }));

    // Refuses a profile without a doctor finding ref.
    let mut profile = load_profiles()
        .into_iter()
        .find(|profile| profile.profile_class == HardenedSafeModeProfileClass::UserInvokedProfile)
        .expect("user-invoked profile exists");
    profile.doctor_finding_ref.clear();
    let report = evaluator
        .validate_profile(&profile)
        .expect_err("missing doctor finding must be rejected");
    assert!(report.violations.iter().any(|violation| {
        violation.check_id == "hardened_safe_mode.doctor_finding_ref_missing"
    }));

    // Refuses a profile with duplicate retained capabilities.
    let mut profile = load_profiles()
        .into_iter()
        .find(|profile| profile.profile_class == HardenedSafeModeProfileClass::UserInvokedProfile)
        .expect("user-invoked profile exists");
    if let Some(first) = profile.retained_capabilities.first().cloned() {
        profile.retained_capabilities.push(first);
    }
    let report = evaluator
        .validate_profile(&profile)
        .expect_err("duplicate retained capability must be rejected");
    assert!(report.violations.iter().any(|violation| {
        violation.check_id == "hardened_safe_mode.duplicate_retained_capability"
    }));

    // Refuses a profile with empty retained capability guidance.
    let mut profile = load_profiles()
        .into_iter()
        .find(|profile| profile.profile_class == HardenedSafeModeProfileClass::PolicyForcedProfile)
        .expect("policy-forced profile exists");
    if let Some(first) = profile.retained_capabilities.first_mut() {
        first.support_guidance.clear();
    }
    let report = evaluator
        .validate_profile(&profile)
        .expect_err("empty guidance must be rejected");
    assert!(report.violations.iter().any(|violation| {
        violation.check_id == "hardened_safe_mode.retained_capability_guidance_empty"
    }));

    // Refuses a profile missing a recovery-ladder rung.
    let mut profile = load_profiles()
        .into_iter()
        .find(|profile| profile.profile_class == HardenedSafeModeProfileClass::PolicyForcedProfile)
        .expect("policy-forced profile exists");
    profile.recovery_ladder_bindings.pop();
    let report = evaluator
        .validate_profile(&profile)
        .expect_err("missing recovery-ladder rung must be rejected");
    assert!(report.violations.iter().any(|violation| {
        violation.check_id == "hardened_safe_mode.required_recovery_ladder_rung_missing"
    }));

    // Refuses a profile missing accessibility postures.
    let mut profile = load_profiles()
        .into_iter()
        .find(|profile| profile.profile_class == HardenedSafeModeProfileClass::PostCrashLoopProfile)
        .expect("post-crash-loop profile exists");
    profile.accessibility_postures.clear();
    let report = evaluator
        .validate_profile(&profile)
        .expect_err("missing accessibility postures must be rejected");
    assert!(report.violations.iter().any(|violation| {
        violation.check_id == "hardened_safe_mode.accessibility_posture_missing_for_surface"
            || violation.check_id
                == "hardened_safe_mode.accessibility_posture_missing_for_capability"
    }));
}

#[test]
fn hardened_safe_mode_post_crash_loop_profile_has_expected_bindings() {
    let evaluator = HardenedSafeModeEvaluator::new();
    let profile = load_profiles()
        .into_iter()
        .find(|profile| profile.profile_class == HardenedSafeModeProfileClass::PostCrashLoopProfile)
        .expect("post-crash-loop profile exists");

    evaluator
        .validate_profile(&profile)
        .expect("profile validates");

    assert!(profile
        .declared_hosts
        .iter()
        .any(|h| h.host_class == HardenedSafeModeHostClass::ExtensionHost));
    assert!(profile
        .declared_services
        .iter()
        .any(|s| s.service_class == HardenedSafeModeServiceClass::SessionRestoreAutoReplay));

    let safe_mode_binding = profile
        .recovery_ladder_bindings
        .iter()
        .find(|b| b.rung_class == RecoveryLadderRungClass::SafeMode)
        .expect("safe_mode binding exists");
    assert_eq!(
        safe_mode_binding.support_class,
        HardenedSafeModeSupportClass::LaunchStable
    );

    let report_issue_binding = profile
        .recovery_ladder_bindings
        .iter()
        .find(|b| b.rung_class == RecoveryLadderRungClass::ReportIssue)
        .expect("report_issue binding exists");
    assert_eq!(
        report_issue_binding.support_class,
        HardenedSafeModeSupportClass::BetaGradeOnly
    );
}
