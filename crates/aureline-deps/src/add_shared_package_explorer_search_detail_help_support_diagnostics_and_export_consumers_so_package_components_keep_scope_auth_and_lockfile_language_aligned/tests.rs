use super::*;

const PACKET_ID: &str = "package-component-consumer:stable:0001";

fn trust_review() -> PackageComponentConsumerTrustReview {
    PackageComponentConsumerTrustReview {
        component_reuse_proven_by_fixtures: true,
        same_object_same_language_across_surfaces: true,
        manifest_scope_identical_across_surfaces: true,
        registry_source_and_auth_identical_across_surfaces: true,
        script_native_build_risk_identical_across_surfaces: true,
        lockfile_churn_never_understated_across_surfaces: true,
        rollback_checkpoint_identity_kept_explicit: true,
        mirror_offline_continuity_kept_explicit: true,
        generic_one_click_language_never_conceals_scope_or_risk: true,
        help_support_export_point_canonical_contracts: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> PackageComponentConsumerProjection {
    PackageComponentConsumerProjection {
        package_explorer_reuses_shared_components: true,
        dependency_search_detail_reuses_shared_components: true,
        help_surface_reuses_shared_components: true,
        support_packet_reuses_shared_components: true,
        diagnostics_reuses_shared_components: true,
        exported_view_reuses_shared_components: true,
        every_component_adopted_by_two_or_more_consumers: true,
        parity_facets_identical_for_same_object: true,
        narrowing_disclosed_not_hidden: true,
        export_preserves_scope_auth_and_lockfile_posture: true,
    }
}

fn proof_freshness() -> PackageComponentConsumerProofFreshness {
    PackageComponentConsumerProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-07-09T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<M5PackageComponentDowngradeTrigger> {
    vec![
        M5PackageComponentDowngradeTrigger::ProofStale,
        M5PackageComponentDowngradeTrigger::MirrorBackedOnly,
        M5PackageComponentDowngradeTrigger::OfflineSnapshotOnly,
        M5PackageComponentDowngradeTrigger::AuthRequired,
        M5PackageComponentDowngradeTrigger::BroadLockfileRegeneration,
        M5PackageComponentDowngradeTrigger::UpstreamDependencyNarrowed,
    ]
}

fn consumer_surfaces() -> Vec<PackageComponentConsumer> {
    PackageComponentConsumer::ALL.to_vec()
}

fn source_contract_refs() -> Vec<String> {
    vec![
        PACKAGE_COMPONENT_CONSUMER_SCHEMA_REF.to_owned(),
        PACKAGE_COMPONENT_CONSUMER_DOC_REF.to_owned(),
        PACKAGE_COMPONENT_CONSUMER_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        PACKAGE_COMPONENT_CONSUMER_EXPLORER_ROW_CONTRACT_REF.to_owned(),
        PACKAGE_COMPONENT_CONSUMER_MANIFEST_SCOPE_REGISTRY_CONTRACT_REF.to_owned(),
        PACKAGE_COMPONENT_CONSUMER_INSTALL_REVIEW_LOCKFILE_CONTRACT_REF.to_owned(),
        PACKAGE_COMPONENT_CONSUMER_SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_CONTRACT_REF.to_owned(),
    ]
}

fn binding_refs(component: M5PackageComponent) -> Vec<String> {
    vec![
        PACKAGE_COMPONENT_CONSUMER_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        component_canonical_schema_ref(component).to_owned(),
    ]
}

/// Builds one binding, deriving render mode, parity state, narrow banner, and
/// disclosure notes from the object's registry state so the fixture stays
/// self-consistent by construction.
#[allow(clippy::too_many_arguments)]
fn binding(
    binding_id: &str,
    object_id: &str,
    object_label: &str,
    component: M5PackageComponent,
    consumer: PackageComponentConsumer,
    registry_state: M5PackageComponentDegradationState,
    facets: &PackageComponentParityFacetValues,
) -> PackageComponentConsumerBinding {
    let disclosure = resolve_package_component_render_disclosure(registry_state);

    let narrow_banner = disclosure.narrow_reason.map(|reason| {
        let (next_action, next_action_label) = match reason {
            PackageComponentNarrowReason::ExactResolutionUnavailableRangeOnly => (
                PackageComponentNarrowNextAction::ReviewManifestRange,
                "Only a manifest range governs here; review the range before resolving an exact pin"
                    .to_owned(),
            ),
            PackageComponentNarrowReason::MirrorOrOfflineContinuity => (
                PackageComponentNarrowNextAction::ReviewMirrorOfflineContinuity,
                "This answer is mirror-backed or offline; review the continuity posture".to_owned(),
            ),
            PackageComponentNarrowReason::RegistryAuthRequired => (
                PackageComponentNarrowNextAction::CompleteRegistryAuth,
                "Registry access needs authentication; complete sign-in to resolve".to_owned(),
            ),
            PackageComponentNarrowReason::PackageStateUnknownOrStale => (
                PackageComponentNarrowNextAction::ReviewPackageStateFreshness,
                "The package state is unknown or stale; review its freshness".to_owned(),
            ),
        };
        PackageComponentNarrowBanner {
            reason,
            preserved_facets_note:
                "Manifest scope, registry source/auth, risk, and recovery language are preserved; only resolution narrowed"
                    .to_owned(),
            next_action,
            next_action_label,
        }
    });

    let continuity_note = if disclosure.needs_continuity_note {
        "Answered from a mirror or offline snapshot; upstream continuity is not confirmed here"
            .to_owned()
    } else {
        String::new()
    };
    let auth_note = if disclosure.needs_auth_note {
        "Registry authentication is required and not yet satisfied for this source".to_owned()
    } else {
        String::new()
    };

    PackageComponentConsumerBinding {
        binding_id: binding_id.to_owned(),
        package_object_id: object_id.to_owned(),
        package_object_label: object_label.to_owned(),
        component,
        consumer,
        registry_state,
        render_mode: disclosure.expected_mode,
        parity_facets: facets.clone(),
        parity_state: parity_state_for_mode(disclosure.expected_mode),
        narrow_banner,
        continuity_note,
        auth_note,
        uses_generic_manage_package_language_hiding_scope: false,
        uses_one_click_update_language_hiding_risk: false,
        conceals_registry_auth_posture: false,
        hides_broad_lockfile_regeneration: false,
        drops_mirror_offline_or_rollback_truth: false,
        source_contract_refs: binding_refs(component),
    }
}

fn facets(
    manifest_scope: &str,
    registry_auth: &str,
    risk: &str,
    recovery: &str,
) -> PackageComponentParityFacetValues {
    PackageComponentParityFacetValues {
        manifest_scope_label: manifest_scope.to_owned(),
        registry_source_auth_label: registry_auth.to_owned(),
        risk_language: risk.to_owned(),
        recovery_language: recovery.to_owned(),
    }
}

/// The canonical binding set: eight components, each adopted by >= 2 consumers,
/// covering all six consumer surfaces and every registry/resolution state. Objects
/// sharing an id share parity facets.
fn consumer_bindings() -> Vec<PackageComponentConsumerBinding> {
    // Object 1: package explorer row, exact pin.
    let per = facets(
        "package.json (root) · direct dependency",
        "public npm registry · authenticated",
        "no install scripts · lockfile unchanged",
        "no update pending · rollback checkpoint available",
    );
    // Object 2: manifest-scope switcher, manifest range only.
    let mss = facets(
        "packages/web/package.json (member) · direct dependency",
        "public npm registry · anonymous read",
        "no install scripts · member lockfile only",
        "range update available · rollback checkpoint available",
    );
    // Object 3: install-review sheet, exact pin.
    let irs = facets(
        "pyproject.toml (root) · direct dependency",
        "PyPI · authenticated",
        "native build required · 12 lockfile lines change",
        "install pending · rollback checkpoint before write",
    );
    // Object 4: registry-or-mirror row, mirror-backed.
    let rmr = facets(
        "Cargo.toml (workspace) · transitive dependency",
        "enterprise crates mirror · token auth",
        "no install scripts · lockfile unchanged",
        "no update pending · rollback checkpoint available",
    );
    // Object 5: script-risk notice, offline snapshot only.
    let srn = facets(
        "package.json (root) · direct dependency",
        "public npm registry · offline cache",
        "known install script runs · native build required",
        "install pending · rollback checkpoint before write",
    );
    // Object 6: lockfile-impact card, exact pin.
    let lic = facets(
        "pnpm-lock.yaml (root) · workspace lockfile",
        "public npm registry · authenticated",
        "no install scripts · broad lockfile regeneration (140 lines)",
        "grouped update pending · rollback checkpoint before write",
    );
    // Object 7: grouped-update planner, auth-required unsatisfied.
    let gup = facets(
        "package.json (root) · direct dependencies",
        "private registry · authentication required",
        "no install scripts · grouped lockfile churn (30 lines)",
        "security grouped update · rollback checkpoint before write",
    );
    // Object 8: rollback-checkpoint strip, unknown or stale.
    let rcs = facets(
        "Cargo.lock (workspace) · workspace lockfile",
        "crates.io · reachability unknown",
        "no install scripts · lockfile churn unknown",
        "post-mutation recovery · rollback checkpoint retained",
    );

    vec![
        binding(
            "bind:per-1:explorer",
            "obj:per-1",
            "lodash",
            M5PackageComponent::PackageExplorerRow,
            PackageComponentConsumer::PackageExplorer,
            M5PackageComponentDegradationState::ResolvedExact,
            &per,
        ),
        binding(
            "bind:per-1:search",
            "obj:per-1",
            "lodash",
            M5PackageComponent::PackageExplorerRow,
            PackageComponentConsumer::DependencySearchDetail,
            M5PackageComponentDegradationState::ResolvedExact,
            &per,
        ),
        binding(
            "bind:mss-2:explorer",
            "obj:mss-2",
            "react (member scope)",
            M5PackageComponent::ManifestScopeSwitcher,
            PackageComponentConsumer::PackageExplorer,
            M5PackageComponentDegradationState::ManifestRangeOnly,
            &mss,
        ),
        binding(
            "bind:mss-2:search",
            "obj:mss-2",
            "react (member scope)",
            M5PackageComponent::ManifestScopeSwitcher,
            PackageComponentConsumer::DependencySearchDetail,
            M5PackageComponentDegradationState::ManifestRangeOnly,
            &mss,
        ),
        binding(
            "bind:irs-3:search",
            "obj:irs-3",
            "numpy (install review)",
            M5PackageComponent::InstallReviewSheet,
            PackageComponentConsumer::DependencySearchDetail,
            M5PackageComponentDegradationState::ResolvedExact,
            &irs,
        ),
        binding(
            "bind:irs-3:diagnostics",
            "obj:irs-3",
            "numpy (install review)",
            M5PackageComponent::InstallReviewSheet,
            PackageComponentConsumer::Diagnostics,
            M5PackageComponentDegradationState::ResolvedExact,
            &irs,
        ),
        binding(
            "bind:rmr-4:search",
            "obj:rmr-4",
            "serde (mirror source)",
            M5PackageComponent::RegistryOrMirrorRow,
            PackageComponentConsumer::DependencySearchDetail,
            M5PackageComponentDegradationState::MirrorBacked,
            &rmr,
        ),
        binding(
            "bind:rmr-4:support",
            "obj:rmr-4",
            "serde (mirror source)",
            M5PackageComponent::RegistryOrMirrorRow,
            PackageComponentConsumer::SupportPacket,
            M5PackageComponentDegradationState::MirrorBacked,
            &rmr,
        ),
        binding(
            "bind:srn-5:diagnostics",
            "obj:srn-5",
            "node-sass (script risk)",
            M5PackageComponent::ScriptRiskNotice,
            PackageComponentConsumer::Diagnostics,
            M5PackageComponentDegradationState::OfflineSnapshotOnly,
            &srn,
        ),
        binding(
            "bind:srn-5:support",
            "obj:srn-5",
            "node-sass (script risk)",
            M5PackageComponent::ScriptRiskNotice,
            PackageComponentConsumer::SupportPacket,
            M5PackageComponentDegradationState::OfflineSnapshotOnly,
            &srn,
        ),
        binding(
            "bind:lic-6:help",
            "obj:lic-6",
            "pnpm-lock.yaml (impact)",
            M5PackageComponent::LockfileImpactCard,
            PackageComponentConsumer::HelpSurface,
            M5PackageComponentDegradationState::ResolvedExact,
            &lic,
        ),
        binding(
            "bind:lic-6:export",
            "obj:lic-6",
            "pnpm-lock.yaml (impact)",
            M5PackageComponent::LockfileImpactCard,
            PackageComponentConsumer::ExportedView,
            M5PackageComponentDegradationState::ResolvedExact,
            &lic,
        ),
        binding(
            "bind:gup-7:help",
            "obj:gup-7",
            "security grouped update",
            M5PackageComponent::GroupedUpdatePlanner,
            PackageComponentConsumer::HelpSurface,
            M5PackageComponentDegradationState::AuthRequiredUnsatisfied,
            &gup,
        ),
        binding(
            "bind:gup-7:support",
            "obj:gup-7",
            "security grouped update",
            M5PackageComponent::GroupedUpdatePlanner,
            PackageComponentConsumer::SupportPacket,
            M5PackageComponentDegradationState::AuthRequiredUnsatisfied,
            &gup,
        ),
        binding(
            "bind:rcs-8:export",
            "obj:rcs-8",
            "Cargo.lock checkpoint",
            M5PackageComponent::RollbackCheckpointStrip,
            PackageComponentConsumer::ExportedView,
            M5PackageComponentDegradationState::UnknownOrStale,
            &rcs,
        ),
        binding(
            "bind:rcs-8:diagnostics",
            "obj:rcs-8",
            "Cargo.lock checkpoint",
            M5PackageComponent::RollbackCheckpointStrip,
            PackageComponentConsumer::Diagnostics,
            M5PackageComponentDegradationState::UnknownOrStale,
            &rcs,
        ),
    ]
}

fn packet_with(bindings: Vec<PackageComponentConsumerBinding>) -> PackageComponentConsumerPacket {
    PackageComponentConsumerPacket::new(PackageComponentConsumerPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label: "Shared package-management-component consumers".to_owned(),
        consumer_bindings: bindings,
        downgrade_triggers: downgrade_triggers(),
        consumer_surfaces: consumer_surfaces(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-09T00:00:00Z".to_owned(),
    })
}

fn packet() -> PackageComponentConsumerPacket {
    packet_with(consumer_bindings())
}

#[test]
fn consumer_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn render_disclosure_maps_registry_state_to_mode() {
    let exact = resolve_package_component_render_disclosure(
        M5PackageComponentDegradationState::ResolvedExact,
    );
    assert_eq!(exact.expected_mode, PackageComponentRenderMode::FullParity);
    assert!(!exact.needs_narrow_banner);
    assert!(!exact.needs_continuity_note);
    assert!(!exact.needs_auth_note);

    let range = resolve_package_component_render_disclosure(
        M5PackageComponentDegradationState::ManifestRangeOnly,
    );
    assert_eq!(
        range.expected_mode,
        PackageComponentRenderMode::ManifestRangeNarrowed
    );
    assert!(range.needs_narrow_banner);
    assert!(!range.needs_continuity_note);
    assert!(!range.needs_auth_note);

    let mirror = resolve_package_component_render_disclosure(
        M5PackageComponentDegradationState::MirrorBacked,
    );
    assert_eq!(
        mirror.expected_mode,
        PackageComponentRenderMode::MirrorOrOfflineNarrowed
    );
    assert!(mirror.needs_continuity_note);
    assert!(!mirror.needs_auth_note);

    let offline = resolve_package_component_render_disclosure(
        M5PackageComponentDegradationState::OfflineSnapshotOnly,
    );
    assert_eq!(
        offline.expected_mode,
        PackageComponentRenderMode::MirrorOrOfflineNarrowed
    );
    assert!(offline.needs_continuity_note);

    let auth = resolve_package_component_render_disclosure(
        M5PackageComponentDegradationState::AuthRequiredUnsatisfied,
    );
    assert_eq!(
        auth.expected_mode,
        PackageComponentRenderMode::AuthRequiredNarrowed
    );
    assert!(auth.needs_auth_note);
    assert!(!auth.needs_continuity_note);

    let stale = resolve_package_component_render_disclosure(
        M5PackageComponentDegradationState::UnknownOrStale,
    );
    assert_eq!(
        stale.expected_mode,
        PackageComponentRenderMode::UnknownOrStaleNarrowed
    );
    assert!(stale.needs_narrow_banner);
    assert!(!stale.needs_continuity_note);
    assert!(!stale.needs_auth_note);
}

#[test]
fn parity_drift_across_surfaces_fails() {
    let mut packet = packet();
    // Reword the manifest-scope label on one surface for a shared object.
    packet.consumer_bindings[1]
        .parity_facets
        .manifest_scope_label = "Reworded scope for the search pane".to_owned();
    assert!(packet
        .validate()
        .contains(&PackageComponentConsumerViolation::ParityDriftAcrossSurfaces));
}

#[test]
fn registry_auth_drift_across_surfaces_fails() {
    let mut packet = packet();
    packet.consumer_bindings[7]
        .parity_facets
        .registry_source_auth_label = "Different registry label".to_owned();
    assert!(packet
        .validate()
        .contains(&PackageComponentConsumerViolation::ParityDriftAcrossSurfaces));
}

#[test]
fn component_reuse_by_single_consumer_fails() {
    let mut bindings = consumer_bindings();
    // Drop the second rollback-checkpoint-strip binding so it is adopted by one consumer.
    bindings.retain(|b| b.binding_id != "bind:rcs-8:diagnostics");
    let packet = packet_with(bindings);
    assert!(packet
        .validate()
        .contains(&PackageComponentConsumerViolation::PackageComponentReuseUnproven));
}

#[test]
fn missing_component_coverage_fails() {
    let mut bindings = consumer_bindings();
    bindings.retain(|b| b.component != M5PackageComponent::RollbackCheckpointStrip);
    let packet = packet_with(bindings);
    assert!(packet
        .validate()
        .contains(&PackageComponentConsumerViolation::ComponentCoverageMissing));
}

#[test]
fn missing_consumer_coverage_fails() {
    let mut bindings = consumer_bindings();
    // Remove the only Help-surface bindings.
    bindings.retain(|b| b.consumer != PackageComponentConsumer::HelpSurface);
    let packet = packet_with(bindings);
    assert!(packet
        .validate()
        .contains(&PackageComponentConsumerViolation::ConsumerCoverageMissing));
}

#[test]
fn help_support_export_without_canonical_refs_fails() {
    let mut packet = packet();
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| b.consumer == PackageComponentConsumer::SupportPacket)
        .expect("support-packet binding present");
    packet.consumer_bindings[index].source_contract_refs =
        vec![PACKAGE_COMPONENT_CONSUMER_COMPONENT_MATRIX_CONTRACT_REF.to_owned()];
    assert!(packet
        .validate()
        .contains(&PackageComponentConsumerViolation::HelpSupportExportReferenceMissing));
}

#[test]
fn render_mode_mismatch_fails() {
    let mut packet = packet();
    // Claim full parity on a mirror-backed registry-or-mirror row.
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| b.registry_state == M5PackageComponentDegradationState::MirrorBacked)
        .expect("mirror-backed binding present");
    packet.consumer_bindings[index].render_mode = PackageComponentRenderMode::FullParity;
    assert!(packet
        .validate()
        .contains(&PackageComponentConsumerViolation::RenderModeMismatch));
}

#[test]
fn parity_state_mismatch_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].parity_state = PackageComponentParityState::FacetsDisclosedNarrowed;
    assert!(packet
        .validate()
        .contains(&PackageComponentConsumerViolation::ParityStateMismatch));
}

