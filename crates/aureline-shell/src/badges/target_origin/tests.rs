use std::path::Path;

use aureline_auth::{
    AccountBoundaryClass, BrowserCallbackHandoff, IdentityModeAlias, PreservedLocalWork,
    PreservedLocalWorkPostureClass, RetryPathClass, ReturnModeClass, ReturnOriginValidationClass,
    ReturnTenantOrWorkspaceMatchRule, StageAccountFreeLocalRequest,
    StageSystemBrowserHandoffRequest, TrustState as AuthTrustState,
};
use aureline_runtime::{
    ActorClass, CapsuleDriftState, EnvironmentCapsuleRef, ExecutionContextRequest,
    ExecutionContextResolver, ExecutionContextResolverConfig, IdentityMode, ScopeClass,
    SurfaceClass, TargetClass, ToolchainClass, TrustState,
};

use serde::Deserialize;

use super::*;

fn baseline_resolver() -> ExecutionContextResolver {
    ExecutionContextResolver::new(ExecutionContextResolverConfig {
        workspace_id: "ws-test".to_owned(),
        profile_id: Some("prof.default".to_owned()),
        identity_mode: IdentityMode::AccountFreeLocal,
        policy_epoch: 1,
        workspace_default_target_class: TargetClass::LocalHost,
        workspace_default_working_directory: Some("/workspace".to_owned()),
        workspace_default_scope_class: ScopeClass::CurrentRoot,
        local_host_canonical_id: "localhost:darwin-arm64".to_owned(),
        environment_capsule_ref: EnvironmentCapsuleRef {
            capsule_id: "caps:ws-test:seed".to_owned(),
            capsule_hash: "sha256:seed".to_owned(),
            resolved_schema_version: "1".to_owned(),
            drift_state: CapsuleDriftState::InSync,
        },
        resolver_version: "seed-0".to_owned(),
    })
}

fn local_only_packet() -> aureline_auth::BrowserCallbackPacket {
    BrowserCallbackHandoff::stage_account_free_local(StageAccountFreeLocalRequest {
        packet_id: "browser_callback_packet.account_free_local.test",
        correlation_id: "callback_correlation.account_free_local.test",
        pending_session_id: "pending_session.account_free_local.test",
        provider_domain_label: "No sign-in required",
        destination_class_label: "No browser handoff required",
        return_anchor_ref: "return_anchor.account_free_local.desktop",
        return_target_label: "Aureline desktop – local workspace",
        minted_at: "2026-04-23T10:00:00Z",
        recovery_copy_label: "Local work stays on this device.",
        execution_context_ref: Some("execution_context.local_desktop.workspace_root"),
    })
    .into_packet()
}

