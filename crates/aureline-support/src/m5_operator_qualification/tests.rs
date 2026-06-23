//! Unit tests for the operator-surface qualification packet.

use super::*;

const EVALUATED: &str = "2026-06-22T00:00:00Z";

/// All-fresh, all-passing proof inputs captured one day before evaluation.
fn fresh_inputs() -> Vec<ProofInput> {
    ProofDimension::ALL
        .iter()
        .map(|dimension| ProofInput {
            dimension: *dimension,
            proof_source_ref: "schemas/ops/test.schema.json".to_owned(),
            contributing_proof_refs: vec!["schemas/ops/test.schema.json".to_owned()],
            captured_as_of: Some("2026-06-21T00:00:00Z".to_owned()),
            passing: true,
            freshness_budget_days: DEFAULT_PROOF_FRESHNESS_BUDGET_DAYS,
            detail: "test".to_owned(),
        })
        .collect()
}

fn set_dimension(
    inputs: &mut [ProofInput],
    dimension: ProofDimension,
    mutate: impl Fn(&mut ProofInput),
) {
    let input = inputs
        .iter_mut()
        .find(|input| input.dimension == dimension)
        .expect("dimension present");
    mutate(input);
}

#[test]
fn canonical_packet_builds_and_every_invariant_holds() {
    let packet = operator_qualification_packet();
    assert_eq!(packet.record_kind, M5_OPERATOR_QUALIFICATION_RECORD_KIND);
    assert_eq!(packet.schema_ref, M5_OPERATOR_QUALIFICATION_SCHEMA_REF);
    assert_eq!(packet.packet_id, M5_OPERATOR_QUALIFICATION_PACKET_ID);
    assert_eq!(packet.as_of, M5_OPERATOR_QUALIFICATION_AS_OF);
    assert!(
        packet.all_invariants_hold(),
        "every frozen invariant must hold: {:?}",
        packet
            .invariants
            .iter()
            .filter(|invariant| !invariant.holds)
            .map(|invariant| &invariant.invariant_id)
            .collect::<Vec<_>>()
    );
    assert!(packet.is_support_export_safe());
    assert!(packet.raw_payload_excluded);
}

