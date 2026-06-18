use super::*;

fn packet() -> AiPackageMutationReviewPacket {
    current_ai_package_mutation_review_packet().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        AI_PACKAGE_MUTATION_REVIEW_SCHEMA_VERSION
    );
    assert_eq!(packet.record_kind, AI_PACKAGE_MUTATION_REVIEW_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_proposals() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn corpus_covers_every_required_state() {
    let packet = packet();
    assert_eq!(packet.corpus_coverage_gaps(), Vec::new());
}

#[test]
fn ai_surface_is_always_propose_only_and_never_executes() {
    let packet = packet();
    assert!(packet.all_propose_only());
    for proposal in &packet.proposals {
        assert!(!proposal.write_authority.can_execute());
        assert!(proposal.is_propose_only());
    }
}

#[test]
fn every_proposal_is_governed_and_preview_first() {
    let packet = packet();
    assert!(packet.all_governed());
    for proposal in &packet.proposals {
        assert!(proposal.preview_first);
        assert!(proposal.routes_through_governed_review);
        assert!(proposal.no_hidden_scripting);
        assert!(proposal.asserts_governed());
    }
}

#[test]
fn result_classes_are_coherent_with_fallback() {
    let packet = packet();
    for proposal in &packet.proposals {
        assert!(
            proposal.result_is_coherent(),
            "proposal {} has an incoherent result/fallback pair",
            proposal.proposal_id
        );
    }
}

#[test]
fn no_field_leaks_a_raw_url() {
    let packet = packet();
    for proposal in &packet.proposals {
        assert!(!proposal.rollback_handle_ref.contains("://"));
        assert!(!proposal.redacted_manifest_path.contains("://"));
    }
}

#[test]
fn export_projection_is_redaction_safe() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.rows.len(), packet.proposals.len());
    assert!(projection.all_propose_only);
    assert!(projection.all_governed);
    for row in &projection.rows {
        assert!(!row.summary.contains("://"));
    }
}

// The cross-surface binding: the AI surface reuses the governed `aureline-deps`
// `automation_governance` contract, the frozen matrix, and the reviewed-mutation
// contract. This is the parity proof that the AI lane is not a bypass lane.
#[test]
fn binds_to_the_governed_cross_surface_contract() {
    let packet = packet();
    let governance = aureline_deps::automation_governance::current_automation_governance()
        .expect("governance packet loads");
    let matrix = aureline_deps::current_m5_package_state_matrix().expect("matrix loads");
    let flows = aureline_deps::reviewed_mutation_flows::current_reviewed_mutation_flows()
        .expect("reviewed flows load");

    // Packet-level binding: the AI surface references the exact governed packet,
    // frozen matrix, and reviewed-mutation contract.
    assert_eq!(packet.references_governance_packet_id, governance.packet_id);
    assert_eq!(packet.references_matrix_id, matrix.packet_id);
    assert_eq!(packet.references_reviewed_flows_id, flows.packet_id);

    // Row-level binding: every AI proposal whose governed ref resolves in the
    // governed packet agrees with it on ecosystem, fallback decision, result
    // class, and rollback handle — the AI surface mirrors the governed truth and
    // does not invent a weaker one.
    let mut matched = 0;
    for proposal in &packet.proposals {
        let Some(governed) = governance.proposal(&proposal.governed_proposal_ref) else {
            continue;
        };
        matched += 1;
        assert_eq!(
            proposal.ecosystem.as_str(),
            governed.reviewed_sheet.ecosystem.as_str(),
            "proposal {} disagrees with governed ecosystem",
            proposal.proposal_id
        );
        assert_eq!(
            proposal.safe_fallback.as_str(),
            governed.execution_decision.as_str(),
            "proposal {} disagrees with governed execution decision",
            proposal.proposal_id
        );
        assert_eq!(
            proposal.result_class.as_str(),
            governed.result_class.as_str(),
            "proposal {} disagrees with governed result class",
            proposal.proposal_id
        );
        assert_eq!(
            proposal.rollback_handle_ref, governed.rollback_handle.checkpoint_ref,
            "proposal {} disagrees with governed rollback handle",
            proposal.proposal_id
        );
        // The AI surface can never execute what the governed contract may.
        assert!(!proposal.write_authority.can_execute());
    }
    assert!(
        matched >= 4,
        "expected the AI corpus to bind to several governed proposals, matched {matched}"
    );
}

