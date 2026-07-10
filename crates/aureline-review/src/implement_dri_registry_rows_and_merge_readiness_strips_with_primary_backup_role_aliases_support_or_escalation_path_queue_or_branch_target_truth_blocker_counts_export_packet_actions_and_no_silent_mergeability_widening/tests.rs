use super::*;

const PACKET_ID: &str = "m5-dri-registry-merge-readiness-controls:stable:0001";
const SEED_TIMESTAMP: &str = "2026-07-10T00:00:00Z";
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn component_source_refs() -> Vec<String> {
    strings(&[
        M5_GOVERNANCE_COMPONENT_MATRIX_DRI_REGISTRY_ROW_CONTRACT_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_MERGE_READINESS_STRIP_CONTRACT_REF,
    ])
}

fn row_downgrade_triggers() -> Vec<M5GovernanceComponentDowngradeTrigger> {
    vec![
        M5GovernanceComponentDowngradeTrigger::ProofStale,
        M5GovernanceComponentDowngradeTrigger::DriCoverageGap,
        M5GovernanceComponentDowngradeTrigger::EscalationHandoffUnavailable,
        M5GovernanceComponentDowngradeTrigger::TrustNarrowing,
    ]
}

fn strip_downgrade_triggers() -> Vec<M5GovernanceComponentDowngradeTrigger> {
    vec![
        M5GovernanceComponentDowngradeTrigger::ProofStale,
        M5GovernanceComponentDowngradeTrigger::PolicyBlocked,
        M5GovernanceComponentDowngradeTrigger::TrustNarrowing,
    ]
}

fn row_consumer_surfaces() -> Vec<M5GovernanceComponentConsumerSurface> {
    vec![
        M5GovernanceComponentConsumerSurface::ReviewWorkspace,
        M5GovernanceComponentConsumerSurface::Shiproom,
        M5GovernanceComponentConsumerSurface::OwnerCoveragePanel,
        M5GovernanceComponentConsumerSurface::SupportExport,
    ]
}

fn strip_consumer_surfaces() -> Vec<M5GovernanceComponentConsumerSurface> {
    vec![
        M5GovernanceComponentConsumerSurface::ReviewWorkspace,
        M5GovernanceComponentConsumerSurface::ReleaseCandidate,
        M5GovernanceComponentConsumerSurface::CliHeadless,
        M5GovernanceComponentConsumerSurface::SupportExport,
    ]
}

fn note_if(needed: bool, text: &str) -> String {
    if needed {
        text.to_owned()
    } else {
        String::new()
    }
}

/// Collects the frozen governance token the shared authority-locus resolver derives, so the vocab
/// never borrows another state's label but always carries the token it must.
fn locus_vocab(locus: &AuthorityLocusDisclosure) -> Vec<M5GovernanceStateVocab> {
    let mut vocab = Vec::new();
    if let Some(token) = locus.governance_vocab {
        vocab.push(token);
    }
    vocab
}

#[allow(clippy::too_many_arguments)]
fn dri_row(
    row_id: &str,
    service_path_identity_label: &str,
    primary_dri_alias: &str,
    backup_dri_alias: &str,
    escalation_alias: &str,
    support_forum_kind: SupportForumKind,
    support_forum_label: &str,
    support_forum_ref: &str,
    benchmark_owner_alias: &str,
    compatibility_owner_alias: &str,
    escalation_continuity_state: EscalationContinuityState,
    escalation_path_label: &str,
    owner_source_signal: OwnerSourceSignal,
    registry_freshness_state: RegistryFreshnessState,
    freshness_label: &str,
    authority_locus_source: AuthorityLocusSource,
    context_note: &str,
    rollback_posture: M5GovernanceComponentRollbackPosture,
    row_actions: Vec<DriRegistryRowAction>,
) -> DriRegistryRow {
    let locus = resolve_authority_locus(authority_locus_source);
    let owner = resolve_owner_source(owner_source_signal);
    DriRegistryRow {
        component: M5GovernanceComponent::DriRegistryRow,
        row_id: row_id.to_owned(),
        service_path_identity_label: service_path_identity_label.to_owned(),
        primary_dri_alias: primary_dri_alias.to_owned(),
        backup_dri_alias: backup_dri_alias.to_owned(),
        escalation_alias: escalation_alias.to_owned(),
        support_forum_kind,
        support_forum_label: support_forum_label.to_owned(),
        support_forum_ref: support_forum_ref.to_owned(),
        benchmark_owner_alias: benchmark_owner_alias.to_owned(),
        compatibility_owner_alias: compatibility_owner_alias.to_owned(),
        escalation_continuity_state,
        escalation_path_label: escalation_path_label.to_owned(),
        owner_source_signal,
        derived_owner_source: owner.posture,
        claims_authoritative_owner: owner.is_authoritative,
        registry_freshness_state,
        freshness_label: freshness_label.to_owned(),
        authority_locus_source,
        derived_authority_locus: locus.posture,
        claims_provider_authoritative: locus.is_provider_authoritative,
        governance_state_vocab: locus_vocab(&locus),
        local_estimate_note: note_if(
            locus.needs_local_estimate_note,
            "Freshness is a local estimate; it is not provider-confirmed",
        ),
        ci_only_note: note_if(
            locus.needs_ci_only_note,
            "Freshness was reported only by CI, not by the provider registry",
        ),
        not_evaluated_note: note_if(
            locus.needs_not_evaluated_note,
            "This registry entry was not evaluated on this build; do not read it as evaluated",
        ),
        stale_note: note_if(
            locus.needs_stale_note,
            "Registry entry is stale relative to the current base/head; refresh before trusting",
        ),
        advisory_owner_note: note_if(
            owner.needs_advisory_note,
            "Owner is an advisory heuristic guessed from the last interacting team; not authoritative",
        ),
        unresolved_owner_note: note_if(
            owner.needs_unresolved_note,
            "Owner is unresolved; escalate to find the accountable owner before handing off",
        ),
        context_note: context_note.to_owned(),
        row_actions,
        downgrade_triggers: row_downgrade_triggers(),
        consumer_surfaces: row_consumer_surfaces(),
        rollback_posture,
        source_contract_refs: component_source_refs(),
        hides_owner_or_escalation_identity: false,
        lets_advisory_owner_read_as_authoritative: false,
        guesses_owner_from_last_interacting_team: false,
        invents_alternate_state_label: false,
    }
}

