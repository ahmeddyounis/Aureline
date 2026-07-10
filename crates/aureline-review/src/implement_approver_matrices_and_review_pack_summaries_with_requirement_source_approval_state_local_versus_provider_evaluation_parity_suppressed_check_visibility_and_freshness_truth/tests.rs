use super::*;

const PACKET_ID: &str = "m5-approver-review-pack-controls:stable:0001";
const SEED_TIMESTAMP: &str = "2026-07-10T00:00:00Z";
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn component_source_refs() -> Vec<String> {
    strings(&[
        M5_GOVERNANCE_COMPONENT_MATRIX_APPROVER_MATRIX_CONTRACT_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_REVIEW_PACK_SUMMARY_CONTRACT_REF,
    ])
}

fn row_downgrade_triggers() -> Vec<M5GovernanceComponentDowngradeTrigger> {
    vec![
        M5GovernanceComponentDowngradeTrigger::ProofStale,
        M5GovernanceComponentDowngradeTrigger::ApproverStateExpired,
        M5GovernanceComponentDowngradeTrigger::TrustNarrowing,
    ]
}

fn summary_downgrade_triggers() -> Vec<M5GovernanceComponentDowngradeTrigger> {
    vec![
        M5GovernanceComponentDowngradeTrigger::ProofStale,
        M5GovernanceComponentDowngradeTrigger::ReviewPackStale,
        M5GovernanceComponentDowngradeTrigger::TrustNarrowing,
    ]
}

fn row_consumer_surfaces() -> Vec<M5GovernanceComponentConsumerSurface> {
    vec![
        M5GovernanceComponentConsumerSurface::ReviewWorkspace,
        M5GovernanceComponentConsumerSurface::GovernanceDashboard,
        M5GovernanceComponentConsumerSurface::CliHeadless,
        M5GovernanceComponentConsumerSurface::SupportExport,
    ]
}

