use super::*;

const CANONICAL_PACKET_ID: &str = "m5-git-history-identity-component:stable:0001";

const CANONICAL_EXPORT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5-git-history-identity-components-proof/support_export.json"
));

const LINKED_WORKTREE_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ui/m5-git-history-identity-components/linked_worktree_separate_context.json"
));

const SHALLOW_PARTIAL_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ui/m5-git-history-identity-components/shallow_partial_incomplete_history.json"
));

fn rows() -> Vec<GitHistoryIdentityRow> {
    use DivergenceState::*;
    use GitWorkingContextTarget::*;
    use IdentityComponentAction::*;
    use M5GitHistoryComponent::*;
    use TopologyCompleteness::*;
    use WorktreeDirtyState::*;

    vec![
        // Commit-graph header on the current primary worktree; ahead of upstream,
        // so it keeps recovery/reflog availability explicit.
        GitHistoryIdentityRow {
            row_id: "row:commit-graph-header".to_owned(),
            component: CommitGraphHeader,
            working_context_target: CurrentRepoWorktree,
            repo_identity_label: "aureline (/work/aureline)".to_owned(),
            checked_out_ref_label: "main".to_owned(),
            worktree_path_label: "/work/aureline".to_owned(),
            divergence: Ahead,
            dirty_state: Clean,
            topology_completeness: Complete,
            claims_current_primary_context: true,
            separate_worktree_context_note: String::new(),
            incomplete_history_marker: String::new(),
            recovery_reflog_availability: "Reflog recovery available: main@{upstream} restores the pre-advance position".to_owned(),
            downgrade_vocab: vec![
                GitHistoryDowngradeState::StaleProviderOverlay,
                GitHistoryDowngradeState::DetachedOrMissingRef,
            ],
            actions: vec![OpenInWorkspace, CompareRefs, OpenProviderInBrowser],
            fields_shown: vec![
                "repo_identity".to_owned(),
                "checked_out_ref".to_owned(),
                "divergence".to_owned(),
            ],
            source_contract_refs: vec![GIT_HISTORY_IDENTITY_COMMIT_HISTORY_CONTRACT_REF.to_owned()],
        },
        // History-graph row on a partial/shallow checkout; incomplete history must
        // be marked, but no recovery is forced (clean, non-divergent).
        GitHistoryIdentityRow {
            row_id: "row:history-graph-row".to_owned(),
            component: HistoryGraphRow,
            working_context_target: PartialOrShallowCheckout,
            repo_identity_label: "aureline (shallow mirror)".to_owned(),
            checked_out_ref_label: "release/24.10".to_owned(),
            worktree_path_label: "/work/aureline-shallow".to_owned(),
            divergence: Unknown,
            dirty_state: Clean,
            topology_completeness: Shallow,
            claims_current_primary_context: false,
            separate_worktree_context_note: String::new(),
            incomplete_history_marker: "Shallow clone (depth 1): older commits and full lineage are not present here".to_owned(),
            recovery_reflog_availability: String::new(),
            downgrade_vocab: vec![
                GitHistoryDowngradeState::ShallowOrPartialTopology,
                GitHistoryDowngradeState::OfflineLocalOnly,
            ],
            actions: vec![OpenInWorkspace, DeepenOrHydrateHistory],
            fields_shown: vec![
                "commit_id".to_owned(),
                "parent_lineage".to_owned(),
                "topology_completeness".to_owned(),
            ],
            source_contract_refs: vec![GIT_HISTORY_IDENTITY_COMMIT_HISTORY_CONTRACT_REF.to_owned()],
        },
        // Branch-comparison chip anchored at a detached/bare root; detached refs
        // keep recovery/reflog availability explicit.
        GitHistoryIdentityRow {
            row_id: "row:branch-comparison-chip".to_owned(),
            component: BranchComparisonChip,
            working_context_target: DetachedOrBareRoot,
            repo_identity_label: "aureline (detached HEAD)".to_owned(),
            checked_out_ref_label: "detached @ a1b2c3d".to_owned(),
            worktree_path_label: "/work/aureline".to_owned(),
            divergence: DetachedNoUpstream,
            dirty_state: Clean,
            topology_completeness: Complete,
            claims_current_primary_context: false,
            separate_worktree_context_note: String::new(),
            incomplete_history_marker: String::new(),
            recovery_reflog_availability: "Detached HEAD: HEAD@{1} reflog entry restores the prior branch tip".to_owned(),
            downgrade_vocab: vec![
                GitHistoryDowngradeState::DetachedOrMissingRef,
                GitHistoryDowngradeState::ReflogOnlyFallback,
            ],
            actions: vec![CompareRefs, OpenRecoveryReflog],
            fields_shown: vec![
                "base_ref".to_owned(),
                "head_ref".to_owned(),
                "merge_base".to_owned(),
            ],
            source_contract_refs: vec![GIT_HISTORY_IDENTITY_TOPOLOGY_CONTRACT_REF.to_owned()],
        },
        // Worktree row for a linked worktree with its own dirty working context;
        // the separate context must never be flattened, and recovery stays explicit.
        GitHistoryIdentityRow {
            row_id: "row:worktree-row".to_owned(),
            component: WorktreeRow,
            working_context_target: LinkedWorktree,
            repo_identity_label: "aureline (linked worktree)".to_owned(),
            checked_out_ref_label: "feature/import".to_owned(),
            worktree_path_label: "/work/aureline-import".to_owned(),
            divergence: Behind,
            dirty_state: DirtyUncommitted,
            topology_completeness: Complete,
            claims_current_primary_context: false,
            separate_worktree_context_note: "Separate worktree: feature/import at /work/aureline-import keeps its own uncommitted changes; not the current context".to_owned(),
            incomplete_history_marker: String::new(),
            recovery_reflog_availability: "Uncommitted changes stash-recoverable; switching preserves them via the stash shelf".to_owned(),
            downgrade_vocab: vec![
                GitHistoryDowngradeState::DirtyOrConflictedWorktree,
                GitHistoryDowngradeState::DetachedOrMissingRef,
            ],
            actions: vec![OpenInWorkspace, SwitchWorktreeContext, OpenRecoveryReflog],
            fields_shown: vec![
                "worktree_path".to_owned(),
                "checked_out_ref".to_owned(),
                "dirty_state".to_owned(),
            ],
            source_contract_refs: vec![
                GIT_HISTORY_IDENTITY_TOPOLOGY_CONTRACT_REF.to_owned(),
                GIT_HISTORY_IDENTITY_RECOVERY_CHECKPOINT_CONTRACT_REF.to_owned(),
            ],
        },
    ]
}

