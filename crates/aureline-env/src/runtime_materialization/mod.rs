//! Runtime-instance materialization parity across local, SSH, container,
//! devcontainer, VM, and managed-workspace targets.
//!
//! The capsule lane materializes the typed environment *definition* — the
//! source digest, target plan, toolchain plan, trust hooks, service graph,
//! and prebuild fingerprint a template hydrates and a prebuild reuses. What
//! it deliberately stops short of is the runtime *instance*: the concrete
//! place code actually ran. This module closes that last gap so the place
//! code runs is explainable in the **same vocabulary** as the place the
//! environment said it would run.
//!
//! A [`RuntimeInstance`] is the explicit runtime-instance object derived
//! from an [`EnvironmentCapsule`]: the [`ProcessNamespace`] the runtime's
//! processes live in, the [`MountPoint`] set its working tree and service
//! volumes resolve to, the [`PortMapping`] map its services publish, the
//! [`ServiceReadiness`] graph of its services, and the [`SecretProjection`]
//! points its declared environment is projected through. It is metadata-first
//! by construction: namespace identities, mounts, ports, and secret
//! projections are carried as ids, digests, handles, and review-safe prose —
//! never raw pids, raw paths, provider payloads, or secret values.
//!
//! [`materialize_runtime`] is the single engine. It derives the declared
//! runtime contract from the capsule, folds in the observed instance, and
//! returns one [`RuntimeMaterialization`] carrying an explicit
//! [`RuntimeParity`] — `aligned`, `degraded`, or `mismatched` — the per-facet
//! [`FacetEvaluation`] behind it, the per-service [`ServiceReadinessEvaluation`]
//! that says which services were involved and which are not ready, and a
//! review-safe `where_code_ran` line. Desktop
//! ([`desktop_runtime_materialization`]), CLI / headless
//! ([`headless_runtime_materialization`]), AI ([`ai_runtime_materialization`]),
//! and support ([`support_runtime_materialization`]) all read that **same**
//! object, so a wrong-target run or a partial-service stack downgrades
//! visibly and identically on every surface instead of collapsing into a
//! generic "workspace started" label.
//!
//! Three guardrails are frozen here:
//!
//! - **Identity never collapses.** Local, SSH, container, devcontainer, VM,
//!   and managed-workspace runtimes keep distinct
//!   [`CapsuleTargetClass`] / [`MaterializationClass`] / [`TargetTransport`] /
//!   [`NamespaceKind`] identities. A runtime that materialized on the wrong
//!   target is [`RuntimeParity::Mismatched`], not silently relabeled.
//! - **Partial is partial.** A service that is not ready, a mount that is
//!   missing, an unpublished port, or a pending secret projection narrows the
//!   parity to [`RuntimeParity::Degraded`] and names the exact facet and
//!   element, rather than presenting the stack as fully up.
//! - **One engine, one object.** [`materialize_runtime`] is the single source
//!   of truth for the parity, shared by the fixtures and every surface, and it
//!   maps the parity back onto the governance materialization-parity
//!   [`EvidenceState`] so the runtime lane narrows in lockstep with the
//!   capsule's materialization-parity dimension.
//!
//! The packet is mirrored by:
//!
//! - [`/schemas/env/runtime-materialization.schema.json`](../../../../schemas/env/runtime-materialization.schema.json)
//! - [`/docs/env/runtime-materialization.md`](../../../../docs/env/runtime-materialization.md)
//! - [`/artifacts/env/runtime-materialization-proof.md`](../../../../artifacts/env/runtime-materialization-proof.md)
//! - [`/fixtures/env/runtime-materialization/`](../../../../fixtures/env/runtime-materialization/)

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::capsules::{
    CapsuleDigest, CapsuleTargetClass, EnvironmentCapsule, RedactionClass, ServiceRole,
    TargetTransport,
};
use crate::m5_env_governance::{
    EnvironmentProfile, EvidenceState, MaterializationClass, ValidationReport, ValidationViolation,
};

#[cfg(test)]
mod tests;

pub mod seed;

pub use seed::{
    seeded_runtime_instances, seeded_runtime_materialization_fixtures,
    seeded_runtime_materializations,
};

/// Schema version stamped onto instances, materializations, and fixtures.
pub const RUNTIME_MATERIALIZATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by a [`RuntimeInstance`].
pub const RUNTIME_INSTANCE_RECORD_KIND: &str = "runtime_instance_record";

/// Stable record-kind tag carried by a [`RuntimeMaterialization`].
pub const RUNTIME_MATERIALIZATION_RECORD_KIND: &str = "runtime_materialization_record";

/// Stable record-kind tag carried by a [`RuntimeExport`].
pub const RUNTIME_MATERIALIZATION_EXPORT_RECORD_KIND: &str =
    "runtime_materialization_export_record";

/// Stable record-kind tag carried by a [`RuntimeMaterializationFixture`].
pub const RUNTIME_MATERIALIZATION_FIXTURE_RECORD_KIND: &str =
    "runtime_materialization_fixture_record";

/// Repo-relative schema ref for the instance, materialization, and fixtures.
pub const RUNTIME_MATERIALIZATION_SCHEMA_REF: &str =
    "schemas/env/runtime-materialization.schema.json";

/// Repo-relative reviewer doc ref.
pub const RUNTIME_MATERIALIZATION_DOC_REF: &str = "docs/env/runtime-materialization.md";

/// Repo-relative human-readable proof report.
pub const RUNTIME_MATERIALIZATION_PROOF_REF: &str =
    "artifacts/env/runtime-materialization-proof.md";

/// Repo-relative fixture directory.
pub const RUNTIME_MATERIALIZATION_FIXTURE_DIR: &str = "fixtures/env/runtime-materialization";

/// Repo-relative fixture manifest.
pub const RUNTIME_MATERIALIZATION_FIXTURE_MANIFEST_REF: &str =
    "fixtures/env/runtime-materialization/manifest.yaml";

// ---------------------------------------------------------------------------
// Vocabulary.
// ---------------------------------------------------------------------------

/// The kind of process namespace a runtime instance's processes live in.
/// The namespace kind is part of runtime identity: a runtime that ran in a
/// host process where a container namespace was declared is mismatched, not
/// silently relabeled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NamespaceKind {
    /// A native process on the local host.
    HostProcess,
    /// A process inside a local container namespace.
    ContainerNamespace,
    /// A process in a session on a remote host.
    RemoteHostSession,
    /// A process inside a virtual-machine guest.
    VmGuest,
    /// A process inside a managed-workspace pod.
    ManagedPod,
}

impl NamespaceKind {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HostProcess => "host_process",
            Self::ContainerNamespace => "container_namespace",
            Self::RemoteHostSession => "remote_host_session",
            Self::VmGuest => "vm_guest",
            Self::ManagedPod => "managed_pod",
        }
    }
}

/// The kind of a runtime mount point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MountKind {
    /// The capsule's working tree.
    WorkingTree,
    /// A backing service's data volume.
    ServiceVolume,
    /// A shared toolchain / dependency cache volume.
    ToolCache,
}

impl MountKind {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkingTree => "working_tree",
            Self::ServiceVolume => "service_volume",
            Self::ToolCache => "tool_cache",
        }
    }
}

/// The observed state of a runtime mount point at materialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MountState {
    /// The mount is present and resolves to its declared target.
    Present,
    /// The declared mount did not materialize.
    Missing,
    /// The mount materialized but resolves to a different target.
    Divergent,
}

