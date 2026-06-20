//! The checked-in hook-review corpus this lane freezes: one canonical hook set
//! spanning the six M5 starter activators, the per-scenario packets the engine
//! reaches for each trust / policy context, the failure / recovery drills, and
//! the fixture corpus that pins every disposition, posture, and repair.

use crate::capsules::{CapsuleDigest, CapsuleTargetClass, LifecyclePhase, TrustGateState};
use crate::m5_env_governance::{DrillPhase, EnvironmentProfile};

use super::{
    export_hook_review, review_hooks, HookActivator, HookHoldReason, HookKind, HookReviewContext,
    HookReviewDrill, HookReviewDrillStep, HookReviewExport, HookReviewFixture, HookReviewPacket,
    HookReviewPosture, HookReviewScenario, LifecycleHook, HOOK_REVIEW_FIXTURE_RECORD_KIND,
    HOOK_REVIEW_SCHEMA_VERSION,
};

const EXECUTION_SCOPE_REF: &str = "artifacts/runtime/execution_scope_matrix.yaml";
const AUTHORITY_CLASSES_REF: &str = "artifacts/runtime/authority_classes.yaml";
const POLICY_REF: &str = "artifacts/policy/lifecycle_hook_policy.yaml";
const APPROVAL_LINEAGE_REF: &str = "artifacts/approvals/approval_lineage.yaml";
const SUPPORT_EXPORT_REF: &str = "crates/aureline-support/src/bundle/mod.rs";

const SHELL_CONSUMER: &str = "crates/aureline-shell/src/environment_inspector/mod.rs";
const SUPPORT_CONSUMER: &str = "crates/aureline-support/src/bundle/mod.rs";
const DOCTOR_CONSUMER: &str = "crates/aureline-doctor/src/repair/mod.rs";
const POLICY_CONSUMER: &str = "crates/aureline-policy/src/lifecycle_gate/mod.rs";

const REQUIRED_SECRET: &str = "DATABASE_URL";

/// Deterministic 64-hex digest from a label, mirroring the capsule seed so a
/// hook's command digest is stable and inspectable without a body.
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

#[allow(clippy::too_many_arguments)]
fn hook(
    hook_id: &str,
    activator: HookActivator,
    kind: HookKind,
    phase: LifecyclePhase,
    gate_state: TrustGateState,
    authority_ref: &str,
    required_secrets: &[&str],
    depends_on: &[&str],
    summary: &str,
) -> LifecycleHook {
    LifecycleHook {
        hook_id: hook_id.to_owned(),
        activator,
        kind,
        phase,
        gate_state,
        authority_ref: authority_ref.to_owned(),
        command_digest: dg(&format!("{hook_id}:command")),
        required_secrets: required_secrets.iter().map(|s| (*s).to_owned()).collect(),
        depends_on: depends_on.iter().map(|s| (*s).to_owned()).collect(),
        summary: summary.to_owned(),
    }
}

/// The canonical declared-hook set: one representative hook per claimed M5
/// starter activator, all trust-gated, with one post-create hook that depends
/// on the bootstrap action and one that requires a secret.
pub fn seeded_lifecycle_hooks() -> Vec<LifecycleHook> {
    vec![
        hook(
            "hook.preflight",
            HookActivator::Bootstrap,
            HookKind::PreflightValidator,
            LifecyclePhase::OnCreate,
            TrustGateState::Gated,
            EXECUTION_SCOPE_REF,
            &[],
            &[],
            "Preflight validator that checks the workspace before setup runs.",
        ),
        hook(
            "hook.bootstrap",
            HookActivator::Bootstrap,
            HookKind::BuildSetupCommand,
            LifecyclePhase::OnCreate,
            TrustGateState::Gated,
            EXECUTION_SCOPE_REF,
            &[],
            &[],
            "Repo bootstrap / setup command that materializes the toolchain.",
        ),
        hook(
            "hook.devcontainer_post_create",
            HookActivator::Devcontainer,
            HookKind::LifecycleHook,
            LifecyclePhase::PostCreate,
            TrustGateState::Gated,
            AUTHORITY_CLASSES_REF,
            &[],
            &["hook.bootstrap"],
            "Devcontainer post-create hook that runs after the bootstrap action.",
        ),
        hook(
            "hook.compose_up",
            HookActivator::Compose,
            HookKind::BuildSetupCommand,
            LifecyclePhase::OnStart,
            TrustGateState::Gated,
            EXECUTION_SCOPE_REF,
            &[],
            &[],
            "Compose service start command for the backing service graph.",
        ),
        hook(
            "hook.nix_activate",
            HookActivator::Nix,
            HookKind::LifecycleHook,
            LifecyclePhase::OnStart,
            TrustGateState::Gated,
            AUTHORITY_CLASSES_REF,
            &[],
            &[],
            "Nix flake shell activation for the pinned toolchain.",
        ),
        hook(
            "hook.direnv_load",
            HookActivator::Direnv,
            HookKind::LifecycleHook,
            LifecyclePhase::OnAttach,
            TrustGateState::Gated,
            AUTHORITY_CLASSES_REF,
            &[],
            &[],
            "direnv environment load on attach.",
        ),
        hook(
            "hook.post_create_seed",
            HookActivator::PostCreate,
            HookKind::LifecycleHook,
            LifecyclePhase::PostStart,
            TrustGateState::Gated,
            AUTHORITY_CLASSES_REF,
            &[REQUIRED_SECRET],
            &[],
            "Post-create data seed that needs the database connection secret.",
        ),
    ]
}