#[allow(clippy::too_many_arguments)]
fn merge_strip(
    strip_id: &str,
    change_title_label: &str,
    merge_target_kind: MergeTargetKind,
    merge_target_label: &str,
    blocker_count: u32,
    blocker_summary_label: &str,
    required_next_action_kind: RequiredNextActionKind,
    required_next_action_label: &str,
    authority_locus_source: AuthorityLocusSource,
    claims_mergeable_here: bool,
    mergeability_label: &str,
    context_note: &str,
    rollback_posture: M5GovernanceComponentRollbackPosture,
    strip_actions: Vec<MergeReadinessStripAction>,
) -> MergeReadinessStrip {
    let locus = resolve_authority_locus(authority_locus_source);
    MergeReadinessStrip {
        component: M5GovernanceComponent::MergeReadinessStrip,
        strip_id: strip_id.to_owned(),
        change_title_label: change_title_label.to_owned(),
        merge_target_kind,
        merge_target_label: merge_target_label.to_owned(),
        blocker_count,
        blocker_summary_label: blocker_summary_label.to_owned(),
        required_next_action_kind,
        required_next_action_label: required_next_action_label.to_owned(),
        export_packet_action_label:
            "Export the merge-readiness packet for review or shiproom handoff".to_owned(),
        authority_locus_source,
        derived_authority_locus: locus.posture,
        claims_provider_authoritative: locus.is_provider_authoritative,
        claims_evaluated_here: locus.is_evaluated_here,
        claims_mergeable_here,
        mergeability_label: mergeability_label.to_owned(),
        export_parity_label: "Export packet mirrors the rendered merge-readiness state".to_owned(),
        governance_state_vocab: locus_vocab(&locus),
        local_estimate_note: note_if(
            locus.needs_local_estimate_note,
            "Mergeability is a local estimate; it is not provider-confirmed",
        ),
        ci_only_note: note_if(
            locus.needs_ci_only_note,
            "Mergeability was reported only by CI, not by the provider gate",
        ),
        not_evaluated_note: note_if(
            locus.needs_not_evaluated_note,
            "This merge gate was not evaluated on this build; do not read it as evaluated",
        ),
        stale_note: note_if(
            locus.needs_stale_note,
            "Merge gate is stale relative to the current base/head; re-evaluate first",
        ),
        context_note: context_note.to_owned(),
        strip_actions,
        downgrade_triggers: strip_downgrade_triggers(),
        consumer_surfaces: strip_consumer_surfaces(),
        rollback_posture,
        source_contract_refs: component_source_refs(),
        hides_target_or_blocker_count: false,
        lets_local_estimate_read_as_provider_mergeable: false,
        widens_local_estimate_to_provider_mergeability: false,
        invents_alternate_state_label: false,
    }
}

