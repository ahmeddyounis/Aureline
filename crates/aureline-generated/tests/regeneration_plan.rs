//! Replay and coverage gate for the regeneration-plan packet.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_generated::{
    seeded_regeneration_plan_fixtures, seeded_regeneration_plan_packet,
    validate_regeneration_plan_fixture, validate_regeneration_plan_packet, PlanReadiness,
    RegenerationPlanFixture, RegenerationPlanPacket, RegenerationPlanSurface, RollbackCoverage,
    TargetOutcome, REGENERATION_PLAN_DOC_REF, REGENERATION_PLAN_FIXTURE_DIR,
    REGENERATION_PLAN_FIXTURE_MANIFEST_REF, REGENERATION_PLAN_PACKET_REF,
    REGENERATION_PLAN_REPORT_REF, REGENERATION_PLAN_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn load_packet() -> RegenerationPlanPacket {
    let path = repo_root().join(REGENERATION_PLAN_PACKET_REF);
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("packet {} must read: {err}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|err| panic!("packet {} must parse: {err}", path.display()))
}

fn load_fixtures() -> Vec<RegenerationPlanFixture> {
    let dir = repo_root().join(REGENERATION_PLAN_FIXTURE_DIR);
    let mut out = Vec::new();
    for entry in fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: RegenerationPlanFixture = serde_json::from_str(&raw)
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
    let seeded = seeded_regeneration_plan_packet();
    assert_eq!(
        packet, seeded,
        "regeneration-plan packet drifted from seeded packet"
    );
    validate_regeneration_plan_packet(&packet)
        .expect("regeneration-plan packet must satisfy the frozen contract");
}

#[test]
fn fixture_corpus_matches_seeded_projection_and_validates() {
    let on_disk = load_fixtures();
    let mut seeded = seeded_regeneration_plan_fixtures();
    seeded.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert_eq!(
        on_disk, seeded,
        "fixture corpus drifted from seeded fixtures"
    );
    for fixture in &on_disk {
        validate_regeneration_plan_fixture(fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn files_exist_on_disk() {
    let root = repo_root();
    for rel in [
        REGENERATION_PLAN_SCHEMA_REF,
        REGENERATION_PLAN_DOC_REF,
        REGENERATION_PLAN_PACKET_REF,
        REGENERATION_PLAN_REPORT_REF,
        REGENERATION_PLAN_FIXTURE_MANIFEST_REF,
    ] {
        assert!(root.join(rel).exists(), "required file must exist: {rel}");
    }
    assert!(
        root.join(REGENERATION_PLAN_FIXTURE_DIR).is_dir(),
        "fixture directory must exist"
    );
}

#[test]
fn packet_covers_every_readiness_and_outcome() {
    let packet = load_packet();
    let readiness: BTreeSet<_> = packet.cases.iter().map(|c| c.plan.readiness).collect();
    for required in PlanReadiness::ALL {
        assert!(
            readiness.contains(&required),
            "packet must cover readiness {}",
            required.as_str()
        );
    }
    let outcomes: BTreeSet<_> = packet
        .cases
        .iter()
        .flat_map(|c| c.plan.targets.iter().map(|t| t.outcome))
        .collect();
    for required in TargetOutcome::ALL {
        assert!(
            outcomes.contains(&required),
            "packet must cover target outcome {}",
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
    for required in RegenerationPlanSurface::ALL {
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
fn every_regenerate_action_has_a_plan_and_side_effect_boundary() {
    let packet = load_packet();
    for plan_case in &packet.cases {
        let plan = &plan_case.plan;
        // Every plan names its targets and a rollback boundary before
        // execution.
        assert!(
            !plan.targets.is_empty(),
            "case {} must carry at least one target",
            plan_case.case_id
        );
        assert!(
            !plan.rollback_boundary.checkpoint_ref.trim().is_empty(),
            "case {} must carry a rollback boundary",
            plan_case.case_id
        );
        // A degraded plan must explain itself, never masquerade as success.
        if plan.readiness != PlanReadiness::Ready {
            assert!(
                !plan.guidance_line.trim().is_empty(),
                "case {} must carry a guidance line, not a generic outcome",
                plan_case.case_id
            );
            assert!(
                !plan.recovery.is_empty(),
                "case {} must offer a recovery path",
                plan_case.case_id
            );
        }
        if !plan.readiness.runs_in_full() {
            assert!(
                !plan.why_blocked_tokens.is_empty(),
                "case {} must name why a target was blocked",
                plan_case.case_id
            );
        }
    }
}

#[test]
fn no_undeclared_side_effect_ever_runs_silently() {
    let packet = load_packet();
    let mut saw_undeclared = false;
    for plan_case in &packet.cases {
        let plan = &plan_case.plan;
        if !plan.side_effect_boundary.all_sensitive_declared {
            saw_undeclared = true;
            // The plan cannot run in full while a sensitive side effect is
            // undeclared.
            assert!(
                !plan.readiness.runs_in_full(),
                "case {} must not run in full with an undeclared side effect",
                plan_case.case_id
            );
        }
    }
    assert!(
        saw_undeclared,
        "packet must exercise an undeclared sensitive side effect"
    );
}

#[test]
fn rollback_coverage_is_honest_about_escaping_writes() {
    let packet = load_packet();
    for plan_case in &packet.cases {
        let plan = &plan_case.plan;
        let escapes = plan
            .side_effect_boundary
            .classes_present
            .iter()
            .any(|c| c.escapes_checkpoint());
        let expected = if escapes {
            RollbackCoverage::PartiallyReversible
        } else {
            RollbackCoverage::FullyReversible
        };
        assert_eq!(
            plan.rollback_coverage, expected,
            "case {} rollback coverage must reflect its side effects",
            plan_case.case_id
        );
    }
}
