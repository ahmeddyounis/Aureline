//! Unit tests for the editor-assist qualification packet.

use super::*;
use crate::m5_editor_assist::{AssistChannelClass, EditorSurfaceClass};

const EVALUATED: &str = "2026-06-22T00:00:00Z";

/// All-fresh, all-passing proof inputs captured one day before evaluation.
fn fresh_inputs() -> Vec<ProofInput> {
    ProofDimension::ALL
        .iter()
        .map(|dimension| ProofInput {
            dimension: *dimension,
            proof_source_ref: "schemas/editor/test.schema.json".to_owned(),
            contributing_proof_refs: vec!["schemas/editor/test.schema.json".to_owned()],
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
    let packet = assist_qualification_packet();
    assert_eq!(packet.record_kind, M5_ASSIST_QUALIFICATION_RECORD_KIND);
    assert_eq!(packet.schema_ref, M5_ASSIST_QUALIFICATION_SCHEMA_REF);
    assert_eq!(packet.packet_id, M5_ASSIST_QUALIFICATION_PACKET_ID);
    assert_eq!(packet.as_of, M5_ASSIST_QUALIFICATION_AS_OF);
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
    let packet = assist_qualification_packet();
    assert_eq!(packet.families.len(), EditorSurfaceClass::ALL.len());
    for surface in EditorSurfaceClass::ALL {
        assert!(
            packet.is_family_fully_supported(surface),
            "family {} must be fully supported in the canonical packet",
            surface.as_str()
        );
    }
    assert_eq!(packet.rollup.fully_supported, EditorSurfaceClass::ALL.len());
    assert_eq!(packet.rollup.narrowed, 0);
    assert_eq!(packet.rollup.blocked, 0);
    assert_eq!(packet.rollup.stale_dimensions, 0);
    assert_eq!(packet.rollup.failing_dimensions, 0);
    assert_eq!(packet.rollup.missing_dimensions, 0);
}

#[test]
fn canonical_packet_carries_every_dimension() {
    let packet = assist_qualification_packet();
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
    let packet = assist_qualification_packet();
    let json = serde_json::to_string(&packet).expect("packet serializes");
    let restored: AssistQualificationPacket =
        serde_json::from_str(&json).expect("packet round-trips");
    assert_eq!(packet, restored);
}

#[test]
fn stale_completion_proof_narrows_families_that_claim_completion() {
    let mut inputs = fresh_inputs();
    // Push the completion proof's capture stamp far enough back to age out.
    set_dimension(&mut inputs, ProofDimension::Completion, |input| {
        input.captured_as_of = Some("2026-01-01T00:00:00Z".to_owned());
    });
    let packet = project_assist_qualification(EVALUATED, &inputs);

    assert_eq!(
        packet.dimension(ProofDimension::Completion).unwrap().state,
        ProofState::Stale
    );

    // A code file claims completion, so it narrows.
    let code = packet.family(EditorSurfaceClass::CodeFile).unwrap();
    assert_eq!(code.support, ClaimSupportClass::Narrowed);
    assert!(code.narrowed_by.contains(&ProofDimension::Completion));

    // A large file suppresses completion entirely, so stale completion proof
    // does not touch it; every other dimension it claims is fresh.
    let large = packet
        .family(EditorSurfaceClass::LargeFileRestricted)
        .unwrap();
    assert!(
        !large
            .verdict(ProofDimension::Completion)
            .unwrap()
            .applicable
    );
    assert_eq!(large.support, ClaimSupportClass::FullySupported);
}

#[test]
fn failing_critical_precedence_blocks_every_family() {
    let mut inputs = fresh_inputs();
    set_dimension(&mut inputs, ProofDimension::Precedence, |input| {
        input.passing = false;
    });
    let packet = project_assist_qualification(EVALUATED, &inputs);

    assert_eq!(
        packet.dimension(ProofDimension::Precedence).unwrap().state,
        ProofState::Failing
    );
    for family in &packet.families {
        assert_eq!(
            family.support,
            ClaimSupportClass::Blocked,
            "precedence is global + critical, so {} must block",
            family.surface.as_str()
        );
        assert!(family.blocked_by.contains(&ProofDimension::Precedence));
    }
    assert_eq!(packet.rollup.blocked, EditorSurfaceClass::ALL.len());
    assert!(packet.all_invariants_hold());
}

#[test]
fn missing_critical_source_honesty_blocks_claim() {
    // Drop the assist-source-honesty proof entirely.
    let inputs: Vec<ProofInput> = fresh_inputs()
        .into_iter()
        .filter(|input| input.dimension != ProofDimension::AssistSourceHonesty)
        .collect();
    let packet = project_assist_qualification(EVALUATED, &inputs);

    assert_eq!(
        packet
            .dimension(ProofDimension::AssistSourceHonesty)
            .unwrap()
            .state,
        ProofState::Missing
    );
    let code = packet.family(EditorSurfaceClass::CodeFile).unwrap();
    assert_eq!(code.support, ClaimSupportClass::Blocked);
    assert!(code
        .blocked_by
        .contains(&ProofDimension::AssistSourceHonesty));
    assert_eq!(packet.rollup.missing_dimensions, 1);
    assert!(packet.all_invariants_hold());
}

#[test]
fn stale_noncritical_proof_narrows_but_does_not_block() {
    let mut inputs = fresh_inputs();
    set_dimension(&mut inputs, ProofDimension::AccessibilityParity, |input| {
        input.passing = false; // failing, but non-critical
    });
    let packet = project_assist_qualification(EVALUATED, &inputs);
    // Accessibility parity is global, so every family narrows (never blocks).
    for family in &packet.families {
        assert_eq!(
            family.support,
            ClaimSupportClass::Narrowed,
            "{} must narrow on failing non-critical proof",
            family.surface.as_str()
        );
    }
    assert_eq!(packet.rollup.blocked, 0);
    assert_eq!(packet.rollup.narrowed, EditorSurfaceClass::ALL.len());
}

#[test]
fn constrained_file_narrowing_applies_only_to_constrained_families() {
    let packet = assist_qualification_packet();
    for family in &packet.families {
        let verdict = family
            .verdict(ProofDimension::ConstrainedFileNarrowing)
            .unwrap();
        assert_eq!(
            verdict.applicable,
            family.is_constrained,
            "constrained-file narrowing applicability must match family constraint for {}",
            family.surface.as_str()
        );
    }
}

#[test]
fn peek_is_not_claimed_where_the_surface_blocks_it() {
    let packet = assist_qualification_packet();
    // Docs-code blocks render peek as blocked/unavailable, so the peek dimension
    // does not govern that family.
    let docs = packet.family(EditorSurfaceClass::DocsCodeBlock).unwrap();
    assert!(!docs.verdict(ProofDimension::Peek).unwrap().applicable);
    // A code file claims peek.
    let code = packet.family(EditorSurfaceClass::CodeFile).unwrap();
    assert!(code.verdict(ProofDimension::Peek).unwrap().applicable);
}

#[test]
fn every_family_verdict_covers_every_dimension() {
    let packet = assist_qualification_packet();
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
        dimension: ProofDimension::Completion,
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
fn ime_multicursor_dimension_governs_completion_and_snippet_surfaces() {
    let packet = assist_qualification_packet();
    let code = packet.family(EditorSurfaceClass::CodeFile).unwrap();
    assert!(
        code.verdict(ProofDimension::ImeMultiCursorSafety)
            .unwrap()
            .applicable
    );
    assert_eq!(
        ProofDimension::ImeMultiCursorSafety.governing_channels(),
        &[
            AssistChannelClass::SnippetSession,
            AssistChannelClass::Completion
        ]
    );
}

#[test]
fn lines_projection_renders_every_section() {
    let packet = assist_qualification_packet();
    let lines = assist_qualification_lines(&packet);
    assert!(lines
        .iter()
        .any(|line| line.contains("Editor-assist qualification")));
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
    for surface in EditorSurfaceClass::ALL {
        assert!(
            lines.iter().any(|line| line.contains(surface.as_str())),
            "lines must mention family {}",
            surface.as_str()
        );
    }
}

#[test]
fn critical_dimensions_are_only_honesty_and_precedence() {
    for dimension in ProofDimension::ALL {
        let critical = dimension.is_critical();
        let expected = matches!(
            dimension,
            ProofDimension::AssistSourceHonesty | ProofDimension::Precedence
        );
        assert_eq!(critical, expected, "{} criticality", dimension.as_str());
    }
}
