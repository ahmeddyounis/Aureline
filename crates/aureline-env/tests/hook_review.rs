//! Replay and coverage gate for the hook-review corpus.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_env::{
    export_hook_review, review_hooks, seeded_hook_review_drills, seeded_hook_review_fixtures,
    validate_hook_review_drill, validate_hook_review_fixture, HookDisposition, HookReviewFixture,
    HookReviewPosture, RedactionClass, TrustGateState, HOOK_REVIEW_DOC_REF,
    HOOK_REVIEW_FIXTURE_DIR, HOOK_REVIEW_FIXTURE_MANIFEST_REF, HOOK_REVIEW_PROOF_REF,
    HOOK_REVIEW_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn load_fixtures() -> Vec<HookReviewFixture> {
    let dir = repo_root().join(HOOK_REVIEW_FIXTURE_DIR);
    let mut out = Vec::new();
    for entry in fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: HookReviewFixture = serde_json::from_str(&raw)
            .unwrap_or_else(|err| panic!("fixture {} must parse: {err}", path.display()));
        out.push(fixture);
    }
    out.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert!(!out.is_empty(), "expected at least one fixture");
    out
}

#[test]
fn fixture_corpus_matches_seeded_projection_and_validates() {
    let on_disk = load_fixtures();
    let mut seeded = seeded_hook_review_fixtures();
    seeded.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert_eq!(
        on_disk, seeded,
        "fixture corpus drifted from seeded fixtures"
    );
    for fixture in &on_disk {
        validate_hook_review_fixture(fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn files_exist_on_disk() {
    let root = repo_root();
    for rel in [
        HOOK_REVIEW_SCHEMA_REF,
        HOOK_REVIEW_DOC_REF,
        HOOK_REVIEW_PROOF_REF,
        HOOK_REVIEW_FIXTURE_MANIFEST_REF,
    ] {
        assert!(root.join(rel).exists(), "required file must exist: {rel}");
    }
    assert!(
        root.join(HOOK_REVIEW_FIXTURE_DIR).is_dir(),
        "fixture directory must exist"
    );
}

#[test]
fn seeded_drills_validate() {
    for drill in seeded_hook_review_drills() {
        validate_hook_review_drill(&drill)
            .unwrap_or_else(|err| panic!("drill {} must validate: {err}", drill.drill_id));
    }
}

#[test]
fn corpus_covers_every_posture() {
    let fixtures = load_fixtures();
    let postures: BTreeSet<HookReviewPosture> = fixtures.iter().map(|f| f.packet.posture).collect();
    for required in HookReviewPosture::ALL {
        assert!(
            postures.contains(&required),
            "corpus must cover {required:?}"
        );
    }
}

#[test]
fn corpus_exercises_every_disposition_including_blocked_and_denied() {
    let fixtures = load_fixtures();
    let dispositions: BTreeSet<HookDisposition> = fixtures
        .iter()
        .flat_map(|f| f.packet.entries.iter().map(|e| e.disposition))
        .collect();
    for required in HookDisposition::ALL {
        assert!(
            dispositions.contains(&required),
            "corpus must exercise {required:?}"
        );
    }
}

#[test]
fn no_ungated_hook_is_ever_runnable() {
    // The guardrail: a trust-gated lifecycle action is never run merely because
    // a capsule or template references it.
    for fixture in load_fixtures() {
        for entry in &fixture.packet.entries {
            if entry.gate_state == TrustGateState::Ungated {
                assert!(
                    !entry.runnable,
                    "ungated hook {} in fixture {} must not run",
                    entry.hook_id, fixture.fixture_id
                );
            }
        }
    }
}

#[test]
fn review_is_metadata_first_and_repairs_preserve_identity_on_every_fixture() {
    for fixture in load_fixtures() {
        let packet = review_hooks(&fixture.hooks, &fixture.context);
        assert_eq!(packet.redaction_class, RedactionClass::MetadataOnly);
        let export = export_hook_review(&packet);
        assert_eq!(export.redaction_class, RedactionClass::MetadataOnly);
        assert_eq!(
            export.packet, packet,
            "support export must wrap the canonical packet"
        );
        // Hook command digests are 64-hex, never bodies.
        for entry in &packet.entries {
            assert_eq!(entry.command_digest.value.len(), 64);
        }
        // Every non-runnable hook carries a repair that names it and a step.
        for entry in &packet.entries {
            if !entry.runnable {
                let repair = entry
                    .repair
                    .as_ref()
                    .unwrap_or_else(|| panic!("held hook {} must carry a repair", entry.hook_id));
                assert_eq!(repair.hook_id, entry.hook_id);
                assert!(!repair.next_step.is_empty());
            }
        }
    }
}