fn summary_consumer_surfaces() -> Vec<M5GovernanceComponentConsumerSurface> {
    vec![
        M5GovernanceComponentConsumerSurface::ReviewWorkspace,
        M5GovernanceComponentConsumerSurface::ReleaseCandidate,
        M5GovernanceComponentConsumerSurface::Shiproom,
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

/// Collects the frozen governance tokens both resolvers derive, so the vocab never borrows another
/// state's label but always carries the tokens it must.
fn locus_and_approver_vocab(
    locus: &EvaluationLocusDisclosure,
    approver: Option<&ApproverStateDisclosure>,
) -> Vec<M5GovernanceStateVocab> {
    let mut vocab = Vec::new();
    if let Some(token) = locus.governance_vocab {
        vocab.push(token);
    }
    if let Some(approver) = approver {
        if let Some(token) = approver.governance_vocab {
            if !vocab.contains(&token) {
                vocab.push(token);
            }
        }
    }
    vocab
}

#[allow(clippy::too_many_arguments)]
fn approver_matrix_row(
    row_id: &str,
    approver_role_label: &str,
    requirement_source_class: RequirementSourceClass,
    requirement_source_label: &str,
    evaluation_locus_source: EvaluationLocusSource,
    approver_state_source: ApproverStateSource,
    evidence_link_kind: EvidenceLinkKind,
    evidence_link_ref: &str,
    expiry_label: &str,
    context_note: &str,
    rollback_posture: M5GovernanceComponentRollbackPosture,
    row_actions: Vec<ApproverMatrixAction>,
) -> ApproverMatrixRow {
    let locus = resolve_evaluation_locus(evaluation_locus_source);
    let approver = resolve_approver_state(approver_state_source);
    ApproverMatrixRow {
        component: M5GovernanceComponent::ApproverMatrix,
        row_id: row_id.to_owned(),
        approver_role_label: approver_role_label.to_owned(),
        requirement_source_class,
        requirement_source_label: requirement_source_label.to_owned(),
        evaluation_locus_source,
        derived_evaluation_locus: locus.posture,
        claims_provider_authoritative: locus.is_provider_authoritative,
        approver_state_source,
        derived_approver_state: approver.posture,
        claims_satisfied: approver.is_satisfied,
        governance_state_vocab: locus_and_approver_vocab(&locus, Some(&approver)),
        local_only_note: note_if(
            locus.needs_local_only_note,
            "Approval was evaluated only locally; it is not the provider's final gate",
        ),
        ci_only_note: note_if(
            locus.needs_ci_only_note,
            "Approval is reported only by CI; it is not the provider's resolved gate",
        ),
        not_evaluated_note: note_if(
            locus.needs_not_evaluated_note,
            "This approval was not evaluated on this build; do not read it as evaluated",
        ),
        stale_note: note_if(
            locus.needs_stale_note,
            "Approval evaluation is stale relative to the current base/head; re-evaluate first",
        ),
        waived_note: note_if(
            approver.needs_waived_note,
            "This required approval is waived by policy; it is not an approval given",
        ),
        expired_note: note_if(
            approver.needs_expired_note,
            "This approval has expired; a fresh approval is required before merge",
        ),
        pending_note: note_if(
            approver.needs_pending_note,
            "This required approval is still pending; it is not yet satisfied",
        ),
        expiry_label: expiry_label.to_owned(),
        evidence_link_kind,
        evidence_link_ref: evidence_link_ref.to_owned(),
        context_note: context_note.to_owned(),
        row_actions,
        downgrade_triggers: row_downgrade_triggers(),
        consumer_surfaces: row_consumer_surfaces(),
        rollback_posture,
        source_contract_refs: component_source_refs(),
        hides_requirement_source_or_state: false,
        lets_waived_or_expired_read_as_satisfied: false,
        lets_ci_or_local_read_as_provider_authoritative: false,
        invents_alternate_state_label: false,
    }
}

#[allow(clippy::too_many_arguments)]
fn review_pack_summary(
    summary_id: &str,
    pack_digest_label: &str,
    base_identity_label: &str,
    head_identity_label: &str,
    capability_set: Vec<PackCapability>,
    capability_set_label: &str,
    evaluation_locus_source: EvaluationLocusSource,
    parity_label: &str,
    freshness_label: &str,
    suppressed_checks: Vec<SuppressedCheck>,
    context_note: &str,
    rollback_posture: M5GovernanceComponentRollbackPosture,
    summary_actions: Vec<ReviewPackSummaryAction>,
) -> ReviewPackSummary {
    let locus = resolve_evaluation_locus(evaluation_locus_source);
    ReviewPackSummary {
        component: M5GovernanceComponent::ReviewPackSummary,
        summary_id: summary_id.to_owned(),
        pack_digest_label: pack_digest_label.to_owned(),
        base_identity_label: base_identity_label.to_owned(),
        head_identity_label: head_identity_label.to_owned(),
        capability_set,
        capability_set_label: capability_set_label.to_owned(),
        evaluation_locus_source,
        derived_evaluation_locus: locus.posture,
        claims_provider_authoritative: locus.is_provider_authoritative,
        claims_evaluated_here: locus.is_evaluated_here,
        parity_label: parity_label.to_owned(),
        freshness_label: freshness_label.to_owned(),
        governance_state_vocab: locus_and_approver_vocab(&locus, None),
        local_only_note: note_if(
            locus.needs_local_only_note,
            "Pack was evaluated only locally; it is not provider-authoritative",
        ),
        ci_only_note: note_if(
            locus.needs_ci_only_note,
            "Pack is reported only by CI; it is not the provider's resolved gate",
        ),
        not_evaluated_note: note_if(
            locus.needs_not_evaluated_note,
            "Pack was not evaluated on this build; do not read it as evaluated here",
        ),
        stale_note: note_if(
            locus.needs_stale_note,
            "Pack evaluation is stale relative to the current base/head; re-evaluate first",
        ),
        suppressed_checks,
        suppressed_checks_label: "Suppressed checks and waivers are listed explicitly".to_owned(),
        context_note: context_note.to_owned(),
        summary_actions,
        downgrade_triggers: summary_downgrade_triggers(),
        consumer_surfaces: summary_consumer_surfaces(),
        rollback_posture,
        source_contract_refs: component_source_refs(),
        hides_parity_or_freshness: false,
        lets_ci_or_local_read_as_provider_authoritative: false,
        hides_suppressed_checks_or_waivers: false,
        invents_alternate_state_label: false,
    }
}

fn suppressed(check_label: &str, class: PackSuppressionClass, reason: &str) -> SuppressedCheck {
    SuppressedCheck {
        check_label: check_label.to_owned(),
        suppression_class: class,
        reason_label: reason.to_owned(),
    }
}

fn approver_matrix_rows() -> Vec<ApproverMatrixRow> {
    use ApproverMatrixAction as Action;
    use ApproverStateSource as State;
    use EvaluationLocusSource as Locus;
    use EvidenceLinkKind as Evidence;
    use M5GovernanceComponentRollbackPosture as Rollback;
    use RequirementSourceClass as Req;

    let full_actions = vec![
        Action::OpenEvidenceLink,
        Action::InspectRequirementSource,
        Action::ReviewApproverState,
        Action::InspectEvaluationParity,
        Action::ReviewExpiry,
        Action::CopyApproverRoles,
    ];

    vec![
        // 1. Provider-enforced gate / recorded approval → provider-authoritative, satisfied.
        approver_matrix_row(
            "apr-crypto-security",
            "security-team",
            Req::BranchProtectionRule,
            "Provider branch-protection rule requires a security review",
            Locus::ProviderEnforcedGate,
            State::RequiredApprovalRecorded,
            Evidence::ProviderApprovalRecord,
            "evidence:approval/crypto-security",
            "",
            "Provider-enforced approval requirement; open the approval record to see who approved",
            Rollback::ReadOnlyNoMutation,
            full_actions.clone(),
        ),
        // 2. Provider-reported status / provider-confirmed → provider-authoritative, satisfied.
        approver_matrix_row(
            "apr-codeowners-api",
            "api-platform-team",
            Req::CodeownersRule,
            "CODEOWNERS rule requires an API-platform owner review",
            Locus::ProviderReportedStatus,
            State::ProviderConfirmedApproval,
            Evidence::CiCheckRun,
            "evidence:check/api-owner-review",
            "",
            "Provider-reported owner approval; open the check run to see the recorded review",
            Rollback::ProviderMutationAttributable,
            full_actions.clone(),
        ),
        // 3. Local evaluation only / awaiting approval → local-only, pending.
        approver_matrix_row(
            "apr-local-policy",
            "release-review-role",
            Req::ReviewPolicyRule,
            "Local review-policy rule requires a release reviewer",
            Locus::LocalEvaluationOnly,
            State::AwaitingRequiredApproval,
            Evidence::LocalEvaluationRecord,
            "evidence:local/release-review",
            "",
            "Locally evaluated requirement; the approval is still pending here",
            Rollback::LocalContinuePreserved,
            full_actions.clone(),
        ),
        // 4. CI-reported only / changes requested → CI-only, pending.
        approver_matrix_row(
            "apr-ci-changes",
            "qa-review-role",
            Req::ManualReviewRequest,
            "A manual review request opened this requirement",
            Locus::CiReportedOnly,
            State::ChangesRequestedPending,
            Evidence::CiCheckRun,
            "evidence:check/qa-changes-requested",
            "",
            "CI-reported changes requested; this is not the provider's final gate",
            Rollback::LocalContinuePreserved,
            full_actions.clone(),
        ),
        // 5. Not evaluated here / waived by policy → not-evaluated-here, waived.
        approver_matrix_row(
            "apr-waived-policy",
            "governance-owner-role",
            Req::ReviewPolicyRule,
            "Local review-policy rule required a governance review",
            Locus::NotEvaluatedHere,
            State::ApprovalWaivedByPolicy,
            Evidence::WaiverRecord,
            "evidence:waiver/governance-review",
            "Waiver expires at the next release-candidate cut",
            "Approval waived by policy; a waiver is not an approval given",
            Rollback::EvidencePreservedNoRevert,
            full_actions.clone(),
        ),
        // 6. Stale against base/head / expired approval → stale, expired.
        approver_matrix_row(
            "apr-expired-review",
            "unassigned-review-role",
            Req::Unresolved,
            "Requirement source could not be resolved on this build",
            Locus::StaleAgainstBaseHead,
            State::ApprovalExpired,
            Evidence::NoEvidenceLink,
            "",
            "Approval expired when head advanced past the reviewed commit",
            "Approval expired and evaluation is stale; re-request before merge",
            Rollback::LocalContinuePreserved,
            vec![
                Action::OpenEvidenceLink,
                Action::InspectRequirementSource,
                Action::ReviewApproverState,
                Action::ReviewExpiry,
            ],
        ),
    ]
}

fn review_pack_summaries() -> Vec<ReviewPackSummary> {
    use EvaluationLocusSource as Locus;
    use M5GovernanceComponentRollbackPosture as Rollback;
    use PackCapability as Cap;
    use PackSuppressionClass as Suppress;
    use ReviewPackSummaryAction as Action;

    let full_actions = vec![
        Action::InspectEvaluationParity,
        Action::ReviewSuppressedChecks,
        Action::OpenPackDigest,
        Action::ReviewBaseHeadIdentity,
        Action::InspectCapabilitySet,
        Action::CopyPackDigest,
    ];

    vec![
        // 1. Provider-enforced gate → provider-authoritative.
        review_pack_summary(
            "rps-provider-authoritative",
            "pack:digest/release-provider",
            "base:main@a1b2c3",
            "head:feature@d4e5f6",
            vec![Cap::OwnershipApproval, Cap::ProtectedPathGate],
            "Ownership approval and protected-path gate",
            Locus::ProviderEnforcedGate,
            "Provider-authoritative: the provider enforced this pack",
            "Current against the recorded base/head",
            vec![suppressed(
                "optional-style-check",
                Suppress::WaivedApproval,
                "Style approval waived by policy for this pack",
            )],
            "Provider-authoritative pack; suppressed checks are still listed",
            Rollback::ReadOnlyNoMutation,
            full_actions.clone(),
        ),
        // 2. Local evaluation only → local-only.
        review_pack_summary(
            "rps-local-only",
            "pack:digest/local-eval",
            "base:main@a1b2c3",
            "head:feature@d4e5f6",
            vec![Cap::PublicSurfaceDiff, Cap::PolicyGate],
            "Public-surface diff and policy gate",
            Locus::LocalEvaluationOnly,
            "Local-only: evaluated locally, not provider-confirmed",
            "Current locally against the recorded base/head",
            vec![suppressed(
                "provider-status-check",
                Suppress::SkippedCheck,
                "Provider status check skipped in the local evaluation",
            )],
            "Local-only pack; it is not provider-authoritative",
            Rollback::LocalContinuePreserved,
            full_actions.clone(),
        ),
        // 3. CI-reported only → CI-only.
        review_pack_summary(
            "rps-ci-only",
            "pack:digest/ci-report",
            "base:main@a1b2c3",
            "head:feature@d4e5f6",
            vec![Cap::CiStatusRollup, Cap::OwnershipApproval],
            "CI status rollup and ownership approval",
            Locus::CiReportedOnly,
            "CI-only: reported by CI, not the provider's resolved gate",
            "Current against the CI-reported base/head",
            vec![suppressed(
                "provider-required-review",
                Suppress::ProviderExcluded,
                "Provider-required review excluded from the CI-only rollup",
            )],
            "CI-only pack; it is not the provider's final gate",
            Rollback::LocalContinuePreserved,
            full_actions.clone(),
        ),
        // 4. Not evaluated here → not-evaluated-here.
        review_pack_summary(
            "rps-not-evaluated",
            "pack:digest/not-evaluated",
            "base:main@a1b2c3",
            "head:feature@d4e5f6",
            vec![Cap::ProtectedPathGate, Cap::PolicyGate],
            "Protected-path gate and policy gate",
            Locus::NotEvaluatedHere,
            "Not evaluated here: this pack was not run on this build",
            "Not evaluated against base/head on this build",
            vec![suppressed(
                "policy-owner-check",
                Suppress::PolicySuppressed,
                "Policy owner check suppressed by policy on this build",
            )],
            "Not-evaluated-here pack; do not read it as evaluated",
            Rollback::EvidencePreservedNoRevert,
            full_actions.clone(),
        ),
        // 5. Stale against base/head → stale.
        review_pack_summary(
            "rps-stale",
            "pack:digest/stale",
            "base:main@a1b2c3",
            "head:feature@d4e5f6",
            vec![Cap::PublicSurfaceDiff, Cap::CiStatusRollup],
            "Public-surface diff and CI status rollup",
            Locus::StaleAgainstBaseHead,
            "Stale: evaluated against an older base/head",
            "Stale relative to the current base/head; re-evaluate first",
            Vec::new(),
            "Stale pack; re-evaluate before trusting the result",
            Rollback::LocalContinuePreserved,
            full_actions,
        ),
    ]
}

fn downgrade_triggers() -> Vec<M5GovernanceComponentDowngradeTrigger> {
    vec![
        M5GovernanceComponentDowngradeTrigger::ProofStale,
        M5GovernanceComponentDowngradeTrigger::ApproverStateExpired,
        M5GovernanceComponentDowngradeTrigger::ReviewPackStale,
        M5GovernanceComponentDowngradeTrigger::PolicyBlocked,
        M5GovernanceComponentDowngradeTrigger::EscalationHandoffUnavailable,
        M5GovernanceComponentDowngradeTrigger::TrustNarrowing,
    ]
}

fn consumer_surfaces() -> Vec<M5GovernanceComponentConsumerSurface> {
    M5GovernanceComponentConsumerSurface::ALL.to_vec()
}

fn review() -> ApproverReviewPackReview {
    ApproverReviewPackReview {
        approver_matrix_shows_requirement_source_and_state: true,
        approver_matrix_names_role_and_evidence: true,
        approver_matrix_offers_open_evidence_link: true,
        review_pack_summary_shows_digest_and_base_head: true,
        review_pack_summary_shows_parity_and_freshness: true,
        review_pack_summary_lists_suppressed_checks_and_waivers: true,
        evaluation_locus_derived_never_asserted: true,
        ci_or_local_never_shown_as_provider_authoritative: true,
        not_evaluated_here_never_shown_as_evaluated: true,
        waived_or_expired_never_shown_as_satisfied: true,
        stale_relative_to_base_head_always_explicit: true,
        approver_roles_use_export_safe_aliases: true,
        capability_set_always_explicit: true,
        no_surface_invents_alternate_state_label: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> ApproverReviewPackConsumerProjection {
    ApproverReviewPackConsumerProjection {
        review_workspace_reads_single_source: true,
        release_candidate_reads_single_source: true,
        governance_and_shiproom_read_single_source: true,
        requirement_and_state_visible_before_signoff: true,
        parity_and_freshness_visible_before_signoff: true,
        support_export_shows_component_truth: true,
    }
}

fn proof_freshness() -> ApproverReviewPackProofFreshness {
    ApproverReviewPackProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        APPROVER_REVIEW_PACK_CONTROLS_SCHEMA_REF,
        APPROVER_REVIEW_PACK_CONTROLS_DOC_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_SCHEMA_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_DOC_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_APPROVER_MATRIX_CONTRACT_REF,
        M5_GOVERNANCE_COMPONENT_MATRIX_REVIEW_PACK_SUMMARY_CONTRACT_REF,
    ])
}

fn packet() -> ApproverReviewPackControlsPacket {
    ApproverReviewPackControlsPacket::new(ApproverReviewPackControlsPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label:
            "M5 approver matrices and review-pack summaries: requirement source, satisfied-pending-waived-expired state, local-versus-provider evaluation parity, suppressed-check visibility, and freshness truth across claimed governed surfaces"
                .to_owned(),
        approver_matrix_rows: approver_matrix_rows(),
        review_pack_summaries: review_pack_summaries(),
        downgrade_triggers: downgrade_triggers(),
        consumer_surfaces: consumer_surfaces(),
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
fn evaluation_locus_is_derived_not_asserted() {
    use EvaluationLocusPosture as Posture;
    use EvaluationLocusSource as Src;
    assert_eq!(
        resolve_evaluation_locus(Src::ProviderEnforcedGate).posture,
        Posture::ProviderAuthoritative
    );
    assert_eq!(
        resolve_evaluation_locus(Src::ProviderReportedStatus).posture,
        Posture::ProviderAuthoritative
    );
    assert_eq!(
        resolve_evaluation_locus(Src::LocalEvaluationOnly).posture,
        Posture::LocalOnly
    );
    assert_eq!(
        resolve_evaluation_locus(Src::CiReportedOnly).posture,
        Posture::CiOnly
    );
    assert_eq!(
        resolve_evaluation_locus(Src::NotEvaluatedHere).posture,
        Posture::NotEvaluatedHere
    );
    assert_eq!(
        resolve_evaluation_locus(Src::StaleAgainstBaseHead).posture,
        Posture::StaleRelativeToHead
    );
}

#[test]
fn only_provider_locus_is_provider_authoritative() {
    for source in EvaluationLocusSource::ALL {
        let disclosure = resolve_evaluation_locus(source);
        let expected = matches!(
            source,
            EvaluationLocusSource::ProviderEnforcedGate
                | EvaluationLocusSource::ProviderReportedStatus
        );
        assert_eq!(
            disclosure.is_provider_authoritative, expected,
            "{source:?} provider-authoritative mismatch"
        );
    }
    // AC-1: a CI-only and a local-only pack are never provider-authoritative.
    assert!(
        !resolve_evaluation_locus(EvaluationLocusSource::CiReportedOnly).is_provider_authoritative
    );
    assert!(
        !resolve_evaluation_locus(EvaluationLocusSource::LocalEvaluationOnly)
            .is_provider_authoritative
    );
    // A not-evaluated-here pack is never evaluated here.
    assert!(!resolve_evaluation_locus(EvaluationLocusSource::NotEvaluatedHere).is_evaluated_here);
}

#[test]
fn approver_state_never_collapses_waived_or_expired_into_satisfied() {
    use ApproverStatePosture as Posture;
    use ApproverStateSource as Src;
    assert!(resolve_approver_state(Src::RequiredApprovalRecorded).is_satisfied);
    assert!(resolve_approver_state(Src::ProviderConfirmedApproval).is_satisfied);
    for source in [Src::ApprovalWaivedByPolicy, Src::ApprovalExpired] {
        assert!(
            !resolve_approver_state(source).is_satisfied,
            "{source:?} must never read as satisfied"
        );
    }
    assert_eq!(
        resolve_approver_state(Src::ApprovalWaivedByPolicy).governance_vocab,
        Some(M5GovernanceStateVocab::Waived)
    );
    assert_eq!(
        resolve_approver_state(Src::ApprovalExpired).governance_vocab,
        Some(M5GovernanceStateVocab::Expired)
    );
    assert_eq!(
        resolve_approver_state(Src::ApprovalWaivedByPolicy).posture,
        Posture::Waived
    );
}

#[test]
fn ci_or_local_claiming_provider_authoritative_fails() {
    let mut packet = packet();
    let summary = packet
        .review_pack_summaries
        .iter_mut()
        .find(|summary| summary.evaluation_locus_source == EvaluationLocusSource::CiReportedOnly)
        .expect("ci-only summary present");
    summary.claims_provider_authoritative = true;
    let violations = packet.validate();
    assert!(violations
        .contains(&ApproverReviewPackControlsViolation::CiOrLocalClaimsProviderAuthoritative));
}

#[test]
fn not_evaluated_here_claiming_evaluated_fails() {
    let mut packet = packet();
    let summary = packet
        .review_pack_summaries
        .iter_mut()
        .find(|summary| summary.evaluation_locus_source == EvaluationLocusSource::NotEvaluatedHere)
        .expect("not-evaluated summary present");
    summary.claims_evaluated_here = true;
    let violations = packet.validate();
    assert!(violations.contains(&ApproverReviewPackControlsViolation::NotEvaluatedClaimsEvaluated));
}

#[test]
fn waived_or_expired_claiming_satisfied_fails() {
    let mut packet = packet();
    let row = packet
        .approver_matrix_rows
        .iter_mut()
        .find(|row| row.approver_state_source == ApproverStateSource::ApprovalWaivedByPolicy)
        .expect("waived row present");
    row.claims_satisfied = true;
    let violations = packet.validate();
    assert!(
        violations.contains(&ApproverReviewPackControlsViolation::WaivedOrExpiredClaimsSatisfied)
    );
}

#[test]
fn waived_note_required() {
    let mut packet = packet();
    let row = packet
        .approver_matrix_rows
        .iter_mut()
        .find(|row| row.approver_state_source == ApproverStateSource::ApprovalWaivedByPolicy)
        .expect("waived row present");
    row.waived_note = String::new();
    assert!(packet
        .validate()
        .contains(&ApproverReviewPackControlsViolation::WaivedNoteMissing));
}

#[test]
fn expiry_label_required_for_expired() {
    let mut packet = packet();
    let row = packet
        .approver_matrix_rows
        .iter_mut()
        .find(|row| row.approver_state_source == ApproverStateSource::ApprovalExpired)
        .expect("expired row present");
    row.expiry_label = String::new();
    assert!(packet
        .validate()
        .contains(&ApproverReviewPackControlsViolation::ExpiryLabelMissing));
}

#[test]
fn stale_note_required() {
    let mut packet = packet();
    let summary = packet
        .review_pack_summaries
        .iter_mut()
        .find(|summary| {
            summary.evaluation_locus_source == EvaluationLocusSource::StaleAgainstBaseHead
        })
        .expect("stale summary present");
    summary.stale_note = String::new();
    assert!(packet
        .validate()
        .contains(&ApproverReviewPackControlsViolation::StaleNoteMissing));
}

#[test]
fn governance_vocab_missing_approver_token_fails() {
    let mut packet = packet();
    let row = packet
        .approver_matrix_rows
        .iter_mut()
        .find(|row| row.approver_state_source == ApproverStateSource::ApprovalExpired)
        .expect("expired row present");
    row.governance_state_vocab
        .retain(|token| *token != M5GovernanceStateVocab::Expired);
    assert!(packet
        .validate()
        .contains(&ApproverReviewPackControlsViolation::GovernanceVocabMissingApproverToken));
}

#[test]
fn governance_vocab_missing_locus_token_fails() {
    let mut packet = packet();
    let summary = packet
        .review_pack_summaries
        .iter_mut()
        .find(|summary| {
            summary.evaluation_locus_source == EvaluationLocusSource::LocalEvaluationOnly
        })
        .expect("local-only summary present");
    summary.governance_state_vocab.clear();
    assert!(packet
        .validate()
        .contains(&ApproverReviewPackControlsViolation::GovernanceVocabMissingLocusToken));
}

#[test]
fn suppressed_check_incomplete_fails() {
    let mut packet = packet();
    packet.review_pack_summaries[0].suppressed_checks[0].reason_label = String::new();
    assert!(packet
        .validate()
        .contains(&ApproverReviewPackControlsViolation::SuppressedCheckIncomplete));
}

#[test]
fn person_contact_detail_in_alias_fails() {
    let mut packet = packet();
    packet.approver_matrix_rows[0].approver_role_label = "person@example.com".to_owned();
    assert!(packet
        .validate()
        .contains(&ApproverReviewPackControlsViolation::PersonContactDetailInAlias));
}

#[test]
fn open_evidence_link_action_required() {
    let mut packet = packet();
    packet.approver_matrix_rows[0]
        .row_actions
        .retain(|action| *action != ApproverMatrixAction::OpenEvidenceLink);
    let violations = packet.validate();
    assert!(
        violations.contains(&ApproverReviewPackControlsViolation::OpenEvidenceLinkActionMissing)
    );
    assert!(violations.contains(&ApproverReviewPackControlsViolation::ComponentActionsIncomplete));
}

#[test]
fn resolvable_evidence_requires_ref() {
    let mut packet = packet();
    let row = packet
        .approver_matrix_rows
        .iter_mut()
        .find(|row| row.evidence_link_kind.is_resolvable())
        .expect("resolvable evidence row present");
    row.evidence_link_ref = String::new();
    assert!(packet
        .validate()
        .contains(&ApproverReviewPackControlsViolation::EvidenceLinkRefMissing));
}

#[test]
fn wrong_component_class_fails() {
    let mut packet = packet();
    packet.approver_matrix_rows[0].component = M5GovernanceComponent::ReviewPackSummary;
    assert!(packet
        .validate()
        .contains(&ApproverReviewPackControlsViolation::ApproverMatrixRowWrongComponentClass));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&ApproverReviewPackControlsViolation::MissingSourceContracts));
}

#[test]
fn review_incomplete_fails() {
    let mut packet = packet();
    packet.review.waived_or_expired_never_shown_as_satisfied = false;
    assert!(packet
        .validate()
        .contains(&ApproverReviewPackControlsViolation::ReviewIncomplete));
}

#[test]
fn summaries_alone_cover_every_locus_posture() {
    let packet = packet();
    assert!(packet.validate().is_empty());
    let mut postures: BTreeSet<EvaluationLocusPosture> = BTreeSet::new();
    for summary in &packet.review_pack_summaries {
        postures.insert(summary.locus_disclosure().posture);
    }
    // AC-1: local-only, provider authoritative, CI-only, not evaluated here, and stale are all
    // distinguishable from the review-pack summaries alone.
    assert_eq!(postures.len(), EvaluationLocusPosture::ALL.len());
}

#[test]
fn approver_rows_cover_every_state_source_and_posture() {
    let packet = packet();
    let mut sources: BTreeSet<ApproverStateSource> = BTreeSet::new();
    let mut postures: BTreeSet<ApproverStatePosture> = BTreeSet::new();
    for row in &packet.approver_matrix_rows {
        sources.insert(row.approver_state_source);
        postures.insert(row.approver_disclosure().posture);
    }
    assert_eq!(sources.len(), ApproverStateSource::ALL.len());
    assert_eq!(postures.len(), ApproverStatePosture::ALL.len());
}

#[test]
fn markdown_summary_lists_every_component() {
    let summary = packet().render_markdown_summary();
    for row in packet().approver_matrix_rows {
        assert!(
            summary.contains(&row.approver_role_label),
            "summary missing row {}",
            row.row_id
        );
    }
    for pack in packet().review_pack_summaries {
        assert!(
            summary.contains(&pack.pack_digest_label),
            "summary missing pack {}",
            pack.summary_id
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_approver_review_pack_controls_export()
        .expect("checked approver review-pack controls export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed_packet() {
    let seed = packet();
    let checked = current_approver_review_pack_controls_export()
        .expect("checked approver review-pack controls export validates");
    assert_eq!(checked, seed);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-approver-review-pack-controls/approver_matrix_waived_expired.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-approver-review-pack-controls/review_pack_summary_ci_only.json"
        )),
    ] {
        let packet: ApproverReviewPackControlsPacket =
            serde_json::from_str(raw).expect("fixture parses as approver review-pack packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

/// Regenerates the checked release artifacts and narrowed fixtures.
///
/// Guarded behind `GEN_APPROVER_REVIEW_PACK_CONTROLS_ARTIFACTS` so ordinary test runs never touch the
/// working tree. Run in isolation with the env gate set, then run the full suite:
/// `GEN_APPROVER_REVIEW_PACK_CONTROLS_ARTIFACTS=1 cargo test -p aureline-review
/// implement_approver_matrices_and_review_pack_summaries_with_requirement_source_approval_state_local_versus_provider_evaluation_parity_suppressed_check_visibility_and_freshness_truth::tests::generate_artifacts
/// -- --exact --ignored`
#[test]
#[ignore = "artifact generator; run explicitly with the env gate set"]
fn generate_artifacts() {
    if std::env::var("GEN_APPROVER_REVIEW_PACK_CONTROLS_ARTIFACTS").is_err() {
        return;
    }

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let repo_root = std::path::Path::new(manifest_dir).join("..").join("..");

    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    let proof_dir = repo_root
        .join("artifacts")
        .join("release")
        .join("m5-approver-review-pack-controls-proof");
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
        .join("m5-approver-review-pack-controls");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");

    // Fixture 1: a waived / expired approver matrix that must never read as satisfied.
    let mut waived_expired = packet.clone();
    waived_expired.packet_id = "m5-approver-review-pack-controls:fixture:waived-expired".to_owned();
    waived_expired.surface_label =
        "M5 approver matrices: a waived or expired approval never reads as satisfied".to_owned();
    assert!(
        waived_expired.validate().is_empty(),
        "{:?}",
        waived_expired.validate()
    );
    std::fs::write(
        fixture_dir.join("approver_matrix_waived_expired.json"),
        format!("{}\n", waived_expired.export_safe_json()),
    )
    .expect("write waived-expired fixture");

    // Fixture 2: a CI-only review pack that must never read as provider-authoritative.
    let mut ci_only = packet;
    ci_only.packet_id = "m5-approver-review-pack-controls:fixture:ci-only".to_owned();
    ci_only.surface_label =
        "M5 review-pack summaries: a CI-only pack never reads as provider-authoritative".to_owned();
    assert!(ci_only.validate().is_empty(), "{:?}", ci_only.validate());
    std::fs::write(
        fixture_dir.join("review_pack_summary_ci_only.json"),
        format!("{}\n", ci_only.export_safe_json()),
    )
    .expect("write ci-only fixture");
}