#[test]
fn narrowed_binding_without_banner_fails() {
    let mut packet = packet();
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| b.is_narrowed())
        .expect("narrowed binding present");
    packet.consumer_bindings[index].narrow_banner = None;
    assert!(packet
        .validate()
        .contains(&PackageComponentConsumerViolation::NarrowBannerMissing));
}

#[test]
fn full_parity_binding_with_banner_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].narrow_banner = Some(PackageComponentNarrowBanner {
        reason: PackageComponentNarrowReason::MirrorOrOfflineContinuity,
        preserved_facets_note: "note".to_owned(),
        next_action: PackageComponentNarrowNextAction::ReviewMirrorOfflineContinuity,
        next_action_label: "Review".to_owned(),
    });
    assert!(packet
        .validate()
        .contains(&PackageComponentConsumerViolation::NarrowBannerMissing));
}

#[test]
fn narrow_reason_mismatch_fails() {
    let mut packet = packet();
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| b.registry_state == M5PackageComponentDegradationState::MirrorBacked)
        .expect("mirror-backed binding present");
    if let Some(banner) = packet.consumer_bindings[index].narrow_banner.as_mut() {
        banner.reason = PackageComponentNarrowReason::RegistryAuthRequired;
    }
    assert!(packet
        .validate()
        .contains(&PackageComponentConsumerViolation::NarrowReasonMismatch));
}