impl MountState {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Present => "present",
            Self::Missing => "missing",
            Self::Divergent => "divergent",
        }
    }

    /// The parity contribution this mount state makes. A present mount keeps
    /// the facet aligned; a missing or divergent mount degrades it.
    pub const fn parity_contribution(self) -> RuntimeParity {
        match self {
            Self::Present => RuntimeParity::Aligned,
            Self::Missing | Self::Divergent => RuntimeParity::Degraded,
        }
    }
}

/// The observed state of a runtime port mapping at materialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortState {
    /// The declared port is published to the host as planned.
    Published,
    /// The declared port did not get published.
    Unpublished,
    /// The declared port published, but onto a conflicting host port.
    Conflicted,
}

impl PortState {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Published => "published",
            Self::Unpublished => "unpublished",
            Self::Conflicted => "conflicted",
        }
    }

    /// The parity contribution this port state makes.
    pub const fn parity_contribution(self) -> RuntimeParity {
        match self {
            Self::Published => RuntimeParity::Aligned,
            Self::Unpublished | Self::Conflicted => RuntimeParity::Degraded,
        }
    }
}

/// The observed readiness of one materialized service.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessState {
    /// The service passed its readiness probe.
    Ready,
    /// The service is still starting and has not passed readiness.
    Starting,
    /// The service started but is failing its readiness probe.
    Unready,
    /// The declared service did not materialize at all.
    Absent,
}

impl ReadinessState {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Starting => "starting",
            Self::Unready => "unready",
            Self::Absent => "absent",
        }
    }

    /// True when the service is not fully ready.
    pub const fn is_degraded(self) -> bool {
        !matches!(self, Self::Ready)
    }

    /// The parity contribution this readiness state makes. Any state short of
    /// ready degrades the service-readiness facet; a partial stack is never
    /// presented as fully up.
    pub const fn parity_contribution(self) -> RuntimeParity {
        match self {
            Self::Ready => RuntimeParity::Aligned,
            Self::Starting | Self::Unready | Self::Absent => RuntimeParity::Degraded,
        }
    }
}

/// Where a capsule's declared environment is projected at runtime. The
/// projection is identified by a handle, never the secret value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretProjectionPoint {
    /// Projected as a process environment variable.
    EnvVar,
    /// Projected as a mounted file.
    MountedFile,
    /// Projected through a runtime secrets API.
    RuntimeApi,
}

impl SecretProjectionPoint {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EnvVar => "env_var",
            Self::MountedFile => "mounted_file",
            Self::RuntimeApi => "runtime_api",
        }
    }
}

/// The observed state of one secret projection at materialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectionState {
    /// The projection point is bound to its handle and available.
    Projected,
    /// The projection point is declared but not yet bound.
    Pending,
    /// The projection point did not materialize.
    Missing,
}

impl ProjectionState {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Projected => "projected",
            Self::Pending => "pending",
            Self::Missing => "missing",
        }
    }

    /// The parity contribution this projection state makes.
    pub const fn parity_contribution(self) -> RuntimeParity {
        match self {
            Self::Projected => RuntimeParity::Aligned,
            Self::Pending | Self::Missing => RuntimeParity::Degraded,
        }
    }
}

/// One facet the runtime instance is checked against the capsule contract on.
/// The six facets are the runtime-truth surface: where code ran, the
/// namespace it ran in, its mounts, its ports, its service readiness, and its
/// secret projections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeFacet {
    /// The materialized target class, materialization class, and transport
    /// match the capsule's declared target.
    TargetIdentity,
    /// The process namespace matches the namespace the declared target
    /// implies.
    ProcessNamespace,
    /// The declared mounts (working tree, service volumes, tool cache)
    /// materialized.
    MountSet,
    /// The declared service ports published as planned.
    PortMap,
    /// The declared services materialized and are ready.
    ServiceReadiness,
    /// The declared environment projected through its secret points.
    SecretProjection,
}

impl RuntimeFacet {
    /// Every facet in canonical order. The order is the precedence order for
    /// the headline facet: target identity and process namespace lead, so a
    /// wrong-target run is headlined as a target mismatch before a partial
    /// service.
    pub const ALL: [Self; 6] = [
        Self::TargetIdentity,
        Self::ProcessNamespace,
        Self::MountSet,
        Self::PortMap,
        Self::ServiceReadiness,
        Self::SecretProjection,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetIdentity => "target_identity",
            Self::ProcessNamespace => "process_namespace",
            Self::MountSet => "mount_set",
            Self::PortMap => "port_map",
            Self::ServiceReadiness => "service_readiness",
            Self::SecretProjection => "secret_projection",
        }
    }
}

/// The explicit parity verdict between a runtime instance and the capsule it
/// claims to materialize. Declaration order is the narrowing order:
/// [`RuntimeParity::Aligned`] is the strongest claim and
/// [`RuntimeParity::Mismatched`] the most conservative, so narrowing always
/// moves toward a later variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeParity {
    /// The runtime materialized exactly the declared contract: the right
    /// target, namespace, mounts, ports, services, and secret projections.
    Aligned,
    /// The runtime materialized the right target, but part of the stack is
    /// not fully up (a service not ready, a missing mount, an unpublished
    /// port, or a pending secret projection).
    Degraded,
    /// The runtime materialized a different target or namespace than the
    /// capsule declared: code ran somewhere other than where the environment
    /// said it would.
    Mismatched,
}

impl RuntimeParity {
    /// Every parity in canonical (narrowing) order.
    pub const ALL: [Self; 3] = [Self::Aligned, Self::Degraded, Self::Mismatched];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Aligned => "aligned",
            Self::Degraded => "degraded",
            Self::Mismatched => "mismatched",
        }
    }

    /// Narrowing severity. Higher is a more conservative parity; the engine
    /// always takes the highest severity among the per-facet contributions.
    pub const fn severity(self) -> u8 {
        match self {
            Self::Aligned => 0,
            Self::Degraded => 1,
            Self::Mismatched => 2,
        }
    }

    /// The governance materialization-parity [`EvidenceState`] this parity
    /// maps to, so the runtime lane narrows the capsule's
    /// materialization-parity dimension in lockstep instead of forking a
    /// parallel model.
    pub const fn materialization_parity_state(self) -> EvidenceState {
        match self {
            Self::Aligned => EvidenceState::Current,
            Self::Degraded => EvidenceState::Partial,
            Self::Mismatched => EvidenceState::Stale,
        }
    }
}

// ---------------------------------------------------------------------------
// Runtime-instance fields.
// ---------------------------------------------------------------------------

/// The process namespace a runtime instance's processes live in. All fields
/// are metadata: a namespace kind, a namespace reference, and the host
/// boundary it sits behind — never raw pids or process tables.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessNamespace {
    /// Kind of namespace the processes run in.
    pub kind: NamespaceKind,
    /// Stable reference to the namespace (metadata, never a raw pid).
    pub namespace_ref: String,
    /// Host-boundary contract the namespace sits behind (metadata ref).
    pub boundary_ref: String,
    /// Review-safe summary of the namespace and its isolation.
    pub summary: String,
}

/// One runtime mount point: the declared target a working tree, service
/// volume, or cache resolved to, and whether it materialized.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MountPoint {
    /// Stable mount id.
    pub mount_id: String,
    /// Kind of mount.
    pub kind: MountKind,
    /// Reference to the mount target (metadata, never a raw host path).
    pub target_ref: String,
    /// Observed state of the mount.
    pub state: MountState,
    /// Review-safe summary of the mount.
    pub summary: String,
}

/// One runtime port mapping: a declared service port and the host port it
/// published to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortMapping {
    /// Service the port belongs to.
    pub service_id: String,
    /// Declared port inside the capsule.
    pub declared_port: u16,
    /// Host port the declared port published to, if any.
    pub published_port: Option<u16>,
    /// Observed state of the mapping.
    pub state: PortState,
    /// Review-safe summary of the mapping.
    pub summary: String,
}

