use super::*;

const PACKET_ID: &str = "manifest-scope-registry:stable:0001";

fn switcher(
    switcher_id: &str,
    active_manifest_label: &str,
    target_scope: ManifestTargetScope,
    lockfile_coupling: ManifestLockfileCoupling,
) -> ManifestScopeSwitcher {
    let disclosure = resolve_manifest_change_scope(target_scope, lockfile_coupling);
    ManifestScopeSwitcher {
        component: M5PackageComponent::ManifestScopeSwitcher,
        switcher_id: switcher_id.to_owned(),
        active_manifest_label: active_manifest_label.to_owned(),
        target_scope,
        scope_disclosure: "Owned by this manifest scope".to_owned(),
        member_selection_note: if disclosure.needs_member_selection_note {
            "Targets packages/web/package.json, selected from 4 workspace members".to_owned()
        } else {
            String::new()
        },
        lockfile_coupling,
        lockfile_coupling_note: if disclosure.needs_lockfile_coupling_note {
            "A change here regenerates the coupled lockfile".to_owned()
        } else {
            String::new()
        },
        affects_root_lockfile: disclosure.affects_shared_root_lockfile,
        change_scope_action_label: "Review change scope".to_owned(),
        change_scope_review_note: "Opens the change-scope review before any write".to_owned(),
        rollback_posture: M5PackageComponentRollbackPosture::StagedReviewNoWrite,
        fields_shown: vec![
            "active_manifest_label".to_owned(),
            "target_scope".to_owned(),
            "lockfile_coupling".to_owned(),
            "change_scope".to_owned(),
        ],
        source_contract_refs: vec![
            M5_PACKAGE_COMPONENT_MATRIX_MANIFEST_SCOPE_CONTRACT_REF.to_owned()
        ],
    }
}

fn switchers() -> Vec<ManifestScopeSwitcher> {
    let root = switcher(
        "switch:root",
        "Cargo.toml (workspace root)",
        ManifestTargetScope::RootManifest,
        ManifestLockfileCoupling::SharedRootLockfile,
    );

    let mut member = switcher(
        "switch:member-web",
        "packages/web/package.json",
        ManifestTargetScope::MemberPackage,
        ManifestLockfileCoupling::SharedRootLockfile,
    );
    member.member_selection_note =
        "Targets packages/web/package.json, selected from 4 workspace members".to_owned();
    member.lockfile_coupling_note =
        "A change here regenerates the shared root lockfile that governs every member".to_owned();

    let mut module = switcher(
        "switch:module-auth",
        "packages/api/auth/module.json",
        ManifestTargetScope::ModuleManifest,
        ManifestLockfileCoupling::MemberScopedLockfile,
    );
    module.member_selection_note =
        "Targets the auth module under packages/api; not the api member root".to_owned();
    module.lockfile_coupling_note =
        "A change here regenerates only the api member lockfile".to_owned();

    let tool = switcher(
        "switch:tool",
        "rust-toolchain.toml (toolchain)",
        ManifestTargetScope::ToolManifest,
        ManifestLockfileCoupling::NoLockfileCoupling,
    );

    vec![root, member, module, tool]
}

