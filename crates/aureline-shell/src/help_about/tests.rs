use std::path::Path;

use aureline_build_info::BuildIdentityRecord;
use aureline_runtime::{
    CapsuleDriftState, EnvironmentCapsuleRef, ExecutionContextRequest, ExecutionContextResolver,
    ExecutionContextResolverConfig, IdentityMode, ScopeClass, TargetClass, TrustState,
};
use serde::Deserialize;

use super::*;
use crate::badges::target_origin::{HostBoundaryCue, TargetBadgeClass};
use crate::embedded::boundary_card::{FreshnessClass, SourceClass, SourceTruthRecord, VersionMatchState};

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

fn fixture_build_identity() -> BuildIdentityRecord {
    BuildIdentityRecord {
        schema_version: 1,
        commit: "0123456789abcdef0123456789abcdef01234567".to_owned(),
        commit_short: "0123456".to_owned(),
        dirty: false,
        toolchain_channel: "stable".to_owned(),
        rustc_version: "rustc 1.78.0".to_owned(),
        cargo_version: "cargo 1.78.0".to_owned(),
        host_triple: "aarch64-apple-darwin".to_owned(),
        target_triple: "aarch64-apple-darwin".to_owned(),
        profile: "release".to_owned(),
        workspace_version: "0.0.0".to_owned(),
        source_date_epoch: 1_714_492_800,
        build_timestamp_utc: "2024-04-30T12:00:00Z".to_owned(),
    }
}

