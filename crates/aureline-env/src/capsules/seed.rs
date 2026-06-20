//! The checked-in environment-capsule corpus this lane freezes: one
//! canonical capsule per claimed target class plus the degraded variants
//! that exercise the inspector's narrowing.

use crate::m5_env_governance::{
    certify_capsule_outcome, ClaimMaturity, EnvironmentProfile, EvidenceState,
    MaterializationClass, WarmStartPosture,
};

use super::{
    inspect_environment, CapsuleDigest, CapsuleIdentity, CapsuleSourceRef, CapsuleTargetClass,
    CompatibilityFingerprint, EnvVarBinding, EnvironmentCapsule, EnvironmentCapsuleFixture,
    FingerprintInput, LifecyclePhase, MaterializationStatus, ObservabilityMetadata, RedactionClass,
    ServiceGraph, ServiceNode, ServiceRole, SourceKind, TargetPlan, TargetTransport,
    ToolchainComponent, ToolchainKind, ToolchainPlan, TrustGateState, TrustHook, WorkingRootKind,
    ENVIRONMENT_CAPSULE_FIXTURE_RECORD_KIND, ENVIRONMENT_CAPSULE_RECORD_KIND,
    ENVIRONMENT_CAPSULE_SCHEMA_VERSION,
};

const EXECUTION_SCOPE_REF: &str = "artifacts/runtime/execution_scope_matrix.yaml";
const AUTHORITY_CLASSES_REF: &str = "artifacts/runtime/authority_classes.yaml";
const ENV_INSPECT_REF: &str = "crates/aureline-runtime/src/env_inspect/mod.rs";
const SUPPORT_BUNDLE_REF: &str = "crates/aureline-support/src/bundle/mod.rs";

/// Deterministic 64-hex placeholder digest derived from a stable label.
/// These are metadata tokens standing in for real content digests, never
/// the bodies they would digest.
fn dg(label: &str) -> CapsuleDigest {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in label.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    let chunk = format!("{hash:016x}");
    CapsuleDigest {
        algorithm: "sha256".to_owned(),
        value: chunk.repeat(4),
    }
}

struct CapsuleSpec {
    target_class: CapsuleTargetClass,
    capsule_id: &'static str,
    profile: EnvironmentProfile,
    materialization_class: MaterializationClass,
    transport: TargetTransport,
    working_root_kind: WorkingRootKind,
    host_boundary_ref: &'static str,
    claimed_maturity: ClaimMaturity,
    claimed_warm_start_posture: WarmStartPosture,
    label: &'static str,
    target_label: &'static str,
    why: &'static str,
    consumer_ref: &'static str,
    notes: &'static str,
}