fn registry_row(
    row_id: &str,
    registry_label: &str,
    source_class: RegistryMirrorSourceClass,
    reachability: RegistryReachabilityState,
) -> RegistryOrMirrorRow {
    let is_policy_pinned = source_class.implies_policy_pin();
    let disclosure =
        resolve_registry_or_mirror_disclosure(source_class, reachability, is_policy_pinned);
    RegistryOrMirrorRow {
        component: M5PackageComponent::RegistryOrMirrorRow,
        row_id: row_id.to_owned(),
        registry_label: registry_label.to_owned(),
        source_class,
        source_disclosure: "Names where metadata and artifacts come from".to_owned(),
        auth_mode: RegistryAuthMode::AnonymousPublic,
        auth_disclosure: String::new(),
        reachability,
        reachability_note: if disclosure.needs_reachability_note {
            "Not a fresh live read; see continuity note".to_owned()
        } else {
            String::new()
        },
        is_policy_pinned,
        policy_pin_note: if disclosure.needs_policy_pin_note {
            "Held by registry policy pinning".to_owned()
        } else {
            String::new()
        },
        offline_cache_only: disclosure.is_offline_cache_only,
        offline_continuity_note: if disclosure.needs_offline_continuity_note {
            "Answered from the offline cache; the source was not reached".to_owned()
        } else {
            String::new()
        },
        degradation_state: M5PackageComponentDegradationState::ResolvedExact,
        degradation_note: String::new(),
        rollback_posture: M5PackageComponentRollbackPosture::ReadOnlyNoMutation,
        fields_shown: vec![
            "registry_label".to_owned(),
            "source_class".to_owned(),
            "auth_mode".to_owned(),
            "reachability".to_owned(),
        ],
        source_contract_refs: vec![
            M5_PACKAGE_COMPONENT_MATRIX_REGISTRY_MIRROR_CONTRACT_REF.to_owned()
        ],
    }
}

fn registry_rows() -> Vec<RegistryOrMirrorRow> {
    let public = registry_row(
        "reg:public",
        "registry.npmjs.org (public default)",
        RegistryMirrorSourceClass::PublicDefault,
        RegistryReachabilityState::FreshReachable,
    );

    let mut mirror = registry_row(
        "reg:mirror",
        "nexus.corp.example (enterprise mirror)",
        RegistryMirrorSourceClass::EnterpriseMirror,
        RegistryReachabilityState::FreshReachable,
    );
    mirror.auth_mode = RegistryAuthMode::TokenAuthenticated;
    mirror.auth_disclosure = "Token-authenticated against the enterprise mirror".to_owned();
    mirror.degradation_state = M5PackageComponentDegradationState::MirrorBacked;
    mirror.degradation_note =
        "Mirror-backed answer; upstream registry was not consulted".to_owned();

    let mut self_hosted = registry_row(
        "reg:self-hosted",
        "packages.internal (self-hosted)",
        RegistryMirrorSourceClass::SelfHosted,
        RegistryReachabilityState::StaleCached,
    );
    self_hosted.auth_mode = RegistryAuthMode::SsoSession;
    self_hosted.auth_disclosure = "SSO session against the self-hosted registry".to_owned();
    self_hosted.reachability_note =
        "Stale cached metadata; the self-hosted registry was last reached 6h ago".to_owned();

    let offline = registry_row(
        "reg:offline",
        "local offline cache",
        RegistryMirrorSourceClass::OfflineCache,
        RegistryReachabilityState::OfflineCacheOnly,
    );

    let pinned = registry_row(
        "reg:pinned",
        "policy-pinned source",
        RegistryMirrorSourceClass::PolicyPinnedSource,
        RegistryReachabilityState::FreshReachable,
    );

    vec![public, mirror, self_hosted, offline, pinned]
}