fn dri_registry_rows() -> Vec<DriRegistryRow> {
    use AuthorityLocusSource as Locus;
    use DriRegistryRowAction as Action;
    use EscalationContinuityState as Escalation;
    use M5GovernanceComponentRollbackPosture as Rollback;
    use OwnerSourceSignal as Owner;
    use RegistryFreshnessState as Fresh;
    use SupportForumKind as Forum;

    let full_actions = vec![
        Action::OpenSupportForum,
        Action::InspectOwnerSource,
        Action::ReviewEscalationPath,
        Action::InspectFreshness,
        Action::CompareOwnerHistory,
        Action::CopyRegistryDigest,
    ];

    vec![
        // 1. CODEOWNERS-authoritative owner, provider-authoritative freshness, verified.
        dri_row(
            "dri-api-gateway",
            "service/api-gateway",
            "role:api-gateway-dri",
            "role:api-gateway-backup",
            "role:platform-escalation",
            Forum::SlackChannel,
            "#api-gateway-support",
            "ref:forum/api-gateway",
            "role:api-gateway-benchmark",
            "role:api-gateway-compat",
            Escalation::ContinuousToOwner,
            "Escalate to platform on-call, then the accountable DRI",
            Owner::ForgeCodeownersRule,
            Fresh::CurrentlyVerified,
            "Verified against the current CODEOWNERS rule",
            Locus::ProviderAuthoritativeState,
            "CODEOWNERS-authoritative owner; provider-authoritative registry freshness",
            Rollback::ReadOnlyNoMutation,
            full_actions.clone(),
        ),
        // 2. Provider-team-assignment owner, provider-reported freshness, refresh due.
        dri_row(
            "dri-billing-core",
            "service/billing-core",
            "role:billing-dri",
            "role:billing-backup",
            "role:finance-escalation",
            Forum::DiscussionThread,
            "billing-core discussion",
            "ref:forum/billing-core",
            "",
            "role:billing-compat",
            Escalation::ContinuousToOwner,
            "Escalate to the finance on-call, then the billing DRI",
            Owner::ProviderTeamAssignment,
            Fresh::RefreshDue,
            "Provider-reported owner assignment; refresh due soon",
            Locus::ProviderReportedState,
            "Provider-team-assigned owner; provider-reported registry freshness",
            Rollback::ProviderMutationAttributable,
            full_actions.clone(),
        ),
        // 3. Manifest-declared owner, CI-only freshness, stale-superseded.
        dri_row(
            "dri-search-index",
            "service/search-index",
            "role:search-dri",
            "role:search-backup",
            "role:search-escalation",
            Forum::MailingList,
            "search-index@lists",
            "ref:forum/search-index",
            "role:search-benchmark",
            "",
            Escalation::DegradedFallback,
            "Primary escalation degraded; falls back to the search guild",
            Owner::RepositoryManifestDeclared,
            Fresh::StaleSuperseded,
            "Manifest-declared owner; registry entry superseded",
            Locus::CiReportedOnly,
            "Manifest-declared owner; freshness reported only by CI, not the provider",
            Rollback::LocalContinuePreserved,
            full_actions.clone(),
        ),
        // 4. Registry-declared owner, not-evaluated-here freshness, never verified.
        dri_row(
            "dri-notifications",
            "path/services/notifications",
            "role:notifications-dri",
            "",
            "role:notifications-escalation",
            Forum::TicketQueue,
            "notifications ticket queue",
            "ref:forum/notifications",
            "",
            "",
            Escalation::NotConfigured,
            "No escalation path configured; falls back to the registry owner",
            Owner::RegistryDeclared,
            Fresh::NeverVerified,
            "Registry-declared owner; entry has never been verified",
            Locus::NotEvaluatedHere,
            "Registry-declared owner; registry freshness not evaluated on this build",
            Rollback::EvidencePreservedNoRevert,
            full_actions.clone(),
        ),
        // 5. Advisory-heuristic owner, local-estimate freshness, unknown; never authoritative.
        dri_row(
            "dri-legacy-import",
            "path/legacy/import",
            "role:last-interacting-team",
            "role:legacy-backup",
            "role:legacy-escalation",
            Forum::NoForum,
            "No support forum bound",
            "",
            "",
            "role:legacy-compat",
            Escalation::BrokenNoFallback,
            "Escalation path is broken with no fallback; find an owner first",
            Owner::LastInteractingTeamHeuristic,
            Fresh::UnknownFreshness,
            "Owner guessed from the last interacting team; freshness unknown",
            Locus::LocalHeuristicEstimate,
            "Advisory owner guessed from the last interacting team; never read as authoritative",
            Rollback::LocalContinuePreserved,
            full_actions.clone(),
        ),
        // 6. Unresolved owner, stale-relative-to-head freshness, verified label but stale.
        dri_row(
            "dri-experimental-graph",
            "path/experimental/graph",
            "role:experimental-dri",
            "role:experimental-backup",
            "role:experimental-escalation",
            Forum::SlackChannel,
            "#experimental-graph",
            "ref:forum/experimental-graph",
            "role:graph-benchmark",
            "role:graph-compat",
            Escalation::ContinuousToOwner,
            "Escalate to the experimental guild, then the accountable owner",
            Owner::OwnerUnresolved,
            Fresh::CurrentlyVerified,
            "Owner unresolved; registry freshness stale relative to head",
            Locus::StaleAgainstBaseHead,
            "Owner unresolved; registry freshness stale relative to the current base/head",
            Rollback::EvidencePreservedNoRevert,
            vec![
                Action::OpenSupportForum,
                Action::InspectOwnerSource,
                Action::ReviewEscalationPath,
                Action::InspectFreshness,
            ],
        ),
    ]
}