#[test]
fn narrow_banner_missing_preserved_facets_fails() {
    let mut packet = packet();
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| b.is_narrowed())
        .expect("narrowed binding present");
    if let Some(banner) = packet.consumer_bindings[index].narrow_banner.as_mut() {
        banner.preserved_facets_note = String::new();
    }
    assert!(packet
        .validate()
        .contains(&PackageComponentConsumerViolation::NarrowBannerPreservedFacetsMissing));
}

#[test]
fn continuity_note_missing_fails() {
    let mut packet = packet();
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| b.registry_state == M5PackageComponentDegradationState::OfflineSnapshotOnly)
        .expect("offline binding present");
    packet.consumer_bindings[index].continuity_note = String::new();
    assert!(packet
        .validate()
        .contains(&PackageComponentConsumerViolation::ContinuityNoteMissing));
}

#[test]
fn auth_note_missing_fails() {
    let mut packet = packet();
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| {
            b.registry_state == M5PackageComponentDegradationState::AuthRequiredUnsatisfied
        })
        .expect("auth-required binding present");
    packet.consumer_bindings[index].auth_note = String::new();
    assert!(packet
        .validate()
        .contains(&PackageComponentConsumerViolation::AuthNoteMissing));
}

#[test]
fn generic_manage_package_language_hiding_scope_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].uses_generic_manage_package_language_hiding_scope = true;
    assert!(packet
        .validate()
        .contains(&PackageComponentConsumerViolation::GenericManagePackageLanguageHidesScope));
}