#[test]
fn protected_walk_local_seed_renders_live_actions_without_honesty_marker() {
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::local_terminal_seed(
        "terminal.open",
        TrustState::Trusted,
        "mono:0",
    ));
    let identity = fixture_build_identity();
    let docs_truth = SourceTruthRecord {
        source_class: SourceClass::ProjectDocs,
        version_match_state: VersionMatchState::ExactBuildMatch,
        freshness_class: FreshnessClass::AuthoritativeLive,
        running_build_identity_ref: "build-id:aureline:dev:0.0.0:aarch64:dev:0123456".to_owned(),
        help_status_badge_ref: None,
        snapshot_age_label: Some("just now".to_owned()),
    };

    let surface = HelpAboutSurface::project(HelpAboutInputs {
        build_identity: &identity,
        release_channel_class_token: "stable",
        execution_context: Some(&context),
        docs_source_truth: Some(&docs_truth),
    });

    assert_eq!(surface.record_kind, HELP_ABOUT_SURFACE_RECORD_KIND);
    assert_eq!(surface.schema_version, HELP_ABOUT_SURFACE_SCHEMA_VERSION);
    assert_eq!(surface.workspace_id.as_deref(), Some("ws-test"));
    assert_eq!(
        surface.execution_context_ref.as_deref(),
        Some(context.execution_context_id.as_str())
    );
    assert!(!surface.honesty_marker_present);

    // Build identity quotes the build-info record verbatim.
    assert_eq!(surface.build_identity.workspace_version, "0.0.0");
    assert_eq!(surface.build_identity.commit_short, "0123456");
    assert_eq!(
        surface.build_identity.tree_state_class,
        TreeStateClass::CleanCheckout
    );
    assert_eq!(
        surface.build_identity.tree_state_class_token,
        "clean_checkout"
    );

    // Install mode resolves to the stable channel.
    assert_eq!(
        surface.install_mode.install_mode_class,
        InstallModeClass::StableLocalInstall
    );
    assert!(!surface.install_mode.honesty_marker_present);

    // Client-scope chip mirrors the shared target/origin badge vocabulary.
    assert_eq!(
        surface.client_scope.target_class,
        TargetBadgeClass::LocalDesktop
    );
    assert_eq!(surface.client_scope.boundary_cue, HostBoundaryCue::Hidden);
    assert!(!surface.client_scope.boundary_cue_visible);
    assert!(!surface.client_scope.context_missing);
    assert!(!surface.client_scope.honesty_marker_present);
    assert!(surface.client_scope.badge.is_some());

    // Docs/help truth quotes the upstream source-truth record verbatim.
    assert_eq!(
        surface.docs_help_truth.source_class,
        Some(SourceClass::ProjectDocs)
    );
    assert_eq!(surface.docs_help_truth.source_class_token, "project_docs");
    assert_eq!(surface.docs_help_truth.freshness_class_token, "authoritative_live");
    assert!(!surface.docs_help_truth.honesty_marker_present);
    assert!(!surface.docs_help_truth.source_missing);

    // Service-health and provenance rows are seed placeholders.
    assert_eq!(surface.service_health.rows.len(), 4);
    for row in &surface.service_health.rows {
        assert_eq!(row.state, ServiceHealthState::SeedPlaceholderAwaitingWiring);
    }
    assert_eq!(surface.provenance.rows.len(), 5);
    for row in &surface.provenance.rows {
        assert_eq!(row.state, ProvenanceRowState::SeedPlaceholderAwaitingWiring);
    }

    // Community-handoff routes are stable.
    assert_eq!(surface.community_handoff.rows.len(), 4);
    let route_tokens: Vec<_> = surface
        .community_handoff
        .rows
        .iter()
        .map(|row| row.route_class_token.as_str())
        .collect();
    assert_eq!(
        route_tokens,
        vec![
            "public_issue_tracker",
            "public_rfc_forum",
            "private_security_channel",
            "private_support_channel",
        ]
    );

    // Live actions stay live; reserved actions stay reserved.
    let live_actions: Vec<_> = surface
        .actions
        .iter()
        .filter(|a| matches!(a.availability, HelpAboutActionAvailability::Live))
        .map(|a| a.action_class)
        .collect();
    assert!(live_actions.contains(&HelpAboutActionClass::OpenExecutionContextInspector));
    assert!(live_actions.contains(&HelpAboutActionClass::CopyContextForSupportExport));

    let reserved_actions: Vec<_> = surface
        .actions
        .iter()
        .filter(|a| {
            matches!(
                a.availability,
                HelpAboutActionAvailability::ReservedForLaterMilestone
            )
        })
        .map(|a| a.action_class)
        .collect();
    for class in [
        HelpAboutActionClass::OpenReleasePacket,
        HelpAboutActionClass::ViewProvenanceDetails,
        HelpAboutActionClass::OpenAdvisoryHistory,
        HelpAboutActionClass::ReportIssueViaCommunityHandoff,
    ] {
        assert!(reserved_actions.contains(&class), "reserved should contain {class:?}");
    }

    // Plaintext renders the headings the chrome will quote verbatim.
    let plaintext = surface.render_plaintext();
    assert!(plaintext.contains("Help / About surface"));
    assert!(plaintext.contains("[Build identity]"));
    assert!(plaintext.contains("[Install mode]"));
    assert!(plaintext.contains("[Client scope]"));
    assert!(plaintext.contains("[Docs and help truth]"));
    assert!(plaintext.contains("[Service health]"));
    assert!(plaintext.contains("[Provenance]"));
    assert!(plaintext.contains("[Community handoff]"));
    assert!(plaintext.contains("Honesty marker: none"));
}

#[test]
fn failure_drill_stale_docs_source_lights_honesty_marker_and_keeps_actions_live() {
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::local_terminal_seed(
        "terminal.open",
        TrustState::Trusted,
        "mono:0",
    ));
    let identity = fixture_build_identity();
    let docs_truth = SourceTruthRecord {
        source_class: SourceClass::MirroredOfficialDocs,
        version_match_state: VersionMatchState::IncompatibleDriftDetected,
        freshness_class: FreshnessClass::Stale,
        running_build_identity_ref: "build-id:aureline:dev:0.0.0:aarch64:dev:0123456".to_owned(),
        help_status_badge_ref: None,
        snapshot_age_label: Some("47 days ago".to_owned()),
    };

    let surface = HelpAboutSurface::project(HelpAboutInputs {
        build_identity: &identity,
        release_channel_class_token: "stable",
        execution_context: Some(&context),
        docs_source_truth: Some(&docs_truth),
    });

    // The docs/help row lights honestly without collapsing the seed surface.
    assert!(surface.docs_help_truth.honesty_marker_present);
    assert_eq!(surface.docs_help_truth.freshness_class_token, "stale");
    assert_eq!(
        surface.docs_help_truth.version_match_token,
        "incompatible_drift_detected"
    );
    // The global honesty marker fires.
    assert!(surface.honesty_marker_present);

    // Client scope stays green because the upstream context is unchanged.
    assert!(!surface.client_scope.honesty_marker_present);

    // Live actions remain live so the user can still copy the surface for a
    // support export when the docs lane is degraded.
    let copy = surface
        .actions
        .iter()
        .find(|a| matches!(a.action_class, HelpAboutActionClass::CopyContextForSupportExport))
        .expect("copy action present");
    assert_eq!(copy.availability, HelpAboutActionAvailability::Live);
}