fn merge_readiness_strips() -> Vec<MergeReadinessStrip> {
    use AuthorityLocusSource as Locus;
    use M5GovernanceComponentRollbackPosture as Rollback;
    use MergeReadinessStripAction as Action;
    use MergeTargetKind as Target;
    use RequiredNextActionKind as Next;

    let full_actions = vec![
        Action::OpenBlockerList,
        Action::InspectMergeTarget,
        Action::ExportReadinessPacket,
        Action::ReviewProviderState,
        Action::CompareLocalProvider,
        Action::CopyReadinessSummary,
    ];

    vec![
        // 1. Provider-authoritative queue target with outstanding blockers; provider says blocked.
        merge_strip(
            "mrs-queue-blocked",
            "Merge blocked: two required checks are failing",
            Target::MergeQueue,
            "Merge queue `main`",
            2,
            "2 blockers: `ci/build` and `required-review`",
            Next::ResolveBlockers,
            "Resolve the two outstanding blockers before requeueing",
            Locus::ProviderAuthoritativeState,
            false,
            "Provider-authoritative: the provider reports this change as blocked",
            "Provider-authoritative blocked merge; the blocker count names the current gate",
            Rollback::ProviderMutationAttributable,
            full_actions.clone(),
        ),
        // 2. Local-estimate branch target; never reads as provider mergeable.
        merge_strip(
            "mrs-branch-local-estimate",
            "Merge estimate: local review is clean but unconfirmed",
            Target::TargetBranch,
            "Branch `release/2026.07`",
            1,
            "1 blocker: provider confirmation pending",
            Next::RequestProviderEvaluation,
            "Request a provider-authoritative evaluation before merge",
            Locus::LocalHeuristicEstimate,
            false,
            "Local estimate: the provider has not confirmed mergeability",
            "Local-estimate mergeability; confirm with the provider before merge",
            Rollback::LocalContinuePreserved,
            full_actions.clone(),
        ),
        // 3. Stale stacked target; re-evaluate against current base/head.
        merge_strip(
            "mrs-stacked-stale",
            "Merge gate stale: evaluated against an older base",
            Target::StackedBranch,
            "Stacked branch `feature/split-3`",
            1,
            "1 blocker: base moved since evaluation",
            Next::RefreshStaleBase,
            "Refresh the stale base and re-evaluate before merge",
            Locus::StaleAgainstBaseHead,
            false,
            "Stale: the merge gate was evaluated against an older base/head",
            "Stale merge readiness; re-evaluate against the current base/head",
            Rollback::LocalContinuePreserved,
            full_actions.clone(),
        ),
        // 4. CI-only protected target; awaiting queue position, not provider-confirmed.
        merge_strip(
            "mrs-protected-ci-only",
            "Merge pending: CI reports pass but the provider gate is unconfirmed",
            Target::ProtectedBranch,
            "Protected branch `main`",
            1,
            "1 blocker: queue position pending",
            Next::AwaitQueuePosition,
            "Await the queue position; CI-only pass is not provider clearance",
            Locus::CiReportedOnly,
            false,
            "CI-only: mergeability was reported by CI, not the provider gate",
            "CI-only merge readiness; not provider-confirmed clearance",
            Rollback::LocalContinuePreserved,
            full_actions.clone(),
        ),
        // 5. Not-evaluated-here strip with no bound target; escalate to owner.
        merge_strip(
            "mrs-not-evaluated",
            "Merge gate not evaluated on this build",
            Target::NoTarget,
            "No merge target bound",
            1,
            "1 blocker: mergeability not computed here",
            Next::EscalateToOwner,
            "Escalate to the accountable owner; mergeability was not computed here",
            Locus::NotEvaluatedHere,
            false,
            "Not evaluated here: mergeability was not computed on this build",
            "Not-evaluated merge readiness; do not read it as an evaluated verdict",
            Rollback::EvidencePreservedNoRevert,
            full_actions.clone(),
        ),
        // 6. Provider-authoritative, unblocked queue target; the only mergeable-here strip.
        merge_strip(
            "mrs-mergeable",
            "Merge allowed: the provider confirms the gate is clear",
            Target::MergeQueue,
            "Merge queue `main`",
            0,
            "No current blocker",
            Next::ReadyToMerge,
            "Ready to merge; the provider confirms the gate is clear",
            Locus::ProviderReportedState,
            true,
            "Provider-authoritative: the provider reports this change as mergeable",
            "Provider-authoritative mergeable gate; no current blocker",
            Rollback::ProviderMutationAttributable,
            full_actions,
        ),
    ]
}

fn packet_downgrade_triggers() -> Vec<M5GovernanceComponentDowngradeTrigger> {
    vec![
        M5GovernanceComponentDowngradeTrigger::ProofStale,
        M5GovernanceComponentDowngradeTrigger::DriCoverageGap,
        M5GovernanceComponentDowngradeTrigger::EscalationHandoffUnavailable,
        M5GovernanceComponentDowngradeTrigger::PolicyBlocked,
        M5GovernanceComponentDowngradeTrigger::TrustNarrowing,
    ]
}

