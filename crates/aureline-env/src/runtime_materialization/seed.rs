//! The checked-in runtime-materialization corpus this lane freezes: one
//! aligned runtime instance per claimed target class, derived from the seeded
//! capsules, plus the degraded and mismatched variants that exercise the
//! engine's narrowing.

use crate::capsules::{
    seeded_environment_capsules, CapsuleTargetClass, EnvironmentCapsule, ServiceRole,
    TargetTransport,
};
use crate::m5_env_governance::MaterializationClass;

use super::{
    derive_runtime_instance, materialize_runtime, MountKind, MountState, NamespaceKind, PortState,
    ProjectionState, ReadinessState, RuntimeInstance, RuntimeMaterialization,
    RuntimeMaterializationFixture, RuntimeScenario, RUNTIME_MATERIALIZATION_FIXTURE_RECORD_KIND,
    RUNTIME_MATERIALIZATION_SCHEMA_VERSION,
};

fn capsule_for(target_class: CapsuleTargetClass) -> EnvironmentCapsule {
    let id = match target_class {
        CapsuleTargetClass::Local => "env.capsule.local",
        CapsuleTargetClass::Ssh => "env.capsule.ssh",
        CapsuleTargetClass::Container => "env.capsule.container",
        CapsuleTargetClass::Devcontainer => "env.capsule.devcontainer",
        CapsuleTargetClass::Vm => "env.capsule.vm",
        CapsuleTargetClass::ManagedWorkspace => "env.capsule.managed_workspace",
    };
    seeded_environment_capsules()
        .into_iter()
        .find(|capsule| capsule.identity.capsule_id == id)
        .expect("every target class has a seeded capsule")
}

/// The canonical runtime instances this lane freezes, one aligned instance
/// per claimed target class, each derived from its seeded capsule.
pub fn seeded_runtime_instances() -> Vec<RuntimeInstance> {
    CapsuleTargetClass::ALL
        .into_iter()
        .map(|target_class| derive_runtime_instance(&capsule_for(target_class)))
        .collect()
}

