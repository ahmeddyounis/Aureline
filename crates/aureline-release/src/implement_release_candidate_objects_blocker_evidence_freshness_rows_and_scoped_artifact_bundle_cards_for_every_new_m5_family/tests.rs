use super::*;

fn graph() -> M5FamilyReleaseGraph {
    current_m5_family_release_graph().expect("graph parses")
}

#[test]
fn embedded_graph_parses_and_validates() {
    let g = graph();
    assert_eq!(
        g.schema_version,
        IMPLEMENT_M5_FAMILY_RELEASE_GRAPH_SCHEMA_VERSION
    );
    assert_eq!(g.record_kind, IMPLEMENT_M5_FAMILY_RELEASE_GRAPH_RECORD_KIND);
    let violations = g.validate();
    assert!(
        violations.is_empty(),
        "graph must validate cleanly: {violations:#?}"
    );
    assert!(!g.candidates.is_empty());
}

#[test]
fn embedded_json_matches_builder() {
    // The checked-in JSON must be exactly what the in-code builder produces, so
    // the embedded consumer and the artifact never drift.
    assert_eq!(graph(), build_m5_family_release_graph());
}

#[test]
fn builder_validates_cleanly() {
    assert_eq!(build_m5_family_release_graph().validate(), Vec::new());
}

#[test]
fn covers_every_family_kind() {
    let g = graph();
    for kind in M5ArtifactFamilyKind::ALL {
        assert!(
            !g.candidates_for_kind(kind).is_empty(),
            "family kind {} must have at least one candidate",
            kind.as_str()
        );
    }
}

#[test]
fn every_candidate_lists_every_bundle_member_kind() {
    let g = graph();
    for c in &g.candidates {
        for kind in BundleMemberKind::ALL {
            assert!(
                c.bundle.member(kind).is_some(),
                "candidate {} must list bundle member {} (never omitted)",
                c.entry_id,
                kind.as_str()
            );
        }
    }
}

#[test]
fn covers_every_declared_release_blocking_artifact() {
    let g = graph();
    assert!(!g.release_blocking_artifact_refs.is_empty());
    let covered: Vec<&str> = g
        .release_blocking_candidates()
        .iter()
        .map(|c| c.artifact_ref.as_str())
        .collect();
    for declared in &g.release_blocking_artifact_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking candidate"
        );
    }
}

#[test]
fn per_family_scope_is_not_flattened() {
    // Each family carries its own release-candidate scope; they are not collapsed
    // into one monolithic release blob.
    let g = graph();
    assert_eq!(g.release_candidates().len(), g.candidates.len());
}

#[test]
fn graph_narrows_at_least_one_family() {
    let g = graph();
    assert!(
        !g.candidates_narrowed().is_empty(),
        "the graph must narrow at least one family below the cutline"
    );
}

#[test]
fn every_gap_reason_has_a_stop_rule() {
    let g = graph();
    let covered: std::collections::BTreeSet<FamilyGapReason> = g
        .stop_rules
        .iter()
        .map(|rule| rule.trigger_reason)
        .collect();
    for reason in FamilyGapReason::ALL {
        assert!(covered.contains(&reason), "{}", reason.as_str());
    }
}

#[test]
fn summary_counts_match_candidates() {
    let g = graph();
    assert_eq!(g.summary, g.computed_summary());
    assert_eq!(
        g.summary.candidates_backed + g.summary.candidates_narrowed,
        g.candidates.len()
    );
}

#[test]
fn publication_decision_matches_computed() {
    let g = graph();
    assert_eq!(g.publication.decision, g.computed_publication_decision());
    assert_eq!(
        g.publication.blocking_rule_ids,
        g.computed_blocking_rule_ids()
    );
    assert_eq!(
        g.publication.blocking_claim_ids,
        g.computed_blocking_candidate_ids()
    );
}

#[test]
fn missing_and_stale_evidence_surface_as_blockers() {
    // The narrowed family must expose its missing bundle member and its stale
    // evidence rather than dropping them from the view.
    let g = graph();
    let projection = g.support_export_projection();
    let narrowed = projection
        .rows
        .iter()
        .find(|row| !row.publishes_stable)
        .expect("a narrowed family exists");
    assert!(
        !narrowed.missing_member_kinds.is_empty() || !narrowed.partial_member_kinds.is_empty(),
        "a narrowed family must show its missing or partial bundle members"
    );
    assert!(
        narrowed.blocking_evidence_count > 0,
        "a narrowed family must surface its stale required evidence as blocking"
    );
}

