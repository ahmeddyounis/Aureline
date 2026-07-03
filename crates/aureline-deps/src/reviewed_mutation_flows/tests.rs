use super::*;

fn packet() -> ReviewedMutationFlows {
    current_reviewed_mutation_flows().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        REVIEWED_MUTATION_FLOWS_SCHEMA_VERSION
    );
    assert_eq!(packet.record_kind, REVIEWED_MUTATION_FLOWS_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_rows() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn path_is_stable() {
    assert_eq!(
        REVIEWED_MUTATION_FLOWS_PATH,
        "artifacts/deps/m5/reviewed-mutation-flows.json"
    );
}

#[test]
fn corpus_covers_every_required_state() {
    let packet = packet();
    assert_eq!(packet.corpus_coverage_gaps(), Vec::new());
}

#[test]
fn every_flow_is_represented() {
    let packet = packet();
    let flows: BTreeSet<MutationFlowClass> = packet.sheets.iter().map(|s| s.flow_class).collect();
    for flow in MutationFlowClass::ALL {
        assert!(
            flows.contains(&flow),
            "no sheet exercises flow {}",
            flow.as_str()
        );
    }
}

#[test]
fn every_sheet_binds_to_the_frozen_matrix() {
    let packet = packet();
    let matrix = current_m5_package_state_matrix().expect("matrix loads");
    assert_eq!(packet.references_matrix_id, matrix.packet_id);
    assert!(packet.all_bind_matrix());
    for sheet in &packet.sheets {
        for label in &sheet.applicable_labels {
            assert!(
                matrix.state(*label).is_some(),
                "sheet {} surfaces unbound label {}",
                sheet.sheet_id,
                label.as_str()
            );
        }
    }
}

#[test]
fn every_sheet_discloses_required_before_commit() {
    let packet = packet();
    assert!(packet.all_disclose_required());
    for sheet in &packet.sheets {
        assert!(
            sheet.discloses_all_required(),
            "sheet {} omits a required disclosure",
            sheet.sheet_id
        );
    }
}

#[test]
fn script_and_native_build_labels_stay_distinct() {
    let packet = packet();
    let known = packet
        .sheet("rmf:update:node:lodash-bump")
        .expect("known scripts");
    assert_eq!(
        known.script_build.label,
        ScriptBuildLabel::KnownInstallScripts
    );
    assert!(known.script_build.label.requires_explicit_ack());
    assert!(!known.script_build.label.blocks_commit());

    let native = packet
        .sheet("rmf:install:pip:numpy-native")
        .expect("native build");
    assert_eq!(
        native.script_build.label,
        ScriptBuildLabel::NativeBuildRequired
    );
    assert!(!native.script_build.label.blocks_commit());
    assert!(!native.script_build.required_toolchain_refs.is_empty());

    // A native build is a different label from an install script: not collapsed.
    assert_ne!(known.script_build.label, native.script_build.label);
}

#[test]
fn unknown_hook_risk_blocks_commit() {
    let packet = packet();
    let sheet = packet
        .sheet("rmf:regenerate:node:relock-unknown")
        .expect("unknown risk");
    assert_eq!(sheet.script_build.label, ScriptBuildLabel::UnknownHookRisk);
    assert!(sheet.intrinsic_commit_blocked());
    assert!(packet.commit_blocked(sheet));
}

#[test]
fn policy_blocked_flow_cannot_commit() {
    let packet = packet();
    let sheet = packet
        .sheet("rmf:regenerate:cargo:policy-blocked")
        .expect("policy blocked");
    assert_eq!(sheet.script_build.label, ScriptBuildLabel::PolicyBlocked);
    assert!(!sheet.script_build.policy_allows);
    assert!(packet.commit_blocked(sheet));
    // An AI proposal cannot bypass the policy block.
    assert!(sheet.proposal_source.is_automated());
}

#[test]
fn regenerate_flows_disclose_resolver_version_and_broad_churn() {
    let packet = packet();
    for sheet in packet
        .sheets
        .iter()
        .filter(|s| s.flow_class == MutationFlowClass::Regenerate)
    {
        assert!(
            !sheet.resolver.resolver_version.trim().is_empty(),
            "regenerate sheet {} hides its resolver version",
            sheet.sheet_id
        );
        if sheet.lockfile.diff_class.is_broad_churn() {
            assert!(
                sheet.lockfile.broad_churn_disclosed,
                "regenerate sheet {} hides a broad churn",
                sheet.sheet_id
            );
        }
    }
}

#[test]
fn lockfile_authority_drives_exact_restore() {
    let packet = packet();
    let exact = packet.sheet("rmf:install:cargo:serde-add").expect("exact");
    assert_eq!(
        exact.lockfile.lockfile_authority,
        LockfileAuthority::ExactLockfilePinned
    );
    assert!(exact.lockfile.exact_restore_supported);

    let governed = packet
        .sheet("rmf:regenerate:node:relock-unknown")
        .expect("range governed");
    assert_eq!(
        governed.lockfile.lockfile_authority,
        LockfileAuthority::ManifestRangeGoverned
    );
    assert!(!governed.lockfile.exact_restore_supported);
}

#[test]
fn no_lockfile_change_has_zero_churn() {
    let packet = packet();
    let blocked = packet
        .sheet("rmf:regenerate:cargo:policy-blocked")
        .expect("no change");
    assert_eq!(
        blocked.lockfile.diff_class,
        LockfileDiffClass::NoLockfileChange
    );
    assert_eq!(blocked.lockfile.churn_total(), 0);
    assert!(blocked.lockfile.affected_lockfile_ids.is_empty());
    assert!(blocked.lockfile.is_consistent());
}

#[test]
fn every_checkpoint_is_durable_with_full_recovery() {
    let packet = packet();
    for receipt in &packet.checkpoints {
        assert!(
            receipt.durable,
            "checkpoint {} is not durable",
            receipt.checkpoint_id
        );
        assert!(receipt.offers_all_recovery_actions());
        assert!(receipt.is_recoverable());
        assert!(receipt.is_durable_recovery());
    }
}

#[test]
fn failed_or_partial_mutation_leaves_durable_recovery() {
    let packet = packet();
    // A rolled-back update keeps a durable, reverted receipt.
    let reverted = packet
        .checkpoint("rmf:cp:update:node:react-rollback")
        .expect("reverted receipt");
    assert_eq!(reverted.state, CheckpointState::Reverted);
    assert!(reverted.durable);

    // A partial post-commit failure leaves recovery pending, not a transient toast.
    let pending = packet
        .checkpoint("rmf:cp:regenerate:cargo:workspace-converge")
        .expect("pending receipt");
    assert_eq!(pending.state, CheckpointState::PartialRecoveryPending);
    assert!(pending.state.is_recovery_pending());
    assert!(pending.is_durable_recovery());
}

#[test]
fn committed_sheet_carries_no_live_block_reason() {
    let packet = packet();
    for sheet in packet
        .sheets
        .iter()
        .filter(|s| s.review_disposition == ReviewDisposition::CommittedAfterReview)
    {
        assert!(
            !packet.commit_blocked(sheet),
            "committed sheet {} is still blocked",
            sheet.sheet_id
        );
    }
}

#[test]
fn whole_workspace_scope_requires_confirmation() {
    let packet = packet();
    let converge = packet
        .sheet("rmf:regenerate:cargo:workspace-converge")
        .expect("whole workspace");
    assert_eq!(
        converge.manifest_scope.scope_class,
        ManifestScopeClass::WholeWorkspace
    );
    assert!(converge.manifest_scope.requires_confirmation());
    assert!(converge.manifest_scope.confirmed_explicitly);
    assert!(converge.manifest_scope.confirmation_satisfied());
}

#[test]
fn the_same_sheet_feeds_every_surface() {
    let packet = packet();
    let id = "rmf:install:cargo:serde-add";
    let desktop = packet
        .surface_projection(id, PackageSurface::DesktopPackageWorkspace)
        .expect("desktop");
    assert!(desktop.can_commit_here);
    assert!(!desktop.redacted);

    let cli = packet
        .surface_projection(id, PackageSurface::CliHeadless)
        .expect("cli");
    assert!(cli.can_commit_here);

    let ai = packet
        .surface_projection(id, PackageSurface::AiContext)
        .expect("ai");
    // An inspect-only surface can never commit, even an unblocked sheet.
    assert!(!ai.can_commit_here);

    let support = packet
        .surface_projection(id, PackageSurface::SupportExport)
        .expect("support");
    assert!(!support.can_commit_here);
    assert!(support.redacted);

    // A blocked sheet is never committable, even from a mutating surface.
    let blocked = packet
        .surface_projection(
            "rmf:regenerate:cargo:policy-blocked",
            PackageSurface::DesktopPackageWorkspace,
        )
        .expect("blocked");
    assert!(!blocked.can_commit_here);
    assert!(blocked.commit_blocked);
}

#[test]
fn manifest_diff_cards_name_files_hooks_constraints_and_rollback() {
    let packet = packet();
    let cards = packet.manifest_diff_cards();
    assert_eq!(cards.len(), packet.sheets.len());
    for card in &cards {
        assert!(
            card.discloses_apply_boundary(),
            "{} omits a pre-apply disclosure",
            card.card_id
        );
        assert!(
            card.fallback_honest(),
            "{} does not honestly handle fallback state",
            card.card_id
        );
        assert!(!card.affected_manifest_refs.is_empty());
        assert!(!card.lockfile_touch_note.trim().is_empty());
        assert!(!card.scripts_hooks_note.trim().is_empty());
        assert!(!card.peer_runtime_constraints_note.trim().is_empty());
        assert_ne!(card.checkpoint_state, ManifestDiffCheckpointState::Missing);
        assert_ne!(card.rollback_state, ManifestDiffRollbackState::Unavailable);
    }
}

#[test]
fn manifest_diff_cards_keep_add_update_remove_classes_distinct() {
    let packet = packet();
    let add = packet
        .manifest_diff_card("rmf:install:cargo:serde-add")
        .expect("add card");
    let update = packet
        .manifest_diff_card("rmf:update:node:lodash-bump")
        .expect("update card");
    let remove = packet
        .manifest_diff_card("rmf:remove:cargo:obsolete")
        .expect("remove card");
    let resolve = packet
        .manifest_diff_card("rmf:regenerate:node:relock-unknown")
        .expect("resolve card");

    assert_eq!(add.action_class, ManifestDiffActionClass::Add);
    assert_eq!(update.action_class, ManifestDiffActionClass::Update);
    assert_eq!(remove.action_class, ManifestDiffActionClass::Remove);
    assert_eq!(resolve.action_class, ManifestDiffActionClass::Resolve);
    assert_eq!(resolve.apply_action, ManifestDiffApplyAction::Blocked);
}

#[test]
fn manifest_diff_cards_share_grammar_across_direct_ai_recipe_and_cli_sources() {
    let packet = packet();
    let cards = packet.manifest_diff_cards();
    for source in ProposalSource::ALL {
        assert!(
            cards.iter().any(|card| card.proposal_source == source),
            "missing card for source {}",
            source.as_str()
        );
    }
    for card in cards {
        assert!(card
            .consumer_surfaces
            .contains(&"package_manager".to_owned()));
        assert!(card.consumer_surfaces.contains(&"review_pane".to_owned()));
        assert!(card.consumer_surfaces.contains(&"ai_recipe_cli".to_owned()));
        assert!(card
            .consumer_surfaces
            .contains(&"support_export".to_owned()));
    }
}

#[test]
fn export_projection_is_redaction_safe() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(
        projection.rows.len(),
        packet.sheets.len() + packet.checkpoints.len()
    );
    assert!(projection.blocks_any_commit);
    assert!(projection.all_disclose_required);
    assert!(projection.all_bind_matrix);
    for row in &projection.rows {
        assert!(!row.summary.contains("://"));
        assert!(!row.label.contains("://"));
    }
}

