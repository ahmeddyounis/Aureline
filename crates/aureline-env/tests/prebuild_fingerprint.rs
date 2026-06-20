//! Replay and coverage gate for the prebuild-fingerprint packet and the
//! warm-versus-cold reuse corpus.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_env::{
    evaluate_prebuild_reuse, seeded_prebuild_fingerprint_fixtures,
    seeded_prebuild_fingerprint_packet, validate_prebuild_fingerprint_fixture,
    validate_prebuild_fingerprint_packet, PrebuildFingerprintFixture, PrebuildFingerprintPacket,
    PrebuildReason, StartOutcome, PREBUILD_FINGERPRINT_DOC_REF, PREBUILD_FINGERPRINT_FIXTURE_DIR,
    PREBUILD_FINGERPRINT_FIXTURE_MANIFEST_REF, PREBUILD_FINGERPRINT_PACKET_REF,
    PREBUILD_FINGERPRINT_PROOF_REF, PREBUILD_FINGERPRINT_SCHEMA_REF, PREBUILD_REUSE_DRILLS_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn load_packet() -> PrebuildFingerprintPacket {
    let path = repo_root().join(PREBUILD_FINGERPRINT_PACKET_REF);
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("packet {} must read: {err}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|err| panic!("packet {} must parse: {err}", path.display()))
}

fn load_fixtures() -> Vec<PrebuildFingerprintFixture> {
    let dir = repo_root().join(PREBUILD_FINGERPRINT_FIXTURE_DIR);
    let mut out = Vec::new();
    for entry in fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: PrebuildFingerprintFixture = serde_json::from_str(&raw)
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
    let seeded = seeded_prebuild_fingerprint_packet();
    assert_eq!(packet, seeded, "artifact packet drifted from seeded packet");
    validate_prebuild_fingerprint_packet(&packet)
        .expect("artifact packet must satisfy the frozen contract");
}

#[test]
fn fixture_corpus_matches_seeded_projection_and_validates() {
    let on_disk = load_fixtures();
    let mut seeded = seeded_prebuild_fingerprint_fixtures();
    seeded.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert_eq!(
        on_disk, seeded,
        "fixture corpus drifted from seeded fixtures"
    );
    for fixture in &on_disk {
        validate_prebuild_fingerprint_fixture(fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn files_exist_on_disk() {
    let root = repo_root();
    for rel in [
        PREBUILD_FINGERPRINT_SCHEMA_REF,
        PREBUILD_FINGERPRINT_DOC_REF,
        PREBUILD_FINGERPRINT_PACKET_REF,
        PREBUILD_FINGERPRINT_PROOF_REF,
        PREBUILD_REUSE_DRILLS_REF,
        PREBUILD_FINGERPRINT_FIXTURE_MANIFEST_REF,
    ] {
        assert!(root.join(rel).exists(), "required file must exist: {rel}");
    }
    assert!(
        root.join(PREBUILD_FINGERPRINT_FIXTURE_DIR).is_dir(),
        "fixture directory must exist"
    );
}

#[test]
fn evidence_packet_refs_point_at_real_artifacts() {
    let packet = load_packet();
    assert!(
        !packet.evidence_packet_refs.is_empty(),
        "packet must cite upstream environment evidence"
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
fn every_case_decision_replays_from_the_engine() {
    // The stamped decision on every case must equal what the engine recomputes
    // from the snapshot and current fingerprint — no precomputed truth.
    let packet = load_packet();
    for case in &packet.cases {
        let replayed = evaluate_prebuild_reuse(&case.snapshot, &case.current_fingerprint);
        assert_eq!(
            case.decision, replayed,
            "case {} decision drifted from the engine",
            case.case_id
        );
    }
}

#[test]
fn cases_cover_every_outcome() {
    let packet = load_packet();
    let outcomes: BTreeSet<StartOutcome> =
        packet.cases.iter().map(|c| c.decision.outcome).collect();
    for required in StartOutcome::ALL {
        assert!(
            outcomes.contains(&required),
            "cases must cover the {} outcome",
            required.as_str()
        );
    }
}

#[test]
fn drills_cover_every_named_drift_class_and_recover() {
    let packet = load_packet();
    let reasons: BTreeSet<PrebuildReason> =
        packet.drills.iter().map(|d| d.injected_reason).collect();
    for required in [
        PrebuildReason::SourceDrift,
        PrebuildReason::PolicyDrift,
        PrebuildReason::PlatformDrift,
        PrebuildReason::ExtensionLockDrift,
        PrebuildReason::PartialArtifactLoss,
    ] {
        assert!(
            reasons.contains(&required),
            "drills must exercise the {} class",
            required.as_str()
        );
    }
    for drill in &packet.drills {
        assert_eq!(
            drill.recovers_to_outcome,
            StartOutcome::Warm,
            "drill {} must recover to warm",
            drill.drill_id
        );
        assert_ne!(
            drill.degraded_outcome,
            StartOutcome::Warm,
            "drill {} must narrow below warm under drift",
            drill.drill_id
        );
    }
}

#[test]
fn corpus_distinguishes_all_four_starts_and_reuse_flags() {
    // Acceptance: users and support tooling can distinguish warm, partially
    // warm, cold, and invalidated starts.
    let fixtures = load_fixtures();
    let outcomes: BTreeSet<StartOutcome> = fixtures.iter().map(|f| f.expected_outcome).collect();
    for required in StartOutcome::ALL {
        assert!(
            outcomes.contains(&required),
            "corpus must distinguish the {} start",
            required.as_str()
        );
    }
    // Only warm and partially warm reuse the snapshot; cold and invalidated do
    // not. Exactly the invalidated fixtures set the invalidated flag.
    for fixture in &fixtures {
        let reuses = matches!(
            fixture.expected_outcome,
            StartOutcome::Warm | StartOutcome::PartiallyWarm
        );
        assert_eq!(
            fixture.expected_reused, reuses,
            "fixture {} reuse flag disagrees with its outcome",
            fixture.fixture_id
        );
        assert_eq!(
            fixture.expected_invalidated,
            fixture.expected_outcome == StartOutcome::Invalidated,
            "fixture {} invalidated flag disagrees with its outcome",
            fixture.fixture_id
        );
    }
}

#[test]
fn source_policy_and_platform_drift_never_warm_reuse() {
    // Guardrail: prebuild reuse no longer silently outruns source, policy, or
    // platform drift — each must reject or invalidate, never reuse.
    let fixtures = load_fixtures();
    for fixture in &fixtures {
        if matches!(
            fixture.injected_reason,
            PrebuildReason::SourceDrift
                | PrebuildReason::PolicyDrift
                | PrebuildReason::PlatformDrift
        ) {
            assert!(
                !fixture.expected_reused,
                "fixture {} must not reuse under {} drift",
                fixture.fixture_id,
                fixture.injected_reason.as_str()
            );
        }
    }
}
