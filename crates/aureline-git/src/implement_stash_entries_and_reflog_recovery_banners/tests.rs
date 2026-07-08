use super::*;

const CANONICAL_PACKET_ID: &str = "m5-stash-reflog-recovery-component:stable:0001";

const CANONICAL_EXPORT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5-stash-reflog-recovery-components-proof/support_export.json"
));

const UNTRACKED_STASH_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ui/m5-stash-reflog-recovery-components/untracked_stash_scope.json"
));

const EXPIRING_RECOVERY_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ui/m5-stash-reflog-recovery-components/expiring_recovery_banner.json"
));

fn stash_entries() -> Vec<StashEntryRow> {
    use GitHistoryDowngradeState::*;
    use StashContentScope::*;
    use StashRestoreVerb::*;

    vec![
        // A default tracked stash created from a feature branch; the message and
        // origin ref keep the `stash@{0}` shorthand from being the only label.
        StashEntryRow {
            row_id: "stash:0".to_owned(),
            component: M5GitHistoryComponent::StashEntry,
            stash_shorthand: "stash@{0}".to_owned(),
            message: "WIP: extract review-queue projection".to_owned(),
            created_from_ref: "feature/review-queue".to_owned(),
            content_scope: TrackedAndStaged,
            scope_disclosure: "Tracked modifications and staged/index content; untracked files were not swept in".to_owned(),
            restore_verbs: vec![Apply, Pop, Drop, CreateBranchFromStash],
            recovery_reflog_note: "Applying leaves stash@{0} on the shelf; the stash commit stays reachable via the stash reflog until dropped".to_owned(),
            downgrade_vocab: vec![DirtyOrConflictedWorktree, OfflineLocalOnly],
            fields_shown: vec![
                "message".to_owned(),
                "created_from_ref".to_owned(),
                "content_scope".to_owned(),
                "restore_verbs".to_owned(),
            ],
            source_contract_refs: vec![STASH_REFLOG_RECOVERY_STASH_CONTRACT_REF.to_owned()],
        },
        // A stash that swept in untracked files; the scope disclosure spells out
        // exactly what a restore would bring back.
        StashEntryRow {
            row_id: "stash:1".to_owned(),
            component: M5GitHistoryComponent::StashEntry,
            stash_shorthand: "stash@{1}".to_owned(),
            message: "spike: local-only telemetry buffer".to_owned(),
            created_from_ref: "main".to_owned(),
            content_scope: TrackedStagedUntracked,
            scope_disclosure: "Tracked, staged, and untracked files all captured; restoring re-creates the untracked scratch files".to_owned(),
            restore_verbs: vec![Apply, Pop, Drop, CreateBranchFromStash],
            recovery_reflog_note: "Popping restores and removes stash@{1}; recover it from the stash reflog if the pop is undone".to_owned(),
            downgrade_vocab: vec![DirtyOrConflictedWorktree, ReflogOnlyFallback],
            fields_shown: vec![
                "message".to_owned(),
                "created_from_ref".to_owned(),
                "content_scope".to_owned(),
                "restore_verbs".to_owned(),
            ],
            source_contract_refs: vec![
                STASH_REFLOG_RECOVERY_STASH_CONTRACT_REF.to_owned(),
                STASH_REFLOG_RECOVERY_CHECKPOINT_CONTRACT_REF.to_owned(),
            ],
        },
    ]
}