/// One node in the runtime readiness graph: a declared service and whether it
/// materialized and is ready.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceReadiness {
    /// Stable service id (matches the capsule's service graph).
    pub service_id: String,
    /// Role the service plays.
    pub role: ServiceRole,
    /// Observed readiness of the service.
    pub readiness: ReadinessState,
    /// Other services this service depends on.
    pub depends_on: Vec<String>,
    /// Health-probe reference for the service (metadata ref).
    pub health_probe_ref: String,
    /// Review-safe summary of the service's runtime state.
    pub summary: String,
}

/// One secret-projection point: where a capsule's declared environment is
/// projected at runtime, identified by a handle and never the value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretProjection {
    /// Stable projection id.
    pub projection_id: String,
    /// Name of the declared environment binding being projected.
    pub env_name: String,
    /// Where the value is projected.
    pub point: SecretProjectionPoint,
    /// Opaque handle the projection resolves through (never the value).
    pub handle_ref: String,
    /// Observed state of the projection.
    pub state: ProjectionState,
    /// Review-safe summary of the projection.
    pub summary: String,
}

/// The explicit runtime-instance object: the concrete place code ran,
/// derived from a capsule and carried in the same vocabulary as the capsule.
/// It is metadata-first — namespaces, mounts, ports, readiness, and secret
/// projections are ids, digests, handles, and prose, never raw pids, paths,
/// payloads, or secret values.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeInstance {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable runtime-instance id.
    pub instance_id: String,
    /// Capsule this instance materializes.
    pub capsule_id: String,
    /// Capsule version this instance materializes.
    pub capsule_version: u32,
    /// Capsule digest this instance materializes.
    pub capsule_digest: CapsuleDigest,
    /// Claimed environment profile.
    pub profile: EnvironmentProfile,
    /// Observed target class the runtime materialized on.
    pub target_class: CapsuleTargetClass,
    /// Observed materialization class.
    pub materialization_class: MaterializationClass,
    /// Observed transport reaching the runtime.
    pub transport: TargetTransport,
    /// The process namespace the runtime's processes live in.
    pub process_namespace: ProcessNamespace,
    /// The runtime mount set (working tree, service volumes, tool cache).
    pub mount_set: Vec<MountPoint>,
    /// The runtime port map.
    pub port_map: Vec<PortMapping>,
    /// The runtime readiness graph, one node per declared service.
    pub readiness_graph: Vec<ServiceReadiness>,
    /// The runtime secret-projection points.
    pub secret_projections: Vec<SecretProjection>,
    /// Redaction posture (always metadata-only).
    pub redaction_class: RedactionClass,
    /// Review-safe summary of the runtime instance.
    pub summary: String,
}

// ---------------------------------------------------------------------------
// Capsule → runtime derivation.
// ---------------------------------------------------------------------------

/// Maps a capsule's materialization class and transport onto the claimed
/// target class. The six claimed M5 target classes are distinguished by the
/// pair, so the mapping is total and unambiguous over the seeded corpus.
pub const fn target_class_for(
    materialization_class: MaterializationClass,
    transport: TargetTransport,
) -> CapsuleTargetClass {
    match (materialization_class, transport) {
        (MaterializationClass::LocalNative, _) => CapsuleTargetClass::Local,
        (MaterializationClass::RemoteHost, TargetTransport::VirtualMachine) => {
            CapsuleTargetClass::Vm
        }
        (MaterializationClass::RemoteHost, _) => CapsuleTargetClass::Ssh,
        (MaterializationClass::Devcontainer, _) => CapsuleTargetClass::Devcontainer,
        (MaterializationClass::Container, _) => CapsuleTargetClass::Container,
        (MaterializationClass::ManagedCloud, _) => CapsuleTargetClass::ManagedWorkspace,
    }
}

/// The process namespace kind a claimed target class implies. A runtime whose
/// observed namespace kind differs from this is a process-namespace mismatch.
pub const fn namespace_kind_for(target_class: CapsuleTargetClass) -> NamespaceKind {
    match target_class {
        CapsuleTargetClass::Local => NamespaceKind::HostProcess,
        CapsuleTargetClass::Ssh => NamespaceKind::RemoteHostSession,
        CapsuleTargetClass::Container | CapsuleTargetClass::Devcontainer => {
            NamespaceKind::ContainerNamespace
        }
        CapsuleTargetClass::Vm => NamespaceKind::VmGuest,
        CapsuleTargetClass::ManagedWorkspace => NamespaceKind::ManagedPod,
    }
}

/// Derives the fully-aligned runtime instance a capsule declares: the runtime
/// that materialized exactly the capsule's target, namespace, mounts, ports,
/// services, and secret projections. Degraded and mismatched instances are
/// produced by mutating this baseline, so the runtime vocabulary stays a
/// projection of the capsule's own declared contract.
pub fn derive_runtime_instance(capsule: &EnvironmentCapsule) -> RuntimeInstance {
    let capsule_id = capsule.identity.capsule_id.as_str();
    let materialization_class = capsule.identity.materialization_class;
    let transport = capsule.identity.transport;
    let target_class = target_class_for(materialization_class, transport);
    let instance_id = format!("runtime.{capsule_id}");

    let process_namespace = ProcessNamespace {
        kind: namespace_kind_for(target_class),
        namespace_ref: format!("runtime/namespace/{capsule_id}"),
        boundary_ref: capsule.target_plan.host_boundary_ref.clone(),
        summary: format!(
            "Processes run in a {} namespace behind the declared host boundary.",
            namespace_kind_for(target_class).as_str()
        ),
    };

    let mut mount_set = vec![MountPoint {
        mount_id: "mount.working_tree".to_owned(),
        kind: MountKind::WorkingTree,
        target_ref: format!("runtime/mount/{capsule_id}/working_tree"),
        state: MountState::Present,
        summary: format!(
            "Working tree rooted as a {}.",
            capsule.target_plan.working_root_kind.as_str()
        ),
    }];
    for service in &capsule.service_graph.services {
        if service.role == ServiceRole::Dependency {
            mount_set.push(MountPoint {
                mount_id: format!("mount.{}", service.service_id),
                kind: MountKind::ServiceVolume,
                target_ref: format!("runtime/mount/{capsule_id}/{}", service.service_id),
                state: MountState::Present,
                summary: format!("Data volume for the {} service.", service.service_id),
            });
        }
    }
    mount_set.push(MountPoint {
        mount_id: "mount.tool_cache".to_owned(),
        kind: MountKind::ToolCache,
        target_ref: format!("runtime/mount/{capsule_id}/tool_cache"),
        state: MountState::Present,
        summary: "Shared toolchain and dependency cache volume.".to_owned(),
    });

    let mut port_map = Vec::new();
    for service in &capsule.service_graph.services {
        for port in &service.exposed_ports {
            port_map.push(PortMapping {
                service_id: service.service_id.clone(),
                declared_port: *port,
                published_port: Some(*port),
                state: PortState::Published,
                summary: format!(
                    "Port {port} of the {} service published as declared.",
                    service.service_id
                ),
            });
        }
    }

    let readiness_graph = capsule
        .service_graph
        .services
        .iter()
        .map(|service| ServiceReadiness {
            service_id: service.service_id.clone(),
            role: service.role,
            readiness: ReadinessState::Ready,
            depends_on: service.depends_on.clone(),
            health_probe_ref: format!(
                "observability/env/{capsule_id}/health/{}",
                service.service_id
            ),
            summary: format!(
                "The {} service materialized and is ready.",
                service.service_id
            ),
        })
        .collect();

    let secret_projections = capsule
        .declared_env
        .iter()
        .map(|binding| SecretProjection {
            projection_id: format!("secret.{}", binding.name),
            env_name: binding.name.clone(),
            point: SecretProjectionPoint::EnvVar,
            handle_ref: binding.value_digest.value.clone(),
            state: ProjectionState::Projected,
            summary: format!(
                "Environment binding {} projected through an env-var handle (value never carried).",
                binding.name
            ),
        })
        .collect();

    RuntimeInstance {
        record_kind: RUNTIME_INSTANCE_RECORD_KIND.to_owned(),
        schema_version: RUNTIME_MATERIALIZATION_SCHEMA_VERSION,
        instance_id,
        capsule_id: capsule_id.to_owned(),
        capsule_version: capsule.identity.capsule_version,
        capsule_digest: capsule.identity.capsule_digest.clone(),
        profile: capsule.identity.profile,
        target_class,
        materialization_class,
        transport,
        process_namespace,
        mount_set,
        port_map,
        readiness_graph,
        secret_projections,
        redaction_class: RedactionClass::MetadataOnly,
        summary: format!(
            "Runtime instance materializing capsule {capsule_id} on the {} target.",
            target_class.as_str()
        ),
    }
}