/// The aligned materializations, one per claimed target class.
pub fn seeded_runtime_materializations() -> Vec<RuntimeMaterialization> {
    CapsuleTargetClass::ALL
        .into_iter()
        .map(|target_class| {
            let capsule = capsule_for(target_class);
            let instance = derive_runtime_instance(&capsule);
            materialize_runtime(&capsule, &instance)
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn fixture(
    fixture_id: &str,
    target_class: CapsuleTargetClass,
    scenario: RuntimeScenario,
    capsule: EnvironmentCapsule,
    instance: RuntimeInstance,
    consumer_ref: &str,
    notes: &str,
) -> RuntimeMaterializationFixture {
    // Re-derive through the engine so the recorded expectations can never
    // drift from the materialization.
    let materialization = materialize_runtime(&capsule, &instance);
    RuntimeMaterializationFixture {
        record_kind: RUNTIME_MATERIALIZATION_FIXTURE_RECORD_KIND.to_owned(),
        schema_version: RUNTIME_MATERIALIZATION_SCHEMA_VERSION,
        fixture_id: fixture_id.to_owned(),
        target_class,
        scenario,
        capsule,
        instance,
        expected_parity: materialization.parity,
        expected_target_matched: materialization.target_matched,
        expected_headline_facet: materialization.headline_facet,
        expected_reason_tokens: materialization.reason_tokens,
        expected_degraded_facet_tokens: materialization.degraded_facet_tokens,
        expected_unready_service_tokens: materialization.unready_service_tokens,
        consumer_ref: consumer_ref.to_owned(),
        notes: notes.to_owned(),
    }
}

const SHELL_CONSUMER: &str = "crates/aureline-shell/src/environment_inspector/mod.rs";
const SUPPORT_CONSUMER: &str = "crates/aureline-support/src/bundle/mod.rs";
const RUNTIME_CONSUMER: &str = "crates/aureline-runtime/src/execution_context/mod.rs";
const REMOTE_CONSUMER: &str = "crates/aureline-remote/src/managed_workspace_lifecycle/mod.rs";

/// The checked-in fixture corpus: one aligned runtime instance per target
/// class plus the degraded and mismatched variants that drive the engine's
/// narrowing across multi-service stacks and wrong-target / wrong-runtime
/// mismatch cases.
pub fn seeded_runtime_materialization_fixtures() -> Vec<RuntimeMaterializationFixture> {
    let mut fixtures = Vec::new();

    // Aligned: every claimed target class materializes exactly as declared.
    for target_class in CapsuleTargetClass::ALL {
        let capsule = capsule_for(target_class);
        let instance = derive_runtime_instance(&capsule);
        let consumer = match target_class {
            CapsuleTargetClass::Local => SHELL_CONSUMER,
            CapsuleTargetClass::Ssh | CapsuleTargetClass::ManagedWorkspace => REMOTE_CONSUMER,
            CapsuleTargetClass::Container | CapsuleTargetClass::Devcontainer => RUNTIME_CONSUMER,
            CapsuleTargetClass::Vm => SUPPORT_CONSUMER,
        };
        fixtures.push(fixture(
            &format!("runtime_{}_aligned", target_class.as_str()),
            target_class,
            RuntimeScenario::Aligned,
            capsule,
            instance,
            consumer,
            "A runtime that materializes the declared contract is aligned on every facet.",
        ));
    }

    // Partial multi-service stack: the backing database service is not ready,
    // so the container runtime is degraded rather than presented as fully up.
    {
        let capsule = capsule_for(CapsuleTargetClass::Container);
        let mut instance = derive_runtime_instance(&capsule);
        instance.instance_id = "runtime.env.capsule.container.partial_service".to_owned();
        if let Some(service) = instance
            .readiness_graph
            .iter_mut()
            .find(|service| service.role == ServiceRole::Dependency)
        {
            service.readiness = ReadinessState::Unready;
            service.summary =
                "The backing database service is failing its readiness probe.".to_owned();
        }
        fixtures.push(fixture(
            "runtime_container_partial_service",
            CapsuleTargetClass::Container,
            RuntimeScenario::PartialServiceReadiness,
            capsule,
            instance,
            RUNTIME_CONSUMER,
            "An unready backing service degrades the container runtime and names the service, instead of a generic started label.",
        ));
    }

    // Degraded mount: the backing service volume did not materialize, so the
    // SSH runtime is degraded on the mount-set facet.
    {
        let capsule = capsule_for(CapsuleTargetClass::Ssh);
        let mut instance = derive_runtime_instance(&capsule);
        instance.instance_id = "runtime.env.capsule.ssh.degraded_mount".to_owned();
        if let Some(mount) = instance
            .mount_set
            .iter_mut()
            .find(|mount| mount.kind == MountKind::ServiceVolume)
        {
            mount.state = MountState::Missing;
            mount.summary = "The backing service data volume did not materialize.".to_owned();
        }
        fixtures.push(fixture(
            "runtime_ssh_degraded_mount",
            CapsuleTargetClass::Ssh,
            RuntimeScenario::DegradedMount,
            capsule,
            instance,
            REMOTE_CONSUMER,
            "A missing service volume degrades the SSH runtime and names the mount.",
        ));
    }

    // Unpublished port: the primary service's port did not publish, so the VM
    // runtime is degraded on the port-map facet.
    {
        let capsule = capsule_for(CapsuleTargetClass::Vm);
        let mut instance = derive_runtime_instance(&capsule);
        instance.instance_id = "runtime.env.capsule.vm.unpublished_port".to_owned();
        if let Some(port) = instance
            .port_map
            .iter_mut()
            .find(|port| port.service_id == "svc.app")
        {
            port.state = PortState::Unpublished;
            port.published_port = None;
            port.summary = "The primary service port did not publish to the host.".to_owned();
        }
        fixtures.push(fixture(
            "runtime_vm_unpublished_port",
            CapsuleTargetClass::Vm,
            RuntimeScenario::UnpublishedPort,
            capsule,
            instance,
            SUPPORT_CONSUMER,
            "An unpublished service port degrades the VM runtime and names the port.",
        ));
    }

    // Pending secret projection: a declared environment binding has not bound
    // its handle yet, so the devcontainer runtime is degraded on the
    // secret-projection facet (the value is never carried regardless).
    {
        let capsule = capsule_for(CapsuleTargetClass::Devcontainer);
        let mut instance = derive_runtime_instance(&capsule);
        instance.instance_id = "runtime.env.capsule.devcontainer.secret_pending".to_owned();
        if let Some(projection) = instance.secret_projections.first_mut() {
            projection.state = ProjectionState::Pending;
            projection.summary =
                "The declared environment binding is not yet bound to its handle.".to_owned();
        }
        fixtures.push(fixture(
            "runtime_devcontainer_secret_pending",
            CapsuleTargetClass::Devcontainer,
            RuntimeScenario::SecretProjectionPending,
            capsule,
            instance,
            RUNTIME_CONSUMER,
            "A pending secret projection degrades the devcontainer runtime without ever carrying the value.",
        ));
    }

    // Wrong target: a container capsule materialized on the local host. Both
    // the target-identity and process-namespace facets mismatch, so the
    // runtime is mismatched rather than relabeled as a generic start.
    {
        let capsule = capsule_for(CapsuleTargetClass::Container);
        let mut instance = derive_runtime_instance(&capsule);
        instance.instance_id = "runtime.env.capsule.container.wrong_target".to_owned();
        instance.target_class = CapsuleTargetClass::Local;
        instance.materialization_class = MaterializationClass::LocalNative;
        instance.transport = TargetTransport::LocalProcess;
        instance.process_namespace.kind = NamespaceKind::HostProcess;
        instance.process_namespace.summary =
            "Processes ran in a local host process, not the declared container namespace."
                .to_owned();
        instance.summary =
            "Runtime instance that ran on the local host while the capsule declared a container."
                .to_owned();
        fixtures.push(fixture(
            "runtime_container_wrong_target",
            CapsuleTargetClass::Container,
            RuntimeScenario::WrongTarget,
            capsule,
            instance,
            SUPPORT_CONSUMER,
            "A container capsule that ran on the local host is mismatched, surfacing where code actually ran.",
        ));
    }

    // Wrong namespace: a devcontainer capsule materialized on its declared
    // target but its processes ran in a host process instead of a container
    // namespace, so the process-namespace facet mismatches.
    {
        let capsule = capsule_for(CapsuleTargetClass::Devcontainer);
        let mut instance = derive_runtime_instance(&capsule);
        instance.instance_id = "runtime.env.capsule.devcontainer.wrong_namespace".to_owned();
        instance.process_namespace.kind = NamespaceKind::HostProcess;
        instance.process_namespace.summary =
            "Processes ran in a host process rather than the declared container namespace."
                .to_owned();
        fixtures.push(fixture(
            "runtime_devcontainer_wrong_namespace",
            CapsuleTargetClass::Devcontainer,
            RuntimeScenario::WrongNamespace,
            capsule,
            instance,
            RUNTIME_CONSUMER,
            "A devcontainer whose processes ran in a host namespace is mismatched on the namespace facet.",
        ));
    }

    fixtures
}