#[test]
fn one_click_update_language_hiding_risk_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].uses_one_click_update_language_hiding_risk = true;
    assert!(packet
        .validate()
        .contains(&PackageComponentConsumerViolation::OneClickUpdateLanguageHidesRisk));
}

#[test]
fn registry_auth_posture_concealed_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].conceals_registry_auth_posture = true;
    assert!(packet
        .validate()
        .contains(&PackageComponentConsumerViolation::RegistryAuthPostureConcealed));
}

#[test]
fn broad_lockfile_regeneration_hidden_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].hides_broad_lockfile_regeneration = true;
    assert!(packet
        .validate()
        .contains(&PackageComponentConsumerViolation::BroadLockfileRegenerationHidden));
}

#[test]
fn mirror_offline_or_rollback_truth_dropped_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].drops_mirror_offline_or_rollback_truth = true;
    assert!(packet
        .validate()
        .contains(&PackageComponentConsumerViolation::MirrorOfflineOrRollbackTruthDropped));
}

#[test]
fn parity_facet_incomplete_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].parity_facets.recovery_language = String::new();
    assert!(packet
        .validate()
        .contains(&PackageComponentConsumerViolation::ParityFacetIncomplete));
}

#[test]
fn incomplete_binding_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].package_object_label = String::new();
    assert!(packet
        .validate()
        .contains(&PackageComponentConsumerViolation::BindingIncomplete));
}