// ---------------------------------------------------------------------------
// The materialization decision and the engine that produces it.
// ---------------------------------------------------------------------------

/// One per-facet evaluation behind a [`RuntimeMaterialization`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FacetEvaluation {
    /// Facet being evaluated.
    pub facet: RuntimeFacet,
    /// Parity this facet contributes.
    pub parity_contribution: RuntimeParity,
    /// Review-safe declared value (what the capsule said).
    pub declared: String,
    /// Review-safe observed value (what the runtime did).
    pub observed: String,
    /// Stable tokens naming each element that degraded or mismatched the
    /// facet (empty when aligned).
    pub element_tokens: Vec<String>,
    /// Review-safe explanation of the facet's contribution.
    pub summary: String,
}

/// One per-service evaluation behind a [`RuntimeMaterialization`], so support
/// and AI surfaces can see which services were involved and which are not
/// ready.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceReadinessEvaluation {
    /// Service id.
    pub service_id: String,
    /// Role the service plays.
    pub role: ServiceRole,
    /// Observed readiness.
    pub readiness: ReadinessState,
    /// Parity this service contributes.
    pub parity_contribution: RuntimeParity,
    /// Review-safe explanation.
    pub summary: String,
}

/// The decision the engine reaches for one runtime instance against the
/// capsule it claims to materialize. This is the single explainability object
/// the desktop, headless, AI, and support surfaces all read; it carries no
/// secrets, raw paths, or provider payloads — only ids, digests, handles,
/// tokens, and review-safe prose.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeMaterialization {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Runtime-instance id under evaluation.
    pub instance_id: String,
    /// Capsule id the instance materializes.
    pub capsule_id: String,
    /// Capsule digest the instance materializes.
    pub capsule_digest: CapsuleDigest,
    /// Claimed environment profile.
    pub profile: EnvironmentProfile,
    /// Target class the capsule declared.
    pub declared_target_class: CapsuleTargetClass,
    /// Target class the runtime materialized on.
    pub observed_target_class: CapsuleTargetClass,
    /// Materialization class the capsule declared.
    pub declared_materialization_class: MaterializationClass,
    /// Materialization class the runtime materialized on.
    pub observed_materialization_class: MaterializationClass,
    /// Transport the capsule declared.
    pub declared_transport: TargetTransport,
    /// Transport the runtime materialized on.
    pub observed_transport: TargetTransport,
    /// The explicit parity verdict.
    pub parity: RuntimeParity,
    /// True when the runtime materialized on the declared target and
    /// namespace.
    pub target_matched: bool,
    /// The governance materialization-parity evidence state this parity maps
    /// to.
    pub materialization_parity_state: EvidenceState,
    /// The facet that headlines the parity (none when aligned).
    pub headline_facet: Option<RuntimeFacet>,
    /// Stable tokens naming every element that degraded or mismatched the
    /// runtime.
    pub reason_tokens: Vec<String>,
    /// Stable tokens naming every facet that is degraded or mismatched.
    pub degraded_facet_tokens: Vec<String>,
    /// Stable tokens naming every declared service involved in the runtime.
    pub involved_service_tokens: Vec<String>,
    /// Stable tokens naming every service that is not ready.
    pub unready_service_tokens: Vec<String>,
    /// Per-facet evaluation behind the parity.
    pub facet_evaluations: Vec<FacetEvaluation>,
    /// Per-service evaluation behind the readiness facet.
    pub service_evaluations: Vec<ServiceReadinessEvaluation>,
    /// The explicit runtime-instance object — where code ran, its namespace,
    /// mounts, ports, readiness, and secret projections.
    pub instance: RuntimeInstance,
    /// Review-safe statement of where code actually ran and how it relates to
    /// the declared target.
    pub where_code_ran: String,
    /// Review-safe headline explaining the parity.
    pub headline: String,
    /// Redaction posture (always metadata-only).
    pub redaction_class: RedactionClass,
}

