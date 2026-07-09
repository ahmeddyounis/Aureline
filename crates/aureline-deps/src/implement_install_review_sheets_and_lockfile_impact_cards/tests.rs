use super::*;

const PACKET_ID: &str = "install-review-lockfile:stable:0001";

fn sheet(
    sheet_id: &str,
    package_label: &str,
    operation: MutationOperationClass,
    affected_manifests: Vec<&str>,
    affected_lockfiles: Vec<&str>,
    peer_conflict_count: u32,
    transitive_churn_count: u32,
    is_grouped: bool,
) -> InstallReviewSheet {
    let disclosure = resolve_review_change_breadth(
        peer_conflict_count,
        affected_manifests.len() as u32,
        affected_lockfiles.len() as u32,
        transitive_churn_count,
        is_grouped,
    );
    InstallReviewSheet {
        component: M5PackageComponent::InstallReviewSheet,
        sheet_id: sheet_id.to_owned(),
        operation,
        package_label: package_label.to_owned(),
        affected_manifests: affected_manifests.iter().map(|m| (*m).to_owned()).collect(),
        affected_lockfiles: affected_lockfiles.iter().map(|l| (*l).to_owned()).collect(),
        version_delta: "1.2.3 -> 1.4.0".to_owned(),
        peer_runtime_shift_note: if disclosure.needs_peer_runtime_note {
            "Peer react@17 conflicts with the requested react@18; runtime shifts to 18".to_owned()
        } else {
            String::new()
        },
        peer_conflict_count,
        transitive_churn_count,
        is_grouped,
        change_breadth: disclosure.change_breadth,
        warrants_deeper_inspection: disclosure.warrants_deeper_inspection,
        deeper_inspection_note: if disclosure.warrants_deeper_inspection {
            "This change is larger than a single package; review the lockfile-impact card"
                .to_owned()
        } else {
            String::new()
        },
        broad_change_note: if disclosure.needs_broad_change_note {
            "Broad change: several lockfiles regenerate or a peer conflict must be resolved"
                .to_owned()
        } else {
            String::new()
        },
        validation_tasks: vec!["build".to_owned(), "test".to_owned()],
        registry_auth_state_note: "registry.npmjs.org, anonymous public, fresh reachable"
            .to_owned(),
        degradation_state: M5PackageComponentDegradationState::ResolvedExact,
        degradation_note: String::new(),
        checkpoint_action_label: "Create checkpoint before applying".to_owned(),
        rollback_action_label: "Roll back to checkpoint".to_owned(),
        rollback_posture: M5PackageComponentRollbackPosture::WriteBackCheckpointed,
        fields_shown: vec![
            "package_label".to_owned(),
            "operation".to_owned(),
            "affected_manifests".to_owned(),
            "change_breadth".to_owned(),
        ],
        source_contract_refs: vec![
            M5_PACKAGE_COMPONENT_MATRIX_INSTALL_REVIEW_CONTRACT_REF.to_owned()
        ],
    }
}

fn sheets() -> Vec<InstallReviewSheet> {
    // Small single install.
    let small = sheet(
        "sheet:install-small",
        "left-pad",
        MutationOperationClass::Install,
        vec!["package.json"],
        vec!["package-lock.json"],
        0,
        2,
        false,
    );

    // Grouped update across members.
    let grouped = sheet(
        "sheet:update-grouped",
        "eslint + plugins",
        MutationOperationClass::Update,
        vec!["packages/web/package.json", "packages/api/package.json"],
        vec!["package-lock.json"],
        0,
        8,
        true,
    );

    // Broad remove with a peer conflict and multiple lockfiles.
    let broad = sheet(
        "sheet:remove-broad",
        "react (with peers)",
        MutationOperationClass::Remove,
        vec!["package.json"],
        vec!["package-lock.json", "packages/api/package-lock.json"],
        2,
        30,
        false,
    );

    vec![small, grouped, broad]
}