fn reflog_banners() -> Vec<ReflogRecoveryBannerRow> {
    use GitHistoryDowngradeState::*;
    use RecoveryAction::*;
    use RecoveryExpiryState::*;
    use RecoveryReachability::*;
    use RecoverySurface::*;

    vec![
        // A reachable banner after a force-push; concrete destination, spans all
        // three surfaces, fresh expiry.
        ReflogRecoveryBannerRow {
            row_id: "banner:force-push".to_owned(),
            component: M5GitHistoryComponent::ReflogRecoveryBanner,
            mutation_label: "Force-push rewrote main".to_owned(),
            recovery_destination: "main@{1} (pre-force-push tip a1b2c3d)".to_owned(),
            reachability: Reachable,
            expiry_state: Fresh,
            expiry_disclosure:
                "Reflog entry retained for 90 days; well within the retention window".to_owned(),
            reachable_from_surfaces: vec![GitHistory, Review, HelpSupport],
            restore_actions: vec![
                RestoreToCheckpoint,
                CompareWithCurrent,
                PinRecoveryPoint,
                OpenProviderInBrowser,
            ],
            downgrade_vocab: vec![DetachedOrMissingRef, StaleProviderOverlay],
            fields_shown: vec![
                "mutation_label".to_owned(),
                "recovery_destination".to_owned(),
                "expiry_state".to_owned(),
            ],
            source_contract_refs: vec![STASH_REFLOG_RECOVERY_CHECKPOINT_CONTRACT_REF.to_owned()],
        },
        // A reachable banner after an interactive rebase; expiring soon, so it is
        // still reachable but the expiry disclosure warns.
        ReflogRecoveryBannerRow {
            row_id: "banner:rebase".to_owned(),
            component: M5GitHistoryComponent::ReflogRecoveryBanner,
            mutation_label: "Interactive rebase squashed 4 commits".to_owned(),
            recovery_destination: "HEAD@{5} (pre-rebase tip e4f5a6b)".to_owned(),
            reachability: Reachable,
            expiry_state: ExpiringSoon,
            expiry_disclosure: "Reflog entry expires in 3 days; pin it to keep this recovery point"
                .to_owned(),
            reachable_from_surfaces: vec![GitHistory, Review, HelpSupport],
            restore_actions: vec![RestoreToCheckpoint, OpenReflogEntry, PinRecoveryPoint],
            downgrade_vocab: vec![ReflogOnlyFallback, DirtyOrConflictedWorktree],
            fields_shown: vec![
                "mutation_label".to_owned(),
                "recovery_destination".to_owned(),
                "expiry_state".to_owned(),
            ],
            source_contract_refs: vec![
                STASH_REFLOG_RECOVERY_CHECKPOINT_CONTRACT_REF.to_owned(),
                STASH_REFLOG_RECOVERY_REVIEW_CONTRACT_REF.to_owned(),
            ],
        },
        // A superseded banner: a newer recovery point replaced it, so it no longer
        // claims reachability and does not have to span every surface.
        ReflogRecoveryBannerRow {
            row_id: "banner:superseded".to_owned(),
            component: M5GitHistoryComponent::ReflogRecoveryBanner,
            mutation_label: "Amend replaced the previous checkpoint".to_owned(),
            recovery_destination: String::new(),
            reachability: Superseded,
            expiry_state: Expired,
            expiry_disclosure:
                "Superseded by a newer recovery point; the prior reflog entry has aged out"
                    .to_owned(),
            reachable_from_surfaces: vec![GitHistory],
            restore_actions: vec![DismissBanner, OpenReflogEntry],
            downgrade_vocab: vec![ReflogOnlyFallback, OfflineLocalOnly],
            fields_shown: vec!["mutation_label".to_owned(), "expiry_state".to_owned()],
            source_contract_refs: vec![STASH_REFLOG_RECOVERY_CHECKPOINT_CONTRACT_REF.to_owned()],
        },
    ]
}