fn managed_handoff_packet() -> aureline_auth::BrowserCallbackPacket {
    BrowserCallbackHandoff::stage_system_browser_handoff(StageSystemBrowserHandoffRequest {
        packet_id: "browser_callback_packet.managed_sign_in.test",
        identity_mode: IdentityModeAlias::ManagedConvenience,
        account_boundary_class: AccountBoundaryClass::Managed,
        trust_state: AuthTrustState::Trusted,
        provider_domain_label: "login.acme.example",
        destination_class_label: "Customer-managed identity provider (system browser)",
        return_target_label: "Aureline desktop – payments-prod workspace",
        return_anchor_ref: "return_anchor.managed_sign_in.payments_prod",
        return_mode_class: ReturnModeClass::LoopbackHttpReturn,
        return_origin_validation_class: ReturnOriginValidationClass::LoopbackPortPinned,
        return_tenant_or_workspace_match_rule:
            ReturnTenantOrWorkspaceMatchRule::MustMatchBoundWorkspaceAndTenant,
        return_policy_check_refs: &[],
        bound_workspace_ref: Some("workspace.payments_prod"),
        bound_tenant_or_org_ref: Some("tenant.acme_prod"),
        bound_actor_subject_ref: Some("actor_subject.sam.acme"),
        correlation_id: "callback_correlation.managed_sign_in.test",
        pending_session_id: "pending_session.managed_sign_in.test",
        state_token_alias: "state_alias.managed_sign_in.test",
        nonce_alias: "nonce_alias.managed_sign_in.test",
        pkce_challenge_alias: Some("pkce_alias.managed_sign_in.test"),
        issued_at: "2026-04-23T10:10:00Z",
        expires_at: "2026-04-23T10:20:00Z",
        recovery_copy_label: "Continue sign-in in your browser.",
        primary_recovery_action: RetryPathClass::RetryInSystemBrowser,
        fallback_recovery_actions: &[
            RetryPathClass::SwitchToDeviceCode,
            RetryPathClass::ContinueLocalWithoutSignIn,
        ],
        repair_hook_ref: None,
        preserved_local_work: PreservedLocalWork {
            posture_class: PreservedLocalWorkPostureClass::LocalWorkIntactWithManagedNarrowed,
            note: "Local work intact while managed sign-in is incomplete.".to_owned(),
            retained_capabilities: vec!["Edit, save, undo, search locally.".to_owned()],
            blocked_capabilities: vec![
                "Fetch managed settings sync while sign-in is incomplete.".to_owned(),
            ],
        },
        execution_context_ref: Some("execution_context.auth.managed_sign_in.payments_prod"),
    })
    .expect("managed handoff stages cleanly")
    .into_packet()
}

#[test]
fn target_class_mirror_covers_every_runtime_variant() {
    // Locking the mirror prevents the badge enum from drifting when the
    // upstream target taxonomy widens.
    let mirrored = [
        (TargetClass::LocalHost, TargetBadgeClass::LocalDesktop),
        (TargetClass::SshRemote, TargetBadgeClass::RemoteHost),
        (TargetClass::ContainerLocal, TargetBadgeClass::LocalContainer),
        (TargetClass::Devcontainer, TargetBadgeClass::Devcontainer),
        (
            TargetClass::RemoteWorkspaceVm,
            TargetBadgeClass::RemoteWorkspaceVm,
        ),
        (TargetClass::PrebuildRuntime, TargetBadgeClass::PrebuildRuntime),
        (
            TargetClass::ManagedWorkspace,
            TargetBadgeClass::ManagedWorkspace,
        ),
        (
            TargetClass::NotebookKernelLocal,
            TargetBadgeClass::NotebookKernelLocal,
        ),
        (
            TargetClass::NotebookKernelRemote,
            TargetBadgeClass::NotebookKernelRemote,
        ),
        (TargetClass::AiSandbox, TargetBadgeClass::AiSandbox),
    ];
    for (runtime, badge) in mirrored {
        assert_eq!(TargetBadgeClass::from_target_class(runtime), badge);
    }
}

#[test]
fn protected_walk_local_seed_renders_hidden_boundary_on_every_entry_point() {
    // Protected walk: open a terminal session against a trusted local desktop
    // seed and project the badge set. Every execution-entry badge must agree
    // on Local target, Local-only origin, and a Hidden boundary cue.
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::local_terminal_seed(
        "terminal.open",
        TrustState::Trusted,
        "mono:0",
    ));

    let set = TargetOriginBadgeSet::project(&context);
    assert_eq!(set.record_kind, TARGET_ORIGIN_BADGE_SET_RECORD_KIND);
    assert_eq!(set.schema_version, TARGET_ORIGIN_BADGE_SCHEMA_VERSION);
    assert_eq!(set.workspace_id, "ws-test");
    assert_eq!(set.execution_context_ref, context.execution_context_id);

    for badge in [
        &set.terminal_badge,
        &set.task_seed_badge,
        &set.debug_prep_badge,
    ] {
        assert_eq!(badge.target_class, TargetBadgeClass::LocalDesktop);
        assert_eq!(badge.target_label, "Local");
        assert_eq!(badge.target_class_token, "local_desktop");
        assert_eq!(badge.origin_class, OriginBadgeClass::AccountFreeLocal);
        assert_eq!(badge.origin_label, "Local only");
        assert_eq!(badge.boundary_cue, HostBoundaryCue::Hidden);
        assert!(!badge.boundary_cue_visible);
        assert!(!badge.honesty_marker_present);
        assert_eq!(badge.trust_state_token, "trusted");
        assert_eq!(badge.execution_context_ref, context.execution_context_id);
    }

    assert!(set.execution_entries_consistent());
    assert!(!set.any_honesty_marker());
    assert!(set.provider_auth_badge.is_none());
}