/// Materializes the runtime parity of an instance against the capsule it
/// claims to materialize.
///
/// This is the canonical engine the fixtures and every surface share. The
/// parity starts aligned and is narrowed to the coldest facet contribution
/// across the six facets; the headline facet is the first facet (in canonical
/// order) whose contribution equals the winning parity, so a wrong-target run
/// is headlined as a target mismatch before a partial service. The decision
/// is metadata-first and self-explaining, so a wrong-target or partial-service
/// runtime can never collapse into a generic "workspace started" label.
pub fn materialize_runtime(
    capsule: &EnvironmentCapsule,
    instance: &RuntimeInstance,
) -> RuntimeMaterialization {
    let declared_materialization_class = capsule.identity.materialization_class;
    let declared_transport = capsule.identity.transport;
    let declared_target_class =
        target_class_for(declared_materialization_class, declared_transport);
    let expected_namespace = namespace_kind_for(declared_target_class);

    let mut facet_evaluations = Vec::new();
    let mut reason_tokens: Vec<String> = Vec::new();

    // 1. Target identity.
    let target_aligned = instance.target_class == declared_target_class
        && instance.materialization_class == declared_materialization_class
        && instance.transport == declared_transport;
    let target_eval = {
        let mut tokens = Vec::new();
        if !target_aligned {
            tokens.push("target_identity_mismatch".to_owned());
        }
        FacetEvaluation {
            facet: RuntimeFacet::TargetIdentity,
            parity_contribution: if target_aligned {
                RuntimeParity::Aligned
            } else {
                RuntimeParity::Mismatched
            },
            declared: format!(
                "{}/{}/{}",
                declared_target_class.as_str(),
                declared_materialization_class.as_str(),
                declared_transport.as_str()
            ),
            observed: format!(
                "{}/{}/{}",
                instance.target_class.as_str(),
                instance.materialization_class.as_str(),
                instance.transport.as_str()
            ),
            element_tokens: tokens.clone(),
            summary: if target_aligned {
                "The runtime materialized on the declared target, materialization class, and transport.".to_owned()
            } else {
                "The runtime materialized on a different target than the capsule declared — code ran in the wrong place.".to_owned()
            },
        }
    };
    reason_tokens.extend(target_eval.element_tokens.clone());
    facet_evaluations.push(target_eval);

    // 2. Process namespace.
    let namespace_aligned = instance.process_namespace.kind == expected_namespace;
    let namespace_eval = {
        let mut tokens = Vec::new();
        if !namespace_aligned {
            tokens.push("process_namespace_mismatch".to_owned());
        }
        FacetEvaluation {
            facet: RuntimeFacet::ProcessNamespace,
            parity_contribution: if namespace_aligned {
                RuntimeParity::Aligned
            } else {
                RuntimeParity::Mismatched
            },
            declared: expected_namespace.as_str().to_owned(),
            observed: instance.process_namespace.kind.as_str().to_owned(),
            element_tokens: tokens.clone(),
            summary: if namespace_aligned {
                "The runtime's process namespace matches the namespace the declared target implies."
                    .to_owned()
            } else {
                "The runtime's processes ran in a different namespace than the declared target implies.".to_owned()
            },
        }
    };
    reason_tokens.extend(namespace_eval.element_tokens.clone());
    facet_evaluations.push(namespace_eval);

    // 3. Mount set.
    let mount_eval = fold_facet(
        RuntimeFacet::MountSet,
        instance.mount_set.iter().map(|mount| {
            (
                mount.state.parity_contribution(),
                if mount.state == MountState::Present {
                    None
                } else {
                    Some(format!("mount_{}_{}", mount.mount_id, mount.state.as_str()))
                },
            )
        }),
        format!("{} declared mount(s)", instance.mount_set.len()),
        format!(
            "{} present",
            instance
                .mount_set
                .iter()
                .filter(|mount| mount.state == MountState::Present)
                .count()
        ),
        "Every declared mount materialized.",
        "A declared mount did not materialize or diverged.",
    );
    reason_tokens.extend(mount_eval.element_tokens.clone());
    facet_evaluations.push(mount_eval);

    // 4. Port map.
    let port_eval = fold_facet(
        RuntimeFacet::PortMap,
        instance.port_map.iter().map(|port| {
            (
                port.state.parity_contribution(),
                if port.state == PortState::Published {
                    None
                } else {
                    Some(format!(
                        "port_{}_{}_{}",
                        port.service_id,
                        port.declared_port,
                        port.state.as_str()
                    ))
                },
            )
        }),
        format!("{} declared port(s)", instance.port_map.len()),
        format!(
            "{} published",
            instance
                .port_map
                .iter()
                .filter(|port| port.state == PortState::Published)
                .count()
        ),
        "Every declared port published as planned.",
        "A declared port did not publish as planned.",
    );
    reason_tokens.extend(port_eval.element_tokens.clone());
    facet_evaluations.push(port_eval);

    // 5. Service readiness.
    let mut service_evaluations = Vec::new();
    let mut involved_service_tokens = Vec::new();
    let mut unready_service_tokens = Vec::new();
    let readiness_eval = {
        let mut worst = RuntimeParity::Aligned;
        let mut tokens = Vec::new();
        for service in &instance.readiness_graph {
            involved_service_tokens.push(service.service_id.clone());
            let contribution = service.readiness.parity_contribution();
            if contribution.severity() > worst.severity() {
                worst = contribution;
            }
            if service.readiness.is_degraded() {
                let token = format!(
                    "service_{}_{}",
                    service.service_id,
                    service.readiness.as_str()
                );
                tokens.push(token);
                unready_service_tokens.push(service.service_id.clone());
            }
            service_evaluations.push(ServiceReadinessEvaluation {
                service_id: service.service_id.clone(),
                role: service.role,
                readiness: service.readiness,
                parity_contribution: contribution,
                summary: if service.readiness.is_degraded() {
                    format!(
                        "The {} service is {} — the stack is not fully up.",
                        service.service_id,
                        service.readiness.as_str()
                    )
                } else {
                    format!(
                        "The {} service materialized and is ready.",
                        service.service_id
                    )
                },
            });
        }
        FacetEvaluation {
            facet: RuntimeFacet::ServiceReadiness,
            parity_contribution: worst,
            declared: format!("{} declared service(s)", instance.readiness_graph.len()),
            observed: format!(
                "{} ready",
                instance
                    .readiness_graph
                    .iter()
                    .filter(|service| !service.readiness.is_degraded())
                    .count()
            ),
            element_tokens: tokens.clone(),
            summary: if worst == RuntimeParity::Aligned {
                "Every declared service materialized and is ready.".to_owned()
            } else {
                "A declared service did not materialize or is not ready.".to_owned()
            },
        }
    };
    reason_tokens.extend(readiness_eval.element_tokens.clone());
    facet_evaluations.push(readiness_eval);

    // 6. Secret projection.
    let secret_eval = fold_facet(
        RuntimeFacet::SecretProjection,
        instance.secret_projections.iter().map(|projection| {
            (
                projection.state.parity_contribution(),
                if projection.state == ProjectionState::Projected {
                    None
                } else {
                    Some(format!(
                        "secret_{}_{}",
                        projection.env_name,
                        projection.state.as_str()
                    ))
                },
            )
        }),
        format!("{} secret projection(s)", instance.secret_projections.len()),
        format!(
            "{} projected",
            instance
                .secret_projections
                .iter()
                .filter(|projection| projection.state == ProjectionState::Projected)
                .count()
        ),
        "Every declared environment binding projected through its point.",
        "A declared environment binding did not project.",
    );
    reason_tokens.extend(secret_eval.element_tokens.clone());
    facet_evaluations.push(secret_eval);

    // Fold the overall parity and headline facet.
    let mut parity = RuntimeParity::Aligned;
    for evaluation in &facet_evaluations {
        if evaluation.parity_contribution.severity() > parity.severity() {
            parity = evaluation.parity_contribution;
        }
    }
    let headline_facet = facet_evaluations
        .iter()
        .find(|evaluation| {
            evaluation.parity_contribution == parity && parity != RuntimeParity::Aligned
        })
        .map(|evaluation| evaluation.facet);

    reason_tokens.sort();
    reason_tokens.dedup();
    let mut degraded_facet_tokens: Vec<String> = facet_evaluations
        .iter()
        .filter(|evaluation| evaluation.parity_contribution != RuntimeParity::Aligned)
        .map(|evaluation| evaluation.facet.as_str().to_owned())
        .collect();
    degraded_facet_tokens.sort();
    degraded_facet_tokens.dedup();
    involved_service_tokens.sort();
    involved_service_tokens.dedup();
    unready_service_tokens.sort();
    unready_service_tokens.dedup();

    let target_matched = target_aligned && namespace_aligned;
    let where_code_ran = where_code_ran(
        instance,
        declared_target_class,
        declared_materialization_class,
        declared_transport,
        expected_namespace,
        target_matched,
    );
    let headline = headline(instance, parity, headline_facet);

    RuntimeMaterialization {
        record_kind: RUNTIME_MATERIALIZATION_RECORD_KIND.to_owned(),
        schema_version: RUNTIME_MATERIALIZATION_SCHEMA_VERSION,
        instance_id: instance.instance_id.clone(),
        capsule_id: instance.capsule_id.clone(),
        capsule_digest: instance.capsule_digest.clone(),
        profile: instance.profile,
        declared_target_class,
        observed_target_class: instance.target_class,
        declared_materialization_class,
        observed_materialization_class: instance.materialization_class,
        declared_transport,
        observed_transport: instance.transport,
        parity,
        target_matched,
        materialization_parity_state: parity.materialization_parity_state(),
        headline_facet,
        reason_tokens,
        degraded_facet_tokens,
        involved_service_tokens,
        unready_service_tokens,
        facet_evaluations,
        service_evaluations,
        instance: instance.clone(),
        where_code_ran,
        headline,
        redaction_class: RedactionClass::MetadataOnly,
    }
}

