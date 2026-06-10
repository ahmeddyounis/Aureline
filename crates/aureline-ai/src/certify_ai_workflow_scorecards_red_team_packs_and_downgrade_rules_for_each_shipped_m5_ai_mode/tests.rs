use super::*;

const PACKET_ID: &str = "m5-ai-mode-certification:stable:0001";

/// A fully passing scorecard covering every required dimension.
fn passing_scorecard() -> Vec<AiModeScorecardRow> {
    AiScorecardDimension::ALL
        .iter()
        .map(|&dimension| AiModeScorecardRow {
            dimension,
            score: 96,
            threshold: 90,
            status: AiScorecardStatus::Pass,
        })
        .collect()
}

/// A scorecard with one borderline (warn) dimension, valid for a Beta claim.
fn beta_scorecard() -> Vec<AiModeScorecardRow> {
    AiScorecardDimension::ALL
        .iter()
        .map(|&dimension| {
            if dimension == AiScorecardDimension::RollbackSafety {
                AiModeScorecardRow {
                    dimension,
                    score: 90,
                    threshold: 90,
                    status: AiScorecardStatus::Warn,
                }
            } else {
                AiModeScorecardRow {
                    dimension,
                    score: 93,
                    threshold: 88,
                    status: AiScorecardStatus::Pass,
                }
            }
        })
        .collect()
}

/// A red-team pack with the given disposition per vector and a shared guard ref.
fn red_team_pack(dispositions: [AiRedTeamHandling; 8]) -> Vec<AiModeRedTeamScenario> {
    AiRedTeamVector::ALL
        .iter()
        .zip(dispositions)
        .map(|(&vector, handling)| AiModeRedTeamScenario {
            vector,
            handling,
            guard_ref: "docs/ai/m5/certify_ai_workflow_scorecards_red_team_packs_and_downgrade_rules_for_each_shipped_m5_ai_mode.md".to_owned(),
        })
        .collect()
}

fn all_blocked() -> Vec<AiModeRedTeamScenario> {
    red_team_pack([AiRedTeamHandling::Blocked; 8])
}

/// Read-only modes mark the four apply-specific vectors not applicable.
fn read_only_pack() -> Vec<AiModeRedTeamScenario> {
    red_team_pack([
        AiRedTeamHandling::Blocked,       // prompt_injection
        AiRedTeamHandling::Blocked,       // tainted_context_exfiltration
        AiRedTeamHandling::NotApplicable, // scope_escape
        AiRedTeamHandling::NotApplicable, // self_approved_mutation
        AiRedTeamHandling::NotApplicable, // worktree_isolation_bypass
        AiRedTeamHandling::NotApplicable, // unreviewed_apply
        AiRedTeamHandling::Blocked,       // credential_leak_in_export
        AiRedTeamHandling::Blocked,       // stale_evidence_promotion
    ])
}

fn stable_downgrade_rules() -> Vec<AiModeDowngradeRule> {
    vec![
        AiModeDowngradeRule {
            trigger: M5AiWorkflowDowngradeTrigger::ProofStale,
            narrowed_to: M5AiWorkflowQualificationClass::Beta,
            auto_enforced: true,
            rationale: "Stale proof narrows the public Stable claim to Beta".to_owned(),
        },
        AiModeDowngradeRule {
            trigger: M5AiWorkflowDowngradeTrigger::PolicyBlocked,
            narrowed_to: M5AiWorkflowQualificationClass::Unavailable,
            auto_enforced: true,
            rationale: "A policy block makes the mode unavailable".to_owned(),
        },
    ]
}

fn beta_downgrade_rules() -> Vec<AiModeDowngradeRule> {
    vec![
        AiModeDowngradeRule {
            trigger: M5AiWorkflowDowngradeTrigger::ProofStale,
            narrowed_to: M5AiWorkflowQualificationClass::Preview,
            auto_enforced: true,
            rationale: "Stale proof narrows the Beta claim to Preview".to_owned(),
        },
        AiModeDowngradeRule {
            trigger: M5AiWorkflowDowngradeTrigger::UpstreamDependencyNarrowed,
            narrowed_to: M5AiWorkflowQualificationClass::Experimental,
            auto_enforced: true,
            rationale: "An upstream narrowing drops the mode to Experimental".to_owned(),
        },
    ]
}

fn evidence(refs: &[&str]) -> Vec<String> {
    refs.iter().map(|r| (*r).to_owned()).collect()
}