#[test]
fn provider_entry_on_local_only_packet_keeps_boundary_hidden_but_quotes_packet() {
    // Provider entries on the no-account local path MUST carry the
    // Local-only origin and the Hidden boundary cue. The badge still records
    // the auth packet ref so a support export can correlate the chip back to
    // the canonical seed packet.
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::local_terminal_seed(
        "terminal.open",
        TrustState::Trusted,
        "mono:0",
    ));
    let packet = local_only_packet();

    let set = TargetOriginBadgeSet::project(&context).with_provider_packet(&context, &packet);
    let provider = set
        .provider_auth_badge
        .as_ref()
        .expect("provider badge attached");
    assert_eq!(provider.entry_point, BadgeEntryPoint::ProviderAuthEntry);
    assert_eq!(provider.target_class, TargetBadgeClass::ProviderEntryPoint);
    assert_eq!(provider.origin_class, OriginBadgeClass::AccountFreeLocal);
    assert_eq!(provider.boundary_cue, HostBoundaryCue::Hidden);
    assert!(!provider.boundary_cue_visible);
    assert_eq!(
        provider.auth_packet_ref.as_deref(),
        Some("browser_callback_packet.account_free_local.test")
    );
    assert_eq!(provider.canonical_target_id, format!("provider:{}", packet.packet_id));
    assert!(!provider.honesty_marker_present);
}

#[test]
fn failure_drill_remote_target_lights_local_to_remote_consistently() {
    // Failure drill: enter the lane from a context that targets a remote SSH
    // host. The terminal, task seed, and debug-prep seed badges MUST all
    // light the same `local_to_remote` boundary cue without one surface
    // collapsing back to a stale Hidden label.
    let mut resolver = baseline_resolver();
    let mut request = ExecutionContextRequest::local_terminal_seed(
        "terminal.open",
        TrustState::Trusted,
        "mono:0",
    );
    request.override_target_class = Some(TargetClass::SshRemote);
    request.override_working_directory = Some("/srv/code");
    let context = resolver.resolve(request);

    let set = TargetOriginBadgeSet::project(&context);
    for badge in [
        &set.terminal_badge,
        &set.task_seed_badge,
        &set.debug_prep_badge,
    ] {
        assert_eq!(badge.target_class, TargetBadgeClass::RemoteHost);
        assert_eq!(badge.target_label, "Remote");
        assert_eq!(badge.boundary_cue, HostBoundaryCue::LocalToRemote);
        assert!(badge.boundary_cue_visible);
        assert!(!badge.honesty_marker_present);
    }
    assert!(set.execution_entries_consistent());
}

#[test]
fn pending_trust_lights_degraded_trust_cue_and_honesty_marker_on_every_badge() {
    // Honesty drill: the resolver settles a local target but the workspace
    // trust posture is unresolved. Every badge MUST carry the
    // `degraded_trust` cue and an honesty marker so the chrome can never
    // render a stale "all clear" Local chip.
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::local_terminal_seed(
        "terminal.open",
        TrustState::PendingEvaluation,
        "mono:0",
    ));

    let set = TargetOriginBadgeSet::project(&context);
    for badge in [
        &set.terminal_badge,
        &set.task_seed_badge,
        &set.debug_prep_badge,
    ] {
        assert_eq!(badge.target_class, TargetBadgeClass::LocalDesktop);
        assert_eq!(badge.boundary_cue, HostBoundaryCue::DegradedTrust);
        assert!(badge.boundary_cue_visible);
        assert!(badge.honesty_marker_present);
        assert_eq!(badge.trust_state_token, "pending_evaluation");
    }
    assert!(set.execution_entries_consistent());
    assert!(set.any_honesty_marker());
}