#[test]
fn canonical_packet_certifies_every_family_fully_supported() {
    let packet = operator_qualification_packet();
    assert_eq!(packet.families.len(), OperatorSurfaceClass::ALL.len());
    for surface in OperatorSurfaceClass::ALL {
        assert!(
            packet.is_family_fully_supported(surface),
            "family {} must be fully supported in the canonical packet",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.rollup.fully_supported,
        OperatorSurfaceClass::ALL.len()
    );
    assert_eq!(packet.rollup.narrowed, 0);
    assert_eq!(packet.rollup.blocked, 0);
    assert_eq!(packet.rollup.stale_dimensions, 0);
    assert_eq!(packet.rollup.failing_dimensions, 0);
    assert_eq!(packet.rollup.missing_dimensions, 0);
}

#[test]
fn canonical_packet_carries_every_dimension() {
    let packet = operator_qualification_packet();
    assert_eq!(packet.dimensions.len(), ProofDimension::ALL.len());
    for dimension in ProofDimension::ALL {
        let proof = packet
            .dimension(dimension)
            .unwrap_or_else(|| panic!("missing dimension {}", dimension.as_str()));
        assert_eq!(proof.state, ProofState::Fresh);
        assert!(!proof.contributing_proof_refs.is_empty());
    }
}

#[test]
fn packet_serialization_round_trips() {
    let packet = operator_qualification_packet();
    let json = serde_json::to_string(&packet).expect("packet serializes");
    let restored: OperatorQualificationPacket =
        serde_json::from_str(&json).expect("packet round-trips");
    assert_eq!(packet, restored);
}

#[test]
fn stale_overview_proof_narrows_only_the_overview_family() {
    let mut inputs = fresh_inputs();
    // Push the overview proof's capture stamp far enough back to age out.
    set_dimension(&mut inputs, ProofDimension::OverviewTruth, |input| {
        input.captured_as_of = Some("2026-01-01T00:00:00Z".to_owned());
    });
    let packet = project_operator_qualification(EVALUATED, &inputs);

    assert_eq!(
        packet
            .dimension(ProofDimension::OverviewTruth)
            .unwrap()
            .state,
        ProofState::Stale
    );

    // The overview board claims overview truth, so it narrows.
    let overview = packet
        .family(OperatorSurfaceClass::OperationalOverviewBoard)
        .unwrap();
    assert_eq!(overview.support, ClaimSupportClass::Narrowed);
    assert!(overview
        .narrowed_by
        .contains(&ProofDimension::OverviewTruth));

    // A triage inbox does not claim overview truth, so stale overview proof does
    // not touch it; every dimension it claims is fresh.
    let triage = packet.family(OperatorSurfaceClass::TriageInbox).unwrap();
    assert!(
        !triage
            .verdict(ProofDimension::OverviewTruth)
            .unwrap()
            .applicable
    );
    assert_eq!(triage.support, ClaimSupportClass::FullySupported);
    assert!(packet.all_invariants_hold());
}

#[test]
fn failing_canonical_matrix_blocks_every_family() {
    let mut inputs = fresh_inputs();
    set_dimension(
        &mut inputs,
        ProofDimension::CanonicalMatrixBinding,
        |input| {
            input.passing = false;
        },
    );
    let packet = project_operator_qualification(EVALUATED, &inputs);

    assert_eq!(
        packet
            .dimension(ProofDimension::CanonicalMatrixBinding)
            .unwrap()
            .state,
        ProofState::Failing
    );
    for family in &packet.families {
        assert_eq!(
            family.support,
            ClaimSupportClass::Blocked,
            "the matrix binding is global + critical, so {} must block",
            family.surface.as_str()
        );
        assert!(family
            .blocked_by
            .contains(&ProofDimension::CanonicalMatrixBinding));
    }
    assert_eq!(packet.rollup.blocked, OperatorSurfaceClass::ALL.len());
    assert!(packet.all_invariants_hold());
}

#[test]
fn missing_runbook_authority_blocks_only_the_runbook_family() {
    let inputs: Vec<ProofInput> = fresh_inputs()
        .into_iter()
        .filter(|input| input.dimension != ProofDimension::RunbookStepAuthority)
        .collect();
    let packet = project_operator_qualification(EVALUATED, &inputs);

    assert_eq!(
        packet
            .dimension(ProofDimension::RunbookStepAuthority)
            .unwrap()
            .state,
        ProofState::Missing
    );
    let runbook = packet
        .family(OperatorSurfaceClass::RunbookStepCard)
        .unwrap();
    assert_eq!(runbook.support, ClaimSupportClass::Blocked);
    assert!(runbook
        .blocked_by
        .contains(&ProofDimension::RunbookStepAuthority));

    // The overview board does not claim runbook authority, so it stays green.
    let overview = packet
        .family(OperatorSurfaceClass::OperationalOverviewBoard)
        .unwrap();
    assert_eq!(overview.support, ClaimSupportClass::FullySupported);
    assert_eq!(packet.rollup.missing_dimensions, 1);
    assert!(packet.all_invariants_hold());
}

#[test]
fn failing_embedded_boundary_blocks_the_embedded_family() {
    let mut inputs = fresh_inputs();
    set_dimension(
        &mut inputs,
        ProofDimension::EmbeddedBoundaryHonesty,
        |input| {
            input.passing = false;
        },
    );
    let packet = project_operator_qualification(EVALUATED, &inputs);

    let embedded = packet
        .family(OperatorSurfaceClass::EmbeddedBoundaryState)
        .unwrap();
    assert_eq!(embedded.support, ClaimSupportClass::Blocked);
    assert!(embedded
        .blocked_by
        .contains(&ProofDimension::EmbeddedBoundaryHonesty));
    assert!(packet.all_invariants_hold());
}

#[test]
fn stale_noncritical_proof_narrows_but_does_not_block() {
    let mut inputs = fresh_inputs();
    set_dimension(
        &mut inputs,
        ProofDimension::MaintenanceFailoverCommunication,
        |input| {
            input.passing = false; // failing, but non-critical
        },
    );
    let packet = project_operator_qualification(EVALUATED, &inputs);

    // Both the maintenance and failover notices claim this dimension, so both
    // narrow (never block).
    for surface in [
        OperatorSurfaceClass::MaintenanceNotice,
        OperatorSurfaceClass::FailoverNotice,
    ] {
        let family = packet.family(surface).unwrap();
        assert_eq!(
            family.support,
            ClaimSupportClass::Narrowed,
            "{} must narrow on failing non-critical proof",
            surface.as_str()
        );
    }
    assert_eq!(packet.rollup.blocked, 0);
    assert_eq!(packet.rollup.narrowed, 2);
}

#[test]
fn handoff_dimension_governs_both_bundle_and_shift_digest() {
    let packet = operator_qualification_packet();
    for surface in [
        OperatorSurfaceClass::HandoffBundle,
        OperatorSurfaceClass::ShiftDigest,
    ] {
        let family = packet.family(surface).unwrap();
        assert!(
            family
                .verdict(ProofDimension::HandoffBundleFidelity)
                .unwrap()
                .applicable,
            "{} must claim handoff-bundle fidelity",
            surface.as_str()
        );
        assert_eq!(
            family.primary_dimension,
            ProofDimension::HandoffBundleFidelity
        );
    }
}

#[test]
fn every_family_claims_the_canonical_matrix_binding() {
    let packet = operator_qualification_packet();
    for family in &packet.families {
        assert!(
            family
                .verdict(ProofDimension::CanonicalMatrixBinding)
                .unwrap()
                .applicable,
            "{} must be anchored to the canonical matrix",
            family.surface.as_str()
        );
    }
}

#[test]
fn every_family_verdict_covers_every_dimension() {
    let packet = operator_qualification_packet();
    for family in &packet.families {
        assert_eq!(family.dimension_verdicts.len(), ProofDimension::ALL.len());
        for dimension in ProofDimension::ALL {
            assert!(
                family.verdict(dimension).is_some(),
                "{} is missing dimension {}",
                family.surface.as_str(),
                dimension.as_str()
            );
        }
    }
}

#[test]
fn freshness_is_derived_from_budget() {
    let input = ProofInput {
        dimension: ProofDimension::OverviewTruth,
        proof_source_ref: "x".to_owned(),
        contributing_proof_refs: vec!["x".to_owned()],
        captured_as_of: Some("2026-05-01T00:00:00Z".to_owned()),
        passing: true,
        freshness_budget_days: 30,
        detail: "x".to_owned(),
    };
    // 52 days elapsed against a 30-day budget → stale.
    assert_eq!(
        input.resolve_state("2026-06-22T00:00:00Z"),
        ProofState::Stale
    );
    // 10 days elapsed → fresh.
    assert_eq!(
        input.resolve_state("2026-05-11T00:00:00Z"),
        ProofState::Fresh
    );
    // Failing overrides freshness.
    let failing = ProofInput {
        passing: false,
        ..input.clone()
    };
    assert_eq!(
        failing.resolve_state("2026-05-02T00:00:00Z"),
        ProofState::Failing
    );
    // No capture stamp → missing.
    let missing = ProofInput {
        captured_as_of: None,
        ..input
    };
    assert_eq!(
        missing.resolve_state("2026-05-02T00:00:00Z"),
        ProofState::Missing
    );
}

#[test]
fn critical_dimensions_are_matrix_runbook_and_embedded() {
    for dimension in ProofDimension::ALL {
        let critical = dimension.is_critical();
        let expected = matches!(
            dimension,
            ProofDimension::CanonicalMatrixBinding
                | ProofDimension::RunbookStepAuthority
                | ProofDimension::EmbeddedBoundaryHonesty
        );
        assert_eq!(critical, expected, "{} criticality", dimension.as_str());
    }
}

#[test]
fn release_evidence_dimensions_are_present() {
    let packet = operator_qualification_packet();
    for dimension in [
        ProofDimension::ServiceOwnership,
        ProofDimension::RunbookStepAuthority,
        ProofDimension::HandoffBundleFidelity,
        ProofDimension::MaintenanceFailoverCommunication,
        ProofDimension::EmbeddedBoundaryHonesty,
    ] {
        assert!(
            packet.dimension(dimension).is_some(),
            "release-evidence dimension {} must be present",
            dimension.as_str()
        );
    }
}

#[test]
fn lines_projection_renders_every_section() {
    let packet = operator_qualification_packet();
    let lines = operator_qualification_lines(&packet);
    assert!(lines
        .iter()
        .any(|line| line.contains("Operator-surface qualification")));
    assert!(lines.iter().any(|line| line.contains("Proof dimensions:")));
    assert!(lines.iter().any(|line| line.contains("Families:")));
    assert!(lines.iter().any(|line| line.contains("rollup:")));
    for dimension in ProofDimension::ALL {
        assert!(
            lines.iter().any(|line| line.contains(dimension.as_str())),
            "lines must mention dimension {}",
            dimension.as_str()
        );
    }
    for surface in OperatorSurfaceClass::ALL {
        assert!(
            lines.iter().any(|line| line.contains(surface.as_str())),
            "lines must mention family {}",
            surface.as_str()
        );
    }
}