#[test]
fn no_field_leaks_a_raw_url() {
    let packet = packet();
    for sheet in &packet.sheets {
        assert!(!sheet.manifest_scope.redacted_manifest_path.contains("://"));
        assert!(!sheet.requested.requested_ref.contains("://"));
        assert!(!sheet.registry_source.redacted_source_label.contains("://"));
        if let Some(resolved) = &sheet.resolved {
            assert!(!resolved.resolved_ref.contains("://"));
        }
    }
    for receipt in &packet.checkpoints {
        assert!(!receipt.lockfile_identity_before.contains("://"));
        assert!(!receipt.lockfile_identity_after.contains("://"));
        for action in &receipt.recovery_actions {
            assert!(!action.target_ref.contains("://"));
        }
    }
}

#[test]
fn validate_flags_script_risk_undisclosed() {
    let mut packet = packet();
    let sheet = packet
        .sheets
        .iter_mut()
        .find(|s| s.sheet_id == "rmf:update:node:lodash-bump")
        .expect("known scripts");
    sheet.script_build.disclosure_note = "  ".to_owned();
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ReviewedMutationFlowsViolation::ScriptRiskUndisclosed { .. }
    )));
}

#[test]
fn validate_flags_policy_allowing_a_blocked_risk() {
    let mut packet = packet();
    let sheet = packet
        .sheets
        .iter_mut()
        .find(|s| s.sheet_id == "rmf:regenerate:cargo:policy-blocked")
        .expect("policy blocked");
    // Claim policy allows a policy-blocked label: inconsistent.
    sheet.script_build.policy_allows = true;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ReviewedMutationFlowsViolation::ScriptRiskInconsistent { .. }
    )));
}