#[test]
fn managed_provider_entry_on_local_target_lights_local_to_provider_cue() {
    // The execution target is local but the provider/auth chip crosses the
    // managed tenant boundary. The provider badge MUST surface the
    // `local_to_provider` cue while the execution-entry badges keep their
    // local Hidden cue.
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::local_terminal_seed(
        "terminal.open",
        TrustState::Trusted,
        "mono:0",
    ));
    let packet = managed_handoff_packet();

    let set = TargetOriginBadgeSet::project(&context).with_provider_packet(&context, &packet);
    let provider = set
        .provider_auth_badge
        .as_ref()
        .expect("provider badge attached");
    assert_eq!(provider.target_class, TargetBadgeClass::ProviderEntryPoint);
    assert_eq!(provider.origin_class, OriginBadgeClass::Managed);
    assert_eq!(provider.boundary_cue, HostBoundaryCue::LocalToProvider);
    assert!(provider.boundary_cue_visible);

    // Execution-entry badges stay local — they must not pull the provider
    // tenant boundary cue onto the terminal/task/debug-prep lanes.
    assert_eq!(set.terminal_badge.boundary_cue, HostBoundaryCue::Hidden);
    assert_eq!(set.task_seed_badge.boundary_cue, HostBoundaryCue::Hidden);
    assert_eq!(set.debug_prep_badge.boundary_cue, HostBoundaryCue::Hidden);
    assert!(set.execution_entries_consistent());
}

#[test]
fn project_for_each_entry_point_records_the_correct_entry_tag() {
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::local_terminal_seed(
        "terminal.open",
        TrustState::Trusted,
        "mono:0",
    ));

    for entry in [
        BadgeEntryPoint::Terminal,
        BadgeEntryPoint::TaskSeed,
        BadgeEntryPoint::DebugPrepSeed,
    ] {
        let badge = TargetOriginBadge::project(entry, &context);
        assert_eq!(badge.entry_point, entry);
        assert_eq!(badge.record_kind, TARGET_ORIGIN_BADGE_RECORD_KIND);
    }
}

#[test]
fn boundary_cue_helper_is_visible_on_every_non_hidden_variant() {
    for cue in [
        HostBoundaryCue::Hidden,
        HostBoundaryCue::LocalToContainer,
        HostBoundaryCue::LocalToRemote,
        HostBoundaryCue::LocalToManaged,
        HostBoundaryCue::LocalToProvider,
        HostBoundaryCue::DegradedTrust,
        HostBoundaryCue::PolicyBlocked,
        HostBoundaryCue::Unknown,
    ] {
        let visible = !matches!(cue, HostBoundaryCue::Hidden);
        assert_eq!(cue.is_visible(), visible);
    }
}

#[test]
fn fixture_protected_walk_replays_into_the_badge_projection() {
    let fixture: BadgeFixture =
        load_fixture("local_terminal_protected_walk.json");
    let set = build_set_from_fixture(&fixture);
    assert_eq!(
        set.execution_entries_consistent(),
        fixture.expect.execution_entries_consistent
    );
    assert_eq!(set.any_honesty_marker(), fixture.expect.any_honesty_marker);
    assert_badge_matches(&set.terminal_badge, &fixture.expect.terminal);
    assert_badge_matches(&set.task_seed_badge, &fixture.expect.task_seed);
    assert_badge_matches(&set.debug_prep_badge, &fixture.expect.debug_prep_seed);
}