fn mode_certifications() -> Vec<AiModeCertification> {
    vec![
        AiModeCertification {
            mode: M5AiMode::InlineEdit,
            claimed_qualification: M5AiWorkflowQualificationClass::Stable,
            scope_summary: "Composer inline edit with bounded scoped-apply, preview/approval, and revert".to_owned(),
            scorecard: passing_scorecard(),
            red_team_pack: red_team_pack([
                AiRedTeamHandling::Blocked,
                AiRedTeamHandling::Blocked,
                AiRedTeamHandling::Blocked,
                AiRedTeamHandling::Blocked,
                AiRedTeamHandling::NotApplicable,
                AiRedTeamHandling::Blocked,
                AiRedTeamHandling::Blocked,
                AiRedTeamHandling::Blocked,
            ]),
            downgrade_rules: stable_downgrade_rules(),
            evidence_packet_refs: evidence(&[
                "evidence:prompt-composer-conformance:m5",
                "evidence:ai-scoped-apply-hardening:m5",
            ]),
        },
        AiModeCertification {
            mode: M5AiMode::PatchReview,
            claimed_qualification: M5AiWorkflowQualificationClass::Stable,
            scope_summary: "AI review-assist findings, publish-to-review sheets, and resolution memory".to_owned(),
            scorecard: passing_scorecard(),
            red_team_pack: red_team_pack([
                AiRedTeamHandling::Blocked,
                AiRedTeamHandling::Blocked,
                AiRedTeamHandling::Blocked,
                AiRedTeamHandling::NotApplicable,
                AiRedTeamHandling::NotApplicable,
                AiRedTeamHandling::NotApplicable,
                AiRedTeamHandling::Blocked,
                AiRedTeamHandling::Blocked,
            ]),
            downgrade_rules: stable_downgrade_rules(),
            evidence_packet_refs: evidence(&[
                "evidence:ai-review-assist-truth:m5",
                "evidence:patch-review-summary:m5",
            ]),
        },
        AiModeCertification {
            mode: M5AiMode::Explain,
            claimed_qualification: M5AiWorkflowQualificationClass::Stable,
            scope_summary: "Explain flow with evidence links to logs, traces, runbooks, and profiles".to_owned(),
            scorecard: passing_scorecard(),
            red_team_pack: read_only_pack(),
            downgrade_rules: stable_downgrade_rules(),
            evidence_packet_refs: evidence(&["evidence:ai-explain-debug-test-flows:m5"]),
        },
        AiModeCertification {
            mode: M5AiMode::Debug,
            claimed_qualification: M5AiWorkflowQualificationClass::Stable,
            scope_summary: "Debug flow with evidence links to logs, traces, and runbooks".to_owned(),
            scorecard: passing_scorecard(),
            red_team_pack: read_only_pack(),
            downgrade_rules: stable_downgrade_rules(),
            evidence_packet_refs: evidence(&["evidence:ai-explain-debug-test-flows:m5"]),
        },
        AiModeCertification {
            mode: M5AiMode::Test,
            claimed_qualification: M5AiWorkflowQualificationClass::Beta,
            scope_summary: "Test-generation proposals with assumption review and isolated sandbox validation".to_owned(),
            scorecard: beta_scorecard(),
            red_team_pack: all_blocked(),
            downgrade_rules: beta_downgrade_rules(),
            evidence_packet_refs: evidence(&["evidence:ai-test-generation-truth:m5"]),
        },
        AiModeCertification {
            mode: M5AiMode::Refactor,
            claimed_qualification: M5AiWorkflowQualificationClass::Beta,
            scope_summary: "Refactor planner with impact sets, candidate previews, and multi-file safety classes".to_owned(),
            scorecard: beta_scorecard(),
            red_team_pack: all_blocked(),
            downgrade_rules: beta_downgrade_rules(),
            evidence_packet_refs: evidence(&["evidence:ai-refactor-planner-truth:m5"]),
        },
        AiModeCertification {
            mode: M5AiMode::BranchOrWorktreeAgent,
            claimed_qualification: M5AiWorkflowQualificationClass::Beta,
            scope_summary: "Background branch-agent lifecycle with isolated worktrees, checkpoints, and completion review".to_owned(),
            scorecard: beta_scorecard(),
            red_team_pack: all_blocked(),
            downgrade_rules: beta_downgrade_rules(),
            evidence_packet_refs: evidence(&["evidence:background-branch-agent-lifecycle:m4"]),
        },
    ]
}

fn proof_freshness() -> AiModeCertificationProofFreshness {
    AiModeCertificationProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        AI_MODE_CERTIFICATION_SCHEMA_REF.to_owned(),
        AI_MODE_CERTIFICATION_DOC_REF.to_owned(),
        M5_AI_WORKFLOW_MATRIX_SCHEMA_REF.to_owned(),
        M5_AI_WORKFLOW_MATRIX_ARTIFACT_REF.to_owned(),
    ]
}