fn card(
    card_id: &str,
    resolver_label: &str,
    affected_lockfiles: Vec<&str>,
    direct_change_count: u32,
    transitive_churn_count: u32,
    platform_sensitive: bool,
    tool_version_sensitive: bool,
    write_mode: LockfileWriteMode,
) -> LockfileImpactCard {
    let disclosure = resolve_lockfile_churn(
        direct_change_count,
        transitive_churn_count,
        platform_sensitive,
        tool_version_sensitive,
    );
    LockfileImpactCard {
        component: M5PackageComponent::LockfileImpactCard,
        card_id: card_id.to_owned(),
        resolver_label: resolver_label.to_owned(),
        resolver_version: "10.8.2".to_owned(),
        affected_lockfiles: affected_lockfiles.iter().map(|l| (*l).to_owned()).collect(),
        direct_change_count,
        transitive_churn_count,
        churn_magnitude: disclosure.churn_magnitude,
        churn_note: if disclosure.needs_churn_note {
            "This write changes lockfile entries; see the counts".to_owned()
        } else {
            String::new()
        },
        platform_sensitive,
        tool_version_sensitive,
        platform_tool_note: if disclosure.needs_platform_tool_note {
            "Resolution is sensitive to the current platform / tool version".to_owned()
        } else {
            String::new()
        },
        write_mode,
        write_mode_note: match write_mode {
            LockfileWriteMode::RegenerateWholeLockfile => {
                "The whole lockfile regenerates from the manifest; manual edits are not kept"
                    .to_owned()
            }
            LockfileWriteMode::EditInPlaceEntries => {
                "Only the affected lockfile entries are edited in place".to_owned()
            }
            LockfileWriteMode::NoLockfileWrite => {
                "No lockfile is written by this change".to_owned()
            }
        },
        degradation_state: M5PackageComponentDegradationState::ResolvedExact,
        degradation_note: String::new(),
        rollback_posture: write_mode.expected_rollback_posture(),
        fields_shown: vec![
            "resolver_label".to_owned(),
            "affected_lockfiles".to_owned(),
            "churn_magnitude".to_owned(),
            "write_mode".to_owned(),
        ],
        source_contract_refs: vec![
            M5_PACKAGE_COMPONENT_MATRIX_LOCKFILE_IMPACT_CONTRACT_REF.to_owned()
        ],
    }
}

fn cards() -> Vec<LockfileImpactCard> {
    // Broad regenerate-from-source write.
    let regenerate = card(
        "card:regenerate-broad",
        "npm",
        vec!["package-lock.json"],
        5,
        40,
        true,
        false,
        LockfileWriteMode::RegenerateWholeLockfile,
    );

    // Narrow in-place edit.
    let edit = card(
        "card:edit-narrow",
        "npm",
        vec!["package-lock.json"],
        1,
        2,
        false,
        false,
        LockfileWriteMode::EditInPlaceEntries,
    );

    // No lockfile write (manifest/tool-only change).
    let none = card(
        "card:no-write",
        "npm",
        vec!["package-lock.json"],
        0,
        0,
        false,
        false,
        LockfileWriteMode::NoLockfileWrite,
    );

    vec![regenerate, edit, none]
}