fn trust_review() -> StashReflogRecoveryTrustReview {
    StashReflogRecoveryTrustReview {
        stash_shorthand_never_only_label: true,
        stash_message_and_origin_explicit: true,
        stash_scope_explicit: true,
        restore_verbs_stay_distinct: true,
        recovery_destination_always_concrete: true,
        recovery_reachable_until_superseded_or_dismissed: true,
        expiry_state_always_disclosed: true,
        local_only_recovery_stays_explicit: true,
        one_component_contract_no_hidden_meaning: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> StashReflogRecoveryConsumerProjection {
    StashReflogRecoveryConsumerProjection {
        git_history_reuses_one_contract: true,
        review_reuses_one_contract: true,
        help_support_reuses_one_contract: true,
        support_export_reuses_one_contract: true,
        recovery_reachable_across_surfaces: true,
        cli_headless_shows_truth: true,
        provider_overlay_shows_truth: true,
        ai_context_shows_truth: true,
    }
}

fn proof_freshness() -> StashReflogRecoveryProofFreshness {
    StashReflogRecoveryProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-07-08T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<GitHistoryDowngradeState> {
    vec![
        GitHistoryDowngradeState::DirtyOrConflictedWorktree,
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
        STASH_REFLOG_RECOVERY_SCHEMA_REF.to_owned(),
        STASH_REFLOG_RECOVERY_DOC_REF.to_owned(),
        STASH_REFLOG_RECOVERY_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        STASH_REFLOG_RECOVERY_STASH_CONTRACT_REF.to_owned(),
        STASH_REFLOG_RECOVERY_CHECKPOINT_CONTRACT_REF.to_owned(),
        STASH_REFLOG_RECOVERY_REVIEW_CONTRACT_REF.to_owned(),
    ]
}

fn seed_packet() -> StashReflogRecoveryPacket {
    StashReflogRecoveryPacket::new(StashReflogRecoveryPacketInput {
        packet_id: CANONICAL_PACKET_ID.to_owned(),
        surface_label:
            "Stash entries and reflog-recovery banners: restore-scope and checkpoint truth"
                .to_owned(),
        stash_entries: stash_entries(),
        reflog_banners: reflog_banners(),
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

fn baseline() -> StashReflogRecoveryPacket {
    seed_packet()
}

/// Regenerates the checked-in artifacts and fixtures.
///
/// Guarded by `GEN_STASH_REFLOG_RECOVERY_ARTIFACTS` so it is inert in CI but can
/// deterministically rewrite the export, summary, and narrowed fixtures.
#[test]
fn generate_artifacts() {
    if std::env::var_os("GEN_STASH_REFLOG_RECOVERY_ARTIFACTS").is_none() {
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
        format!("{root}/{STASH_REFLOG_RECOVERY_ARTIFACT_REF}"),
        format!("{}\n", canonical.export_safe_json()),
    )
    .expect("write support export");
    std::fs::write(
        format!("{root}/{STASH_REFLOG_RECOVERY_SUMMARY_REF}"),
        canonical.render_markdown_summary(),
    )
    .expect("write summary");

    // Untracked-stash fixture: the second entry keeps its untracked scope explicit.
    let mut untracked = seed_packet();
    untracked.packet_id = "m5-stash-reflog-recovery-component:untracked-scope:0001".to_owned();
    assert!(
        untracked.validate().is_empty(),
        "{:?}",
        untracked.validate()
    );
    std::fs::write(
        format!("{root}/{STASH_REFLOG_RECOVERY_FIXTURE_DIR}/untracked_stash_scope.json"),
        format!("{}\n", untracked.export_safe_json()),
    )
    .expect("write untracked-stash fixture");

    // Expiring-recovery fixture: the rebase banner narrows to expiring-soon but
    // stays reachable across all three surfaces.
    let mut expiring = seed_packet();
    {
        let banner = expiring
            .reflog_banners
            .iter_mut()
            .find(|banner| banner.row_id == "banner:rebase")
            .expect("rebase banner present");
        banner.expiry_state = RecoveryExpiryState::ExpiringSoon;
        banner.expiry_disclosure =
            "Reflog entry expires in 12 hours; pin it now or the recovery point will be lost"
                .to_owned();
        if !banner
            .downgrade_vocab
            .contains(&GitHistoryDowngradeState::ReflogOnlyFallback)
        {
            banner
                .downgrade_vocab
                .push(GitHistoryDowngradeState::ReflogOnlyFallback);
        }
    }
    expiring.packet_id = "m5-stash-reflog-recovery-component:expiring-recovery:0001".to_owned();
    assert!(expiring.validate().is_empty(), "{:?}", expiring.validate());
    std::fs::write(
        format!("{root}/{STASH_REFLOG_RECOVERY_FIXTURE_DIR}/expiring_recovery_banner.json"),
        format!("{}\n", expiring.export_safe_json()),
    )
    .expect("write expiring-recovery fixture");
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
    let packet = current_stash_reflog_recovery_export()
        .expect("checked stash reflog recovery export validates");
    assert_eq!(packet.packet_id, CANONICAL_PACKET_ID);
}

#[test]
fn checked_export_matches_seed() {
    let checked: StashReflogRecoveryPacket =
        serde_json::from_str(CANONICAL_EXPORT).expect("canonical export deserializes");
    assert_eq!(checked, seed_packet());
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [UNTRACKED_STASH_FIXTURE, EXPIRING_RECOVERY_FIXTURE] {
        let packet: StashReflogRecoveryPacket =
            serde_json::from_str(raw).expect("fixture parses as recovery packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn resolver_keeps_recovery_reachable_until_terminal() {
    // A reachable, fresh recovery point must show a concrete destination, span
    // every surface, and stay recoverable.
    let reachable = resolve_recovery_banner_disclosure(
        RecoveryReachability::Reachable,
        RecoveryExpiryState::Fresh,
    );
    assert!(reachable.must_show_concrete_destination);
    assert!(reachable.must_span_history_review_help_support);
    assert!(reachable.must_show_expiry_state);
    assert!(reachable.is_still_recoverable);

    // A reachable point that has expired is no longer genuinely recoverable.
    let expired = resolve_recovery_banner_disclosure(
        RecoveryReachability::Reachable,
        RecoveryExpiryState::Expired,
    );
    assert!(!expired.is_still_recoverable);

    // A superseded point no longer forces destination/surface span.
    let superseded = resolve_recovery_banner_disclosure(
        RecoveryReachability::Superseded,
        RecoveryExpiryState::Pruned,
    );
    assert!(!superseded.must_show_concrete_destination);
    assert!(!superseded.must_span_history_review_help_support);
    assert!(superseded.must_show_expiry_state);
}

#[test]
fn stash_shorthand_only_label_fails() {
    let mut packet = baseline();
    // The first stash drops its message, leaving only the `stash@{0}` shorthand.
    packet.stash_entries[0].message = String::new();
    assert!(packet
        .validate()
        .contains(&StashReflogRecoveryViolation::StashShorthandOnlyLabel));
}

#[test]
fn stash_shorthand_echoed_as_message_fails() {
    let mut packet = baseline();
    // Echoing the shorthand into the message is still shorthand-only.
    packet.stash_entries[0].message = "stash@{0}".to_owned();
    assert!(packet
        .validate()
        .contains(&StashReflogRecoveryViolation::StashShorthandOnlyLabel));
}

#[test]
fn stash_scope_disclosure_missing_fails() {
    let mut packet = baseline();
    packet.stash_entries[1].scope_disclosure = String::new();
    assert!(packet
        .validate()
        .contains(&StashReflogRecoveryViolation::StashScopeDisclosureMissing));
}

#[test]
fn restore_verb_coverage_missing_fails() {
    let mut packet = baseline();
    // Dropping create-branch-from-stash loses a required distinct verb.
    packet.stash_entries[0].restore_verbs = vec![
        StashRestoreVerb::Apply,
        StashRestoreVerb::Pop,
        StashRestoreVerb::Drop,
    ];
    assert!(packet
        .validate()
        .contains(&StashReflogRecoveryViolation::RestoreVerbCoverageMissing));
}

#[test]
fn restore_verbs_collapsed_fails() {
    let mut packet = baseline();
    // Aliasing pop as a duplicate apply collapses the distinct verbs.
    packet.stash_entries[0].restore_verbs = vec![
        StashRestoreVerb::Apply,
        StashRestoreVerb::Apply,
        StashRestoreVerb::Pop,
        StashRestoreVerb::Drop,
        StashRestoreVerb::CreateBranchFromStash,
    ];
    assert!(packet
        .validate()
        .contains(&StashReflogRecoveryViolation::RestoreVerbsCollapsed));
}

#[test]
fn stash_recovery_note_missing_fails() {
    let mut packet = baseline();
    packet.stash_entries[0].recovery_reflog_note = String::new();
    assert!(packet
        .validate()
        .contains(&StashReflogRecoveryViolation::StashRecoveryNoteMissing));
}

#[test]
fn wrong_component_for_stash_row_fails() {
    let mut packet = baseline();
    packet.stash_entries[0].component = M5GitHistoryComponent::ReflogRecoveryBanner;
    assert!(packet
        .validate()
        .contains(&StashReflogRecoveryViolation::WrongComponentForStashRow));
}

#[test]
fn incomplete_stash_row_fails() {
    let mut packet = baseline();
    packet.stash_entries[0].stash_shorthand = String::new();
    assert!(packet
        .validate()
        .contains(&StashReflogRecoveryViolation::StashRowIncomplete));
}

#[test]
fn missing_stash_entries_fails() {
    let mut packet = baseline();
    packet.stash_entries.clear();
    assert!(packet
        .validate()
        .contains(&StashReflogRecoveryViolation::StashEntriesMissing));
}

#[test]
fn recovery_destination_missing_fails() {
    let mut packet = baseline();
    // The reachable force-push banner drops its concrete destination.
    packet.reflog_banners[0].recovery_destination = String::new();
    assert!(packet
        .validate()
        .contains(&StashReflogRecoveryViolation::RecoveryDestinationMissing));
}

#[test]
fn recovery_not_reachable_across_surfaces_fails() {
    let mut packet = baseline();
    // The reachable force-push banner stops spanning review and help/support.
    packet.reflog_banners[0].reachable_from_surfaces = vec![RecoverySurface::GitHistory];
    assert!(packet
        .validate()
        .contains(&StashReflogRecoveryViolation::RecoveryNotReachableAcrossSurfaces));
}

#[test]
fn expiry_state_undisclosed_fails() {
    let mut packet = baseline();
    packet.reflog_banners[0].expiry_disclosure = String::new();
    assert!(packet
        .validate()
        .contains(&StashReflogRecoveryViolation::ExpiryStateUndisclosed));
}

#[test]
fn expired_recovery_still_reachable_fails() {
    let mut packet = baseline();
    // A reachable banner whose recovery point has expired is a contradiction.
    packet.reflog_banners[0].expiry_state = RecoveryExpiryState::Expired;
    assert!(packet
        .validate()
        .contains(&StashReflogRecoveryViolation::ExpiredRecoveryStillReachable));
}

#[test]
fn forced_raw_provider_navigation_fails() {
    let mut packet = baseline();
    packet.reflog_banners[0].restore_actions = vec![RecoveryAction::OpenProviderInBrowser];
    assert!(packet
        .validate()
        .contains(&StashReflogRecoveryViolation::ForcedRawProviderNavigation));
}

#[test]
fn wrong_component_for_banner_row_fails() {
    let mut packet = baseline();
    packet.reflog_banners[0].component = M5GitHistoryComponent::StashEntry;
    assert!(packet
        .validate()
        .contains(&StashReflogRecoveryViolation::WrongComponentForBannerRow));
}

#[test]
fn incomplete_banner_row_fails() {
    let mut packet = baseline();
    packet.reflog_banners[0].mutation_label = String::new();
    assert!(packet
        .validate()
        .contains(&StashReflogRecoveryViolation::BannerRowIncomplete));
}

#[test]
fn missing_reflog_banners_fails() {
    let mut packet = baseline();
    packet.reflog_banners.clear();
    assert!(packet
        .validate()
        .contains(&StashReflogRecoveryViolation::ReflogBannersMissing));
}

#[test]
fn recovery_reachability_coverage_missing_fails() {
    let mut packet = baseline();
    // With every banner terminal, no reachable recovery remains to prove AC2.
    packet
        .reflog_banners
        .retain(|banner| !banner.reachability.is_reachable());
    assert!(packet
        .validate()
        .contains(&StashReflogRecoveryViolation::RecoveryReachabilityCoverageMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = baseline();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&StashReflogRecoveryViolation::MissingSourceContracts));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = baseline();
    packet.downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&StashReflogRecoveryViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = baseline();
    packet.consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&StashReflogRecoveryViolation::ConsumerSurfacesMissing));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = baseline();
    packet.trust_review.stash_shorthand_never_only_label = false;
    assert!(packet
        .validate()
        .contains(&StashReflogRecoveryViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = baseline();
    packet
        .consumer_projection
        .recovery_reachable_across_surfaces = false;
    assert!(packet
        .validate()
        .contains(&StashReflogRecoveryViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = baseline();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&StashReflogRecoveryViolation::ProofFreshnessIncomplete));
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = baseline();
    packet.record_kind = "something_else".to_owned();
    assert!(packet
        .validate()
        .contains(&StashReflogRecoveryViolation::WrongRecordKind));
}

#[test]
fn raw_boundary_material_in_export_fails() {
    let mut packet = baseline();
    packet.surface_label = "leak: bearer abc123".to_owned();
    assert!(packet
        .validate()
        .contains(&StashReflogRecoveryViolation::RawBoundaryMaterialInExport));
}

#[test]
fn markdown_summary_lists_stash_and_banner_sections() {
    let summary = baseline().render_markdown_summary();
    assert!(summary.contains("## Stash entries"));
    assert!(summary.contains("## Reflog-recovery banners"));
    for entry in stash_entries() {
        assert!(
            summary.contains(&entry.stash_shorthand),
            "summary missing stash {}",
            entry.stash_shorthand
        );
    }
}

#[test]
fn both_recovery_components_are_the_frozen_pair() {
    assert!(STASH_REFLOG_RECOVERY_COMPONENTS.contains(&M5GitHistoryComponent::StashEntry));
    assert!(STASH_REFLOG_RECOVERY_COMPONENTS.contains(&M5GitHistoryComponent::ReflogRecoveryBanner));
    // The stash entry is a risky-mutation surface; the recovery banner is not.
    assert!(M5GitHistoryComponent::StashEntry.is_risky_mutation_surface());
    assert!(!M5GitHistoryComponent::ReflogRecoveryBanner.is_risky_mutation_surface());
}

#[test]
fn restore_verb_semantics_stay_distinct() {
    assert!(StashRestoreVerb::Pop.removes_stash_from_shelf());
    assert!(StashRestoreVerb::Drop.removes_stash_from_shelf());
    assert!(!StashRestoreVerb::Apply.removes_stash_from_shelf());
    assert!(StashRestoreVerb::Drop.discards_without_restoring());
    assert!(!StashRestoreVerb::Pop.discards_without_restoring());
    assert!(!StashRestoreVerb::CreateBranchFromStash.removes_stash_from_shelf());
}

#[test]
fn untracked_fixture_keeps_scope_explicit() {
    let packet: StashReflogRecoveryPacket =
        serde_json::from_str(UNTRACKED_STASH_FIXTURE).expect("untracked-stash fixture parses");
    let entry = packet
        .stash_entries
        .iter()
        .find(|entry| entry.content_scope.includes_untracked())
        .expect("untracked-scope stash present");
    assert!(!entry.scope_disclosure.trim().is_empty());
    assert!(entry.has_meaning_beyond_shorthand());
}

#[test]
fn expiring_fixture_stays_reachable() {
    let packet: StashReflogRecoveryPacket =
        serde_json::from_str(EXPIRING_RECOVERY_FIXTURE).expect("expiring-recovery fixture parses");
    let banner = packet
        .reflog_banners
        .iter()
        .find(|banner| banner.expiry_state == RecoveryExpiryState::ExpiringSoon)
        .expect("expiring banner present");
    assert!(banner.reachability.is_reachable());
    assert!(banner.spans_required_surfaces());
    assert!(!banner.recovery_destination.trim().is_empty());
}
