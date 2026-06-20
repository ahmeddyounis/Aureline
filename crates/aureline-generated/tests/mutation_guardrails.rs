//! Replay and coverage gate for the mutation-guardrails packet.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_generated::{
    seeded_mutation_guardrails_fixtures, seeded_mutation_guardrails_packet,
    validate_mutation_guardrails_fixture, validate_mutation_guardrails_packet, GuardrailOutcome,
    MutationGuardrailFixture, MutationGuardrailPacket, MutationGuardrailSurface, MutationRoute,
    MUTATION_GUARDRAILS_DOC_REF, MUTATION_GUARDRAILS_FIXTURE_DIR,
    MUTATION_GUARDRAILS_FIXTURE_MANIFEST_REF, MUTATION_GUARDRAILS_PACKET_REF,
    MUTATION_GUARDRAILS_REPORT_REF, MUTATION_GUARDRAILS_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn load_packet() -> MutationGuardrailPacket {
    let path = repo_root().join(MUTATION_GUARDRAILS_PACKET_REF);
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("packet {} must read: {err}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|err| panic!("packet {} must parse: {err}", path.display()))
}

fn load_fixtures() -> Vec<MutationGuardrailFixture> {
    let dir = repo_root().join(MUTATION_GUARDRAILS_FIXTURE_DIR);
    let mut out = Vec::new();
    for entry in fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: MutationGuardrailFixture = serde_json::from_str(&raw)
            .unwrap_or_else(|err| panic!("fixture {} must parse: {err}", path.display()));
        out.push(fixture);
    }
    out.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert!(!out.is_empty(), "expected at least one fixture");
    out
}

#[test]
fn packet_matches_seeded_projection_and_validates() {
    let packet = load_packet();
    let seeded = seeded_mutation_guardrails_packet();
    assert_eq!(
        packet, seeded,
        "mutation-guardrails packet drifted from seeded packet"
    );
    validate_mutation_guardrails_packet(&packet)
        .expect("mutation-guardrails packet must satisfy the frozen contract");
}

#[test]
fn fixture_corpus_matches_seeded_projection_and_validates() {
    let on_disk = load_fixtures();
    let mut seeded = seeded_mutation_guardrails_fixtures();
    seeded.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert_eq!(
        on_disk, seeded,
        "fixture corpus drifted from seeded fixtures"
    );
    for fixture in &on_disk {
        validate_mutation_guardrails_fixture(fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn files_exist_on_disk() {
    let root = repo_root();
    for rel in [
        MUTATION_GUARDRAILS_SCHEMA_REF,
        MUTATION_GUARDRAILS_DOC_REF,
        MUTATION_GUARDRAILS_PACKET_REF,
        MUTATION_GUARDRAILS_REPORT_REF,
        MUTATION_GUARDRAILS_FIXTURE_MANIFEST_REF,
    ] {
        assert!(root.join(rel).exists(), "required file must exist: {rel}");
    }
    assert!(
        root.join(MUTATION_GUARDRAILS_FIXTURE_DIR).is_dir(),
        "fixture directory must exist"
    );
}

#[test]
fn packet_covers_every_route_and_outcome() {
    let packet = load_packet();
    let routes: BTreeSet<_> = packet.cases.iter().map(|c| c.decision.route).collect();
    for required in MutationRoute::ALL {
        assert!(
            routes.contains(&required),
            "packet must cover mutation route {}",
            required.as_str()
        );
    }
    let outcomes: BTreeSet<_> = packet
        .cases
        .iter()
        .map(|c| c.decision.guardrail_outcome)
        .collect();
    for required in GuardrailOutcome::ALL {
        assert!(
            outcomes.contains(&required),
            "packet must cover guardrail outcome {}",
            required.as_str()
        );
    }
}

#[test]
fn evidence_packet_refs_point_at_real_artifacts() {
    let packet = load_packet();
    assert!(
        !packet.evidence_packet_refs.is_empty(),
        "packet must cite upstream generated-artifact evidence"
    );
    let root = repo_root();
    for reference in &packet.evidence_packet_refs {
        assert!(
            root.join(reference).exists(),
            "evidence packet ref must exist on disk: {reference}"
        );
    }
}

#[test]
fn packet_binds_every_surface_with_real_consumers() {
    let packet = load_packet();
    let surfaces: BTreeSet<_> = packet
        .surface_bindings
        .iter()
        .map(|binding| binding.surface)
        .collect();
    for required in MutationGuardrailSurface::ALL {
        assert!(
            surfaces.contains(&required),
            "packet must bind surface {}",
            required.as_str()
        );
    }
    let root = repo_root();
    for binding in &packet.surface_bindings {
        assert!(
            root.join(&binding.consumer_ref).exists(),
            "surface consumer ref must exist on disk: {}",
            binding.consumer_ref
        );
    }
}

#[test]
fn every_case_consumer_ref_exists_on_disk() {
    let packet = load_packet();
    let root = repo_root();
    for guard_case in &packet.cases {
        assert!(
            root.join(&guard_case.consumer_ref).exists(),
            "case {} consumer ref must exist on disk: {}",
            guard_case.case_id,
            guard_case.consumer_ref
        );
    }
}

#[test]
fn no_automated_route_silently_mutates_a_generated_artifact() {
    let packet = load_packet();
    for guard_case in &packet.cases {
        let decision = &guard_case.decision;
        // An admitted mutation is only ever a direct edit of the artifact's own
        // in-sync canonical source — never a silent cross-boundary write.
        if decision.mutation_admitted {
            match decision.guardrail_outcome {
                GuardrailOutcome::AdmittedDirect => {
                    assert!(
                        !decision.crosses_canonical_boundary,
                        "case {} admitted-direct must not cross a canonical boundary",
                        guard_case.case_id
                    );
                }
                GuardrailOutcome::AdmittedWithPreviewAndOverride => {
                    assert!(
                        decision.safety_envelope_complete,
                        "case {} admitted crossing must carry a complete safety envelope",
                        guard_case.case_id
                    );
                    assert!(
                        decision.boundary_decision.diverged_from_generator.is_some(),
                        "case {} admitted crossing must leave a divergence",
                        guard_case.case_id
                    );
                }
                other => panic!(
                    "case {} admitted under unexpected outcome {:?}",
                    guard_case.case_id, other
                ),
            }
        } else {
            // Every blocked mutation names why and explains it for support.
            assert!(
                !decision.why_blocked_tokens.is_empty(),
                "case {} blocked mutation must name why",
                guard_case.case_id
            );
            assert!(
                !decision.support_summary.trim().is_empty(),
                "case {} must carry a support summary",
                guard_case.case_id
            );
        }
    }
}

#[test]
fn missing_boundary_data_is_always_blocked() {
    let packet = load_packet();
    let mut saw_missing = false;
    for guard_case in &packet.cases {
        let decision = &guard_case.decision;
        if decision.guardrail_outcome == GuardrailOutcome::BlockedMissingBoundaryData {
            saw_missing = true;
            assert!(
                !decision.mutation_admitted,
                "case {} missing-boundary-data must not admit a mutation",
                guard_case.case_id
            );
        }
    }
    assert!(
        saw_missing,
        "packet must exercise a missing-boundary-data block"
    );
}