/// Folds an iterator of per-element `(parity, token)` pairs into one facet
/// evaluation, taking the coldest contribution and collecting the degrading
/// element tokens.
fn fold_facet(
    facet: RuntimeFacet,
    elements: impl Iterator<Item = (RuntimeParity, Option<String>)>,
    declared: String,
    observed: String,
    aligned_summary: &str,
    degraded_summary: &str,
) -> FacetEvaluation {
    let mut worst = RuntimeParity::Aligned;
    let mut tokens = Vec::new();
    for (contribution, token) in elements {
        if contribution.severity() > worst.severity() {
            worst = contribution;
        }
        if let Some(token) = token {
            tokens.push(token);
        }
    }
    FacetEvaluation {
        facet,
        parity_contribution: worst,
        declared,
        observed,
        element_tokens: tokens,
        summary: if worst == RuntimeParity::Aligned {
            aligned_summary.to_owned()
        } else {
            degraded_summary.to_owned()
        },
    }
}

fn where_code_ran(
    instance: &RuntimeInstance,
    declared_target_class: CapsuleTargetClass,
    declared_materialization_class: MaterializationClass,
    declared_transport: TargetTransport,
    expected_namespace: NamespaceKind,
    target_matched: bool,
) -> String {
    if target_matched {
        format!(
            "Code ran in a {} namespace on the {} target ({}/{}) materializing capsule {}.",
            instance.process_namespace.kind.as_str(),
            instance.target_class.as_str(),
            instance.materialization_class.as_str(),
            instance.transport.as_str(),
            instance.capsule_id
        )
    } else {
        format!(
            "Code ran in a {} namespace on the {} target ({}/{}), but capsule {} declared the {} target ({}/{}, {} namespace) — wrong target.",
            instance.process_namespace.kind.as_str(),
            instance.target_class.as_str(),
            instance.materialization_class.as_str(),
            instance.transport.as_str(),
            instance.capsule_id,
            declared_target_class.as_str(),
            declared_materialization_class.as_str(),
            declared_transport.as_str(),
            expected_namespace.as_str()
        )
    }
}

fn headline(
    instance: &RuntimeInstance,
    parity: RuntimeParity,
    headline_facet: Option<RuntimeFacet>,
) -> String {
    match parity {
        RuntimeParity::Aligned => format!(
            "Runtime {} is aligned: it materialized capsule {} exactly as declared on the {} target.",
            instance.instance_id,
            instance.capsule_id,
            instance.target_class.as_str()
        ),
        RuntimeParity::Degraded => format!(
            "Runtime {} is degraded ({}): it materialized the {} target but part of the stack is not fully up.",
            instance.instance_id,
            headline_facet.map(RuntimeFacet::as_str).unwrap_or("service_readiness"),
            instance.target_class.as_str()
        ),
        RuntimeParity::Mismatched => format!(
            "Runtime {} is mismatched ({}): it ran on the {} target, not the target capsule {} declared.",
            instance.instance_id,
            headline_facet.map(RuntimeFacet::as_str).unwrap_or("target_identity"),
            instance.target_class.as_str(),
            instance.capsule_id
        ),
    }
}

/// The desktop runtime materialization. Desktop reads the same
/// [`RuntimeMaterialization`] object as every other surface.
pub fn desktop_runtime_materialization(
    capsule: &EnvironmentCapsule,
    instance: &RuntimeInstance,
) -> RuntimeMaterialization {
    materialize_runtime(capsule, instance)
}

/// The headless / CLI runtime materialization. Headless reads the same
/// [`RuntimeMaterialization`] object as every other surface.
pub fn headless_runtime_materialization(
    capsule: &EnvironmentCapsule,
    instance: &RuntimeInstance,
) -> RuntimeMaterialization {
    materialize_runtime(capsule, instance)
}

/// The AI-path runtime materialization. The AI surface reads the same
/// [`RuntimeMaterialization`] object — including where code ran and which
/// services are involved — as every other surface.
pub fn ai_runtime_materialization(
    capsule: &EnvironmentCapsule,
    instance: &RuntimeInstance,
) -> RuntimeMaterialization {
    materialize_runtime(capsule, instance)
}

/// The support-path runtime export: the metadata-first projection wrapping
/// the same [`RuntimeMaterialization`] object support and release surfaces
/// read.
pub fn support_runtime_materialization(
    capsule: &EnvironmentCapsule,
    instance: &RuntimeInstance,
) -> RuntimeExport {
    export_runtime_materialization(&materialize_runtime(capsule, instance))
}

// ---------------------------------------------------------------------------
// Metadata-first export.
// ---------------------------------------------------------------------------

/// A metadata-first projection of a runtime materialization for support and
/// release surfaces. It carries the distinguishable parity, where code ran,
/// the degraded facets, and the involved / unready services — never secrets,
/// raw paths, or provider payloads — and wraps the canonical materialization
/// so support never re-derives the parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Runtime-instance id under evaluation.
    pub instance_id: String,
    /// Capsule id the instance materializes.
    pub capsule_id: String,
    /// Claimed environment profile.
    pub profile: EnvironmentProfile,
    /// The explicit parity verdict.
    pub parity: RuntimeParity,
    /// True when the runtime materialized on the declared target and
    /// namespace.
    pub target_matched: bool,
    /// The governance materialization-parity evidence state this parity maps
    /// to.
    pub materialization_parity_state: EvidenceState,
    /// Target class the capsule declared.
    pub declared_target_class: CapsuleTargetClass,
    /// Target class the runtime materialized on.
    pub observed_target_class: CapsuleTargetClass,
    /// The facet that headlines the parity (none when aligned).
    pub headline_facet: Option<RuntimeFacet>,
    /// Stable tokens naming every element that degraded or mismatched the
    /// runtime.
    pub reason_tokens: Vec<String>,
    /// Stable tokens naming every facet that is degraded or mismatched.
    pub degraded_facet_tokens: Vec<String>,
    /// Stable tokens naming every declared service involved.
    pub involved_service_tokens: Vec<String>,
    /// Stable tokens naming every service that is not ready.
    pub unready_service_tokens: Vec<String>,
    /// Review-safe statement of where code actually ran.
    pub where_code_ran: String,
    /// The canonical materialization this export wraps.
    pub materialization: RuntimeMaterialization,
    /// Redaction posture (always metadata-only).
    pub redaction_class: RedactionClass,
}

/// Projects a metadata-first [`RuntimeExport`] from a materialization.
pub fn export_runtime_materialization(materialization: &RuntimeMaterialization) -> RuntimeExport {
    RuntimeExport {
        record_kind: RUNTIME_MATERIALIZATION_EXPORT_RECORD_KIND.to_owned(),
        schema_version: RUNTIME_MATERIALIZATION_SCHEMA_VERSION,
        instance_id: materialization.instance_id.clone(),
        capsule_id: materialization.capsule_id.clone(),
        profile: materialization.profile,
        parity: materialization.parity,
        target_matched: materialization.target_matched,
        materialization_parity_state: materialization.materialization_parity_state,
        declared_target_class: materialization.declared_target_class,
        observed_target_class: materialization.observed_target_class,
        headline_facet: materialization.headline_facet,
        reason_tokens: materialization.reason_tokens.clone(),
        degraded_facet_tokens: materialization.degraded_facet_tokens.clone(),
        involved_service_tokens: materialization.involved_service_tokens.clone(),
        unready_service_tokens: materialization.unready_service_tokens.clone(),
        where_code_ran: materialization.where_code_ran.clone(),
        materialization: materialization.clone(),
        redaction_class: RedactionClass::MetadataOnly,
    }
}

// ---------------------------------------------------------------------------
// Diff across two runtime instances.
// ---------------------------------------------------------------------------