const SPECS: &[CapsuleSpec] = &[
    CapsuleSpec {
        target_class: CapsuleTargetClass::Local,
        capsule_id: "env.capsule.local",
        profile: EnvironmentProfile::WorkspaceTemplate,
        materialization_class: MaterializationClass::LocalNative,
        transport: TargetTransport::LocalProcess,
        working_root_kind: WorkingRootKind::LocalPath,
        host_boundary_ref: "artifacts/remote/host_boundary_matrix.yaml",
        claimed_maturity: ClaimMaturity::Stable,
        claimed_warm_start_posture: WarmStartPosture::ColdBuild,
        label: "Local native capsule",
        target_label: "Local host process",
        why: "This environment is a template's source digest hydrated natively on the local host: the target plan, toolchain plan, trust-gated hooks, and service graph all derive from the capsule rather than from whatever the scaffold ran.",
        consumer_ref: "crates/aureline-shell/src/environment_inspector/mod.rs",
        notes: "A local capsule cold-builds and never claims warm reuse; its lifecycle hooks stay trust-gated.",
    },
    CapsuleSpec {
        target_class: CapsuleTargetClass::Ssh,
        capsule_id: "env.capsule.ssh",
        profile: EnvironmentProfile::RemoteContainer,
        materialization_class: MaterializationClass::RemoteHost,
        transport: TargetTransport::Ssh,
        working_root_kind: WorkingRootKind::RemoteMount,
        host_boundary_ref: "artifacts/remote/host_boundary_matrix.yaml",
        claimed_maturity: ClaimMaturity::Beta,
        claimed_warm_start_posture: WarmStartPosture::WarmPartialReuse,
        label: "SSH remote-host capsule",
        target_label: "Remote host over SSH",
        why: "This environment is materialized on a remote host reached over SSH within its declared boundary: the capsule's target plan and service graph match the host-boundary matrix, and a current fingerprint lets it warm-reuse a partial layer cache.",
        consumer_ref: "crates/aureline-remote/src/managed_workspace_lifecycle/mod.rs",
        notes: "An SSH capsule narrows when its toolchain plan goes stale or its materialization diverges from the capsule object across surfaces.",
    },
    CapsuleSpec {
        target_class: CapsuleTargetClass::Container,
        capsule_id: "env.capsule.container",
        profile: EnvironmentProfile::Prebuild,
        materialization_class: MaterializationClass::Container,
        transport: TargetTransport::Container,
        working_root_kind: WorkingRootKind::ContainerVolume,
        host_boundary_ref: "artifacts/remote/host_boundary_matrix.yaml",
        claimed_maturity: ClaimMaturity::Beta,
        claimed_warm_start_posture: WarmStartPosture::WarmFullReuse,
        label: "Container prebuild capsule",
        target_label: "Local container runtime",
        why: "This environment is a prebuilt container snapshot whose fingerprint currently matches the source digest, so the whole capsule is warm-reused; a fingerprint mismatch invalidates the snapshot rather than serving stale truth.",
        consumer_ref: "crates/aureline-runtime/src/capsule_resolver/mod.rs",
        notes: "A container prebuild claims full warm reuse only while its fingerprint matches the source digest; a stale fingerprint forces a cold build.",
    },
    CapsuleSpec {
        target_class: CapsuleTargetClass::Devcontainer,
        capsule_id: "env.capsule.devcontainer",
        profile: EnvironmentProfile::Devcontainer,
        materialization_class: MaterializationClass::Devcontainer,
        transport: TargetTransport::Container,
        working_root_kind: WorkingRootKind::ContainerVolume,
        host_boundary_ref: "artifacts/remote/host_boundary_matrix.yaml",
        claimed_maturity: ClaimMaturity::Beta,
        claimed_warm_start_posture: WarmStartPosture::WarmPartialReuse,
        label: "Devcontainer capsule",
        target_label: "Devcontainer runtime",
        why: "This environment materializes a devcontainer definition: the capsule declares its target plan, toolchain plan, trust-gated hooks, and service graph from the devcontainer config rather than inferring them.",
        consumer_ref: "crates/aureline-runtime/src/execution_context/mod.rs",
        notes: "A devcontainer warm-reuses a partial layer cache; an incomplete service graph or stale toolchain narrows the claim.",
    },
    CapsuleSpec {
        target_class: CapsuleTargetClass::Vm,
        capsule_id: "env.capsule.vm",
        profile: EnvironmentProfile::Starter,
        materialization_class: MaterializationClass::RemoteHost,
        transport: TargetTransport::VirtualMachine,
        working_root_kind: WorkingRootKind::RemoteMount,
        host_boundary_ref: "artifacts/remote/host_boundary_matrix.yaml",
        claimed_maturity: ClaimMaturity::Stable,
        claimed_warm_start_posture: WarmStartPosture::WarmPartialReuse,
        label: "Virtual-machine capsule",
        target_label: "Virtual machine",
        why: "This environment is a starter capsule materialized inside a virtual machine: its source digest pins the seed, and a current prebuild fingerprint lets it warm-reuse cached dependencies while the rest is rebuilt.",
        consumer_ref: "crates/aureline-cli/src/environment/mod.rs",
        notes: "A VM capsule warm-reuses cached dependencies only while the source digest and fingerprint are current.",
    },
    CapsuleSpec {
        target_class: CapsuleTargetClass::ManagedWorkspace,
        capsule_id: "env.capsule.managed_workspace",
        profile: EnvironmentProfile::ManagedWorkspace,
        materialization_class: MaterializationClass::ManagedCloud,
        transport: TargetTransport::CloudManaged,
        working_root_kind: WorkingRootKind::ManagedVolume,
        host_boundary_ref: "artifacts/runtime/managed_workspace_lifecycle.yaml",
        claimed_maturity: ClaimMaturity::Beta,
        claimed_warm_start_posture: WarmStartPosture::WarmFullReuse,
        label: "Managed-workspace capsule",
        target_label: "Managed cloud workspace",
        why: "This environment is a managed-workspace row materialized in the cloud: its capsule, prebuild fingerprint, and service graph are mirrored so support and release read the captured environment claim, not live truth.",
        consumer_ref: "crates/aureline-support/src/bundle/mod.rs",
        notes: "A managed-workspace capsule claims full warm reuse from a current prebuild; a materialization skew narrows the claim.",
    },
];