#[test]
fn validate_flags_broad_churn_undisclosed() {
    let mut packet = packet();
    let sheet = packet
        .sheets
        .iter_mut()
        .find(|s| s.sheet_id == "rmf:regenerate:cargo:workspace-converge")
        .expect("broad churn");
    sheet.lockfile.broad_churn_disclosed = false;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ReviewedMutationFlowsViolation::BroadChurnUndisclosed { .. }
    )));
}

#[test]
fn validate_flags_inconsistent_exact_restore_flag() {
    let mut packet = packet();
    let sheet = packet
        .sheets
        .iter_mut()
        .find(|s| s.sheet_id == "rmf:install:cargo:serde-add")
        .expect("exact");
    sheet.lockfile.exact_restore_supported = false;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ReviewedMutationFlowsViolation::LockfileBlastRadiusInconsistent { .. }
    )));
}

#[test]
fn validate_flags_commit_gate_violation() {
    let mut packet = packet();
    let sheet = packet
        .sheets
        .iter_mut()
        .find(|s| s.sheet_id == "rmf:remove:cargo:obsolete")
        .expect("committed");
    // Force a live block reason onto a committed sheet.
    sheet.script_build.label = ScriptBuildLabel::PolicyBlocked;
    sheet.script_build.policy_allows = false;
    sheet.script_build.requires_explicit_ack = false;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, ReviewedMutationFlowsViolation::CommitGateViolated { .. })));
}