/// How a runtime field changed between two instances.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeChangeKind {
    /// The element exists only in the target instance.
    Added,
    /// The element exists only in the base instance.
    Removed,
    /// The element changed value between the two instances.
    Changed,
}

impl RuntimeChangeKind {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Added => "added",
            Self::Removed => "removed",
            Self::Changed => "changed",
        }
    }
}

/// One field-level change between two runtime instances. Values are metadata
/// tokens (target classes, namespace kinds, states), never secrets or bodies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeFieldChange {
    /// Dotted path naming the changed field.
    pub path: String,
    /// Kind of change.
    pub change_kind: RuntimeChangeKind,
    /// Metadata token before the change (empty for additions).
    pub before: String,
    /// Metadata token after the change (empty for removals).
    pub after: String,
}

/// The diff between two runtime instances, surfacing how two materializations
/// of (potentially) the same environment differ in identity and readiness, so
/// parity across claimed target classes stays visible.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeInstanceDiff {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Base instance id.
    pub base_instance_id: String,
    /// Target instance id.
    pub target_instance_id: String,
    /// True when the two instances are identity- and readiness-identical.
    pub identical: bool,
    /// Ordered field-level changes.
    pub changes: Vec<RuntimeFieldChange>,
    /// Review-safe summary of the diff.
    pub summary: String,
}

fn push_change(changes: &mut Vec<RuntimeFieldChange>, path: &str, before: String, after: String) {
    if before != after {
        changes.push(RuntimeFieldChange {
            path: path.to_owned(),
            change_kind: RuntimeChangeKind::Changed,
            before,
            after,
        });
    }
}

/// Diffs two runtime instances on target identity, namespace, and per-service
/// readiness, reporting the changes as metadata tokens.
pub fn diff_runtime_instances(
    base: &RuntimeInstance,
    target: &RuntimeInstance,
) -> RuntimeInstanceDiff {
    let mut changes = Vec::new();
    push_change(
        &mut changes,
        "target_class",
        base.target_class.as_str().to_owned(),
        target.target_class.as_str().to_owned(),
    );
    push_change(
        &mut changes,
        "materialization_class",
        base.materialization_class.as_str().to_owned(),
        target.materialization_class.as_str().to_owned(),
    );
    push_change(
        &mut changes,
        "transport",
        base.transport.as_str().to_owned(),
        target.transport.as_str().to_owned(),
    );
    push_change(
        &mut changes,
        "process_namespace.kind",
        base.process_namespace.kind.as_str().to_owned(),
        target.process_namespace.kind.as_str().to_owned(),
    );

    let base_services: BTreeSet<&str> = base
        .readiness_graph
        .iter()
        .map(|service| service.service_id.as_str())
        .collect();
    for service in &base.readiness_graph {
        match target
            .readiness_graph
            .iter()
            .find(|other| other.service_id == service.service_id)
        {
            Some(other) => push_change(
                &mut changes,
                &format!("readiness_graph.{}.readiness", service.service_id),
                service.readiness.as_str().to_owned(),
                other.readiness.as_str().to_owned(),
            ),
            None => changes.push(RuntimeFieldChange {
                path: format!("readiness_graph.{}", service.service_id),
                change_kind: RuntimeChangeKind::Removed,
                before: service.readiness.as_str().to_owned(),
                after: String::new(),
            }),
        }
    }
    for service in &target.readiness_graph {
        if !base_services.contains(service.service_id.as_str()) {
            changes.push(RuntimeFieldChange {
                path: format!("readiness_graph.{}", service.service_id),
                change_kind: RuntimeChangeKind::Added,
                before: String::new(),
                after: service.readiness.as_str().to_owned(),
            });
        }
    }

    let identical = changes.is_empty();
    let summary = if identical {
        format!(
            "Runtime instances {} and {} are identity- and readiness-identical.",
            base.instance_id, target.instance_id
        )
    } else {
        format!(
            "{} runtime change(s) between instances {} and {}.",
            changes.len(),
            base.instance_id,
            target.instance_id
        )
    };

    RuntimeInstanceDiff {
        record_kind: "runtime_instance_diff_record".to_owned(),
        schema_version: RUNTIME_MATERIALIZATION_SCHEMA_VERSION,
        base_instance_id: base.instance_id.clone(),
        target_instance_id: target.instance_id.clone(),
        identical,
        changes,
        summary,
    }
}

// ---------------------------------------------------------------------------
// Fixture record.
// ---------------------------------------------------------------------------

/// The scenario a runtime-materialization fixture exercises.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeScenario {
    /// The runtime materialized exactly the declared contract.
    Aligned,
    /// A backing service is not ready (a partial multi-service stack).
    PartialServiceReadiness,
    /// A declared mount did not materialize.
    DegradedMount,
    /// A declared port did not publish.
    UnpublishedPort,
    /// A declared environment binding did not project.
    SecretProjectionPending,
    /// The runtime materialized on a different target than declared.
    WrongTarget,
    /// The runtime's processes ran in a different namespace than declared.
    WrongNamespace,
}

impl RuntimeScenario {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Aligned => "aligned",
            Self::PartialServiceReadiness => "partial_service_readiness",
            Self::DegradedMount => "degraded_mount",
            Self::UnpublishedPort => "unpublished_port",
            Self::SecretProjectionPending => "secret_projection_pending",
            Self::WrongTarget => "wrong_target",
            Self::WrongNamespace => "wrong_namespace",
        }
    }
}

/// One checked-in fixture: a capsule, the runtime instance that materialized
/// it, and the parity outcome the engine must reach for the pair.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeMaterializationFixture {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Target class the fixture exercises.
    pub target_class: CapsuleTargetClass,
    /// Scenario the fixture exercises.
    pub scenario: RuntimeScenario,
    /// The capsule under test.
    pub capsule: EnvironmentCapsule,
    /// The runtime instance that materialized the capsule.
    pub instance: RuntimeInstance,
    /// Expected parity verdict.
    pub expected_parity: RuntimeParity,
    /// Expected target-matched flag.
    pub expected_target_matched: bool,
    /// Expected headline facet.
    pub expected_headline_facet: Option<RuntimeFacet>,
    /// Expected reason tokens.
    pub expected_reason_tokens: Vec<String>,
    /// Expected degraded-facet tokens.
    pub expected_degraded_facet_tokens: Vec<String>,
    /// Expected unready-service tokens.
    pub expected_unready_service_tokens: Vec<String>,
    /// One consumer surface that ingests this materialization.
    pub consumer_ref: String,
    /// Short reviewer note.
    pub notes: String,
}

// ---------------------------------------------------------------------------
// Validation.
// ---------------------------------------------------------------------------

fn violation(report: &mut ValidationReport, check_id: &'static str, message: impl Into<String>) {
    report.violations.push(ValidationViolation {
        check_id,
        message: message.into(),
    });
}

fn is_hex64(digest: &CapsuleDigest) -> bool {
    digest.value.len() == 64 && digest.value.bytes().all(|b| b.is_ascii_hexdigit())
}