fn source_refs(spec: &CapsuleSpec) -> Vec<CapsuleSourceRef> {
    let id = spec.capsule_id;
    let mut refs = vec![
        CapsuleSourceRef {
            source_id: "src.template".to_owned(),
            kind: SourceKind::WorkspaceTemplate,
            reference: "artifacts/workspace/archetype_confidence_rows.yaml".to_owned(),
            digest: dg(&format!("{id}:template")),
            coverage: EvidenceState::Current,
            summary: "The workspace template the capsule hydrates from.".to_owned(),
        },
        CapsuleSourceRef {
            source_id: "src.lockfile".to_owned(),
            kind: SourceKind::Lockfile,
            reference: "artifacts/build/build_identity.json".to_owned(),
            digest: dg(&format!("{id}:lockfile")),
            coverage: EvidenceState::Current,
            summary: "The dependency lockfile pinning resolved versions.".to_owned(),
        },
        CapsuleSourceRef {
            source_id: "src.toolchain".to_owned(),
            kind: SourceKind::ToolchainManifest,
            reference: "artifacts/install/state_root_matrix.yaml".to_owned(),
            digest: dg(&format!("{id}:toolchain")),
            coverage: EvidenceState::Current,
            summary: "The toolchain manifest pinning language and runtime versions.".to_owned(),
        },
        CapsuleSourceRef {
            source_id: "src.services".to_owned(),
            kind: SourceKind::ServiceManifest,
            reference: "artifacts/runtime/managed_workspace_lifecycle.yaml".to_owned(),
            digest: dg(&format!("{id}:services")),
            coverage: EvidenceState::Current,
            summary: "The service manifest defining the materialized service graph.".to_owned(),
        },
        CapsuleSourceRef {
            source_id: "src.prebuild".to_owned(),
            kind: SourceKind::PrebuildSnapshot,
            reference: "artifacts/entry/warm_start_chooser_contract.md".to_owned(),
            digest: dg(&format!("{id}:prebuild")),
            coverage: EvidenceState::Current,
            summary: "The prebuilt environment snapshot warm start may reuse.".to_owned(),
        },
    ];
    if spec.target_class == CapsuleTargetClass::Devcontainer {
        refs.push(CapsuleSourceRef {
            source_id: "src.devcontainer".to_owned(),
            kind: SourceKind::DevcontainerConfig,
            reference: "artifacts/runtime/execution_scope_matrix.yaml".to_owned(),
            digest: dg(&format!("{id}:devcontainer")),
            coverage: EvidenceState::Current,
            summary: "The devcontainer configuration defining the environment.".to_owned(),
        });
    }
    refs
}

fn toolchain_plan(id: &str) -> ToolchainPlan {
    ToolchainPlan {
        components: vec![
            ToolchainComponent {
                component_id: "tool.language_runtime".to_owned(),
                kind: ToolchainKind::LanguageRuntime,
                pinned_version: "1.84.0".to_owned(),
                source_id: "src.toolchain".to_owned(),
                summary: "Pinned language runtime.".to_owned(),
            },
            ToolchainComponent {
                component_id: "tool.package_manager".to_owned(),
                kind: ToolchainKind::PackageManager,
                pinned_version: "10.4.1".to_owned(),
                source_id: "src.lockfile".to_owned(),
                summary: "Pinned package manager.".to_owned(),
            },
        ],
        coverage: EvidenceState::Current,
        summary: format!("Deterministic toolchain plan pinned by the {id} capsule."),
    }
}

fn service_graph() -> ServiceGraph {
    ServiceGraph {
        services: vec![
            ServiceNode {
                service_id: "svc.app".to_owned(),
                role: ServiceRole::Primary,
                exposed_ports: vec![8080],
                depends_on: vec!["svc.db".to_owned()],
                summary: "The primary application service.".to_owned(),
            },
            ServiceNode {
                service_id: "svc.db".to_owned(),
                role: ServiceRole::Dependency,
                exposed_ports: vec![5432],
                depends_on: vec![],
                summary: "The backing database dependency.".to_owned(),
            },
        ],
        coverage: EvidenceState::Current,
        summary: "Declared service graph of the primary service and its database dependency."
            .to_owned(),
    }
}