#[test]
fn missing_bindings_fails() {
    let mut packet = packet();
    packet.consumer_bindings.clear();
    assert!(packet
        .validate()
        .contains(&PackageComponentConsumerViolation::ConsumerBindingsMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&PackageComponentConsumerViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.mirror_offline_continuity_kept_explicit = false;
    assert!(packet
        .validate()
        .contains(&PackageComponentConsumerViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .every_component_adopted_by_two_or_more_consumers = false;
    assert!(packet
        .validate()
        .contains(&PackageComponentConsumerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&PackageComponentConsumerViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_bindings() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Consumer bindings"));
    assert!(summary.contains("package_explorer_row"));
    assert!(summary.contains("rollback_checkpoint_strip"));
    assert!(summary.contains("mirror_or_offline_narrowed"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_package_component_consumer_export()
        .expect("checked package-consumer export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-package-management-component-consumers/mirror_and_offline_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-package-management-component-consumers/auth_required_and_stale.json"
        )),
    ] {
        let packet: PackageComponentConsumerPacket =
            serde_json::from_str(raw).expect("fixture parses as package-consumer packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

// --- Fixture builders ---------------------------------------------------------

/// Re-derives the canonical bindings after overriding some objects' registry state,
/// keeping the parity facets identical per object so the packet still validates.
fn bindings_with_registry_overrides(
    overrides: &[(&str, M5PackageComponentDegradationState)],
) -> Vec<PackageComponentConsumerBinding> {
    consumer_bindings()
        .into_iter()
        .map(|existing| {
            if let Some((_, state)) = overrides
                .iter()
                .find(|(object_id, _)| *object_id == existing.package_object_id)
            {
                binding(
                    &existing.binding_id,
                    &existing.package_object_id,
                    &existing.package_object_label,
                    existing.component,
                    existing.consumer,
                    *state,
                    &existing.parity_facets,
                )
            } else {
                existing
            }
        })
        .collect()
}

fn fixture_mirror_and_offline_narrowed() -> PackageComponentConsumerPacket {
    let bindings = bindings_with_registry_overrides(&[
        (
            "obj:per-1",
            M5PackageComponentDegradationState::MirrorBacked,
        ),
        (
            "obj:irs-3",
            M5PackageComponentDegradationState::OfflineSnapshotOnly,
        ),
    ]);
    PackageComponentConsumerPacket::new(PackageComponentConsumerPacketInput {
        packet_id: "package-component-consumer:fixture:mirror-and-offline-narrowed".to_owned(),
        surface_label: "Shared package-management-component consumers: mirror and offline narrowed"
            .to_owned(),
        consumer_bindings: bindings,
        downgrade_triggers: vec![
            M5PackageComponentDowngradeTrigger::MirrorBackedOnly,
            M5PackageComponentDowngradeTrigger::OfflineSnapshotOnly,
        ],
        consumer_surfaces: consumer_surfaces(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-09T00:00:00Z".to_owned(),
    })
}

fn fixture_auth_required_and_stale() -> PackageComponentConsumerPacket {
    let bindings = bindings_with_registry_overrides(&[
        (
            "obj:lic-6",
            M5PackageComponentDegradationState::AuthRequiredUnsatisfied,
        ),
        (
            "obj:per-1",
            M5PackageComponentDegradationState::UnknownOrStale,
        ),
    ]);
    PackageComponentConsumerPacket::new(PackageComponentConsumerPacketInput {
        packet_id: "package-component-consumer:fixture:auth-required-and-stale".to_owned(),
        surface_label: "Shared package-management-component consumers: auth required and stale"
            .to_owned(),
        consumer_bindings: bindings,
        downgrade_triggers: vec![
            M5PackageComponentDowngradeTrigger::AuthRequired,
            M5PackageComponentDowngradeTrigger::RegistryUnreachable,
        ],
        consumer_surfaces: consumer_surfaces(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-09T00:00:00Z".to_owned(),
    })
}

/// Regenerates the checked-in support export, summary, and fixtures.
///
/// Gated behind `GEN_PACKAGE_COMPONENT_CONSUMER_ARTIFACTS` so it never writes during
/// a normal test run. Run with the env var set to refresh the artifacts after a
/// contract change, then review the diff.
#[test]
fn regenerate_package_component_consumer_artifacts() {
    if std::env::var("GEN_PACKAGE_COMPONENT_CONSUMER_ARTIFACTS").is_err() {
        return;
    }

    let manifest = env!("CARGO_MANIFEST_DIR");
    let root = format!("{manifest}/../..");

    let canonical = packet();
    assert!(
        canonical.validate().is_empty(),
        "{:?}",
        canonical.validate()
    );

    let artifact_dir =
        format!("{root}/artifacts/release/m5-package-management-component-consumers-proof");
    std::fs::create_dir_all(&artifact_dir).expect("create artifact dir");
    std::fs::write(
        format!("{artifact_dir}/support_export.json"),
        format!("{}\n", canonical.export_safe_json()),
    )
    .expect("write support export");
    std::fs::write(
        format!("{artifact_dir}/summary.md"),
        canonical.render_markdown_summary(),
    )
    .expect("write summary");

    let fixture_dir = format!("{root}/fixtures/ui/m5-package-management-component-consumers");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");
    for (name, fixture) in [
        (
            "mirror_and_offline_narrowed.json",
            fixture_mirror_and_offline_narrowed(),
        ),
        (
            "auth_required_and_stale.json",
            fixture_auth_required_and_stale(),
        ),
    ] {
        assert!(
            fixture.validate().is_empty(),
            "{name}: {:?}",
            fixture.validate()
        );
        std::fs::write(
            format!("{fixture_dir}/{name}"),
            format!("{}\n", fixture.export_safe_json()),
        )
        .expect("write fixture");
    }
}