#[test]
fn validate_flags_disposition_checkpoint_mismatch() {
    let mut packet = packet();
    let sheet = packet
        .sheets
        .iter_mut()
        .find(|s| s.sheet_id == "rmf:update:node:react-rollback")
        .expect("rolled back");
    // A rolled-back sheet must point at a reverted checkpoint.
    sheet.review_disposition = ReviewDisposition::ReviewedReady;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ReviewedMutationFlowsViolation::DispositionCheckpointMismatch { .. }
    )));
}

#[test]
fn validate_flags_non_durable_checkpoint() {
    let mut packet = packet();
    packet.checkpoints[0].durable = false;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ReviewedMutationFlowsViolation::NonDurableCheckpoint { .. }
    )));
}

#[test]
fn validate_flags_missing_recovery_action() {
    let mut packet = packet();
    packet.checkpoints[0]
        .recovery_actions
        .retain(|a| a.kind != RecoveryActionKind::ExportPatch);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ReviewedMutationFlowsViolation::MissingRecoveryAction { .. }
    )));
}

#[test]
fn validate_flags_dangling_checkpoint_ref() {
    let mut packet = packet();
    packet.sheets[0].rollback_checkpoint_id = "rmf:cp:does-not-exist".to_owned();
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ReviewedMutationFlowsViolation::DanglingCheckpointRef { .. }
    )));
}

#[test]
fn validate_flags_raw_url_leak() {
    let mut packet = packet();
    packet.sheets[0].registry_source.redacted_source_label =
        "https://secret.example.com/registry".to_owned();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, ReviewedMutationFlowsViolation::RawUrlLeak { .. })));
}

#[test]
fn validate_flags_matrix_binding_mismatch() {
    let mut packet = packet();
    packet.references_matrix_id = "some-other-matrix:v9".to_owned();
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ReviewedMutationFlowsViolation::MatrixBindingMismatch { .. }
    )));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_sheets = packet.summary.total_sheets.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&ReviewedMutationFlowsViolation::SummaryMismatch));
}