fn trust_hooks(id: &str) -> Vec<TrustHook> {
    vec![
        TrustHook {
            hook_id: "hook.on_create".to_owned(),
            phase: LifecyclePhase::OnCreate,
            gate_state: TrustGateState::Gated,
            authority_ref: EXECUTION_SCOPE_REF.to_owned(),
            command_digest: dg(&format!("{id}:hook:on_create")),
            summary: "Trust-gated create hook.".to_owned(),
        },
        TrustHook {
            hook_id: "hook.post_start".to_owned(),
            phase: LifecyclePhase::PostStart,
            gate_state: TrustGateState::Gated,
            authority_ref: AUTHORITY_CLASSES_REF.to_owned(),
            command_digest: dg(&format!("{id}:hook:post_start")),
            summary: "Trust-gated post-start hook.".to_owned(),
        },
    ]
}

fn compatibility_fingerprint(id: &str) -> CompatibilityFingerprint {
    CompatibilityFingerprint {
        fingerprint: dg(&format!("{id}:fingerprint")),
        inputs: vec![
            FingerprintInput {
                input_id: "fp.lockfile".to_owned(),
                source_id: "src.lockfile".to_owned(),
                digest: dg(&format!("{id}:fp:lockfile")),
            },
            FingerprintInput {
                input_id: "fp.toolchain".to_owned(),
                source_id: "src.toolchain".to_owned(),
                digest: dg(&format!("{id}:fp:toolchain")),
            },
            FingerprintInput {
                input_id: "fp.prebuild".to_owned(),
                source_id: "src.prebuild".to_owned(),
                digest: dg(&format!("{id}:fp:prebuild")),
            },
        ],
        coverage: EvidenceState::Current,
        summary: "Compatibility fingerprint over the lockfile, toolchain, and prebuild inputs."
            .to_owned(),
    }
}

fn base_capsule(spec: &CapsuleSpec) -> EnvironmentCapsule {
    let id = spec.capsule_id;
    let sources = source_refs(spec);
    EnvironmentCapsule {
        record_kind: ENVIRONMENT_CAPSULE_RECORD_KIND.to_owned(),
        schema_version: ENVIRONMENT_CAPSULE_SCHEMA_VERSION,
        identity: CapsuleIdentity {
            capsule_id: id.to_owned(),
            capsule_version: 1,
            profile: spec.profile,
            label: spec.label.to_owned(),
            materialization_class: spec.materialization_class,
            transport: spec.transport,
            capsule_digest: dg(&format!("{id}:capsule")),
            summary: format!("Identity of the {} capsule.", spec.target_class.as_str()),
        },
        source_refs: sources,
        target_plan: TargetPlan {
            materialization_class: spec.materialization_class,
            transport: spec.transport,
            target_label: spec.target_label.to_owned(),
            host_boundary_ref: spec.host_boundary_ref.to_owned(),
            working_root_kind: spec.working_root_kind,
            coverage: EvidenceState::Current,
            summary: format!(
                "Materialization target plan for the {} capsule.",
                spec.target_label
            ),
        },
        service_graph: service_graph(),
        toolchain_plan: toolchain_plan(id),
        trust_hooks: trust_hooks(id),
        declared_env: vec![EnvVarBinding {
            name: "APP_ENV".to_owned(),
            value_digest: dg(&format!("{id}:env:APP_ENV")),
            source_id: "src.template".to_owned(),
        }],
        compatibility_fingerprint: compatibility_fingerprint(id),
        materialization: MaterializationStatus {
            materialization_class: spec.materialization_class,
            parity_state: EvidenceState::Current,
            aligned_surface_refs: vec![ENV_INSPECT_REF.to_owned(), SUPPORT_BUNDLE_REF.to_owned()],
            summary: "Materialization verified aligned with the capsule object across surfaces."
                .to_owned(),
        },
        observability: ObservabilityMetadata {
            capsule_event_stream_ref: format!("observability/env/{id}/lifecycle"),
            materialization_span_ref: format!("observability/env/{id}/materialization"),
            health_probe_refs: vec![
                format!("observability/env/{id}/health/svc.app"),
                format!("observability/env/{id}/health/svc.db"),
            ],
            redaction_class: RedactionClass::MetadataOnly,
            summary: "Observability references for the capsule lifecycle and materialization."
                .to_owned(),
        },
        claimed_maturity: spec.claimed_maturity,
        claimed_warm_start_posture: spec.claimed_warm_start_posture,
        why_this_environment: spec.why.to_owned(),
        notes: spec.notes.to_owned(),
    }
}

/// The canonical capsule objects this lane freezes, one per claimed
/// target class.
pub fn seeded_environment_capsules() -> Vec<EnvironmentCapsule> {
    SPECS.iter().map(base_capsule).collect()
}