fn packet_consumer_surfaces() -> Vec<M5GovernanceComponentConsumerSurface> {
    M5GovernanceComponentConsumerSurface::ALL.to_vec()
}

fn review() -> DriRegistryMergeReadinessReview {
    DriRegistryMergeReadinessReview {
        dri_row_shows_service_and_owner_identity: true,
        dri_row_shows_escalation_and_support_path: true,
        dri_row_offers_inspect_owner_source: true,
        merge_strip_shows_target_and_blocker_count: true,
        merge_strip_shows_required_next_action: true,
        merge_strip_offers_export_packet_action: true,
        authority_locus_derived_never_asserted: true,
        local_or_ci_never_shown_as_provider_authoritative: true,
        not_evaluated_here_never_shown_as_evaluated: true,
        advisory_owner_never_shown_as_authoritative: true,
        mergeable_here_never_widens_from_local_estimate: true,
        required_next_action_present_when_blocked: true,
        stale_relative_to_base_head_always_explicit: true,
        no_surface_invents_alternate_state_label: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> DriRegistryMergeReadinessConsumerProjection {
    DriRegistryMergeReadinessConsumerProjection {
        review_workspace_reads_single_source: true,
        release_candidate_reads_single_source: true,
        governance_and_shiproom_read_single_source: true,
        owner_and_escalation_visible_before_handoff: true,
        target_and_blocker_visible_before_merge: true,
        support_export_shows_component_truth: true,
    }
}

fn proof_freshness() -> DriRegistryMergeReadinessProofFreshness {
    DriRegistryMergeReadinessProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        DRI_REGISTRY_MERGE_READINESS_CONTROLS_SCHEMA_REF,
        DRI_REGISTRY_MERGE_READINESS_CONTROLS_DOC_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_SCHEMA_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_DOC_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_DRI_REGISTRY_ROW_CONTRACT_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_MERGE_READINESS_STRIP_CONTRACT_REF,
    ])
}