/// The baseline trusted, unrestricted context: every activator supported,
/// every secret projected, nothing denied or failed. Reviewed against this,
/// every seeded hook is cleared.
fn baseline_context(
    profile: EnvironmentProfile,
    target_class: CapsuleTargetClass,
) -> HookReviewContext {
    HookReviewContext {
        profile,
        target_class,
        trusted: true,
        restricted_mode: false,
        supported_activators: HookActivator::ALL.to_vec(),
        denied_activators: Vec::new(),
        denied_hook_kinds: Vec::new(),
        projected_secrets: vec![REQUIRED_SECRET.to_owned()],
        failed_actions: Vec::new(),
        policy_ref: POLICY_REF.to_owned(),
        approval_lineage_ref: APPROVAL_LINEAGE_REF.to_owned(),
        support_export_ref: SUPPORT_EXPORT_REF.to_owned(),
    }
}

/// The hooks and context one scenario reviews.
fn scenario_inputs(
    scenario: HookReviewScenario,
    profile: EnvironmentProfile,
    target_class: CapsuleTargetClass,
) -> (Vec<LifecycleHook>, HookReviewContext) {
    let mut hooks = seeded_lifecycle_hooks();
    let mut context = baseline_context(profile, target_class);
    match scenario {
        HookReviewScenario::AllCleared => {}
        HookReviewScenario::ReviewRequired => {
            context.trusted = false;
        }
        HookReviewScenario::Ungated => {
            if let Some(target) = hooks.iter_mut().find(|h| h.hook_id == "hook.nix_activate") {
                target.gate_state = TrustGateState::Ungated;
            }
        }
        HookReviewScenario::Restricted => {
            context.restricted_mode = true;
        }
        HookReviewScenario::PolicyDenied => {
            context.denied_activators = vec![HookActivator::Nix];
        }
        HookReviewScenario::MissingSecret => {
            context.projected_secrets = Vec::new();
        }
        HookReviewScenario::UnsupportedActivator => {
            context.supported_activators = HookActivator::ALL
                .into_iter()
                .filter(|a| *a != HookActivator::Nix)
                .collect();
        }
        HookReviewScenario::BootstrapFailed | HookReviewScenario::BlockedPostCreate => {
            context.failed_actions = vec!["hook.bootstrap".to_owned()];
        }
        HookReviewScenario::FullyBlocked => {
            context.denied_activators = HookActivator::ALL.to_vec();
        }
    }
    (hooks, context)
}

/// The per-scenario packets the engine reaches, one per fixture scenario.
pub fn seeded_hook_review_packets() -> Vec<HookReviewPacket> {
    fixture_specs()
        .into_iter()
        .map(|(_, scenario, profile, target_class, _, _)| {
            let (hooks, context) = scenario_inputs(scenario, profile, target_class);
            review_hooks(&hooks, &context)
        })
        .collect()
}