#[test]
fn fixture_failure_drill_replays_consistent_remote_boundary_cue() {
    let fixture: BadgeFixture = load_fixture("remote_target_failure_drill.json");
    let set = build_set_from_fixture(&fixture);
    assert_eq!(
        set.execution_entries_consistent(),
        fixture.expect.execution_entries_consistent
    );
    assert_eq!(set.any_honesty_marker(), fixture.expect.any_honesty_marker);
    assert_badge_matches(&set.terminal_badge, &fixture.expect.terminal);
    assert_badge_matches(&set.task_seed_badge, &fixture.expect.task_seed);
    assert_badge_matches(&set.debug_prep_badge, &fixture.expect.debug_prep_seed);
}

#[test]
fn fixture_pending_trust_replays_honesty_marker_on_every_badge() {
    let fixture: BadgeFixture = load_fixture("pending_trust_honesty_drill.json");
    let set = build_set_from_fixture(&fixture);
    assert_eq!(
        set.execution_entries_consistent(),
        fixture.expect.execution_entries_consistent
    );
    assert_eq!(set.any_honesty_marker(), fixture.expect.any_honesty_marker);
    assert_badge_matches(&set.terminal_badge, &fixture.expect.terminal);
    assert_badge_matches(&set.task_seed_badge, &fixture.expect.task_seed);
    assert_badge_matches(&set.debug_prep_badge, &fixture.expect.debug_prep_seed);
}

fn load_fixture(name: &str) -> BadgeFixture {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/runtime/target_origin_cases")
        .join(name);
    let payload = std::fs::read_to_string(&path).expect("fixture must read");
    serde_json::from_str(&payload).expect("fixture must parse")
}

fn build_set_from_fixture(fixture: &BadgeFixture) -> TargetOriginBadgeSet {
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest {
        command_id: &fixture.input.command_id,
        surface: fixture.input.surface,
        actor_class: fixture.input.actor_class,
        trust_state: fixture.input.trust_state,
        observed_at: &fixture.input.observed_at,
        requested_target_class: fixture.input.requested_target_class,
        requested_working_directory: fixture.input.requested_working_directory.as_deref(),
        requested_toolchain_class: fixture.input.requested_toolchain_class,
        override_target_class: fixture.input.override_target_class,
        override_working_directory: fixture.input.override_working_directory.as_deref(),
        override_toolchain_class: fixture.input.override_toolchain_class,
    });
    TargetOriginBadgeSet::project(&context)
}

fn assert_badge_matches(badge: &TargetOriginBadge, expect: &BadgeExpect) {
    assert_eq!(badge.target_class_token, expect.target_class);
    assert_eq!(badge.target_label, expect.target_label);
    assert_eq!(badge.origin_class_token, expect.origin_class);
    assert_eq!(badge.origin_label, expect.origin_label);
    assert_eq!(badge.boundary_cue_token, expect.boundary_cue);
    assert_eq!(badge.boundary_cue_visible, expect.boundary_cue_visible);
}

#[derive(Debug, Deserialize)]
struct BadgeFixture {
    input: FixtureInput,
    expect: FixtureExpect,
}

#[derive(Debug, Deserialize)]
struct FixtureInput {
    command_id: String,
    surface: SurfaceClass,
    actor_class: ActorClass,
    trust_state: TrustState,
    observed_at: String,
    #[serde(default)]
    requested_target_class: Option<TargetClass>,
    #[serde(default)]
    requested_working_directory: Option<String>,
    #[serde(default)]
    requested_toolchain_class: Option<ToolchainClass>,
    #[serde(default)]
    override_target_class: Option<TargetClass>,
    #[serde(default)]
    override_working_directory: Option<String>,
    #[serde(default)]
    override_toolchain_class: Option<ToolchainClass>,
}

#[derive(Debug, Deserialize)]
struct FixtureExpect {
    execution_entries_consistent: bool,
    any_honesty_marker: bool,
    terminal: BadgeExpect,
    task_seed: BadgeExpect,
    debug_prep_seed: BadgeExpect,
}

#[derive(Debug, Deserialize)]
struct BadgeExpect {
    target_class: String,
    target_label: String,
    origin_class: String,
    origin_label: String,
    boundary_cue: String,
    boundary_cue_visible: bool,
}