fn packet() -> DriRegistryMergeReadinessControlsPacket {
    DriRegistryMergeReadinessControlsPacket::new(DriRegistryMergeReadinessControlsPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label:
            "M5 DRI-registry rows and merge-readiness strips: primary/backup role aliases, support-or-escalation path, queue-or-branch target truth, blocker counts, export-packet actions, and no-silent-mergeability widening across claimed governed review, release, and shiproom surfaces"
                .to_owned(),
        dri_registry_rows: dri_registry_rows(),
        merge_readiness_strips: merge_readiness_strips(),
        downgrade_triggers: packet_downgrade_triggers(),
        consumer_surfaces: packet_consumer_surfaces(),
        review: review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

#[test]
fn packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn authority_locus_is_derived_not_asserted() {
    use AuthorityLocusPosture as Posture;
    use AuthorityLocusSource as Src;
    assert_eq!(
        resolve_authority_locus(Src::ProviderAuthoritativeState).posture,
        Posture::ProviderAuthoritative
    );
    assert_eq!(
        resolve_authority_locus(Src::ProviderReportedState).posture,
        Posture::ProviderAuthoritative
    );
    assert_eq!(
        resolve_authority_locus(Src::LocalHeuristicEstimate).posture,
        Posture::LocalEstimate
    );
    assert_eq!(
        resolve_authority_locus(Src::CiReportedOnly).posture,
        Posture::CiOnly
    );
    assert_eq!(
        resolve_authority_locus(Src::NotEvaluatedHere).posture,
        Posture::NotEvaluatedHere
    );
    assert_eq!(
        resolve_authority_locus(Src::StaleAgainstBaseHead).posture,
        Posture::StaleRelativeToHead
    );
}

#[test]
fn only_provider_locus_is_provider_authoritative() {
    for source in AuthorityLocusSource::ALL {
        let disclosure = resolve_authority_locus(source);
        let expected = matches!(
            source,
            AuthorityLocusSource::ProviderAuthoritativeState
                | AuthorityLocusSource::ProviderReportedState
        );
        assert_eq!(
            disclosure.is_provider_authoritative, expected,
            "{source:?} provider-authoritative mismatch"
        );
    }
    // A local estimate and a CI-only signal are never provider-authoritative.
    assert!(
        !resolve_authority_locus(AuthorityLocusSource::LocalHeuristicEstimate)
            .is_provider_authoritative
    );
    assert!(
        !resolve_authority_locus(AuthorityLocusSource::CiReportedOnly).is_provider_authoritative
    );
    // A not-evaluated-here signal is never evaluated here.
    assert!(!resolve_authority_locus(AuthorityLocusSource::NotEvaluatedHere).is_evaluated_here);
}

#[test]
fn owner_source_never_promotes_advisory_to_authoritative() {
    use OwnerSourcePosture as Posture;
    use OwnerSourceSignal as Src;
    assert!(resolve_owner_source(Src::ForgeCodeownersRule).is_authoritative);
    assert!(resolve_owner_source(Src::ProviderTeamAssignment).is_authoritative);
    assert_eq!(
        resolve_owner_source(Src::LastInteractingTeamHeuristic).posture,
        Posture::AdvisoryHeuristic
    );
    // An advisory owner is never authoritative.
    assert!(!resolve_owner_source(Src::LastInteractingTeamHeuristic).is_authoritative);
    assert!(resolve_owner_source(Src::LastInteractingTeamHeuristic).is_advisory);
    assert!(!resolve_owner_source(Src::OwnerUnresolved).is_resolved);
}

#[test]
fn advisory_owner_claiming_authoritative_fails() {
    let mut packet = packet();
    let row = packet
        .dri_registry_rows
        .iter_mut()
        .find(|row| row.owner_disclosure().is_advisory)
        .expect("advisory-owner row present");
    row.claims_authoritative_owner = true;
    let violations = packet.validate();
    assert!(
        violations.contains(&DriRegistryMergeReadinessControlsViolation::OwnerSourceMisrepresented)
    );
    assert!(violations
        .contains(&DriRegistryMergeReadinessControlsViolation::AdvisoryOwnerClaimsAuthoritative));
}

#[test]
fn local_estimate_strip_claiming_provider_authoritative_fails() {
    let mut packet = packet();
    let strip = packet
        .merge_readiness_strips
        .iter_mut()
        .find(|strip| strip.authority_locus_source == AuthorityLocusSource::LocalHeuristicEstimate)
        .expect("local-estimate strip present");
    strip.claims_provider_authoritative = true;
    let violations = packet.validate();
    assert!(violations.contains(
        &DriRegistryMergeReadinessControlsViolation::LocalOrCiClaimsProviderAuthoritative
    ));
}

#[test]
fn ci_only_strip_claiming_provider_authoritative_fails() {
    let mut packet = packet();
    let strip = packet
        .merge_readiness_strips
        .iter_mut()
        .find(|strip| strip.authority_locus_source == AuthorityLocusSource::CiReportedOnly)
        .expect("ci-only strip present");
    strip.claims_provider_authoritative = true;
    let violations = packet.validate();
    assert!(violations.contains(
        &DriRegistryMergeReadinessControlsViolation::LocalOrCiClaimsProviderAuthoritative
    ));
}

#[test]
fn not_evaluated_here_claiming_evaluated_fails() {
    let mut packet = packet();
    let strip = packet
        .merge_readiness_strips
        .iter_mut()
        .find(|strip| strip.authority_locus_source == AuthorityLocusSource::NotEvaluatedHere)
        .expect("not-evaluated strip present");
    strip.claims_evaluated_here = true;
    let violations = packet.validate();
    assert!(violations
        .contains(&DriRegistryMergeReadinessControlsViolation::NotEvaluatedClaimsEvaluated));
}

#[test]
fn mergeable_here_without_provider_clearance_fails() {
    let mut packet = packet();
    // The local-estimate strip must never widen to mergeable-here.
    let strip = packet
        .merge_readiness_strips
        .iter_mut()
        .find(|strip| strip.authority_locus_source == AuthorityLocusSource::LocalHeuristicEstimate)
        .expect("local-estimate strip present");
    strip.claims_mergeable_here = true;
    assert!(packet.validate().contains(
        &DriRegistryMergeReadinessControlsViolation::MergeableHereWithoutProviderClearance
    ));
}

#[test]
fn mergeable_here_with_outstanding_blocker_fails() {
    let mut packet = packet();
    // Even a provider-authoritative strip cannot claim mergeable-here while a blocker remains.
    let strip = packet
        .merge_readiness_strips
        .iter_mut()
        .find(|strip| strip.blocker_count > 0 && strip.locus_disclosure().is_provider_authoritative)
        .expect("provider-authoritative blocked strip present");
    strip.claims_mergeable_here = true;
    assert!(packet.validate().contains(
        &DriRegistryMergeReadinessControlsViolation::MergeableHereWithoutProviderClearance
    ));
}

#[test]
fn ci_only_note_required() {
    let mut packet = packet();
    let row = packet
        .dri_registry_rows
        .iter_mut()
        .find(|row| row.authority_locus_source == AuthorityLocusSource::CiReportedOnly)
        .expect("ci-only row present");
    row.ci_only_note = String::new();
    assert!(packet
        .validate()
        .contains(&DriRegistryMergeReadinessControlsViolation::CiOnlyNoteMissing));
}

#[test]
fn stale_note_required() {
    let mut packet = packet();
    let row = packet
        .dri_registry_rows
        .iter_mut()
        .find(|row| row.authority_locus_source == AuthorityLocusSource::StaleAgainstBaseHead)
        .expect("stale row present");
    row.stale_note = String::new();
    assert!(packet
        .validate()
        .contains(&DriRegistryMergeReadinessControlsViolation::StaleNoteMissing));
}

#[test]
fn advisory_owner_note_required() {
    let mut packet = packet();
    let row = packet
        .dri_registry_rows
        .iter_mut()
        .find(|row| row.owner_disclosure().is_advisory)
        .expect("advisory-owner row present");
    row.advisory_owner_note = String::new();
    assert!(packet
        .validate()
        .contains(&DriRegistryMergeReadinessControlsViolation::AdvisoryOwnerNoteMissing));
}

#[test]
fn unresolved_owner_note_required() {
    let mut packet = packet();
    let row = packet
        .dri_registry_rows
        .iter_mut()
        .find(|row| !row.owner_disclosure().is_resolved)
        .expect("unresolved-owner row present");
    row.unresolved_owner_note = String::new();
    assert!(packet
        .validate()
        .contains(&DriRegistryMergeReadinessControlsViolation::UnresolvedOwnerNoteMissing));
}

#[test]
fn governance_vocab_missing_locus_token_fails() {
    let mut packet = packet();
    let strip = packet
        .merge_readiness_strips
        .iter_mut()
        .find(|strip| strip.authority_locus_source == AuthorityLocusSource::LocalHeuristicEstimate)
        .expect("local-estimate strip present");
    strip.governance_state_vocab.clear();
    assert!(packet
        .validate()
        .contains(&DriRegistryMergeReadinessControlsViolation::GovernanceVocabMissingLocusToken));
}

#[test]
fn person_contact_detail_in_alias_fails() {
    let mut packet = packet();
    packet.dri_registry_rows[0].primary_dri_alias = "person@example.com".to_owned();
    assert!(packet
        .validate()
        .contains(&DriRegistryMergeReadinessControlsViolation::PersonContactDetailInAlias));
}

#[test]
fn inspect_owner_source_action_required() {
    let mut packet = packet();
    packet.dri_registry_rows[0]
        .row_actions
        .retain(|action| *action != DriRegistryRowAction::InspectOwnerSource);
    let violations = packet.validate();
    assert!(violations
        .contains(&DriRegistryMergeReadinessControlsViolation::InspectOwnerSourceActionMissing));
    assert!(violations
        .contains(&DriRegistryMergeReadinessControlsViolation::ComponentActionsIncomplete));
}

#[test]
fn export_readiness_action_required() {
    let mut packet = packet();
    packet.merge_readiness_strips[0]
        .strip_actions
        .retain(|action| *action != MergeReadinessStripAction::ExportReadinessPacket);
    let violations = packet.validate();
    assert!(violations
        .contains(&DriRegistryMergeReadinessControlsViolation::ExportReadinessActionMissing));
    assert!(violations
        .contains(&DriRegistryMergeReadinessControlsViolation::ComponentActionsIncomplete));
}

#[test]
fn resolvable_support_forum_requires_ref() {
    let mut packet = packet();
    let row = packet
        .dri_registry_rows
        .iter_mut()
        .find(|row| row.support_forum_kind.is_resolvable())
        .expect("resolvable-forum row present");
    row.support_forum_ref = String::new();
    assert!(packet
        .validate()
        .contains(&DriRegistryMergeReadinessControlsViolation::SupportForumRefMissing));
}

#[test]
fn blocked_strip_missing_summary_fails() {
    let mut packet = packet();
    let strip = packet
        .merge_readiness_strips
        .iter_mut()
        .find(|strip| strip.blocker_count > 0)
        .expect("blocked strip present");
    strip.blocker_summary_label = String::new();
    assert!(packet
        .validate()
        .contains(&DriRegistryMergeReadinessControlsViolation::BlockedStripMissingNextAction));
}

#[test]
fn wrong_component_class_fails() {
    let mut packet = packet();
    packet.dri_registry_rows[0].component = M5GovernanceComponent::MergeReadinessStrip;
    assert!(packet
        .validate()
        .contains(&DriRegistryMergeReadinessControlsViolation::DriRegistryRowWrongComponentClass));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&DriRegistryMergeReadinessControlsViolation::MissingSourceContracts));
}