#[test]
fn pending_trust_blocks_inspector_action_and_lights_client_scope_honesty() {
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::local_terminal_seed(
        "terminal.open",
        TrustState::PendingEvaluation,
        "mono:0",
    ));
    let identity = fixture_build_identity();

    let surface = HelpAboutSurface::project(HelpAboutInputs {
        build_identity: &identity,
        release_channel_class_token: "stable",
        execution_context: Some(&context),
        docs_source_truth: None,
    });

    assert!(surface.client_scope.honesty_marker_present);
    assert_eq!(
        surface.client_scope.boundary_cue,
        HostBoundaryCue::DegradedTrust
    );

    let inspector = surface
        .actions
        .iter()
        .find(|a| matches!(a.action_class, HelpAboutActionClass::OpenExecutionContextInspector))
        .expect("inspector action present");
    assert_eq!(
        inspector.availability,
        HelpAboutActionAvailability::BlockedByPendingTrust
    );

    // Copy-for-support stays live so a support packet can quote the lane.
    let copy = surface
        .actions
        .iter()
        .find(|a| matches!(a.action_class, HelpAboutActionClass::CopyContextForSupportExport))
        .expect("copy action present");
    assert_eq!(copy.availability, HelpAboutActionAvailability::Live);

    // Docs/help row lights its own seed-placeholder honesty marker because no
    // upstream source was wired.
    assert!(surface.docs_help_truth.source_missing);
    assert!(surface.docs_help_truth.honesty_marker_present);
}

#[test]
fn missing_execution_context_keeps_seed_honest_about_client_scope() {
    let identity = fixture_build_identity();
    let surface = HelpAboutSurface::project(HelpAboutInputs {
        build_identity: &identity,
        release_channel_class_token: "dev_local",
        execution_context: None,
        docs_source_truth: None,
    });

    assert!(surface.client_scope.context_missing);
    assert!(surface.client_scope.honesty_marker_present);
    assert!(surface.client_scope.badge.is_none());
    assert_eq!(
        surface.client_scope.boundary_cue,
        HostBoundaryCue::Unknown
    );

    // The dev_local channel resolves to the dev install mode without any
    // honesty marker on the install-mode row itself.
    assert_eq!(
        surface.install_mode.install_mode_class,
        InstallModeClass::DevLocalBuiltFromSource
    );
    assert!(!surface.install_mode.honesty_marker_present);

    // The global honesty marker is present because the client-scope row
    // degraded to a typed placeholder and no docs source was wired.
    assert!(surface.honesty_marker_present);
}

#[test]
fn unknown_channel_token_lights_install_mode_honesty_marker() {
    let identity = fixture_build_identity();
    let surface = HelpAboutSurface::project(HelpAboutInputs {
        build_identity: &identity,
        release_channel_class_token: "weird-channel",
        execution_context: None,
        docs_source_truth: None,
    });
    assert_eq!(
        surface.install_mode.install_mode_class,
        InstallModeClass::UnknownInstallMode
    );
    assert!(surface.install_mode.honesty_marker_present);
    assert!(surface.honesty_marker_present);
}

