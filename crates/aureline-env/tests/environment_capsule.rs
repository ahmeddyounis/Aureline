//! Replay and coverage gate for the typed environment-capsule corpus.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_env::{
    diff_capsules, export_capsule_metadata, inspect_environment,
    seeded_environment_capsule_fixtures, seeded_environment_capsules, validate_environment_capsule,
    validate_environment_capsule_fixture, CapsuleTargetClass, EnvironmentCapsuleFixture,
    RedactionClass, RowVerdict, ENVIRONMENT_CAPSULE_DOC_REF, ENVIRONMENT_CAPSULE_FIXTURE_DIR,
    ENVIRONMENT_CAPSULE_FIXTURE_MANIFEST_REF, ENVIRONMENT_CAPSULE_PROOF_REF,
    ENVIRONMENT_CAPSULE_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn load_fixtures() -> Vec<EnvironmentCapsuleFixture> {
    let dir = repo_root().join(ENVIRONMENT_CAPSULE_FIXTURE_DIR);
    let mut out = Vec::new();
    for entry in fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: EnvironmentCapsuleFixture = serde_json::from_str(&raw)
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
    let mut seeded = seeded_environment_capsule_fixtures();
    seeded.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert_eq!(
        on_disk, seeded,
        "fixture corpus drifted from seeded fixtures"
    );
    for fixture in &on_disk {
        validate_environment_capsule_fixture(fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn files_exist_on_disk() {
    let root = repo_root();
    for rel in [
        ENVIRONMENT_CAPSULE_SCHEMA_REF,
        ENVIRONMENT_CAPSULE_DOC_REF,
        ENVIRONMENT_CAPSULE_PROOF_REF,
        ENVIRONMENT_CAPSULE_FIXTURE_MANIFEST_REF,
    ] {
        assert!(root.join(rel).exists(), "required file must exist: {rel}");
    }
    assert!(
        root.join(ENVIRONMENT_CAPSULE_FIXTURE_DIR).is_dir(),
        "fixture directory must exist"
    );
}

#[test]
fn seeded_capsules_validate_and_their_source_refs_exist() {
    let root = repo_root();
    for capsule in seeded_environment_capsules() {
        validate_environment_capsule(&capsule).unwrap_or_else(|err| {
            panic!(
                "capsule {} must validate: {err}",
                capsule.identity.capsule_id
            )
        });
        for source in &capsule.source_refs {
            // Source refs that point at checked-in artifacts must exist.
            if source.reference.starts_with("artifacts/") {
                assert!(
                    root.join(&source.reference).exists(),
                    "capsule {} source ref must exist on disk: {}",
                    capsule.identity.capsule_id,
                    source.reference
                );
            }
        }
    }
}

#[test]
fn corpus_covers_every_target_class_and_local_plus_non_local() {
    let fixtures = load_fixtures();
    let mut classes = BTreeSet::new();
    for fixture in &fixtures {
        classes.insert(fixture.target_class);
    }
    for required in CapsuleTargetClass::ALL {
        assert!(
            classes.contains(&required),
            "corpus must cover target class {}",
            required.as_str()
        );
    }
    assert!(
        classes.contains(&CapsuleTargetClass::Local),
        "corpus must cover a local path"
    );
    let non_local = classes
        .iter()
        .any(|class| !matches!(class, CapsuleTargetClass::Local));
    assert!(non_local, "corpus must cover at least one non-local path");
}

#[test]
fn inspection_is_metadata_first_on_every_capsule() {
    for capsule in seeded_environment_capsules() {
        let inspection = inspect_environment(&capsule);
        assert_eq!(inspection.redaction_class, RedactionClass::MetadataOnly);
        let export = export_capsule_metadata(&capsule);
        assert_eq!(export.redaction_class, RedactionClass::MetadataOnly);
        assert_eq!(
            export.inspection, inspection,
            "support export must wrap the canonical inspection"
        );
    }
}

#[test]
fn corpus_is_diffable_across_a_local_and_non_local_capsule() {
    let capsules = seeded_environment_capsules();
    let local = capsules
        .iter()
        .find(|c| c.identity.capsule_id == "env.capsule.local")
        .expect("local capsule");
    let managed = capsules
        .iter()
        .find(|c| c.identity.capsule_id == "env.capsule.managed_workspace")
        .expect("managed capsule");
    let diff = diff_capsules(local, managed);
    assert!(!diff.identical, "a local and managed capsule must differ");
    assert!(
        diff.changes.iter().any(|c| c.path == "identity.transport"),
        "diff must surface the transport change"
    );
}

#[test]
fn corpus_exercises_certified_narrowed_and_withheld_verdicts() {
    let fixtures = load_fixtures();
    let verdicts: BTreeSet<RowVerdict> = fixtures.iter().map(|f| f.expected_verdict).collect();
    for required in [
        RowVerdict::Certified,
        RowVerdict::Narrowed,
        RowVerdict::Withheld,
    ] {
        assert!(
            verdicts.contains(&required),
            "corpus must exercise {required:?}"
        );
    }
}