#[test]
fn review_incomplete_fails() {
    let mut packet = packet();
    packet
        .review
        .mergeable_here_never_widens_from_local_estimate = false;
    assert!(packet
        .validate()
        .contains(&DriRegistryMergeReadinessControlsViolation::ReviewIncomplete));
}

#[test]
fn rows_alone_cover_every_owner_source_and_posture() {
    let packet = packet();
    assert!(packet.validate().is_empty());
    let mut signals: BTreeSet<OwnerSourceSignal> = BTreeSet::new();
    let mut postures: BTreeSet<OwnerSourcePosture> = BTreeSet::new();
    for row in &packet.dri_registry_rows {
        signals.insert(row.owner_source_signal);
        postures.insert(row.owner_disclosure().posture);
    }
    // AC-1: an advisory owner is always separable from an authoritative one, from the rows alone.
    assert_eq!(signals.len(), OwnerSourceSignal::ALL.len());
    assert_eq!(postures.len(), OwnerSourcePosture::ALL.len());
}

#[test]
fn components_cover_every_authority_locus_source_and_posture() {
    let packet = packet();
    let mut sources: BTreeSet<AuthorityLocusSource> = BTreeSet::new();
    let mut postures: BTreeSet<AuthorityLocusPosture> = BTreeSet::new();
    for row in &packet.dri_registry_rows {
        sources.insert(row.authority_locus_source);
        postures.insert(row.locus_disclosure().posture);
    }
    for strip in &packet.merge_readiness_strips {
        sources.insert(strip.authority_locus_source);
        postures.insert(strip.locus_disclosure().posture);
    }
    assert_eq!(sources.len(), AuthorityLocusSource::ALL.len());
    assert_eq!(postures.len(), AuthorityLocusPosture::ALL.len());
}