#[test]
fn fixture_protected_walk_replays_into_the_help_about_surface() {
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/help/about_cases/protected_walk_local_dev.json");
    let payload = std::fs::read_to_string(&fixture_path).expect("fixture must read");
    let fixture: HelpAboutFixture = serde_json::from_str(&payload).expect("fixture must parse");

    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::local_terminal_seed(
        "terminal.open",
        fixture.input.trust_state,
        "mono:0",
    ));
    let identity = fixture_build_identity();
    let docs_truth = fixture.input.docs_source_truth.as_ref().map(|truth| {
        SourceTruthRecord {
            source_class: truth.source_class,
            version_match_state: truth.version_match_state,
            freshness_class: truth.freshness_class,
            running_build_identity_ref: truth.running_build_identity_ref.clone(),
            help_status_badge_ref: truth.help_status_badge_ref.clone(),
            snapshot_age_label: truth.snapshot_age_label.clone(),
        }
    });

    let surface = HelpAboutSurface::project(HelpAboutInputs {
        build_identity: &identity,
        release_channel_class_token: &fixture.input.release_channel_class_token,
        execution_context: Some(&context),
        docs_source_truth: docs_truth.as_ref(),
    });

    assert_eq!(surface.record_kind, fixture.expect.record_kind);
    assert_eq!(
        surface.honesty_marker_present,
        fixture.expect.honesty_marker_present
    );
    assert_eq!(
        surface.install_mode.install_mode_token,
        fixture.expect.install_mode_token
    );
    assert_eq!(
        surface.client_scope.target_class_token,
        fixture.expect.target_class_token
    );
    assert_eq!(
        surface.client_scope.boundary_cue_token,
        fixture.expect.boundary_cue_token
    );
    assert_eq!(
        surface.docs_help_truth.freshness_class_token,
        fixture.expect.docs_freshness_token
    );
}

#[test]
fn fixture_failure_drill_replays_stale_docs_source() {
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/help/about_cases/failure_drill_stale_docs_source.json");
    let payload = std::fs::read_to_string(&fixture_path).expect("fixture must read");
    let fixture: HelpAboutFixture = serde_json::from_str(&payload).expect("fixture must parse");

    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::local_terminal_seed(
        "terminal.open",
        fixture.input.trust_state,
        "mono:0",
    ));
    let identity = fixture_build_identity();
    let docs_truth = fixture.input.docs_source_truth.as_ref().map(|truth| {
        SourceTruthRecord {
            source_class: truth.source_class,
            version_match_state: truth.version_match_state,
            freshness_class: truth.freshness_class,
            running_build_identity_ref: truth.running_build_identity_ref.clone(),
            help_status_badge_ref: truth.help_status_badge_ref.clone(),
            snapshot_age_label: truth.snapshot_age_label.clone(),
        }
    });

    let surface = HelpAboutSurface::project(HelpAboutInputs {
        build_identity: &identity,
        release_channel_class_token: &fixture.input.release_channel_class_token,
        execution_context: Some(&context),
        docs_source_truth: docs_truth.as_ref(),
    });

    assert!(surface.honesty_marker_present);
    assert!(surface.docs_help_truth.honesty_marker_present);
    assert_eq!(
        surface.docs_help_truth.freshness_class_token,
        fixture.expect.docs_freshness_token
    );

    // Even with a stale docs source, the support-export copy stays live.
    let copy = surface
        .actions
        .iter()
        .find(|a| matches!(a.action_class, HelpAboutActionClass::CopyContextForSupportExport))
        .expect("copy action present");
    assert_eq!(copy.availability, HelpAboutActionAvailability::Live);
}

#[derive(Debug, Deserialize)]
struct HelpAboutFixture {
    input: HelpAboutFixtureInput,
    expect: HelpAboutFixtureExpect,
}

#[derive(Debug, Deserialize)]
struct HelpAboutFixtureInput {
    release_channel_class_token: String,
    trust_state: TrustState,
    #[serde(default)]
    docs_source_truth: Option<HelpAboutFixtureSourceTruth>,
}

#[derive(Debug, Deserialize)]
struct HelpAboutFixtureSourceTruth {
    source_class: SourceClass,
    version_match_state: VersionMatchState,
    freshness_class: FreshnessClass,
    running_build_identity_ref: String,
    #[serde(default)]
    help_status_badge_ref: Option<String>,
    #[serde(default)]
    snapshot_age_label: Option<String>,
}

#[derive(Debug, Deserialize)]
struct HelpAboutFixtureExpect {
    record_kind: String,
    honesty_marker_present: bool,
    install_mode_token: String,
    target_class_token: String,
    boundary_cue_token: String,
    docs_freshness_token: String,
}