#[test]
fn export_projection_mirrors_candidates() {
    let g = graph();
    let projection = g.support_export_projection();
    assert_eq!(projection.rows.len(), g.candidates.len());
    for (c, proj) in g.candidates.iter().zip(&projection.rows) {
        assert_eq!(c.entry_id, proj.entry_id);
        assert_eq!(c.publishes_stable(), proj.publishes_stable);
        // Every member kind is present in the projection's presence list.
        assert_eq!(proj.member_presence.len(), BundleMemberKind::ALL.len());
    }
}

#[test]
fn validate_flags_a_backed_candidate_with_active_gap() {
    let mut g = graph();
    let c = g
        .candidates
        .iter_mut()
        .find(|c| c.publishes_stable())
        .expect("a backed candidate exists");
    c.active_gap_reasons
        .push(FamilyGapReason::ProofPacketMissing);
    g.summary = g.computed_summary();
    assert!(g
        .validate()
        .iter()
        .any(|v| matches!(v, M5FamilyReleaseGraphViolation::HeldWithActiveGap { .. })));
}

#[test]
fn validate_flags_an_omitted_bundle_member() {
    let mut g = graph();
    let c = g
        .candidates
        .iter_mut()
        .find(|c| c.publishes_stable())
        .expect("a backed candidate exists");
    c.bundle
        .members
        .retain(|m| m.member_kind != BundleMemberKind::Schema);
    g.summary = g.computed_summary();
    assert!(g
        .validate()
        .iter()
        .any(|v| matches!(v, M5FamilyReleaseGraphViolation::BundleMemberOmitted { .. })));
}

#[test]
fn validate_flags_a_narrowing_candidate_that_does_not_narrow() {
    let mut g = graph();
    let c = g
        .candidates
        .iter_mut()
        .find(|c| c.publishes_stable())
        .expect("a backed candidate exists");
    // Drop a member to not_provided (breaks linkage) but keep the label stable.
    if let Some(member) = c
        .bundle
        .members
        .iter_mut()
        .find(|m| m.member_kind == BundleMemberKind::DocsPack)
    {
        member.presence = MemberPresence::NotProvided;
        member.artifact_ref = String::new();
        member.digest_algorithm = String::new();
        member.digest_ref = String::new();
    }
    c.active_gap_reasons
        .push(FamilyGapReason::BundleMemberMissing);
    g.summary = g.computed_summary();
    g.publication.decision = g.computed_publication_decision();
    g.publication.blocking_rule_ids = g.computed_blocking_rule_ids();
    g.publication.blocking_claim_ids = g.computed_blocking_candidate_ids();
    assert!(g.validate().iter().any(|v| matches!(
        v,
        M5FamilyReleaseGraphViolation::PublishedLabelNotNarrowed { .. }
    )));
}

#[test]
fn validate_flags_a_backed_candidate_without_signoff() {
    let mut g = graph();
    let c = g
        .candidates
        .iter_mut()
        .find(|c| c.publishes_stable())
        .expect("a backed candidate exists");
    c.owner_signoff.signed_off = false;
    c.owner_signoff.signed_at = None;
    g.summary = g.computed_summary();
    assert!(g
        .validate()
        .iter()
        .any(|v| matches!(v, M5FamilyReleaseGraphViolation::HeldWithoutSignoff { .. })));
}

#[test]
fn validate_flags_a_provided_member_without_digest() {
    let mut g = graph();
    let c = g
        .candidates
        .iter_mut()
        .find(|c| c.publishes_stable())
        .expect("a backed candidate exists");
    if let Some(member) = c
        .bundle
        .members
        .iter_mut()
        .find(|m| m.presence == MemberPresence::Provided)
    {
        member.digest_ref = String::new();
        member.digest_algorithm = String::new();
    }
    g.summary = g.computed_summary();
    assert!(g.validate().iter().any(|v| matches!(
        v,
        M5FamilyReleaseGraphViolation::ProvidedMemberWithoutDigest { .. }
    )));
}
