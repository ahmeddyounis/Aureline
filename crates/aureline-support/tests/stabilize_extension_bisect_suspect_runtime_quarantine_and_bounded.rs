//! Protected tests for the stabilized extension-bisect, suspect-runtime
//! quarantine, and bounded repair orchestration evaluator.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_support::stabilize_extension_bisect_suspect_runtime_quarantine_and_bounded::{
    load_stabilized_orchestration_profile, PreservedStateClass, RecoveryLadderRungClass,
    RetainedCapabilityClass, StabilizedOrchestrationEvaluator, StabilizedOrchestrationProfileClass,
    STABILIZED_ORCHESTRATION_DOC_REF, STABILIZED_ORCHESTRATION_PROFILE_RECORD_KIND,
    STABILIZED_ORCHESTRATION_SCHEMA_REF, STABILIZED_ORCHESTRATION_SUPPORT_PACKET_RECORD_KIND,
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
    repo_root().join(
        "fixtures/support/m4/stabilize-extension-bisect-suspect-runtime-quarantine-and-bounded",
    )
}

fn load_manifest() -> Manifest {
    let path = fixture_dir().join("manifest.yaml");
    let yaml = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_yaml::from_str(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn load_profiles() -> Vec<aureline_support::stabilize_extension_bisect_suspect_runtime_quarantine_and_bounded::StabilizedOrchestrationProfile>{
    load_manifest()
        .profile_files
        .into_iter()
        .map(|file| {
            let path = fixture_dir().join(file);
            let yaml =
                std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
            load_stabilized_orchestration_profile(&yaml)
                .unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
        })
        .collect()
}

#[test]
fn stabilized_orchestration_profiles_validate_successfully() {
    let evaluator = StabilizedOrchestrationEvaluator::new();
    let profiles = load_profiles();
    assert_eq!(profiles.len(), 3);

    let mut covered_profile_classes = BTreeSet::new();
    for profile in &profiles {
        evaluator
            .validate_profile(profile)
            .unwrap_or_else(|err| panic!("{} failed: {err:?}", profile.profile_id));

        assert_eq!(
            profile.record_kind,
            STABILIZED_ORCHESTRATION_PROFILE_RECORD_KIND
        );
        assert!(!profile.destructive_resets_present);
        assert!(profile.doctor_finding_ref.starts_with("doctor.finding."));
        covered_profile_classes.insert(profile.profile_class);
    }

    assert_eq!(
        covered_profile_classes,
        [
            StabilizedOrchestrationProfileClass::PostCrashLoopOrchestration,
            StabilizedOrchestrationProfileClass::UserInvokedOrchestration,
            StabilizedOrchestrationProfileClass::PolicyForcedOrchestration,
        ]
        .into_iter()
        .collect::<BTreeSet<_>>()
    );
}

#[test]
fn stabilized_orchestration_profiles_preserve_all_required_state() {
    let evaluator = StabilizedOrchestrationEvaluator::new();
    for profile in &load_profiles() {
        evaluator
            .validate_profile(profile)
            .expect("profile validates");
        let preserved: BTreeSet<_> = profile.preserved_state_classes.iter().copied().collect();
        for required in PreservedStateClass::REQUIRED {
            assert!(
                preserved.contains(&required),
                "profile {} missing preserved state class {:?}",
                profile.profile_id,
                required
            );
        }
    }
}

#[test]
fn stabilized_orchestration_profiles_admit_all_required_retained_capabilities() {
    let evaluator = StabilizedOrchestrationEvaluator::new();
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

        assert_eq!(
            admitted,
            RetainedCapabilityClass::REQUIRED
                .iter()
                .copied()
                .collect::<BTreeSet<_>>(),
            "profile {} missing retained capabilities",
            profile.profile_id
        );

        for record in &profile.retained_capabilities {
            assert!(
                !record.rationale.trim().is_empty(),
                "profile {} capability {:?} has empty rationale",
                profile.profile_id,
                record.capability_class
            );
            assert!(
                !record.support_guidance.trim().is_empty(),
                "profile {} capability {:?} has empty support_guidance",
                profile.profile_id,
                record.capability_class
            );
        }
    }
}

#[test]
fn stabilized_orchestration_profiles_cover_all_recovery_ladder_rungs() {
    let evaluator = StabilizedOrchestrationEvaluator::new();
    let profiles = load_profiles();

    for profile in &profiles {
        evaluator
            .validate_profile(profile)
            .expect("profile validates");

        let covered: BTreeSet<RecoveryLadderRungClass> = profile
            .recovery_ladder_bindings
            .iter()
            .map(|b| b.rung_class)
            .collect();

        assert_eq!(
            covered,
            RecoveryLadderRungClass::REQUIRED
                .iter()
                .copied()
                .collect::<BTreeSet<_>>(),
            "profile {} missing recovery-ladder rungs",
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
                "profile {} rung {:?} has empty evidence_refs",
                profile.profile_id,
                binding.rung_class
            );
        }
    }
}

#[test]
fn stabilized_orchestration_profiles_declare_non_empty_bindings() {
    let evaluator = StabilizedOrchestrationEvaluator::new();
    for profile in &load_profiles() {
        evaluator
            .validate_profile(profile)
            .expect("profile validates");

        let bisect = &profile.extension_bisect_binding;
        assert!(!bisect.session_ref.trim().is_empty());
        assert!(!bisect.finding_ref.trim().is_empty());
        assert!(!bisect.restore_ref.trim().is_empty());
        assert!(!bisect.support_packet_ref.trim().is_empty());

        let quarantine = &profile.suspect_runtime_quarantine_binding;
        assert!(!quarantine.quarantine_ref.trim().is_empty());
        assert!(!quarantine.lane_ref.trim().is_empty());
        assert!(!quarantine.owner_ref.trim().is_empty());
        assert!(!quarantine.clear_action_ref.trim().is_empty());
        assert!(!quarantine.reenable_action_ref.trim().is_empty());
        assert!(!quarantine.evidence_refs.is_empty());

        let repair = &profile.bounded_repair_binding;
        assert!(!repair.transaction_ref.trim().is_empty());
        assert!(!repair.preview_ref.trim().is_empty());
        assert!(!repair.outcome_ref.trim().is_empty());
    }
}

#[test]
fn stabilized_orchestration_profiles_have_accessibility_postures() {
    let evaluator = StabilizedOrchestrationEvaluator::new();
    for profile in &load_profiles() {
        evaluator
            .validate_profile(profile)
            .expect("profile validates");
        assert!(
            !profile.accessibility_postures.is_empty(),
            "profile {} must declare accessibility postures",
            profile.profile_id
        );
    }
}

#[test]
fn support_packets_are_export_safe() {
    let evaluator = StabilizedOrchestrationEvaluator::new();
    for profile in &load_profiles() {
        let packet = evaluator
            .support_packet(
                format!("support_packet:{}", profile.profile_id),
                profile.captured_at.clone(),
                profile,
            )
            .expect("support packet builds");

        assert_eq!(
            packet.record_kind,
            STABILIZED_ORCHESTRATION_SUPPORT_PACKET_RECORD_KIND
        );
        assert!(packet.is_export_safe());
        assert!(packet.raw_private_material_excluded);
        assert!(packet.ambient_authority_excluded);
        assert!(!packet.destructive_resets_present);
        assert_eq!(packet.schema_ref, STABILIZED_ORCHESTRATION_SCHEMA_REF);
        assert_eq!(packet.doc_ref, STABILIZED_ORCHESTRATION_DOC_REF);
    }
}
