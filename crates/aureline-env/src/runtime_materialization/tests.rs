use std::collections::BTreeSet;

use super::*;
use crate::capsules::{seeded_environment_capsules, CapsuleTargetClass, EnvironmentCapsule};

fn capsule(id: &str) -> EnvironmentCapsule {
    seeded_environment_capsules()
        .into_iter()
        .find(|capsule| capsule.identity.capsule_id == id)
        .unwrap_or_else(|| panic!("capsule {id} exists"))
}

#[test]
fn every_seeded_instance_validates() {
    for instance in seeded_runtime_instances() {
        validate_runtime_instance(&instance)
            .unwrap_or_else(|err| panic!("instance {} must validate: {err}", instance.instance_id));
    }
}

#[test]
fn every_seeded_fixture_validates() {
    for fixture in seeded_runtime_materialization_fixtures() {
        validate_runtime_materialization_fixture(&fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn aligned_instance_materializes_with_full_parity() {
    let capsule = capsule("env.capsule.local");
    let instance = derive_runtime_instance(&capsule);
    let materialization = materialize_runtime(&capsule, &instance);
    assert_eq!(materialization.parity, RuntimeParity::Aligned);
    assert!(materialization.target_matched);
    assert!(materialization.headline_facet.is_none());
    assert!(materialization.reason_tokens.is_empty());
    assert!(materialization.degraded_facet_tokens.is_empty());
    assert!(materialization.unready_service_tokens.is_empty());
    assert_eq!(materialization.facet_evaluations.len(), 6);
    assert_eq!(
        materialization.materialization_parity_state,
        EvidenceState::Current
    );
}

#[test]
fn derived_instance_keeps_distinct_target_identity_per_class() {
    // The guardrail: identity never collapses into one generic label. Every
    // claimed target class derives a distinct target class and namespace kind.
    let mut classes = BTreeSet::new();
    let mut namespaces = BTreeSet::new();
    for instance in seeded_runtime_instances() {
        classes.insert(instance.target_class);
        namespaces.insert(instance.process_namespace.kind);
    }
    for required in CapsuleTargetClass::ALL {
        assert!(
            classes.contains(&required),
            "instances must cover target class {}",
            required.as_str()
        );
    }
    // Local, SSH, VM, and managed each have their own namespace kind; the two
    // container-family classes share the container namespace.
    assert!(namespaces.contains(&NamespaceKind::HostProcess));
    assert!(namespaces.contains(&NamespaceKind::RemoteHostSession));
    assert!(namespaces.contains(&NamespaceKind::ContainerNamespace));
    assert!(namespaces.contains(&NamespaceKind::VmGuest));
    assert!(namespaces.contains(&NamespaceKind::ManagedPod));
}

#[test]
fn partial_service_degrades_and_names_the_service() {
    let capsule = capsule("env.capsule.container");
    let mut instance = derive_runtime_instance(&capsule);
    instance
        .readiness_graph
        .iter_mut()
        .find(|service| service.service_id == "svc.db")
        .expect("db service")
        .readiness = ReadinessState::Unready;
    let materialization = materialize_runtime(&capsule, &instance);
    assert_eq!(materialization.parity, RuntimeParity::Degraded);
    assert!(materialization.target_matched, "the target still matched");
    assert_eq!(
        materialization.headline_facet,
        Some(RuntimeFacet::ServiceReadiness)
    );
    assert_eq!(
        materialization.unready_service_tokens,
        vec!["svc.db".to_owned()]
    );
    assert!(materialization
        .reason_tokens
        .contains(&"service_svc.db_unready".to_owned()));
    assert_eq!(
        materialization.materialization_parity_state,
        EvidenceState::Partial
    );
}

#[test]
fn wrong_target_is_mismatched_not_relabeled() {
    // The marquee guardrail: a container capsule that ran on the local host is
    // mismatched, and where_code_ran says so explicitly.
    let capsule = capsule("env.capsule.container");
    let mut instance = derive_runtime_instance(&capsule);
    instance.target_class = CapsuleTargetClass::Local;
    instance.materialization_class = MaterializationClass::LocalNative;
    instance.transport = TargetTransport::LocalProcess;
    instance.process_namespace.kind = NamespaceKind::HostProcess;
    let materialization = materialize_runtime(&capsule, &instance);
    assert_eq!(materialization.parity, RuntimeParity::Mismatched);
    assert!(!materialization.target_matched);
    assert_eq!(
        materialization.headline_facet,
        Some(RuntimeFacet::TargetIdentity)
    );
    assert!(materialization
        .reason_tokens
        .contains(&"target_identity_mismatch".to_owned()));
    assert!(materialization
        .reason_tokens
        .contains(&"process_namespace_mismatch".to_owned()));
    assert!(
        materialization.where_code_ran.contains("wrong target"),
        "where_code_ran must call out the wrong target: {}",
        materialization.where_code_ran
    );
    assert_eq!(
        materialization.materialization_parity_state,
        EvidenceState::Stale
    );
}

#[test]
fn wrong_namespace_is_mismatched_on_namespace_facet_only() {
    let capsule = capsule("env.capsule.devcontainer");
    let mut instance = derive_runtime_instance(&capsule);
    instance.process_namespace.kind = NamespaceKind::HostProcess;
    let materialization = materialize_runtime(&capsule, &instance);
    assert_eq!(materialization.parity, RuntimeParity::Mismatched);
    assert!(!materialization.target_matched);
    assert_eq!(
        materialization.headline_facet,
        Some(RuntimeFacet::ProcessNamespace)
    );
    // Target identity stayed aligned; only the namespace facet degraded.
    assert_eq!(
        materialization.degraded_facet_tokens,
        vec!["process_namespace".to_owned()]
    );
}

#[test]
fn desktop_headless_ai_and_support_share_one_object() {
    // Acceptance: every surface reuses the same materialization object.
    let capsule = capsule("env.capsule.managed_workspace");
    let instance = derive_runtime_instance(&capsule);
    let desktop = desktop_runtime_materialization(&capsule, &instance);
    let headless = headless_runtime_materialization(&capsule, &instance);
    let ai = ai_runtime_materialization(&capsule, &instance);
    let support = support_runtime_materialization(&capsule, &instance);
    assert_eq!(desktop, headless, "desktop and headless must be identical");
    assert_eq!(desktop, ai, "desktop and AI must be identical");
    assert_eq!(
        support.materialization, desktop,
        "support export must wrap the same materialization object"
    );
}

#[test]
fn export_is_metadata_first_and_carries_no_values() {
    let capsule = capsule("env.capsule.local");
    let instance = derive_runtime_instance(&capsule);
    let materialization = materialize_runtime(&capsule, &instance);
    let export = export_runtime_materialization(&materialization);
    assert_eq!(export.redaction_class, RedactionClass::MetadataOnly);
    // Secret projections carry handles, never values.
    for projection in &materialization.instance.secret_projections {
        assert_eq!(
            projection.handle_ref.len(),
            64,
            "secret handle must be a 64-hex digest, never a value"
        );
    }
}

#[test]
fn diff_surfaces_a_target_and_readiness_change() {
    let local = derive_runtime_instance(&capsule("env.capsule.local"));
    let container = derive_runtime_instance(&capsule("env.capsule.container"));
    let diff = diff_runtime_instances(&local, &container);
    assert!(!diff.identical);
    let paths: Vec<&str> = diff.changes.iter().map(|c| c.path.as_str()).collect();
    assert!(paths.contains(&"target_class"));
    assert!(paths.contains(&"process_namespace.kind"));
}

#[test]
fn diff_of_identical_instances_is_empty() {
    let instance = derive_runtime_instance(&capsule("env.capsule.vm"));
    let diff = diff_runtime_instances(&instance, &instance);
    assert!(diff.identical);
    assert!(diff.changes.is_empty());
}

#[test]
fn instance_round_trips_through_json() {
    let instance = derive_runtime_instance(&capsule("env.capsule.container"));
    let json = serde_json::to_string(&instance).expect("instance serializes");
    let back: RuntimeInstance = serde_json::from_str(&json).expect("instance deserializes");
    assert_eq!(instance, back);
}

#[test]
fn fixtures_cover_aligned_degraded_and_mismatched_parities() {
    let fixtures = seeded_runtime_materialization_fixtures();
    let parities: BTreeSet<RuntimeParity> = fixtures.iter().map(|f| f.expected_parity).collect();
    for required in RuntimeParity::ALL {
        assert!(
            parities.contains(&required),
            "fixtures must cover {required:?}"
        );
    }
    // The wrong-target scenario proves a multi-service capsule that ran in the
    // wrong place is visible.
    assert!(
        fixtures
            .iter()
            .any(|f| f.scenario == RuntimeScenario::WrongTarget),
        "fixtures must cover a wrong-target case"
    );
    assert!(
        fixtures
            .iter()
            .any(|f| f.scenario == RuntimeScenario::PartialServiceReadiness),
        "fixtures must cover a partial multi-service stack"
    );
}