#[allow(clippy::type_complexity)]
fn fixture_specs() -> Vec<(
    &'static str,
    HookReviewScenario,
    EnvironmentProfile,
    CapsuleTargetClass,
    &'static str,
    &'static str,
)> {
    vec![
        (
            "hook_review_all_cleared",
            HookReviewScenario::AllCleared,
            EnvironmentProfile::Starter,
            CapsuleTargetClass::Local,
            SHELL_CONSUMER,
            "Every trust-gated hook on a trusted, unrestricted starter path is cleared to run.",
        ),
        (
            "hook_review_review_required",
            HookReviewScenario::ReviewRequired,
            EnvironmentProfile::WorkspaceTemplate,
            CapsuleTargetClass::Container,
            SHELL_CONSUMER,
            "An untrusted workspace holds every hook for review instead of running it.",
        ),
        (
            "hook_review_ungated_hook",
            HookReviewScenario::Ungated,
            EnvironmentProfile::Devcontainer,
            CapsuleTargetClass::Devcontainer,
            POLICY_CONSUMER,
            "An ungated hook is held for review and never run, even on a trusted path.",
        ),
        (
            "hook_review_restricted_mode",
            HookReviewScenario::Restricted,
            EnvironmentProfile::RemoteContainer,
            CapsuleTargetClass::Ssh,
            SHELL_CONSUMER,
            "Restricted mode turns every hook into a visible manual suggestion with a safe next step.",
        ),
        (
            "hook_review_policy_denied",
            HookReviewScenario::PolicyDenied,
            EnvironmentProfile::ManagedWorkspace,
            CapsuleTargetClass::ManagedWorkspace,
            POLICY_CONSUMER,
            "A policy-denied activator is surfaced with a deny reason while the safe subset still runs.",
        ),
        (
            "hook_review_missing_secret",
            HookReviewScenario::MissingSecret,
            EnvironmentProfile::Prebuild,
            CapsuleTargetClass::Container,
            DOCTOR_CONSUMER,
            "A hook that needs a missing secret is blocked and offered a provide-secret repair.",
        ),
        (
            "hook_review_unsupported_activator",
            HookReviewScenario::UnsupportedActivator,
            EnvironmentProfile::Devcontainer,
            CapsuleTargetClass::Vm,
            DOCTOR_CONSUMER,
            "An unsupported activator is blocked and named rather than disappearing as a no-op.",
        ),
        (
            "hook_review_bootstrap_failed",
            HookReviewScenario::BootstrapFailed,
            EnvironmentProfile::Starter,
            CapsuleTargetClass::Container,
            DOCTOR_CONSUMER,
            "A failed bootstrap action blocks itself and the post-create hook that depends on it.",
        ),
        (
            "hook_review_fully_blocked",
            HookReviewScenario::FullyBlocked,
            EnvironmentProfile::RemoteContainer,
            CapsuleTargetClass::Container,
            SUPPORT_CONSUMER,
            "When policy denies every activator no hook runs, and each carries a reason and a repair.",
        ),
    ]
}