#[test]
fn validate_flags_ungoverned_proposal() {
    let mut packet = packet();
    packet.proposals[0].routes_through_governed_review = false;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        AiPackageMutationReviewViolation::UngovernedProposal { .. }
    )));
}

#[test]
fn validate_flags_hidden_scripting_allowed() {
    let mut packet = packet();
    packet.proposals[0].no_hidden_scripting = false;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        AiPackageMutationReviewViolation::HiddenScriptingAllowed { .. }
    )));
}

#[test]
fn validate_flags_result_fallback_mismatch() {
    let mut packet = packet();
    let proposal = packet
        .proposals
        .iter_mut()
        .find(|p| p.proposal_id == "ai:pmr:auth-blocked")
        .expect("auth blocked");
    // A blocked fallback can never produce a committed result.
    proposal.result_class = AiResultClass::CommittedReviewed;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        AiPackageMutationReviewViolation::ResultFallbackMismatch { .. }
    )));
}

#[test]
fn validate_flags_raw_url_leak() {
    let mut packet = packet();
    packet.proposals[0].redacted_manifest_path = "https://secret.example.com/Cargo.toml".to_owned();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, AiPackageMutationReviewViolation::RawUrlLeak { .. })));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_proposals = packet.summary.total_proposals.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&AiPackageMutationReviewViolation::SummaryMismatch));
}

#[test]
fn validate_flags_duplicate_proposal_id() {
    let mut packet = packet();
    let clone = packet.proposals[0].clone();
    packet.proposals.push(clone);
    packet.summary = packet.computed_summary();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, AiPackageMutationReviewViolation::DuplicateRowId { .. })));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(
        AiMutationIntent::RelockDependencies.as_str(),
        "relock_dependencies"
    );
    assert_eq!(AiWriteAuthority::ProposeOnly.as_str(), "propose_only");
    assert_eq!(
        SafeFallbackClass::BlockedNoSafePath.as_str(),
        "blocked_no_safe_path"
    );
    assert_eq!(
        AiResultClass::NarrowedInspectOnly.as_str(),
        "narrowed_inspect_only"
    );
    assert_eq!(AiValidationKind::LockfileVerify.as_str(), "lockfile_verify");
    assert_eq!(AiEcosystem::NodePnpm.as_str(), "node_pnpm");
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
    round_trip(&AiMutationIntent::ALL);
    round_trip(&AiWriteAuthority::ALL);
    round_trip(&SafeFallbackClass::ALL);
    round_trip(&AiResultClass::ALL);
    round_trip(&AiValidationKind::ALL);
    round_trip(&AiEcosystem::ALL);
}

/// Scenario fixtures, embedded so they validate without a runtime walk.
const FIXTURE_ADD_PROCEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ai/m5/package-mutation-review/ai_add_proceed.json"
));
const FIXTURE_BLOCKED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ai/m5/package-mutation-review/ai_capability_gap_blocked.json"
));

#[test]
fn fixtures_parse_and_validate() {
    for (name, json) in [
        ("ai_add_proceed", FIXTURE_ADD_PROCEED),
        ("ai_capability_gap_blocked", FIXTURE_BLOCKED),
    ] {
        let packet: AiPackageMutationReviewPacket =
            serde_json::from_str(json).unwrap_or_else(|e| panic!("{name} parses: {e}"));
        assert_eq!(packet.validate(), Vec::new(), "{name} validates");
        assert!(packet.all_propose_only(), "{name} is propose-only");
        assert!(packet.all_governed(), "{name} is governed");
    }
}

#[test]
fn fixtures_cover_the_guards() {
    let proceed: AiPackageMutationReviewPacket =
        serde_json::from_str(FIXTURE_ADD_PROCEED).expect("add proceed fixture");
    assert_eq!(
        proceed.proposals[0].safe_fallback,
        SafeFallbackClass::ProceedAfterReview
    );
    assert_eq!(
        proceed.proposals[0].result_class,
        AiResultClass::CommittedReviewed
    );

    let blocked: AiPackageMutationReviewPacket =
        serde_json::from_str(FIXTURE_BLOCKED).expect("blocked fixture");
    assert_eq!(
        blocked.proposals[0].write_authority,
        AiWriteAuthority::InspectOnly
    );
    assert_eq!(
        blocked.proposals[0].safe_fallback,
        SafeFallbackClass::BlockedNoSafePath
    );
}