fn spec_for(target_class: CapsuleTargetClass) -> &'static CapsuleSpec {
    SPECS
        .iter()
        .find(|spec| spec.target_class == target_class)
        .expect("every target class has a spec")
}

fn fixture(
    fixture_id: &str,
    target_class: CapsuleTargetClass,
    capsule: EnvironmentCapsule,
    consumer_ref: &str,
    notes: &str,
) -> EnvironmentCapsuleFixture {
    let inspection = inspect_environment(&capsule);
    // Re-derive through the engine so the recorded expectations can never
    // drift from the inspector.
    let outcome = certify_capsule_outcome(
        capsule.claimed_maturity,
        capsule.claimed_warm_start_posture,
        &capsule.dimension_evidence(),
    );
    debug_assert_eq!(outcome.verdict, inspection.verdict);
    EnvironmentCapsuleFixture {
        record_kind: ENVIRONMENT_CAPSULE_FIXTURE_RECORD_KIND.to_owned(),
        schema_version: ENVIRONMENT_CAPSULE_SCHEMA_VERSION,
        fixture_id: fixture_id.to_owned(),
        target_class,
        capsule,
        expected_verdict: inspection.verdict,
        expected_effective_maturity: inspection.effective_maturity,
        expected_warm_start_posture: inspection.effective_warm_start_posture,
        expected_narrow_reason_tokens: inspection.narrow_reason_tokens,
        expected_warm_start_downgrade_tokens: inspection.warm_start_downgrade_tokens,
        consumer_ref: consumer_ref.to_owned(),
        notes: notes.to_owned(),
    }
}

/// The checked-in fixture corpus: one certified capsule per target class
/// plus the degraded variants that drive the inspector's narrowing,
/// withholding, and warm-start downgrade.
pub fn seeded_environment_capsule_fixtures() -> Vec<EnvironmentCapsuleFixture> {
    let mut fixtures = Vec::new();

    for spec in SPECS {
        fixtures.push(fixture(
            &format!(
                "fixture.environment_capsule.{}_certified",
                spec.target_class.as_str()
            ),
            spec.target_class,
            base_capsule(spec),
            spec.consumer_ref,
            "A fully current capsule certifies at its claimed maturity and warm-start posture.",
        ));
    }

    // Non-local: a stale prebuild fingerprint narrows the claim and forces
    // a cold build instead of presenting a stale warm snapshot.
    let container = spec_for(CapsuleTargetClass::Container);
    let mut stale_fingerprint = base_capsule(container);
    stale_fingerprint.compatibility_fingerprint.coverage = EvidenceState::Stale;
    stale_fingerprint.compatibility_fingerprint.summary =
        "Fingerprint trails the current source digest; warm reuse is no longer trustworthy."
            .to_owned();
    fixtures.push(fixture(
        "fixture.environment_capsule.container_prebuild_fingerprint_stale",
        CapsuleTargetClass::Container,
        stale_fingerprint,
        container.consumer_ref,
        "A stale prebuild fingerprint narrows the beta container claim to preview and forces a cold build.",
    ));

    // Local: an ungated lifecycle hook withholds the capsule entirely.
    let local = spec_for(CapsuleTargetClass::Local);
    let mut ungated_hook = base_capsule(local);
    if let Some(hook) = ungated_hook.trust_hooks.first_mut() {
        hook.gate_state = TrustGateState::Ungated;
        hook.summary =
            "Lifecycle hook with no trust gate; running it would bypass the contract.".to_owned();
    }
    fixtures.push(fixture(
        "fixture.environment_capsule.local_trust_hook_ungated",
        CapsuleTargetClass::Local,
        ungated_hook,
        local.consumer_ref,
        "An ungated lifecycle hook withholds the local capsule claim rather than running silently.",
    ));

    // Non-local: a stale toolchain plan narrows the claim without touching
    // the warm-start posture (the toolchain plan does not govern warm reuse).
    let ssh = spec_for(CapsuleTargetClass::Ssh);
    let mut stale_toolchain = base_capsule(ssh);
    stale_toolchain.toolchain_plan.coverage = EvidenceState::Stale;
    stale_toolchain.toolchain_plan.summary =
        "Toolchain plan aged past its freshness window after the host base image rolled."
            .to_owned();
    fixtures.push(fixture(
        "fixture.environment_capsule.ssh_toolchain_stale",
        CapsuleTargetClass::Ssh,
        stale_toolchain,
        ssh.consumer_ref,
        "A stale toolchain plan narrows the SSH claim to preview while warm reuse stays partial.",
    ));

    fixtures
}