fn packet() -> AiModeCertificationPacket {
    AiModeCertificationPacket::new(AiModeCertificationPacketInput {
        packet_id: PACKET_ID.to_owned(),
        certification_label: "M5 AI Mode Certification".to_owned(),
        mode_certifications: mode_certifications(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

#[test]
fn ai_mode_certification_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn all_shipped_modes_are_certified() {
    let packet = packet();
    let modes: BTreeSet<M5AiMode> = packet
        .mode_certifications
        .iter()
        .map(|cert| cert.mode)
        .collect();
    for mode in M5AiMode::ALL {
        assert!(modes.contains(&mode), "missing mode: {}", mode.as_str());
    }
}

#[test]
fn missing_mode_fails_validation() {
    let mut packet = packet();
    packet
        .mode_certifications
        .retain(|cert| cert.mode != M5AiMode::Refactor);
    assert!(packet
        .validate()
        .contains(&AiModeCertificationViolation::RequiredModeMissing));
}

#[test]
fn duplicate_mode_fails_validation() {
    let mut packet = packet();
    let first = packet.mode_certifications[0].clone();
    packet.mode_certifications.push(first);
    assert!(packet
        .validate()
        .contains(&AiModeCertificationViolation::DuplicateMode));
}

#[test]
fn scorecard_dimension_missing_fails() {
    let mut packet = packet();
    packet.mode_certifications[0].scorecard.pop();
    assert!(packet
        .validate()
        .contains(&AiModeCertificationViolation::ScorecardDimensionMissing));
}

#[test]
fn scorecard_status_inconsistent_fails() {
    let mut packet = packet();
    // Claim a pass while the score is below threshold.
    packet.mode_certifications[0].scorecard[0].score = 10;
    assert!(packet
        .validate()
        .contains(&AiModeCertificationViolation::ScorecardStatusInconsistent));
}

#[test]
fn stable_mode_with_warn_dimension_fails() {
    let mut packet = packet();
    packet.mode_certifications[0].scorecard[0].status = AiScorecardStatus::Warn;
    assert!(packet
        .validate()
        .contains(&AiModeCertificationViolation::StableModeScorecardNotPassing));
}

#[test]
fn beta_mode_with_failing_dimension_fails() {
    let mut packet = packet();
    // Test mode is index 4 (Beta).
    let row = &mut packet.mode_certifications[4].scorecard[0];
    row.score = 10;
    row.threshold = 90;
    row.status = AiScorecardStatus::Fail;
    assert!(packet
        .validate()
        .contains(&AiModeCertificationViolation::BetaModeScorecardFailing));
}

#[test]
fn red_team_vector_missing_fails() {
    let mut packet = packet();
    packet.mode_certifications[0].red_team_pack.pop();
    assert!(packet
        .validate()
        .contains(&AiModeCertificationViolation::RedTeamVectorMissing));
}

#[test]
fn red_team_critical_vector_uncovered_fails() {
    let mut packet = packet();
    // prompt_injection is an always-applicable vector at index 0.
    packet.mode_certifications[0].red_team_pack[0].handling = AiRedTeamHandling::NotApplicable;
    assert!(packet
        .validate()
        .contains(&AiModeCertificationViolation::RedTeamCriticalVectorUncovered));
}

#[test]
fn red_team_guard_missing_fails() {
    let mut packet = packet();
    packet.mode_certifications[0].red_team_pack[0]
        .guard_ref
        .clear();
    assert!(packet
        .validate()
        .contains(&AiModeCertificationViolation::RedTeamGuardMissing));
}

#[test]
fn claimed_mode_missing_evidence_fails() {
    let mut packet = packet();
    packet.mode_certifications[0].evidence_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&AiModeCertificationViolation::ClaimedModeMissingEvidence));
}

#[test]
fn downgrade_rules_missing_fails() {
    let mut packet = packet();
    packet.mode_certifications[0].downgrade_rules.clear();
    assert!(packet
        .validate()
        .contains(&AiModeCertificationViolation::DowngradeRulesMissing));
}

#[test]
fn downgrade_rule_missing_proof_stale_fails() {
    let mut packet = packet();
    packet.mode_certifications[0]
        .downgrade_rules
        .retain(|rule| rule.trigger != M5AiWorkflowDowngradeTrigger::ProofStale);
    assert!(packet
        .validate()
        .contains(&AiModeCertificationViolation::DowngradeRuleMissingProofStale));
}

#[test]
fn downgrade_rule_not_narrowing_fails() {
    let mut packet = packet();
    // Narrowing "to" Stable from a Stable claim does not narrow.
    packet.mode_certifications[0].downgrade_rules[0].narrowed_to =
        M5AiWorkflowQualificationClass::Stable;
    assert!(packet
        .validate()
        .contains(&AiModeCertificationViolation::DowngradeRuleNotNarrowing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&AiModeCertificationViolation::MissingSourceContracts));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&AiModeCertificationViolation::ProofFreshnessIncomplete));
}

#[test]
fn narrowed_qualification_applies_matching_rule() {
    let cert = &packet().mode_certifications[0];
    assert_eq!(
        cert.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProofStale),
        M5AiWorkflowQualificationClass::Beta
    );
    // A trigger with no rule leaves the claim unchanged.
    assert_eq!(
        cert.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProviderUnavailable),
        M5AiWorkflowQualificationClass::Stable
    );
}

#[test]
fn stable_mode_count_matches_fixture() {
    assert_eq!(packet().stable_mode_count(), 4);
}

#[test]
fn markdown_summary_lists_every_mode() {
    let summary = packet().render_markdown_summary();
    for mode in M5AiMode::ALL {
        assert!(summary.contains(mode.as_str()), "missing {}", mode.as_str());
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_ai_mode_certification_export()
        .expect("checked AI mode certification export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
    assert_eq!(packet.mode_certifications.len(), M5AiMode::ALL.len());
}