/// Validates a checked-in runtime instance against the frozen contract.
pub fn validate_runtime_instance(instance: &RuntimeInstance) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if instance.record_kind != RUNTIME_INSTANCE_RECORD_KIND {
        violation(
            &mut report,
            "instance.record_kind",
            "runtime instance record_kind does not match the frozen token",
        );
    }
    if instance.schema_version != RUNTIME_MATERIALIZATION_SCHEMA_VERSION {
        violation(
            &mut report,
            "instance.schema_version",
            "runtime instance schema_version must be 1",
        );
    }
    if instance.instance_id.trim().is_empty() {
        violation(
            &mut report,
            "instance.id",
            "runtime instance must carry an id",
        );
    }
    if instance.capsule_id.trim().is_empty() {
        violation(
            &mut report,
            "instance.capsule_id",
            "runtime instance must name the capsule it materializes",
        );
    }
    if instance.capsule_version == 0 {
        violation(
            &mut report,
            "instance.capsule_version",
            "runtime instance capsule version must be at least 1",
        );
    }
    if !is_hex64(&instance.capsule_digest) {
        violation(
            &mut report,
            "instance.capsule_digest",
            "runtime instance capsule digest must be 64 lowercase hex",
        );
    }
    if instance.target_class != target_class_for(instance.materialization_class, instance.transport)
    {
        violation(
            &mut report,
            "instance.target_class_agrees",
            "runtime instance target class must agree with its materialization class and transport",
        );
    }
    if instance.redaction_class != RedactionClass::MetadataOnly {
        violation(
            &mut report,
            "instance.redaction_class",
            "runtime instance must declare a metadata-only redaction class",
        );
    }

    let mut mount_ids = BTreeSet::new();
    let mut saw_working_tree = false;
    for mount in &instance.mount_set {
        if mount.mount_id.trim().is_empty() {
            violation(&mut report, "instance.mount_id", "mount must carry an id");
        } else if !mount_ids.insert(mount.mount_id.as_str()) {
            violation(
                &mut report,
                "instance.mount_id_unique",
                format!("runtime instance repeats mount id {}", mount.mount_id),
            );
        }
        if mount.kind == MountKind::WorkingTree {
            saw_working_tree = true;
        }
        if mount.target_ref.trim().is_empty() || mount.summary.trim().is_empty() {
            violation(
                &mut report,
                "instance.mount_fields",
                format!(
                    "mount {} must carry a target ref and summary",
                    mount.mount_id
                ),
            );
        }
    }
    if !saw_working_tree {
        violation(
            &mut report,
            "instance.working_tree",
            "runtime instance must declare a working-tree mount",
        );
    }

    let service_ids: BTreeSet<&str> = instance
        .readiness_graph
        .iter()
        .map(|service| service.service_id.as_str())
        .collect();
    let mut seen_services = BTreeSet::new();
    for service in &instance.readiness_graph {
        if !seen_services.insert(service.service_id.as_str()) {
            violation(
                &mut report,
                "instance.service_unique",
                format!("runtime instance repeats service id {}", service.service_id),
            );
        }
        for dependency in &service.depends_on {
            if !service_ids.contains(dependency.as_str()) {
                violation(
                    &mut report,
                    "instance.service_edge",
                    format!(
                        "service {} depends on unknown service {}",
                        service.service_id, dependency
                    ),
                );
            }
        }
        if service.health_probe_ref.trim().is_empty() {
            violation(
                &mut report,
                "instance.health_probe",
                format!(
                    "service {} must carry a health-probe ref",
                    service.service_id
                ),
            );
        }
    }

    for port in &instance.port_map {
        if !service_ids.contains(port.service_id.as_str()) {
            violation(
                &mut report,
                "instance.port_service",
                format!(
                    "port mapping references unknown service {}",
                    port.service_id
                ),
            );
        }
    }

    let mut projection_ids = BTreeSet::new();
    for projection in &instance.secret_projections {
        if !projection_ids.insert(projection.projection_id.as_str()) {
            violation(
                &mut report,
                "instance.projection_unique",
                format!(
                    "runtime instance repeats projection id {}",
                    projection.projection_id
                ),
            );
        }
        if projection.env_name.trim().is_empty() || projection.handle_ref.trim().is_empty() {
            violation(
                &mut report,
                "instance.projection_fields",
                format!(
                    "projection {} must carry an env name and a handle ref",
                    projection.projection_id
                ),
            );
        }
    }

    if instance.process_namespace.namespace_ref.trim().is_empty()
        || instance.process_namespace.boundary_ref.trim().is_empty()
    {
        violation(
            &mut report,
            "instance.namespace_refs",
            "runtime instance namespace must carry a namespace ref and a boundary ref",
        );
    }
    if instance.summary.trim().is_empty() {
        violation(
            &mut report,
            "instance.summary",
            "runtime instance must carry a summary",
        );
    }

    if report.violations.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

/// Validates a checked-in fixture: the capsule and instance themselves, and
/// that the recorded expectations equal what the engine computes.
pub fn validate_runtime_materialization_fixture(
    fixture: &RuntimeMaterializationFixture,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if fixture.record_kind != RUNTIME_MATERIALIZATION_FIXTURE_RECORD_KIND {
        violation(
            &mut report,
            "fixture.record_kind",
            "fixture record_kind does not match the frozen token",
        );
    }
    if fixture.schema_version != RUNTIME_MATERIALIZATION_SCHEMA_VERSION {
        violation(
            &mut report,
            "fixture.schema_version",
            "fixture schema_version must be 1",
        );
    }
    if fixture.fixture_id.trim().is_empty() {
        violation(&mut report, "fixture.id", "fixture must carry a stable id");
    }
    if fixture.consumer_ref.trim().is_empty() {
        violation(
            &mut report,
            "fixture.consumer_ref",
            format!("fixture {} must cite a consumer ref", fixture.fixture_id),
        );
    }
    if fixture.notes.trim().is_empty() {
        violation(
            &mut report,
            "fixture.notes",
            format!("fixture {} must carry a reviewer note", fixture.fixture_id),
        );
    }

    if let Err(capsule_report) = crate::capsules::validate_environment_capsule(&fixture.capsule) {
        for inner in capsule_report.violations {
            report.violations.push(inner);
        }
    }
    if let Err(instance_report) = validate_runtime_instance(&fixture.instance) {
        for inner in instance_report.violations {
            report.violations.push(inner);
        }
    }

    // The instance must name the capsule it claims to materialize.
    if fixture.instance.capsule_id != fixture.capsule.identity.capsule_id {
        violation(
            &mut report,
            "fixture.instance_capsule_id",
            format!(
                "fixture {} instance materializes capsule {} but carries capsule {}",
                fixture.fixture_id,
                fixture.capsule.identity.capsule_id,
                fixture.instance.capsule_id
            ),
        );
    }

    let materialization = materialize_runtime(&fixture.capsule, &fixture.instance);
    if fixture.expected_parity != materialization.parity {
        violation(
            &mut report,
            "fixture.expected_parity",
            format!(
                "fixture {} expected parity {} disagrees with the engine ({})",
                fixture.fixture_id,
                fixture.expected_parity.as_str(),
                materialization.parity.as_str()
            ),
        );
    }
    if fixture.expected_target_matched != materialization.target_matched {
        violation(
            &mut report,
            "fixture.expected_target_matched",
            format!(
                "fixture {} expected target_matched disagrees with the engine",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_headline_facet != materialization.headline_facet {
        violation(
            &mut report,
            "fixture.expected_headline_facet",
            format!(
                "fixture {} expected headline facet disagrees with the engine",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_reason_tokens != materialization.reason_tokens {
        violation(
            &mut report,
            "fixture.expected_reason_tokens",
            format!(
                "fixture {} expected reason tokens disagree with the engine",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_degraded_facet_tokens != materialization.degraded_facet_tokens {
        violation(
            &mut report,
            "fixture.expected_degraded_facet_tokens",
            format!(
                "fixture {} expected degraded-facet tokens disagree with the engine",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_unready_service_tokens != materialization.unready_service_tokens {
        violation(
            &mut report,
            "fixture.expected_unready_service_tokens",
            format!(
                "fixture {} expected unready-service tokens disagree with the engine",
                fixture.fixture_id
            ),
        );
    }

    if report.violations.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}