fn trust_review() -> InstallReviewLockfileTrustReview {
    InstallReviewLockfileTrustReview {
        manifest_writes_always_explicit: true,
        lockfile_churn_never_understated: true,
        version_delta_always_explicit: true,
        peer_runtime_shifts_explicit: true,
        validation_expectations_explicit: true,
        registry_auth_state_explicit: true,
        change_breadth_quantified: true,
        resolver_identity_always_named: true,
        platform_tool_sensitivity_explicit: true,
        regenerate_versus_edit_explicit: true,
        rollback_checkpoint_always_offered: true,
        no_generic_confirm_language: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> InstallReviewLockfileConsumerProjection {
    InstallReviewLockfileConsumerProjection {
        install_review_sheet_shows_scope_and_churn: true,
        version_delta_and_peer_shifts_shown_inline: true,
        validation_and_registry_state_shown_inline: true,
        lockfile_card_shows_resolver_and_churn: true,
        regenerate_versus_edit_shown_inline: true,
        cli_headless_shows_control_truth: true,
        support_export_shows_control_truth: true,
        help_about_shows_control_truth: true,
    }
}

fn proof_freshness() -> InstallReviewLockfileProofFreshness {
    InstallReviewLockfileProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-07-08T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<M5PackageComponentDowngradeTrigger> {
    vec![
        M5PackageComponentDowngradeTrigger::ProofStale,
        M5PackageComponentDowngradeTrigger::LockfileDivergent,
        M5PackageComponentDowngradeTrigger::ScriptOrNativeBuildRisk,
        M5PackageComponentDowngradeTrigger::BroadLockfileRegeneration,
        M5PackageComponentDowngradeTrigger::RollbackUnavailable,
        M5PackageComponentDowngradeTrigger::UpstreamDependencyNarrowed,
    ]
}

fn consumer_surfaces() -> Vec<M5PackageComponentConsumerSurface> {
    vec![
        M5PackageComponentConsumerSurface::InstallUpdateReview,
        M5PackageComponentConsumerSurface::RollbackRecovery,
        M5PackageComponentConsumerSurface::CliHeadless,
        M5PackageComponentConsumerSurface::SupportExport,
        M5PackageComponentConsumerSurface::HelpAbout,
    ]
}

fn source_contract_refs() -> Vec<String> {
    vec![
        INSTALL_REVIEW_LOCKFILE_SCHEMA_REF.to_owned(),
        INSTALL_REVIEW_LOCKFILE_DOC_REF.to_owned(),
        M5_PACKAGE_COMPONENT_MATRIX_SCHEMA_REF.to_owned(),
        M5_PACKAGE_COMPONENT_MATRIX_DOC_REF.to_owned(),
        M5_PACKAGE_COMPONENT_MATRIX_INSTALL_REVIEW_CONTRACT_REF.to_owned(),
        M5_PACKAGE_COMPONENT_MATRIX_LOCKFILE_IMPACT_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> InstallReviewLockfileControlsPacket {
    InstallReviewLockfileControlsPacket::new(InstallReviewLockfileControlsPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label: "Install-review sheets and lockfile-impact cards".to_owned(),
        install_review_sheets: sheets(),
        lockfile_impact_cards: cards(),
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
fn install_review_lockfile_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn change_breadth_resolver_derives_from_blast_radius() {
    // Single manifest, single lockfile, little churn -> small single.
    let small = resolve_review_change_breadth(0, 1, 1, 2, false);
    assert_eq!(small.change_breadth, ReviewChangeBreadth::SmallSingle);
    assert!(!small.warrants_deeper_inspection);

    // Grouped flag lifts a bounded change to grouped.
    let grouped = resolve_review_change_breadth(0, 2, 1, 8, true);
    assert_eq!(grouped.change_breadth, ReviewChangeBreadth::GroupedChange);
    assert!(grouped.warrants_deeper_inspection);
    assert!(!grouped.needs_broad_change_note);

    // A peer conflict forces broad regardless of size.
    let broad = resolve_review_change_breadth(1, 1, 1, 1, false);
    assert_eq!(broad.change_breadth, ReviewChangeBreadth::BroadChange);
    assert!(broad.needs_peer_runtime_note);
    assert!(broad.needs_broad_change_note);

    // Several lockfiles also force broad.
    let multi_lock = resolve_review_change_breadth(0, 1, 2, 1, false);
    assert_eq!(multi_lock.change_breadth, ReviewChangeBreadth::BroadChange);
}

#[test]
fn lockfile_churn_resolver_derives_magnitude() {
    assert_eq!(
        resolve_lockfile_churn(0, 0, false, false).churn_magnitude,
        LockfileChurnMagnitude::NoChurn
    );
    assert_eq!(
        resolve_lockfile_churn(1, 2, false, false).churn_magnitude,
        LockfileChurnMagnitude::NarrowChurn
    );
    assert_eq!(
        resolve_lockfile_churn(5, 10, false, false).churn_magnitude,
        LockfileChurnMagnitude::ModerateChurn
    );
    let broad = resolve_lockfile_churn(5, 40, true, false);
    assert_eq!(broad.churn_magnitude, LockfileChurnMagnitude::BroadChurn);
    assert!(broad.is_broad_regeneration);
    assert!(broad.needs_platform_tool_note);
}

#[test]
fn write_mode_implies_rollback_posture() {
    assert_eq!(
        LockfileWriteMode::RegenerateWholeLockfile.expected_rollback_posture(),
        M5PackageComponentRollbackPosture::RegenerateOnlyNoManualEdit
    );
    assert_eq!(
        LockfileWriteMode::EditInPlaceEntries.expected_rollback_posture(),
        M5PackageComponentRollbackPosture::WriteBackCheckpointed
    );
    assert_eq!(
        LockfileWriteMode::NoLockfileWrite.expected_rollback_posture(),
        M5PackageComponentRollbackPosture::ReadOnlyNoMutation
    );
}

#[test]
fn sheet_misrepresenting_breadth_fails() {
    let mut packet = packet();
    let idx = packet
        .install_review_sheets
        .iter()
        .position(|s| s.change_breadth == ReviewChangeBreadth::BroadChange)
        .expect("broad sheet present");
    packet.install_review_sheets[idx].change_breadth = ReviewChangeBreadth::SmallSingle;
    assert!(packet
        .validate()
        .contains(&InstallReviewLockfileViolation::ChangeBreadthMisrepresented));
}

#[test]
fn sheet_hiding_manifest_writes_fails() {
    let mut packet = packet();
    packet.install_review_sheets[0].affected_manifests.clear();
    assert!(packet
        .validate()
        .contains(&InstallReviewLockfileViolation::AffectedManifestsMissing));
}

#[test]
fn sheet_hiding_validation_tasks_fails() {
    let mut packet = packet();
    packet.install_review_sheets[0].validation_tasks.clear();
    assert!(packet
        .validate()
        .contains(&InstallReviewLockfileViolation::ValidationTasksMissing));
}

#[test]
fn sheet_missing_version_delta_fails() {
    let mut packet = packet();
    packet.install_review_sheets[0].version_delta = String::new();
    assert!(packet
        .validate()
        .contains(&InstallReviewLockfileViolation::VersionDeltaMissing));
}

#[test]
fn broad_sheet_without_peer_note_fails() {
    let mut packet = packet();
    let idx = packet
        .install_review_sheets
        .iter()
        .position(|s| s.peer_conflict_count > 0)
        .expect("peer-conflict sheet present");
    packet.install_review_sheets[idx].peer_runtime_shift_note = String::new();
    assert!(packet
        .validate()
        .contains(&InstallReviewLockfileViolation::PeerRuntimeNoteMissing));
}

#[test]
fn broad_sheet_without_broad_note_fails() {
    let mut packet = packet();
    let idx = packet
        .install_review_sheets
        .iter()
        .position(|s| s.change_breadth == ReviewChangeBreadth::BroadChange)
        .expect("broad sheet present");
    packet.install_review_sheets[idx].broad_change_note = String::new();
    assert!(packet
        .validate()
        .contains(&InstallReviewLockfileViolation::BroadChangeNoteMissing));
}

#[test]
fn sheet_missing_checkpoint_action_fails() {
    let mut packet = packet();
    packet.install_review_sheets[0].checkpoint_action_label = String::new();
    assert!(packet
        .validate()
        .contains(&InstallReviewLockfileViolation::CheckpointActionMissing));
}

#[test]
fn sheet_write_back_directly_posture_fails() {
    let mut packet = packet();
    packet.install_review_sheets[0].rollback_posture =
        M5PackageComponentRollbackPosture::ReadOnlyNoMutation;
    assert!(packet
        .validate()
        .contains(&InstallReviewLockfileViolation::ReviewSheetRollbackPostureInconsistent));
}

#[test]
fn sheet_wrong_component_class_fails() {
    let mut packet = packet();
    packet.install_review_sheets[0].component = M5PackageComponent::LockfileImpactCard;
    assert!(packet
        .validate()
        .contains(&InstallReviewLockfileViolation::ReviewSheetWrongComponentClass));
}

#[test]
fn missing_operation_coverage_fails() {
    let mut packet = packet();
    packet
        .install_review_sheets
        .retain(|s| s.operation != MutationOperationClass::Remove);
    let violations = packet.validate();
    assert!(violations.contains(&InstallReviewLockfileViolation::OperationCoverageMissing));
}

#[test]
fn missing_breadth_coverage_fails() {
    let mut packet = packet();
    packet
        .install_review_sheets
        .retain(|s| s.change_breadth != ReviewChangeBreadth::GroupedChange);
    assert!(packet
        .validate()
        .contains(&InstallReviewLockfileViolation::BreadthCoverageMissing));
}

#[test]
fn empty_sheets_fails() {
    let mut packet = packet();
    packet.install_review_sheets.clear();
    assert!(packet
        .validate()
        .contains(&InstallReviewLockfileViolation::ReviewSheetsMissing));
}

#[test]
fn card_misrepresenting_churn_fails() {
    let mut packet = packet();
    let idx = packet
        .lockfile_impact_cards
        .iter()
        .position(|c| c.churn_magnitude == LockfileChurnMagnitude::BroadChurn)
        .expect("broad-churn card present");
    packet.lockfile_impact_cards[idx].churn_magnitude = LockfileChurnMagnitude::NarrowChurn;
    assert!(packet
        .validate()
        .contains(&InstallReviewLockfileViolation::ChurnMagnitudeMisrepresented));
}

#[test]
fn card_hiding_resolver_identity_fails() {
    let mut packet = packet();
    packet.lockfile_impact_cards[0].resolver_version = String::new();
    assert!(packet
        .validate()
        .contains(&InstallReviewLockfileViolation::ResolverIdentityMissing));
}

#[test]
fn card_hiding_affected_lockfiles_fails() {
    let mut packet = packet();
    packet.lockfile_impact_cards[0].affected_lockfiles.clear();
    assert!(packet
        .validate()
        .contains(&InstallReviewLockfileViolation::AffectedLockfilesMissing));
}

#[test]
fn card_without_churn_note_fails() {
    let mut packet = packet();
    let idx = packet
        .lockfile_impact_cards
        .iter()
        .position(|c| c.churn_magnitude != LockfileChurnMagnitude::NoChurn)
        .expect("churned card present");
    packet.lockfile_impact_cards[idx].churn_note = String::new();
    assert!(packet
        .validate()
        .contains(&InstallReviewLockfileViolation::ChurnNoteMissing));
}

#[test]
fn platform_sensitive_card_without_note_fails() {
    let mut packet = packet();
    let idx = packet
        .lockfile_impact_cards
        .iter()
        .position(|c| c.platform_sensitive || c.tool_version_sensitive)
        .expect("platform-sensitive card present");
    packet.lockfile_impact_cards[idx].platform_tool_note = String::new();
    assert!(packet
        .validate()
        .contains(&InstallReviewLockfileViolation::PlatformToolNoteMissing));
}

#[test]
fn card_without_write_mode_note_fails() {
    let mut packet = packet();
    packet.lockfile_impact_cards[0].write_mode_note = String::new();
    assert!(packet
        .validate()
        .contains(&InstallReviewLockfileViolation::WriteModeNoteMissing));
}

#[test]
fn regenerate_card_claiming_manual_edit_fails() {
    let mut packet = packet();
    let idx = packet
        .lockfile_impact_cards
        .iter()
        .position(|c| c.write_mode == LockfileWriteMode::RegenerateWholeLockfile)
        .expect("regenerate card present");
    packet.lockfile_impact_cards[idx].rollback_posture =
        M5PackageComponentRollbackPosture::WriteBackCheckpointed;
    assert!(packet
        .validate()
        .contains(&InstallReviewLockfileViolation::CardRollbackPostureInconsistent));
}

#[test]
fn card_wrong_component_class_fails() {
    let mut packet = packet();
    packet.lockfile_impact_cards[0].component = M5PackageComponent::InstallReviewSheet;
    assert!(packet
        .validate()
        .contains(&InstallReviewLockfileViolation::LockfileCardWrongComponentClass));
}

#[test]
fn missing_write_mode_coverage_fails() {
    let mut packet = packet();
    packet
        .lockfile_impact_cards
        .retain(|c| c.write_mode != LockfileWriteMode::EditInPlaceEntries);
    assert!(packet
        .validate()
        .contains(&InstallReviewLockfileViolation::WriteModeCoverageMissing));
}

#[test]
fn empty_cards_fails() {
    let mut packet = packet();
    packet.lockfile_impact_cards.clear();
    assert!(packet
        .validate()
        .contains(&InstallReviewLockfileViolation::LockfileCardsMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&InstallReviewLockfileViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.no_generic_confirm_language = false;
    assert!(packet
        .validate()
        .contains(&InstallReviewLockfileViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .regenerate_versus_edit_shown_inline = false;
    assert!(packet
        .validate()
        .contains(&InstallReviewLockfileViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&InstallReviewLockfileViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_controls() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Install-review sheets"));
    assert!(summary.contains("## Lockfile-impact cards"));
    assert!(summary.contains("broad_change"));
    assert!(summary.contains("regenerate_whole_lockfile"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_install_review_lockfile_export()
        .expect("checked install review lockfile export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-install-review-lockfile-controls/broad_peer_conflict.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-install-review-lockfile-controls/regenerate_broad_churn.json"
        )),
    ] {
        let packet: InstallReviewLockfileControlsPacket =
            serde_json::from_str(raw).expect("fixture parses as install review lockfile packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

/// Regenerates the checked release artifacts and narrowed fixtures.
///
/// Guarded behind `GEN_INSTALL_REVIEW_LOCKFILE_ARTIFACTS` so ordinary test runs
/// never touch the working tree.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_INSTALL_REVIEW_LOCKFILE_ARTIFACTS").is_err() {
        return;
    }

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let repo_root = std::path::Path::new(manifest_dir).join("..").join("..");

    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    let proof_dir = repo_root
        .join("artifacts")
        .join("release")
        .join("m5-install-review-lockfile-proof");
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
        .join("m5-install-review-lockfile-controls");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");

    // Fixture 1: spotlight a broad remove that must resolve a peer conflict and
    // regenerates several lockfiles — it can never read as a small change.
    let mut broad = packet.clone();
    broad.packet_id = "install-review-lockfile:fixture:broad-peer-conflict".to_owned();
    broad.install_review_sheets = sheets();
    broad.lockfile_impact_cards = cards();
    assert!(broad.validate().is_empty(), "{:?}", broad.validate());
    std::fs::write(
        fixture_dir.join("broad_peer_conflict.json"),
        format!("{}\n", broad.export_safe_json()),
    )
    .expect("write broad fixture");

    // Fixture 2: a regenerate-from-source lockfile with broad churn — the write
    // mode and rollback posture must stay consistent and churn is never
    // understated.
    let mut regenerate = packet.clone();
    regenerate.packet_id = "install-review-lockfile:fixture:regenerate-broad-churn".to_owned();
    for card in regenerate.lockfile_impact_cards.iter_mut() {
        if card.write_mode == LockfileWriteMode::RegenerateWholeLockfile {
            card.degradation_state = M5PackageComponentDegradationState::OfflineSnapshotOnly;
            card.degradation_note =
                "Offline snapshot only; churn is estimated from the local cache".to_owned();
        }
    }
    assert!(
        regenerate.validate().is_empty(),
        "{:?}",
        regenerate.validate()
    );
    std::fs::write(
        fixture_dir.join("regenerate_broad_churn.json"),
        format!("{}\n", regenerate.export_safe_json()),
    )
    .expect("write regenerate fixture");
}