fn trust_review() -> ManifestScopeRegistryTrustReview {
    ManifestScopeRegistryTrustReview {
        active_manifest_always_named: true,
        target_scope_always_explicit: true,
        lockfile_coupling_always_explicit: true,
        change_scope_matches_scope_and_lockfile: true,
        no_generic_manage_package_language: true,
        registry_source_always_explicit: true,
        auth_mode_always_explicit: true,
        freshness_reachability_always_explicit: true,
        policy_pinning_explicit: true,
        offline_cache_continuity_explicit: true,
        inherited_registry_state_never_hidden: true,
        copy_and_export_safe: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> ManifestScopeRegistryConsumerProjection {
    ManifestScopeRegistryConsumerProjection {
        switcher_shows_active_manifest_and_scope: true,
        lockfile_coupling_shown_inline: true,
        change_scope_action_reflects_truth: true,
        registry_row_shows_source_auth_and_reachability: true,
        offline_and_policy_pin_shown_inline: true,
        cli_headless_shows_control_truth: true,
        support_export_shows_control_truth: true,
        help_about_shows_control_truth: true,
    }
}

fn proof_freshness() -> ManifestScopeRegistryProofFreshness {
    ManifestScopeRegistryProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-07-08T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<M5PackageComponentDowngradeTrigger> {
    vec![
        M5PackageComponentDowngradeTrigger::ProofStale,
        M5PackageComponentDowngradeTrigger::RegistryUnreachable,
        M5PackageComponentDowngradeTrigger::MirrorBackedOnly,
        M5PackageComponentDowngradeTrigger::OfflineSnapshotOnly,
        M5PackageComponentDowngradeTrigger::AuthRequired,
        M5PackageComponentDowngradeTrigger::PolicyBlocked,
        M5PackageComponentDowngradeTrigger::UpstreamDependencyNarrowed,
    ]
}

fn consumer_surfaces() -> Vec<M5PackageComponentConsumerSurface> {
    vec![
        M5PackageComponentConsumerSurface::PackageWorkspace,
        M5PackageComponentConsumerSurface::RegistryAuthWorkspace,
        M5PackageComponentConsumerSurface::CliHeadless,
        M5PackageComponentConsumerSurface::SupportExport,
        M5PackageComponentConsumerSurface::HelpAbout,
    ]
}

fn source_contract_refs() -> Vec<String> {
    vec![
        MANIFEST_SCOPE_REGISTRY_SCHEMA_REF.to_owned(),
        MANIFEST_SCOPE_REGISTRY_DOC_REF.to_owned(),
        M5_PACKAGE_COMPONENT_MATRIX_SCHEMA_REF.to_owned(),
        M5_PACKAGE_COMPONENT_MATRIX_DOC_REF.to_owned(),
        M5_PACKAGE_COMPONENT_MATRIX_MANIFEST_SCOPE_CONTRACT_REF.to_owned(),
        M5_PACKAGE_COMPONENT_MATRIX_REGISTRY_MIRROR_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> ManifestScopeRegistryControlsPacket {
    ManifestScopeRegistryControlsPacket::new(ManifestScopeRegistryControlsPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label: "Manifest-scope switchers and registry/mirror rows".to_owned(),
        manifest_scope_switchers: switchers(),
        registry_or_mirror_rows: registry_rows(),
        downgrade_triggers: downgrade_triggers(),
        consumer_surfaces: consumer_surfaces(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-08T00:00:00Z".to_owned(),
    })
}

#[test]
fn manifest_scope_registry_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn change_scope_resolver_derives_from_scope_and_coupling() {
    // Root manifest is a root-wide change.
    let root = resolve_manifest_change_scope(
        ManifestTargetScope::RootManifest,
        ManifestLockfileCoupling::SharedRootLockfile,
    );
    assert_eq!(
        root.change_scope_class,
        ManifestChangeScopeClass::RootWideChange
    );
    assert!(!root.needs_member_selection_note);

    // Member manifest on a shared root lockfile regenerates the root lockfile.
    let member = resolve_manifest_change_scope(
        ManifestTargetScope::MemberPackage,
        ManifestLockfileCoupling::SharedRootLockfile,
    );
    assert_eq!(
        member.change_scope_class,
        ManifestChangeScopeClass::MemberChangeSharedLock
    );
    assert!(member.needs_member_selection_note);
    assert!(member.affects_shared_root_lockfile);

    // Member manifest with its own lockfile stays scoped.
    let scoped = resolve_manifest_change_scope(
        ManifestTargetScope::MemberPackage,
        ManifestLockfileCoupling::MemberScopedLockfile,
    );
    assert_eq!(
        scoped.change_scope_class,
        ManifestChangeScopeClass::MemberScopedChange
    );
    assert!(!scoped.affects_shared_root_lockfile);
}

#[test]
fn registry_resolver_derives_offline_and_pin_truth() {
    // Offline-cache source is offline/cache-only.
    let offline = resolve_registry_or_mirror_disclosure(
        RegistryMirrorSourceClass::OfflineCache,
        RegistryReachabilityState::OfflineCacheOnly,
        false,
    );
    assert!(offline.is_offline_cache_only);
    assert!(offline.needs_offline_continuity_note);

    // An unreachable public source is still offline/cache-only.
    let unreachable = resolve_registry_or_mirror_disclosure(
        RegistryMirrorSourceClass::PublicDefault,
        RegistryReachabilityState::Unreachable,
        false,
    );
    assert!(unreachable.is_offline_cache_only);

    // A policy-pinned source implies policy pinning.
    let pinned = resolve_registry_or_mirror_disclosure(
        RegistryMirrorSourceClass::PolicyPinnedSource,
        RegistryReachabilityState::FreshReachable,
        false,
    );
    assert!(pinned.source_implies_policy_pin);
    assert!(pinned.needs_policy_pin_note);
    assert!(!pinned.is_offline_cache_only);
}

#[test]
fn switcher_hiding_shared_lockfile_regeneration_fails() {
    let mut packet = packet();
    let idx = packet
        .manifest_scope_switchers
        .iter()
        .position(|s| {
            s.target_scope == ManifestTargetScope::MemberPackage
                && s.lockfile_coupling == ManifestLockfileCoupling::SharedRootLockfile
        })
        .expect("member-on-shared-root switcher present");
    packet.manifest_scope_switchers[idx].affects_root_lockfile = false;
    assert!(packet
        .validate()
        .contains(&ManifestScopeRegistryViolation::ChangeScopeMisrepresented));
}

#[test]
fn below_root_switcher_without_member_note_fails() {
    let mut packet = packet();
    let idx = packet
        .manifest_scope_switchers
        .iter()
        .position(|s| s.target_scope.is_below_root())
        .expect("below-root switcher present");
    packet.manifest_scope_switchers[idx].member_selection_note = String::new();
    assert!(packet
        .validate()
        .contains(&ManifestScopeRegistryViolation::MemberSelectionNoteMissing));
}

#[test]
fn lockfile_coupled_switcher_without_note_fails() {
    let mut packet = packet();
    let idx = packet
        .manifest_scope_switchers
        .iter()
        .position(|s| s.lockfile_coupling.needs_coupling_note())
        .expect("lockfile-coupled switcher present");
    packet.manifest_scope_switchers[idx].lockfile_coupling_note = String::new();
    assert!(packet
        .validate()
        .contains(&ManifestScopeRegistryViolation::LockfileCouplingNoteMissing));
}

#[test]
fn switcher_missing_active_manifest_label_fails() {
    let mut packet = packet();
    packet.manifest_scope_switchers[0].active_manifest_label = String::new();
    assert!(packet
        .validate()
        .contains(&ManifestScopeRegistryViolation::ActiveManifestLabelMissing));
}

#[test]
fn switcher_wrong_component_class_fails() {
    let mut packet = packet();
    packet.manifest_scope_switchers[0].component = M5PackageComponent::RegistryOrMirrorRow;
    assert!(packet
        .validate()
        .contains(&ManifestScopeRegistryViolation::SwitcherWrongComponentClass));
}

#[test]
fn switcher_write_back_posture_fails() {
    let mut packet = packet();
    packet.manifest_scope_switchers[0].rollback_posture =
        M5PackageComponentRollbackPosture::WriteBackCheckpointed;
    assert!(packet
        .validate()
        .contains(&ManifestScopeRegistryViolation::SwitcherRollbackPostureInconsistent));
}

#[test]
fn missing_scope_coverage_fails() {
    let mut packet = packet();
    packet
        .manifest_scope_switchers
        .retain(|s| s.target_scope != ManifestTargetScope::ToolManifest);
    assert!(packet
        .validate()
        .contains(&ManifestScopeRegistryViolation::ScopeCoverageMissing));
}

#[test]
fn empty_switchers_fails() {
    let mut packet = packet();
    packet.manifest_scope_switchers.clear();
    assert!(packet
        .validate()
        .contains(&ManifestScopeRegistryViolation::SwitchersMissing));
}

#[test]
fn registry_row_hiding_offline_continuity_fails() {
    let mut packet = packet();
    let idx = packet
        .registry_or_mirror_rows
        .iter()
        .position(|r| r.source_class == RegistryMirrorSourceClass::OfflineCache)
        .expect("offline row present");
    packet.registry_or_mirror_rows[idx].offline_cache_only = false;
    assert!(packet
        .validate()
        .contains(&ManifestScopeRegistryViolation::OfflineContinuityMisrepresented));
}

#[test]
fn offline_row_without_continuity_note_fails() {
    let mut packet = packet();
    let idx = packet
        .registry_or_mirror_rows
        .iter()
        .position(|r| r.offline_cache_only)
        .expect("offline row present");
    packet.registry_or_mirror_rows[idx].offline_continuity_note = String::new();
    assert!(packet
        .validate()
        .contains(&ManifestScopeRegistryViolation::OfflineContinuityNoteMissing));
}

#[test]
fn policy_pinned_source_not_marked_pinned_fails() {
    let mut packet = packet();
    let idx = packet
        .registry_or_mirror_rows
        .iter()
        .position(|r| r.source_class == RegistryMirrorSourceClass::PolicyPinnedSource)
        .expect("pinned row present");
    packet.registry_or_mirror_rows[idx].is_policy_pinned = false;
    let violations = packet.validate();
    assert!(violations.contains(&ManifestScopeRegistryViolation::PolicyPinningMisrepresented));
}

#[test]
fn pinned_row_without_pin_note_fails() {
    let mut packet = packet();
    let idx = packet
        .registry_or_mirror_rows
        .iter()
        .position(|r| r.is_policy_pinned)
        .expect("pinned row present");
    packet.registry_or_mirror_rows[idx].policy_pin_note = String::new();
    assert!(packet
        .validate()
        .contains(&ManifestScopeRegistryViolation::PolicyPinNoteMissing));
}

#[test]
fn authenticated_row_without_auth_disclosure_fails() {
    let mut packet = packet();
    let idx = packet
        .registry_or_mirror_rows
        .iter()
        .position(|r| r.auth_mode.needs_auth_disclosure())
        .expect("authenticated row present");
    packet.registry_or_mirror_rows[idx].auth_disclosure = String::new();
    assert!(packet
        .validate()
        .contains(&ManifestScopeRegistryViolation::AuthDisclosureMissing));
}

#[test]
fn non_fresh_row_without_reachability_note_fails() {
    let mut packet = packet();
    let idx = packet
        .registry_or_mirror_rows
        .iter()
        .position(|r| r.reachability == RegistryReachabilityState::StaleCached)
        .expect("stale row present");
    packet.registry_or_mirror_rows[idx].reachability_note = String::new();
    assert!(packet
        .validate()
        .contains(&ManifestScopeRegistryViolation::ReachabilityNoteMissing));
}

#[test]
fn degraded_registry_row_without_note_fails() {
    let mut packet = packet();
    let idx = packet
        .registry_or_mirror_rows
        .iter()
        .position(|r| r.degradation_state == M5PackageComponentDegradationState::MirrorBacked)
        .expect("mirror-backed row present");
    packet.registry_or_mirror_rows[idx].degradation_note = String::new();
    assert!(packet
        .validate()
        .contains(&ManifestScopeRegistryViolation::RegistryDegradationNoteMissing));
}

#[test]
fn registry_row_write_back_posture_fails() {
    let mut packet = packet();
    packet.registry_or_mirror_rows[0].rollback_posture =
        M5PackageComponentRollbackPosture::WriteBackCheckpointed;
    assert!(packet
        .validate()
        .contains(&ManifestScopeRegistryViolation::RegistryRowRollbackInconsistent));
}

#[test]
fn registry_row_wrong_component_class_fails() {
    let mut packet = packet();
    packet.registry_or_mirror_rows[0].component = M5PackageComponent::ManifestScopeSwitcher;
    assert!(packet
        .validate()
        .contains(&ManifestScopeRegistryViolation::RegistryRowWrongComponentClass));
}

#[test]
fn missing_source_coverage_fails() {
    let mut packet = packet();
    packet
        .registry_or_mirror_rows
        .retain(|r| r.source_class != RegistryMirrorSourceClass::SelfHosted);
    assert!(packet
        .validate()
        .contains(&ManifestScopeRegistryViolation::SourceCoverageMissing));
}

#[test]
fn empty_registry_rows_fails() {
    let mut packet = packet();
    packet.registry_or_mirror_rows.clear();
    assert!(packet
        .validate()
        .contains(&ManifestScopeRegistryViolation::RegistryRowsMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&ManifestScopeRegistryViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.no_generic_manage_package_language = false;
    assert!(packet
        .validate()
        .contains(&ManifestScopeRegistryViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .offline_and_policy_pin_shown_inline = false;
    assert!(packet
        .validate()
        .contains(&ManifestScopeRegistryViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&ManifestScopeRegistryViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_controls() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Manifest-scope switchers"));
    assert!(summary.contains("## Registry / mirror rows"));
    assert!(summary.contains("member_change_shared_lock"));
    assert!(summary.contains("offline_cache"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_manifest_scope_registry_export()
        .expect("checked manifest scope registry export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-manifest-scope-registry-controls/member_shared_root_lockfile.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-manifest-scope-registry-controls/offline_cache_only.json"
        )),
    ] {
        let packet: ManifestScopeRegistryControlsPacket =
            serde_json::from_str(raw).expect("fixture parses as manifest scope registry packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

/// Regenerates the checked release artifacts and narrowed fixtures.
///
/// Guarded behind `GEN_MANIFEST_SCOPE_REGISTRY_ARTIFACTS` so ordinary test runs
/// never touch the working tree.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_MANIFEST_SCOPE_REGISTRY_ARTIFACTS").is_err() {
        return;
    }

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let repo_root = std::path::Path::new(manifest_dir).join("..").join("..");

    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    let proof_dir = repo_root
        .join("artifacts")
        .join("release")
        .join("m5-manifest-scope-registry-proof");
    std::fs::create_dir_all(&proof_dir).expect("create proof dir");
    std::fs::write(
        proof_dir.join("support_export.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .expect("write support export");
    std::fs::write(
        proof_dir.join("summary.md"),
        packet.render_markdown_summary(),
    )
    .expect("write summary");

    let fixture_dir = repo_root
        .join("fixtures")
        .join("ui")
        .join("m5-manifest-scope-registry-controls");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");

    // Fixture 1: spotlight a member manifest on a shared root lockfile — the
    // change regenerates the shared root lockfile and must say so.
    let mut member = packet.clone();
    member.packet_id = "manifest-scope-registry:fixture:member-shared-root".to_owned();
    member.manifest_scope_switchers = switchers();
    member.registry_or_mirror_rows = registry_rows();
    assert!(member.validate().is_empty(), "{:?}", member.validate());
    std::fs::write(
        fixture_dir.join("member_shared_root_lockfile.json"),
        format!("{}\n", member.export_safe_json()),
    )
    .expect("write member fixture");

    // Fixture 2: an offline/cache-only resolution that never reads as a clean
    // live upstream read.
    let mut offline = packet.clone();
    offline.packet_id = "manifest-scope-registry:fixture:offline-cache-only".to_owned();
    for row in offline.registry_or_mirror_rows.iter_mut() {
        if row.source_class == RegistryMirrorSourceClass::PublicDefault {
            *row = registry_row(
                &row.row_id,
                "registry.npmjs.org (unreachable, offline cache)",
                RegistryMirrorSourceClass::PublicDefault,
                RegistryReachabilityState::Unreachable,
            );
            row.reachability_note =
                "The public registry was unreachable; served from the offline cache".to_owned();
            row.offline_continuity_note =
                "Answered from the offline cache; the registry was not reached".to_owned();
            row.degradation_state = M5PackageComponentDegradationState::OfflineSnapshotOnly;
            row.degradation_note =
                "Offline snapshot only; resolution continues from the local cache".to_owned();
        }
    }
    assert!(offline.validate().is_empty(), "{:?}", offline.validate());
    std::fs::write(
        fixture_dir.join("offline_cache_only.json"),
        format!("{}\n", offline.export_safe_json()),
    )
    .expect("write offline fixture");
}
