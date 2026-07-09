use super::*;

const PACKET_ID: &str = "package-explorer-row:stable:0001";

fn row(
    row_id: &str,
    package_label: &str,
    lifecycle: PackageLifecycleState,
    relation: PackageDependencyRelation,
) -> PackageExplorerRow {
    PackageExplorerRow {
        component: M5PackageComponent::PackageExplorerRow,
        row_id: row_id.to_owned(),
        package_label: package_label.to_owned(),
        ecosystem: "npm".to_owned(),
        current_version: "1.2.3".to_owned(),
        candidate_version: String::new(),
        relation,
        lifecycle,
        manifest_scope: PackageManifestScopeClass::RuntimeDependency,
        manifest_scope_disclosure: "Declared in packages/web/package.json (dependencies)"
            .to_owned(),
        relation_note: String::new(),
        registry_source: PackageRegistrySourceClass::PublicRegistry,
        registry_source_disclosure: "Resolved from the public npm registry".to_owned(),
        degradation_state: M5PackageComponentDegradationState::ResolvedExact,
        degradation_note: String::new(),
        license_signal: PackageLicenseSignal::Allowed,
        advisory_signal: PackageAdvisorySignal::NoKnownAdvisory,
        changelog_signal: PackageChangelogSignal::Available,
        signal_disclosure: "License allowed, no known advisory, changelog available".to_owned(),
        offers_direct_action: false,
        primary_action_label: "View".to_owned(),
        action_provenance_note: String::new(),
        blocked_reason: String::new(),
        rollback_posture: M5PackageComponentRollbackPosture::ReadOnlyNoMutation,
        fields_shown: vec![
            "package_label".to_owned(),
            "manifest_scope".to_owned(),
            "relation".to_owned(),
            "lifecycle".to_owned(),
            "registry_source".to_owned(),
            "signals".to_owned(),
        ],
        source_contract_refs: vec![M5_PACKAGE_COMPONENT_MATRIX_EXPLORER_ROW_CONTRACT_REF.to_owned()],
    }
}

fn rows() -> Vec<PackageExplorerRow> {
    let mut installed = row(
        "row:left-pad",
        "left-pad",
        PackageLifecycleState::Installed,
        PackageDependencyRelation::Direct,
    );
    installed.offers_direct_action = true;
    installed.primary_action_label = "Update or remove".to_owned();
    installed.action_provenance_note =
        "Manages the direct runtime dependency in packages/web/package.json".to_owned();
    installed.rollback_posture = M5PackageComponentRollbackPosture::WriteBackCheckpointed;

    let mut available = row(
        "row:chalk",
        "chalk",
        PackageLifecycleState::Available,
        PackageDependencyRelation::Direct,
    );
    available.current_version = String::new();
    available.candidate_version = "5.3.0".to_owned();
    available.offers_direct_action = true;
    available.primary_action_label = "Install 5.3.0".to_owned();
    available.action_provenance_note =
        "Installs into packages/web/package.json (dependencies) from the enterprise mirror"
            .to_owned();
    available.registry_source = PackageRegistrySourceClass::EnterpriseMirror;
    available.registry_source_disclosure =
        "Answered by the enterprise mirror, not the upstream registry".to_owned();
    available.degradation_state = M5PackageComponentDegradationState::MirrorBacked;
    available.degradation_note =
        "Mirror-backed answer; upstream registry was not consulted for this resolution".to_owned();
    available.rollback_posture = M5PackageComponentRollbackPosture::WriteBackCheckpointed;

    let mut outdated = row(
        "row:lodash",
        "lodash",
        PackageLifecycleState::Outdated,
        PackageDependencyRelation::Direct,
    );
    outdated.candidate_version = "4.17.21".to_owned();
    outdated.offers_direct_action = true;
    outdated.primary_action_label = "Update to 4.17.21".to_owned();
    outdated.action_provenance_note =
        "Updates the direct runtime dependency in packages/web/package.json".to_owned();
    outdated.advisory_signal = PackageAdvisorySignal::AdvisoryHigh;
    outdated.changelog_signal = PackageChangelogSignal::BreakingChangeNoted;
    outdated.signal_disclosure =
        "License allowed, high-severity advisory on the current version, breaking change noted"
            .to_owned();
    outdated.rollback_posture = M5PackageComponentRollbackPosture::WriteBackCheckpointed;

    let mut transitive = row(
        "row:ms",
        "ms",
        PackageLifecycleState::Installed,
        PackageDependencyRelation::Transitive,
    );
    transitive.relation_note = "Pulled in transitively by chalk; not directly declared".to_owned();
    transitive.blocked_reason =
        "Transitive dependency: update the parent (chalk) rather than this row directly".to_owned();
    transitive.primary_action_label = "View parent".to_owned();

    let mut imported = row(
        "row:vendored-icons",
        "vendored-icons",
        PackageLifecycleState::Imported,
        PackageDependencyRelation::Direct,
    );
    imported.registry_source = PackageRegistrySourceClass::PathOrVendored;
    imported.registry_source_disclosure = "Vendored under third_party/icons".to_owned();
    imported.blocked_reason =
        "Imported/vendored snapshot; the canonical source lives in the upstream project".to_owned();
    imported.primary_action_label = "View source".to_owned();

    let mut pinned = row(
        "row:openssl-sys",
        "openssl-sys",
        PackageLifecycleState::PolicyPinned,
        PackageDependencyRelation::Direct,
    );
    pinned.blocked_reason =
        "Pinned by security policy; the version is held until policy is updated".to_owned();
    pinned.primary_action_label = "View policy".to_owned();

    let mut remove_blocked = row(
        "row:react",
        "react",
        PackageLifecycleState::RemoveBlocked,
        PackageDependencyRelation::Direct,
    );
    remove_blocked.blocked_reason =
        "Removal blocked: react-dom and 3 other packages depend on this".to_owned();
    remove_blocked.primary_action_label = "View dependents".to_owned();

    vec![
        installed,
        available,
        outdated,
        transitive,
        imported,
        pinned,
        remove_blocked,
    ]
}