fn trust_review() -> GitHistoryIdentityTrustReview {
    GitHistoryIdentityTrustReview {
        worktree_identity_never_flattened: true,
        exact_repo_ref_identity_explicit: true,
        divergence_state_explicit: true,
        dirty_state_explicit: true,
        shallow_partial_sparse_marked: true,
        worktree_path_explicit: true,
        recovery_reflog_availability_explicit: true,
        separate_working_context_preserved: true,
        current_versus_other_context_unambiguous: true,
        one_component_contract_no_hidden_meaning: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> GitHistoryIdentityConsumerProjection {
    GitHistoryIdentityConsumerProjection {
        review_reuses_one_contract: true,
        shell_reuses_one_contract: true,
        help_reuses_one_contract: true,
        support_export_reuses_one_contract: true,
        component_distinguishes_current_other_partial: true,
        cli_headless_shows_truth: true,
        provider_overlay_shows_truth: true,
        ai_context_shows_truth: true,
    }
}

fn proof_freshness() -> GitHistoryIdentityProofFreshness {
    GitHistoryIdentityProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-07-08T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<GitHistoryDowngradeState> {
    vec![
        GitHistoryDowngradeState::DirtyOrConflictedWorktree,
        GitHistoryDowngradeState::ShallowOrPartialTopology,
        GitHistoryDowngradeState::DetachedOrMissingRef,
        GitHistoryDowngradeState::ReflogOnlyFallback,
        GitHistoryDowngradeState::OfflineLocalOnly,
        GitHistoryDowngradeState::StaleProviderOverlay,
    ]
}

fn consumer_surfaces() -> Vec<ComponentConsumerSurface> {
    ComponentConsumerSurface::ALL.to_vec()
}

fn source_contract_refs() -> Vec<String> {
    vec![
        GIT_HISTORY_IDENTITY_SCHEMA_REF.to_owned(),
        GIT_HISTORY_IDENTITY_DOC_REF.to_owned(),
        GIT_HISTORY_IDENTITY_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        GIT_HISTORY_IDENTITY_COMMIT_HISTORY_CONTRACT_REF.to_owned(),
        GIT_HISTORY_IDENTITY_TOPOLOGY_CONTRACT_REF.to_owned(),
        GIT_HISTORY_IDENTITY_RECOVERY_CHECKPOINT_CONTRACT_REF.to_owned(),
    ]
}

fn seed_packet() -> GitHistoryIdentityPacket {
    GitHistoryIdentityPacket::new(GitHistoryIdentityPacketInput {
        packet_id: CANONICAL_PACKET_ID.to_owned(),
        surface_label: "Git-history identity components: working-context and topology truth"
            .to_owned(),
        rows: rows(),
        downgrade_triggers: downgrade_triggers(),
        consumer_surfaces: consumer_surfaces(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "aureline.support.redaction.v1".to_owned(),
        minted_at: "2026-07-08T00:00:00Z".to_owned(),
    })
}

fn baseline() -> GitHistoryIdentityPacket {
    seed_packet()
}

/// Regenerates the checked-in artifacts and fixtures.
///
/// Guarded by `GEN_GIT_HISTORY_IDENTITY_ARTIFACTS` so it is inert in CI but can
/// deterministically rewrite the export, summary, and narrowed fixtures.
#[test]
fn generate_artifacts() {
    if std::env::var_os("GEN_GIT_HISTORY_IDENTITY_ARTIFACTS").is_none() {
        return;
    }
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");

    let canonical = seed_packet();
    assert!(
        canonical.validate().is_empty(),
        "{:?}",
        canonical.validate()
    );
    std::fs::write(
        format!("{root}/{GIT_HISTORY_IDENTITY_ARTIFACT_REF}"),
        format!("{}\n", canonical.export_safe_json()),
    )
    .expect("write support export");
    std::fs::write(
        format!("{root}/{GIT_HISTORY_IDENTITY_SUMMARY_REF}"),
        canonical.render_markdown_summary(),
    )
    .expect("write summary");

    // Linked-worktree fixture: the worktree row keeps its own separate context.
    let mut linked = seed_packet();
    linked.packet_id = "m5-git-history-identity-component:linked-worktree:0001".to_owned();
    assert!(linked.validate().is_empty(), "{:?}", linked.validate());
    std::fs::write(
        format!("{root}/{GIT_HISTORY_IDENTITY_FIXTURE_DIR}/linked_worktree_separate_context.json"),
        format!("{}\n", linked.export_safe_json()),
    )
    .expect("write linked-worktree fixture");

    // Shallow/partial fixture: the history-graph row narrows to a partial checkout.
    let mut shallow = seed_packet();
    {
        let history = shallow
            .rows
            .iter_mut()
            .find(|row| row.component == M5GitHistoryComponent::HistoryGraphRow)
            .expect("history-graph row present");
        history.topology_completeness = TopologyCompleteness::Partial;
        history.incomplete_history_marker =
            "Partial clone: some objects are fetched lazily and are not present until hydrated"
                .to_owned();
        if !history
            .downgrade_vocab
            .contains(&GitHistoryDowngradeState::ShallowOrPartialTopology)
        {
            history
                .downgrade_vocab
                .push(GitHistoryDowngradeState::ShallowOrPartialTopology);
        }
    }
    shallow.packet_id = "m5-git-history-identity-component:shallow-partial:0001".to_owned();
    assert!(shallow.validate().is_empty(), "{:?}", shallow.validate());
    std::fs::write(
        format!(
            "{root}/{GIT_HISTORY_IDENTITY_FIXTURE_DIR}/shallow_partial_incomplete_history.json"
        ),
        format!("{}\n", shallow.export_safe_json()),
    )
    .expect("write shallow-partial fixture");
}

#[test]
fn seed_packet_validates_clean() {
    assert!(
        baseline().validate().is_empty(),
        "{:?}",
        baseline().validate()
    );
}

#[test]
fn checked_support_export_validates() {
    let packet = current_git_history_identity_export()
        .expect("checked git history identity export validates");
    assert_eq!(packet.packet_id, CANONICAL_PACKET_ID);
}

#[test]
fn checked_export_matches_seed() {
    let checked: GitHistoryIdentityPacket =
        serde_json::from_str(CANONICAL_EXPORT).expect("canonical export deserializes");
    assert_eq!(checked, seed_packet());
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [LINKED_WORKTREE_FIXTURE, SHALLOW_PARTIAL_FIXTURE] {
        let packet: GitHistoryIdentityPacket =
            serde_json::from_str(raw).expect("fixture parses as identity packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn resolver_derives_current_context_from_target_alone() {
    // A partial/shallow checkout never claims to be the current context, even when clean.
    let partial = resolve_git_history_identity_disclosure(
        GitWorkingContextTarget::PartialOrShallowCheckout,
        DivergenceState::Current,
        WorktreeDirtyState::Clean,
        TopologyCompleteness::Complete,
    );
    assert!(!partial.asserts_current_primary_context);
    assert!(partial.needs_incomplete_history_marker);

    // Only the current primary worktree asserts the current context.
    let current = resolve_git_history_identity_disclosure(
        GitWorkingContextTarget::CurrentRepoWorktree,
        DivergenceState::Current,
        WorktreeDirtyState::Clean,
        TopologyCompleteness::Complete,
    );
    assert!(current.asserts_current_primary_context);
    assert!(!current.needs_recovery_reflog_availability);

    // A linked worktree keeps its own context and, when dirty, keeps recovery explicit.
    let linked = resolve_git_history_identity_disclosure(
        GitWorkingContextTarget::LinkedWorktree,
        DivergenceState::Behind,
        WorktreeDirtyState::DirtyUncommitted,
        TopologyCompleteness::Complete,
    );
    assert!(linked.needs_separate_worktree_context);
    assert!(linked.needs_recovery_reflog_availability);
}

#[test]
fn ambiguous_context_claim_fails() {
    let mut packet = baseline();
    // The partial/shallow history row falsely claims to be the current context.
    packet.rows[1].claims_current_primary_context = true;
    assert!(packet
        .validate()
        .contains(&GitHistoryIdentityViolation::AmbiguousContextClaimed));
}

#[test]
fn current_context_dropping_claim_fails() {
    let mut packet = baseline();
    // The current primary worktree header drops its current-context claim.
    packet.rows[0].claims_current_primary_context = false;
    assert!(packet
        .validate()
        .contains(&GitHistoryIdentityViolation::AmbiguousContextClaimed));
}

#[test]
fn non_identity_component_fails() {
    let mut packet = baseline();
    packet.rows[0].component = M5GitHistoryComponent::ForcePushReviewDialog;
    assert!(packet
        .validate()
        .contains(&GitHistoryIdentityViolation::NonIdentityComponent));
}

#[test]
fn missing_separate_worktree_context_fails() {
    let mut packet = baseline();
    // The linked-worktree row drops its separate-context note.
    packet.rows[3].separate_worktree_context_note = String::new();
    assert!(packet
        .validate()
        .contains(&GitHistoryIdentityViolation::SeparateWorktreeContextMissing));
}

#[test]
fn missing_incomplete_history_marker_fails() {
    let mut packet = baseline();
    // The shallow history row drops its incomplete-history marker.
    packet.rows[1].incomplete_history_marker = String::new();
    assert!(packet
        .validate()
        .contains(&GitHistoryIdentityViolation::IncompleteHistoryMarkerMissing));
}

#[test]
fn missing_recovery_reflog_availability_fails() {
    let mut packet = baseline();
    // The detached-root chip drops its recovery/reflog availability.
    packet.rows[2].recovery_reflog_availability = String::new();
    assert!(packet
        .validate()
        .contains(&GitHistoryIdentityViolation::RecoveryReflogAvailabilityMissing));
}

#[test]
fn forced_raw_provider_navigation_fails() {
    let mut packet = baseline();
    packet.rows[0].actions = vec![IdentityComponentAction::OpenProviderInBrowser];
    assert!(packet
        .validate()
        .contains(&GitHistoryIdentityViolation::ForcedRawProviderNavigation));
}

#[test]
fn missing_working_context_coverage_fails() {
    let mut packet = baseline();
    packet
        .rows
        .retain(|row| row.working_context_target != GitWorkingContextTarget::LinkedWorktree);
    assert!(packet
        .validate()
        .contains(&GitHistoryIdentityViolation::WorkingContextCoverageMissing));
}

#[test]
fn missing_component_coverage_fails() {
    let mut packet = baseline();
    packet
        .rows
        .retain(|row| row.component != M5GitHistoryComponent::WorktreeRow);
    assert!(packet
        .validate()
        .contains(&GitHistoryIdentityViolation::ComponentCoverageMissing));
}

#[test]
fn missing_rows_fails() {
    let mut packet = baseline();
    packet.rows.clear();
    assert!(packet
        .validate()
        .contains(&GitHistoryIdentityViolation::RowsMissing));
}

#[test]
fn incomplete_row_fails() {
    let mut packet = baseline();
    packet.rows[0].repo_identity_label = String::new();
    assert!(packet
        .validate()
        .contains(&GitHistoryIdentityViolation::RowIncomplete));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = baseline();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&GitHistoryIdentityViolation::MissingSourceContracts));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = baseline();
    packet.downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&GitHistoryIdentityViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = baseline();
    packet.consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&GitHistoryIdentityViolation::ConsumerSurfacesMissing));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = baseline();
    packet.trust_review.worktree_identity_never_flattened = false;
    assert!(packet
        .validate()
        .contains(&GitHistoryIdentityViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = baseline();
    packet
        .consumer_projection
        .component_distinguishes_current_other_partial = false;
    assert!(packet
        .validate()
        .contains(&GitHistoryIdentityViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = baseline();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&GitHistoryIdentityViolation::ProofFreshnessIncomplete));
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = baseline();
    packet.record_kind = "something_else".to_owned();
    assert!(packet
        .validate()
        .contains(&GitHistoryIdentityViolation::WrongRecordKind));
}

#[test]
fn raw_boundary_material_in_export_fails() {
    let mut packet = baseline();
    packet.surface_label = "leak: bearer abc123".to_owned();
    assert!(packet
        .validate()
        .contains(&GitHistoryIdentityViolation::RawBoundaryMaterialInExport));
}

#[test]
fn markdown_summary_lists_every_component() {
    let summary = baseline().render_markdown_summary();
    for component in GIT_HISTORY_IDENTITY_COMPONENTS {
        assert!(
            summary.contains(component.as_str()),
            "summary missing component {}",
            component.as_str()
        );
    }
    assert!(summary.contains("## Components"));
}

#[test]
fn every_identity_component_is_display_only() {
    // The four implemented components are exactly the non-risky matrix components.
    for component in GIT_HISTORY_IDENTITY_COMPONENTS {
        assert!(
            !component.is_risky_mutation_surface(),
            "{} must be a display/identity component",
            component.as_str()
        );
    }
}

#[test]
fn linked_worktree_fixture_preserves_separate_context() {
    let packet: GitHistoryIdentityPacket =
        serde_json::from_str(LINKED_WORKTREE_FIXTURE).expect("linked-worktree fixture parses");
    let row = packet
        .rows
        .iter()
        .find(|row| row.working_context_target == GitWorkingContextTarget::LinkedWorktree)
        .expect("linked-worktree row present");
    assert!(!row.separate_worktree_context_note.trim().is_empty());
    assert!(!row.claims_current_primary_context);
}

#[test]
fn shallow_partial_fixture_marks_incomplete_history() {
    let packet: GitHistoryIdentityPacket =
        serde_json::from_str(SHALLOW_PARTIAL_FIXTURE).expect("shallow-partial fixture parses");
    let row = packet
        .rows
        .iter()
        .find(|row| row.component == M5GitHistoryComponent::HistoryGraphRow)
        .expect("history-graph row present");
    assert_eq!(row.topology_completeness, TopologyCompleteness::Partial);
    assert!(!row.incomplete_history_marker.trim().is_empty());
}
