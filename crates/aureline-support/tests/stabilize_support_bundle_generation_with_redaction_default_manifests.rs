//! Protected tests for stabilized support-bundle generation with
//! redaction-default manifests and chain-of-custody fields.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_support::stabilize_support_bundle_generation_with_redaction_default_manifests::{
    load_stabilized_support_bundle_manifest, ConsentEscalationClass, DestinationClass,
    DiagnosticDataClass, RecoveryLadderHookClass, StabilizedSupportBundleEvaluator,
    SupportBundleGenerationMode, STABILIZED_SUPPORT_BUNDLE_ARTIFACT_REF,
    STABILIZED_SUPPORT_BUNDLE_DOC_REF, STABILIZED_SUPPORT_BUNDLE_SCHEMA_REF,
    STABILIZED_SUPPORT_BUNDLE_SUPPORT_PACKET_RECORD_KIND,
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
        "fixtures/support/m4/stabilize-support-bundle-generation-with-redaction-default-manifests",
    )
}

fn load_manifest() -> Manifest {
    let path = fixture_dir().join("manifest.yaml");
    let yaml = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_yaml::from_str(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn load_profiles() -> Vec<aureline_support::stabilize_support_bundle_generation_with_redaction_default_manifests::StabilizedSupportBundleManifest> {
    load_manifest()
        .profile_files
        .into_iter()
        .map(|file| {
            let path = fixture_dir().join(file);
            let yaml =
                std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
            load_stabilized_support_bundle_manifest(&yaml)
                .unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
        })
        .collect()
}

#[test]
fn stabilized_manifests_validate_successfully() {
    let evaluator = StabilizedSupportBundleEvaluator::new();
    let profiles = load_profiles();
    assert_eq!(profiles.len(), 3);

    for profile in &profiles {
        evaluator
            .validate_manifest(profile)
            .unwrap_or_else(|err| panic!("{} failed: {err:?}", profile.manifest_id));

        assert_eq!(profile.schema_version, 1);
        assert!(!profile.manifest_id.trim().is_empty());
        assert!(!profile.build_identity.build_id.trim().is_empty());
        assert!(!profile.build_identity.exact_build_refs.is_empty());
    }
}

#[test]
fn stabilized_manifests_cover_required_generation_modes() {
    let evaluator = StabilizedSupportBundleEvaluator::new();
    let profiles = load_profiles();

    let mut covered_modes = BTreeSet::new();
    for profile in &profiles {
        evaluator.validate_manifest(profile).expect("profile validates");
        covered_modes.insert(profile.generation_mode);
    }

    assert_eq!(
        covered_modes,
        [
            SupportBundleGenerationMode::OrdinaryRedactionDefault,
            SupportBundleGenerationMode::HighFidelityIncidentCapture,
        ]
        .into_iter()
        .collect::<BTreeSet<_>>()
    );
}

#[test]
fn ordinary_redaction_default_has_no_consent_escalation() {
    let evaluator = StabilizedSupportBundleEvaluator::new();
    for profile in &load_profiles() {
        evaluator.validate_manifest(profile).expect("profile validates");
        if profile.is_ordinary_redaction_default() {
            assert_eq!(
                profile.consent_escalation_class,
                ConsentEscalationClass::NotRequired
            );
            assert!(profile.consent_escalation_ref.is_none());
            assert!(profile.incident_capture_scenario.is_none());
        }
    }
}

#[test]
fn high_fidelity_incident_capture_has_consent_and_scenario() {
    let evaluator = StabilizedSupportBundleEvaluator::new();
    for profile in &load_profiles() {
        evaluator.validate_manifest(profile).expect("profile validates");
        if profile.is_high_fidelity_incident_capture() {
            assert!(
                profile.incident_capture_scenario.is_some(),
                "{} missing incident_capture_scenario",
                profile.manifest_id
            );
            assert!(
                profile.consent_escalation_ref.is_some(),
                "{} missing consent_escalation_ref",
                profile.manifest_id
            );
            assert_ne!(
                profile.consent_escalation_class,
                ConsentEscalationClass::NotRequired,
                "{} must have consent escalation",
                profile.manifest_id
            );
        }
    }
}

#[test]
fn stabilized_manifests_cover_all_recovery_ladder_hooks() {
    let evaluator = StabilizedSupportBundleEvaluator::new();
    let profiles = load_profiles();

    for profile in &profiles {
        evaluator.validate_manifest(profile).expect("profile validates");

        let covered: BTreeSet<RecoveryLadderHookClass> = profile
            .recovery_ladder_hooks
            .iter()
            .map(|h| h.hook_class)
            .collect();

        assert_eq!(
            covered,
            RecoveryLadderHookClass::REQUIRED
                .iter()
                .copied()
                .collect::<BTreeSet<_>>(),
            "profile {} missing recovery-ladder hooks",
            profile.manifest_id
        );

        for hook in &profile.recovery_ladder_hooks {
            assert!(
                !hook.hook_ref.trim().is_empty(),
                "profile {} hook {:?} has empty hook_ref",
                profile.manifest_id,
                hook.hook_class
            );
            assert!(
                !hook.label.trim().is_empty(),
                "profile {} hook {:?} has empty label",
                profile.manifest_id,
                hook.hook_class
            );
            assert!(
                hook.preserves_user_state,
                "profile {} hook {:?} must preserve user state",
                profile.manifest_id,
                hook.hook_class
            );
        }
    }
}

#[test]
fn stabilized_manifests_have_non_empty_chain_of_custody() {
    let evaluator = StabilizedSupportBundleEvaluator::new();
    for profile in &load_profiles() {
        evaluator.validate_manifest(profile).expect("profile validates");
        assert!(
            !profile.chain_of_custody.is_empty(),
            "profile {} has empty chain_of_custody",
            profile.manifest_id
        );

        let mut last_sequence: Option<u32> = None;
        for entry in &profile.chain_of_custody {
            if let Some(prev) = last_sequence {
                assert!(
                    entry.sequence > prev,
                    "profile {} non-monotonic sequence at {}",
                    profile.manifest_id,
                    entry.sequence
                );
            }
            last_sequence = Some(entry.sequence);
            assert!(
                !entry.actor_ref.trim().is_empty(),
                "profile {} empty actor_ref at sequence {}",
                profile.manifest_id,
                entry.sequence
            );
            assert!(
                !entry.note.trim().is_empty(),
                "profile {} empty note at sequence {}",
                profile.manifest_id,
                entry.sequence
            );
        }
    }
}

#[test]
fn stabilized_manifests_exclude_high_risk_by_default() {
    let evaluator = StabilizedSupportBundleEvaluator::new();
    for profile in &load_profiles() {
        evaluator.validate_manifest(profile).expect("profile validates");

        let has_high_risk_included = profile.included_classes.iter().any(|e| {
            matches!(e.data_class, DiagnosticDataClass::HighRisk)
        });
        assert!(
            !has_high_risk_included,
            "profile {} must not include high-risk data classes",
            profile.manifest_id
        );
    }
}

#[test]
fn stabilized_manifests_support_offline_inspection_when_local_only() {
    let evaluator = StabilizedSupportBundleEvaluator::new();
    for profile in &load_profiles() {
        evaluator.validate_manifest(profile).expect("profile validates");
        if matches!(profile.destination_class, DestinationClass::LocalOnlyReview) {
            assert!(
                profile.supports_offline_inspection,
                "profile {} is local_only_review but does not support offline inspection",
                profile.manifest_id
            );
        }
    }
}

#[test]
fn support_packets_project_successfully() {
    let evaluator = StabilizedSupportBundleEvaluator::new();
    for profile in &load_profiles() {
        let packet = evaluator
            .project_support_packet(profile, format!("packet:{}", profile.manifest_id))
            .expect("project support packet");

        assert_eq!(packet.schema_version, 1);
        assert_eq!(
            packet.record_kind,
            STABILIZED_SUPPORT_BUNDLE_SUPPORT_PACKET_RECORD_KIND
        );
        assert_eq!(packet.manifest_ref, profile.manifest_id);
        assert_eq!(packet.generation_mode, profile.generation_mode);
        assert_eq!(packet.destination_class, profile.destination_class);
        assert_eq!(packet.included_class_count, profile.included_classes.len());
        assert_eq!(packet.excluded_class_count, profile.excluded_classes.len());
        assert_eq!(packet.custody_event_count, profile.chain_of_custody.len());
        assert_eq!(packet.schema_ref, STABILIZED_SUPPORT_BUNDLE_SCHEMA_REF);
        assert_eq!(packet.doc_ref, STABILIZED_SUPPORT_BUNDLE_DOC_REF);
        assert_eq!(packet.artifact_ref, STABILIZED_SUPPORT_BUNDLE_ARTIFACT_REF);
    }
}

#[test]
fn schema_ref_and_doc_ref_are_repo_relative() {
    assert!(
        STABILIZED_SUPPORT_BUNDLE_SCHEMA_REF.starts_with("schemas/"),
        "schema_ref must be repo-relative"
    );
    assert!(
        STABILIZED_SUPPORT_BUNDLE_DOC_REF.starts_with("docs/"),
        "doc_ref must be repo-relative"
    );
    assert!(
        STABILIZED_SUPPORT_BUNDLE_ARTIFACT_REF.starts_with("artifacts/"),
        "artifact_ref must be repo-relative"
    );
}