fn trust_review() -> PackageExplorerRowTrustReview {
    PackageExplorerRowTrustReview {
        package_identity_always_explicit: true,
        manifest_scope_always_explicit: true,
        direct_transitive_relation_explicit: true,
        registry_source_always_explicit: true,
        license_advisory_changelog_signals_explicit: true,
        lifecycle_state_visually_distinct: true,
        action_truth_matches_state: true,
        no_generic_action_without_provenance: true,
        transitive_and_blocked_states_name_reason: true,
        mirror_offline_degradation_explicit: true,
        copy_and_export_safe: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> PackageExplorerRowConsumerProjection {
    PackageExplorerRowConsumerProjection {
        row_shows_label_scope_and_relation: true,
        lifecycle_state_shown_distinctly: true,
        registry_source_and_signals_shown: true,
        action_reflects_state_and_provenance: true,
        blocked_reason_shown_inline: true,
        cli_headless_shows_row_truth: true,
        support_export_shows_row_truth: true,
        help_about_shows_row_truth: true,
    }
}

fn proof_freshness() -> PackageExplorerRowProofFreshness {
    PackageExplorerRowProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-07-08T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<M5PackageComponentDowngradeTrigger> {
    vec![
        M5PackageComponentDowngradeTrigger::ProofStale,
        M5PackageComponentDowngradeTrigger::MirrorBackedOnly,
        M5PackageComponentDowngradeTrigger::OfflineSnapshotOnly,
        M5PackageComponentDowngradeTrigger::PolicyBlocked,
        M5PackageComponentDowngradeTrigger::UpstreamDependencyNarrowed,
    ]
}

fn consumer_surfaces() -> Vec<M5PackageComponentConsumerSurface> {
    vec![
        M5PackageComponentConsumerSurface::PackageWorkspace,
        M5PackageComponentConsumerSurface::DependencyExplorer,
        M5PackageComponentConsumerSurface::CliHeadless,
        M5PackageComponentConsumerSurface::SupportExport,
        M5PackageComponentConsumerSurface::HelpAbout,
    ]
}

fn source_contract_refs() -> Vec<String> {
    vec![
        PACKAGE_EXPLORER_ROW_SCHEMA_REF.to_owned(),
        PACKAGE_EXPLORER_ROW_DOC_REF.to_owned(),
        M5_PACKAGE_COMPONENT_MATRIX_SCHEMA_REF.to_owned(),
        M5_PACKAGE_COMPONENT_MATRIX_DOC_REF.to_owned(),
        M5_PACKAGE_COMPONENT_MATRIX_EXPLORER_ROW_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> PackageExplorerRowPacket {
    PackageExplorerRowPacket::new(PackageExplorerRowPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label: "Package explorer rows".to_owned(),
        rows: rows(),
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
fn package_explorer_row_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn resolver_derives_action_from_lifecycle_and_relation() {
    // Direct + available is directly installable and needs a candidate.
    let available = resolve_package_explorer_row_action(
        PackageLifecycleState::Available,
        PackageDependencyRelation::Direct,
    );
    assert_eq!(
        available.action_class,
        PackageRowActionClass::InstallAvailable
    );
    assert!(available.is_directly_actionable);
    assert!(available.needs_candidate_version);
    assert!(!available.needs_blocked_reason);

    // Transitive dominates: an installed transitive package is read-only.
    let transitive = resolve_package_explorer_row_action(
        PackageLifecycleState::Installed,
        PackageDependencyRelation::Transitive,
    );
    assert_eq!(
        transitive.action_class,
        PackageRowActionClass::TransitiveReadOnly
    );
    assert!(!transitive.is_directly_actionable);
    assert!(transitive.needs_blocked_reason);
    assert!(transitive.needs_relation_note);

    // Policy-pinned is blocked regardless of relation.
    let pinned = resolve_package_explorer_row_action(
        PackageLifecycleState::PolicyPinned,
        PackageDependencyRelation::Direct,
    );
    assert_eq!(
        pinned.action_class,
        PackageRowActionClass::PolicyPinnedBlocked
    );
    assert!(!pinned.is_directly_actionable);
}

#[test]
fn transitive_row_claiming_direct_action_fails() {
    let mut packet = packet();
    // The transitive row must not offer a direct action.
    let idx = packet
        .rows
        .iter()
        .position(|row| row.relation.is_pure_transitive())
        .expect("transitive row present");
    packet.rows[idx].offers_direct_action = true;
    packet.rows[idx].action_provenance_note = "should not be allowed".to_owned();
    assert!(packet
        .validate()
        .contains(&PackageExplorerRowViolation::ActionTruthMisrepresented));
}

#[test]
fn installed_row_hiding_its_action_fails() {
    let mut packet = packet();
    packet.rows[0].offers_direct_action = false;
    assert!(packet
        .validate()
        .contains(&PackageExplorerRowViolation::ActionTruthMisrepresented));
}

#[test]
fn direct_action_without_provenance_fails() {
    let mut packet = packet();
    packet.rows[0].action_provenance_note = String::new();
    assert!(packet
        .validate()
        .contains(&PackageExplorerRowViolation::ActionProvenanceMissing));
}

#[test]
fn blocked_row_without_reason_fails() {
    let mut packet = packet();
    let idx = packet
        .rows
        .iter()
        .position(|row| row.lifecycle == PackageLifecycleState::RemoveBlocked)
        .expect("remove-blocked row present");
    packet.rows[idx].blocked_reason = String::new();
    assert!(packet
        .validate()
        .contains(&PackageExplorerRowViolation::BlockedReasonMissing));
}

#[test]
fn transitive_row_without_relation_note_fails() {
    let mut packet = packet();
    let idx = packet
        .rows
        .iter()
        .position(|row| row.relation.is_pure_transitive())
        .expect("transitive row present");
    packet.rows[idx].relation_note = String::new();
    assert!(packet
        .validate()
        .contains(&PackageExplorerRowViolation::TransitiveRelationNotExplained));
}

#[test]
fn install_row_without_candidate_version_fails() {
    let mut packet = packet();
    let idx = packet
        .rows
        .iter()
        .position(|row| row.lifecycle == PackageLifecycleState::Available)
        .expect("available row present");
    packet.rows[idx].candidate_version = String::new();
    assert!(packet
        .validate()
        .contains(&PackageExplorerRowViolation::CandidateVersionMissing));
}

#[test]
fn degraded_resolution_without_note_fails() {
    let mut packet = packet();
    let idx = packet
        .rows
        .iter()
        .position(|row| row.degradation_state == M5PackageComponentDegradationState::MirrorBacked)
        .expect("mirror-backed row present");
    packet.rows[idx].degradation_note = String::new();
    assert!(packet
        .validate()
        .contains(&PackageExplorerRowViolation::DegradationNoteMissing));
}

#[test]
fn missing_manifest_scope_disclosure_fails() {
    let mut packet = packet();
    packet.rows[0].manifest_scope_disclosure = String::new();
    assert!(packet
        .validate()
        .contains(&PackageExplorerRowViolation::ManifestScopeDisclosureMissing));
}

#[test]
fn missing_registry_source_disclosure_fails() {
    let mut packet = packet();
    packet.rows[0].registry_source_disclosure = String::new();
    assert!(packet
        .validate()
        .contains(&PackageExplorerRowViolation::RegistrySourceDisclosureMissing));
}

#[test]
fn missing_signal_disclosure_fails() {
    let mut packet = packet();
    packet.rows[0].signal_disclosure = String::new();
    assert!(packet
        .validate()
        .contains(&PackageExplorerRowViolation::SignalDisclosureMissing));
}

#[test]
fn wrong_component_class_fails() {
    let mut packet = packet();
    packet.rows[0].component = M5PackageComponent::ManifestScopeSwitcher;
    assert!(packet
        .validate()
        .contains(&PackageExplorerRowViolation::RowWrongComponentClass));
}

#[test]
fn inconsistent_rollback_posture_fails() {
    let mut packet = packet();
    // A directly-actionable installed row must not be read-only.
    packet.rows[0].rollback_posture = M5PackageComponentRollbackPosture::ReadOnlyNoMutation;
    assert!(packet
        .validate()
        .contains(&PackageExplorerRowViolation::RollbackPostureInconsistent));
}

#[test]
fn missing_lifecycle_coverage_fails() {
    let mut packet = packet();
    packet
        .rows
        .retain(|row| row.lifecycle != PackageLifecycleState::Outdated);
    assert!(packet
        .validate()
        .contains(&PackageExplorerRowViolation::LifecycleCoverageMissing));
}

#[test]
fn missing_non_actionable_coverage_fails() {
    let mut packet = packet();
    packet
        .rows
        .retain(|row| row.action_disclosure().is_directly_actionable);
    assert!(packet
        .validate()
        .contains(&PackageExplorerRowViolation::NonActionableStateCoverageMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&PackageExplorerRowViolation::MissingSourceContracts));
}

#[test]
fn empty_rows_fails() {
    let mut packet = packet();
    packet.rows.clear();
    assert!(packet
        .validate()
        .contains(&PackageExplorerRowViolation::RowsMissing));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.action_truth_matches_state = false;
    assert!(packet
        .validate()
        .contains(&PackageExplorerRowViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet.consumer_projection.blocked_reason_shown_inline = false;
    assert!(packet
        .validate()
        .contains(&PackageExplorerRowViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&PackageExplorerRowViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_rows() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Rows"));
    assert!(summary.contains("left-pad"));
    assert!(summary.contains("transitive_read_only"));
    assert!(summary.contains("install_available"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_package_explorer_row_export()
        .expect("checked package explorer row export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-package-explorer-row/transitive_read_only.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-package-explorer-row/offline_snapshot_degraded.json"
        )),
    ] {
        let packet: PackageExplorerRowPacket =
            serde_json::from_str(raw).expect("fixture parses as package explorer row packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

/// Regenerates the checked release artifacts and narrowed fixtures.
///
/// Guarded behind `GEN_PACKAGE_EXPLORER_ROW_ARTIFACTS` so ordinary test runs
/// never touch the working tree.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_PACKAGE_EXPLORER_ROW_ARTIFACTS").is_err() {
        return;
    }

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let repo_root = std::path::Path::new(manifest_dir).join("..").join("..");

    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    let proof_dir = repo_root
        .join("artifacts")
        .join("release")
        .join("m5-package-explorer-row-proof");
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
        .join("m5-package-explorer-row");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");

    // Fixture 1: spotlight a purely transitive, read-only row that names its
    // parent instead of offering a direct action.
    let mut transitive = packet.clone();
    transitive.packet_id = "package-explorer-row:fixture:transitive-read-only".to_owned();
    transitive.rows = packet
        .rows
        .iter()
        .filter(|row| {
            matches!(
                row.lifecycle,
                PackageLifecycleState::Installed
                    | PackageLifecycleState::Available
                    | PackageLifecycleState::Outdated
            ) || row.relation.is_pure_transitive()
        })
        .cloned()
        .collect();
    assert!(
        transitive.validate().is_empty(),
        "{:?}",
        transitive.validate()
    );
    std::fs::write(
        fixture_dir.join("transitive_read_only.json"),
        format!("{}\n", transitive.export_safe_json()),
    )
    .expect("write transitive fixture");

    // Fixture 2: an offline-snapshot resolution that never reads as a clean
    // upstream install.
    let mut offline = packet.clone();
    offline.packet_id = "package-explorer-row:fixture:offline-snapshot".to_owned();
    if let Some(row) = offline
        .rows
        .iter_mut()
        .find(|row| row.lifecycle == PackageLifecycleState::Available)
    {
        row.registry_source = PackageRegistrySourceClass::OfflineSnapshot;
        row.registry_source_disclosure =
            "Resolved from the offline snapshot; the registry was not reached".to_owned();
        row.degradation_state = M5PackageComponentDegradationState::OfflineSnapshotOnly;
        row.degradation_note =
            "Offline snapshot only; install continues from the local cache".to_owned();
        row.action_provenance_note =
            "Installs into packages/web/package.json from the offline snapshot".to_owned();
    }
    assert!(offline.validate().is_empty(), "{:?}", offline.validate());
    std::fs::write(
        fixture_dir.join("offline_snapshot_degraded.json"),
        format!("{}\n", offline.export_safe_json()),
    )
    .expect("write offline fixture");
}