#[test]
fn validate_flags_duplicate_sheet_id() {
    let mut packet = packet();
    let clone = packet.sheets[0].clone();
    packet.sheets.push(clone);
    packet.summary = packet.computed_summary();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, ReviewedMutationFlowsViolation::DuplicateRowId { .. })));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(ManifestDiffActionClass::Add.as_str(), "add");
    assert_eq!(ManifestDiffActionClass::Update.as_str(), "update");
    assert_eq!(ManifestDiffActionClass::Remove.as_str(), "remove");
    assert_eq!(ManifestDiffPreviewState::NoPreview.as_str(), "no_preview");
    assert_eq!(
        ManifestDiffCheckpointState::NarrowedNoCheckpoint.as_str(),
        "narrowed_no_checkpoint"
    );
    assert_eq!(
        ManifestDiffApplyAction::StageForReview.as_str(),
        "stage_for_review"
    );
    assert_eq!(MutationFlowClass::Regenerate.as_str(), "regenerate");
    assert_eq!(
        ScriptBuildLabel::KnownInstallScripts.as_str(),
        "known_install_scripts"
    );
    assert_eq!(
        ScriptBuildLabel::NativeBuildRequired.as_str(),
        "native_build_required"
    );
    assert_eq!(
        ScriptBuildLabel::UnknownHookRisk.as_str(),
        "unknown_hook_risk"
    );
    assert_eq!(
        LockfileDiffClass::FullRegeneration.as_str(),
        "full_regeneration"
    );
    assert_eq!(
        ProposalSource::CliHeadlessDryRun.as_str(),
        "cli_headless_dry_run"
    );
    assert_eq!(
        CheckpointState::PartialRecoveryPending.as_str(),
        "partial_recovery_pending"
    );
    assert_eq!(RecoveryActionKind::ExportPatch.as_str(), "export_patch");
}

#[test]
fn every_vocabulary_round_trips_through_serde() {
    fn round_trip<T>(all: &[T])
    where
        T: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug,
    {
        for value in all {
            let json = serde_json::to_string(value).expect("serialize");
            let back: T = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(&back, value);
        }
    }
    round_trip(&MutationFlowClass::ALL);
    round_trip(&ScriptBuildLabel::ALL);
    round_trip(&LockfileDiffClass::ALL);
    round_trip(&ProposalSource::ALL);
    round_trip(&ReviewDisposition::ALL);
    round_trip(&CheckpointState::ALL);
    round_trip(&RecoveryActionKind::ALL);
    round_trip(&[
        ManifestDiffActionClass::Add,
        ManifestDiffActionClass::Update,
        ManifestDiffActionClass::Remove,
        ManifestDiffActionClass::Resolve,
    ]);
}

/// Scenario fixtures, embedded so they validate without a runtime walk.
const FIXTURE_INSTALL: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/dependencies/m5/install-update-remove-resolve/install_native_build_reviewed.json"
));
const FIXTURE_UPDATE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/dependencies/m5/install-update-remove-resolve/update_known_scripts_committed.json"
));
const FIXTURE_REMOVE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/dependencies/m5/install-update-remove-resolve/remove_reverted_recovery.json"
));
const FIXTURE_RESOLVE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/dependencies/m5/install-update-remove-resolve/regenerate_policy_blocked.json"
));

#[test]
fn fixtures_parse_and_validate() {
    for (name, json) in [
        ("install_native_build_reviewed", FIXTURE_INSTALL),
        ("update_known_scripts_committed", FIXTURE_UPDATE),
        ("remove_reverted_recovery", FIXTURE_REMOVE),
        ("regenerate_policy_blocked", FIXTURE_RESOLVE),
    ] {
        let packet: ReviewedMutationFlows =
            serde_json::from_str(json).unwrap_or_else(|e| panic!("{name} parses: {e}"));
        assert_eq!(packet.validate(), Vec::new(), "{name} validates");
        assert!(packet.all_bind_matrix(), "{name} binds the matrix");
    }
}

#[test]
fn fixtures_cover_the_mutation_guards() {
    let install: ReviewedMutationFlows =
        serde_json::from_str(FIXTURE_INSTALL).expect("install fixture");
    assert_eq!(install.sheets[0].flow_class, MutationFlowClass::Install);
    assert_eq!(
        install.sheets[0].script_build.label,
        ScriptBuildLabel::NativeBuildRequired
    );

    let remove: ReviewedMutationFlows =
        serde_json::from_str(FIXTURE_REMOVE).expect("remove fixture");
    assert_eq!(remove.sheets[0].flow_class, MutationFlowClass::Remove);
    assert_eq!(remove.checkpoints[0].state, CheckpointState::Reverted);

    let resolve: ReviewedMutationFlows =
        serde_json::from_str(FIXTURE_RESOLVE).expect("resolve fixture");
    assert_eq!(resolve.sheets[0].flow_class, MutationFlowClass::Regenerate);
    assert!(resolve.commit_blocked(&resolve.sheets[0]));
}