#[test]
fn strips_cover_every_target_and_next_action() {
    let packet = packet();
    let mut targets: BTreeSet<MergeTargetKind> = BTreeSet::new();
    let mut next_actions: BTreeSet<RequiredNextActionKind> = BTreeSet::new();
    for strip in &packet.merge_readiness_strips {
        targets.insert(strip.merge_target_kind);
        next_actions.insert(strip.required_next_action_kind);
    }
    assert_eq!(targets.len(), MergeTargetKind::ALL.len());
    assert_eq!(next_actions.len(), RequiredNextActionKind::ALL.len());
}

#[test]
fn markdown_summary_lists_every_component() {
    let summary = packet().render_markdown_summary();
    for row in packet().dri_registry_rows {
        assert!(
            summary.contains(&row.service_path_identity_label),
            "summary missing row {}",
            row.row_id
        );
    }
    for strip in packet().merge_readiness_strips {
        assert!(
            summary.contains(&strip.change_title_label),
            "summary missing strip {}",
            strip.strip_id
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_dri_registry_merge_readiness_controls_export()
        .expect("checked dri-registry merge-readiness controls export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed_packet() {
    let seed = packet();
    let checked = current_dri_registry_merge_readiness_controls_export()
        .expect("checked dri-registry merge-readiness controls export validates");
    assert_eq!(checked, seed);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-dri-registry-merge-readiness-controls/dri_registry_row_advisory_owner.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-dri-registry-merge-readiness-controls/merge_readiness_strip_local_estimate.json"
        )),
    ] {
        let packet: DriRegistryMergeReadinessControlsPacket =
            serde_json::from_str(raw).expect("fixture parses as dri-registry merge-readiness packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

/// Regenerates the checked release artifacts and narrowed fixtures.
///
/// Guarded behind `GEN_DRI_REGISTRY_MERGE_READINESS_CONTROLS_ARTIFACTS` so ordinary test runs never
/// touch the working tree. Run in isolation with the env gate set, then run the full suite:
/// `GEN_DRI_REGISTRY_MERGE_READINESS_CONTROLS_ARTIFACTS=1 cargo test -p aureline-review
/// implement_dri_registry_rows_and_merge_readiness_strips_with_primary_backup_role_aliases_support_or_escalation_path_queue_or_branch_target_truth_blocker_counts_export_packet_actions_and_no_silent_mergeability_widening::tests::generate_artifacts
/// -- --exact --ignored`
#[test]
#[ignore = "artifact generator; run explicitly with the env gate set"]
fn generate_artifacts() {
    if std::env::var("GEN_DRI_REGISTRY_MERGE_READINESS_CONTROLS_ARTIFACTS").is_err() {
        return;
    }

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let repo_root = std::path::Path::new(manifest_dir).join("..").join("..");

    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    let proof_dir = repo_root
        .join("artifacts")
        .join("release")
        .join("m5-dri-registry-merge-readiness-controls-proof");
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
        .join("m5-dri-registry-merge-readiness-controls");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");

    // Fixture 1: an advisory-owner DRI row that must never read as an authoritative owner.
    let mut advisory_owner = packet.clone();
    advisory_owner.packet_id =
        "m5-dri-registry-merge-readiness-controls:fixture:advisory-owner".to_owned();
    advisory_owner.surface_label =
        "M5 DRI-registry rows: an advisory owner is never shown as authoritative".to_owned();
    assert!(
        advisory_owner.validate().is_empty(),
        "{:?}",
        advisory_owner.validate()
    );
    std::fs::write(
        fixture_dir.join("dri_registry_row_advisory_owner.json"),
        format!("{}\n", advisory_owner.export_safe_json()),
    )
    .expect("write advisory-owner fixture");

    // Fixture 2: a local-estimate merge-readiness strip that must never read as provider mergeable.
    let mut local_estimate = packet;
    local_estimate.packet_id =
        "m5-dri-registry-merge-readiness-controls:fixture:local-estimate".to_owned();
    local_estimate.surface_label =
        "M5 merge-readiness strips: a local-estimate gate never reads as provider-mergeable"
            .to_owned();
    assert!(
        local_estimate.validate().is_empty(),
        "{:?}",
        local_estimate.validate()
    );
    std::fs::write(
        fixture_dir.join("merge_readiness_strip_local_estimate.json"),
        format!("{}\n", local_estimate.export_safe_json()),
    )
    .expect("write local-estimate fixture");
}