/// The checked-in fixture corpus: one fixture per scenario, each embedding the
/// canonical packet the engine reaches so the recorded expectations can never
/// drift from the review.
pub fn seeded_hook_review_fixtures() -> Vec<HookReviewFixture> {
    fixture_specs()
        .into_iter()
        .map(
            |(fixture_id, scenario, profile, target_class, consumer_ref, notes)| {
                let (hooks, context) = scenario_inputs(scenario, profile, target_class);
                let packet = review_hooks(&hooks, &context);
                HookReviewFixture {
                    record_kind: HOOK_REVIEW_FIXTURE_RECORD_KIND.to_owned(),
                    schema_version: HOOK_REVIEW_SCHEMA_VERSION,
                    fixture_id: fixture_id.to_owned(),
                    profile,
                    scenario,
                    hooks,
                    context,
                    expected_posture: packet.posture,
                    expected_runnable_hooks: packet.runnable_hooks,
                    expected_reason_tokens: packet.reason_tokens.clone(),
                    expected_denied_hook_tokens: packet.denied_hook_tokens.clone(),
                    expected_blocked_hook_tokens: packet.blocked_hook_tokens.clone(),
                    packet,
                    consumer_ref: consumer_ref.to_owned(),
                    notes: notes.to_owned(),
                }
            },
        )
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn drill(
    drill_id: &str,
    title: &str,
    profile: EnvironmentProfile,
    target_class: CapsuleTargetClass,
    scenario: HookReviewScenario,
    hook_id: &str,
    injected_reason: HookHoldReason,
    notes: &str,
) -> HookReviewDrill {
    let (hooks, degraded_context) = scenario_inputs(scenario, profile, target_class);
    let degraded = review_hooks(&hooks, &degraded_context);
    let recovered = review_hooks(&hooks, &baseline_context(profile, target_class));
    let expected_disposition = injected_reason.disposition();
    let expected_repair_kind = injected_reason.repair_kind();
    let steps = vec![
        HookReviewDrillStep {
            phase: DrillPhase::Inject,
            observed_posture: HookReviewPosture::AllCleared,
            narration: format!(
                "Before the failure the {hook_id} hook is trust-gated and cleared on the {} path.",
                profile.as_str()
            ),
        },
        HookReviewDrillStep {
            phase: DrillPhase::Observe,
            observed_posture: degraded.posture,
            narration: format!(
                "The injected {} failure holds the {hook_id} hook as {} instead of running it.",
                injected_reason.as_str(),
                expected_disposition.as_str()
            ),
        },
        HookReviewDrillStep {
            phase: DrillPhase::Narrow,
            observed_posture: degraded.posture,
            narration: format!(
                "The review narrows to {} and surfaces the reason and a {} repair.",
                degraded.posture.as_str(),
                expected_repair_kind.as_str()
            ),
        },
        HookReviewDrillStep {
            phase: DrillPhase::Refresh,
            observed_posture: degraded.posture,
            narration: format!(
                "The {} repair is applied to the {hook_id} hook through the approval lineage.",
                expected_repair_kind.as_str()
            ),
        },
        HookReviewDrillStep {
            phase: DrillPhase::Recover,
            observed_posture: recovered.posture,
            narration: format!("With the failure repaired the {hook_id} hook is cleared again."),
        },
        HookReviewDrillStep {
            phase: DrillPhase::Verify,
            observed_posture: recovered.posture,
            narration: "The engine re-derives an all-cleared review, proving the recovery."
                .to_owned(),
        },
    ];
    HookReviewDrill {
        drill_id: drill_id.to_owned(),
        title: title.to_owned(),
        profile,
        hook_id: hook_id.to_owned(),
        injected_reason,
        expected_disposition,
        degraded_posture: degraded.posture,
        expected_repair_kind,
        recovers_to_posture: recovered.posture,
        steps,
        asserts_no_silent_execution: true,
        asserts_reason_and_next_step_preserved: true,
        asserts_recovers_after_repair: true,
        notes: notes.to_owned(),
    }
}

/// The failure / recovery drills this lane freezes: a blocked post-create hook,
/// a failed bootstrap action, a missing secret, an unsupported activator, and a
/// policy-denied lifecycle step, each walking from injection through a visible
/// held state and a repair back to a cleared review.
pub fn seeded_hook_review_drills() -> Vec<HookReviewDrill> {
    vec![
        drill(
            "drill.blocked_post_create",
            "Blocked post-create hook recovers after the upstream is repaired",
            EnvironmentProfile::Devcontainer,
            CapsuleTargetClass::Devcontainer,
            HookReviewScenario::BlockedPostCreate,
            "hook.devcontainer_post_create",
            HookHoldReason::UpstreamBlocked,
            "A post-create hook whose bootstrap dependency failed is blocked, not silently skipped.",
        ),
        drill(
            "drill.failed_bootstrap",
            "Failed bootstrap action is held for repair",
            EnvironmentProfile::Starter,
            CapsuleTargetClass::Container,
            HookReviewScenario::BootstrapFailed,
            "hook.bootstrap",
            HookHoldReason::BootstrapFailed,
            "A failed bootstrap action is surfaced with a retry repair instead of a silent failure.",
        ),
        drill(
            "drill.missing_secret",
            "Missing secret blocks a hook until it is provided",
            EnvironmentProfile::Prebuild,
            CapsuleTargetClass::Container,
            HookReviewScenario::MissingSecret,
            "hook.post_create_seed",
            HookHoldReason::MissingSecret,
            "A hook that needs an unprojected secret is blocked and offered a provide-secret repair.",
        ),
        drill(
            "drill.unsupported_activator",
            "Unsupported activator is named rather than dropped",
            EnvironmentProfile::Devcontainer,
            CapsuleTargetClass::Vm,
            HookReviewScenario::UnsupportedActivator,
            "hook.nix_activate",
            HookHoldReason::UnsupportedActivator,
            "An activator the target does not support is blocked and named, not a confusing no-op.",
        ),
        drill(
            "drill.policy_denied",
            "Policy-denied lifecycle step is surfaced with a deny reason",
            EnvironmentProfile::ManagedWorkspace,
            CapsuleTargetClass::ManagedWorkspace,
            HookReviewScenario::PolicyDenied,
            "hook.nix_activate",
            HookHoldReason::PolicyDenied,
            "A policy-denied lifecycle step is surfaced with a deny reason and a policy-exception repair.",
        ),
    ]
}

/// The metadata-first exports for the seeded packets, one per scenario.
pub fn seeded_hook_review_exports() -> Vec<HookReviewExport> {
    seeded_hook_review_packets()
        .iter()
        .map(export_hook_review)
        .collect()
}
