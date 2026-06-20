//! Replay and coverage gate for the runtime-materialization corpus.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_env::{
    derive_runtime_instance, export_runtime_materialization, materialize_runtime,
    seeded_runtime_instances, seeded_runtime_materialization_fixtures, validate_runtime_instance,
    validate_runtime_materialization_fixture, CapsuleTargetClass, RedactionClass,
    RuntimeMaterializationFixture, RuntimeParity, RuntimeScenario, RUNTIME_MATERIALIZATION_DOC_REF,
    RUNTIME_MATERIALIZATION_FIXTURE_DIR, RUNTIME_MATERIALIZATION_FIXTURE_MANIFEST_REF,
    RUNTIME_MATERIALIZATION_PROOF_REF, RUNTIME_MATERIALIZATION_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn load_fixtures() -> Vec<RuntimeMaterializationFixture> {
    let dir = repo_root().join(RUNTIME_MATERIALIZATION_FIXTURE_DIR);
    let mut out = Vec::new();
    for entry in fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: RuntimeMaterializationFixture = serde_json::from_str(&raw)
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
    let mut seeded = seeded_runtime_materialization_fixtures();
    seeded.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert_eq!(
        on_disk, seeded,
        "fixture corpus drifted from seeded fixtures"
    );
    for fixture in &on_disk {
        validate_runtime_materialization_fixture(fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn files_exist_on_disk() {
    let root = repo_root();
    for rel in [
        RUNTIME_MATERIALIZATION_SCHEMA_REF,
        RUNTIME_MATERIALIZATION_DOC_REF,
        RUNTIME_MATERIALIZATION_PROOF_REF,
        RUNTIME_MATERIALIZATION_FIXTURE_MANIFEST_REF,
    ] {
        assert!(root.join(rel).exists(), "required file must exist: {rel}");
    }
    assert!(
        root.join(RUNTIME_MATERIALIZATION_FIXTURE_DIR).is_dir(),
        "fixture directory must exist"
    );
}

#[test]
fn seeded_instances_validate() {
    for instance in seeded_runtime_instances() {
        validate_runtime_instance(&instance)
            .unwrap_or_else(|err| panic!("instance {} must validate: {err}", instance.instance_id));
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
fn corpus_exercises_every_parity_and_the_wrong_target_case() {
    let fixtures = load_fixtures();
    let parities: BTreeSet<RuntimeParity> = fixtures.iter().map(|f| f.expected_parity).collect();
    for required in RuntimeParity::ALL {
        assert!(
            parities.contains(&required),
            "corpus must exercise {required:?}"
        );
    }
    // A multi-service capsule that ran in the wrong place must be visible.
    let wrong_target = fixtures
        .iter()
        .find(|f| f.scenario == RuntimeScenario::WrongTarget)
        .expect("corpus must include a wrong-target case");
    assert_eq!(wrong_target.expected_parity, RuntimeParity::Mismatched);
    assert!(!wrong_target.expected_target_matched);
}

#[test]
fn materialization_is_metadata_first_on_every_instance() {
    for fixture in load_fixtures() {
        let materialization = materialize_runtime(&fixture.capsule, &fixture.instance);
        assert_eq!(
            materialization.redaction_class,
            RedactionClass::MetadataOnly
        );
        let export = export_runtime_materialization(&materialization);
        assert_eq!(export.redaction_class, RedactionClass::MetadataOnly);
        assert_eq!(
            export.materialization, materialization,
            "support export must wrap the canonical materialization"
        );
        // Secret projections never carry values, only handles.
        for projection in &materialization.instance.secret_projections {
            assert_eq!(projection.handle_ref.len(), 64);
        }
    }
}

#[test]
fn derived_instance_round_trips_the_capsule_target() {
    // The place code runs is explainable in the same vocabulary as the place
    // the environment said it would run: a derived instance is always aligned.
    let fixtures = seeded_runtime_materialization_fixtures();
    for fixture in fixtures
        .iter()
        .filter(|f| f.scenario == RuntimeScenario::Aligned)
    {
        let derived = derive_runtime_instance(&fixture.capsule);
        let materialization = materialize_runtime(&fixture.capsule, &derived);
        assert_eq!(
            materialization.parity,
            RuntimeParity::Aligned,
            "a freshly derived instance must align with its capsule"
        );
        assert!(materialization.target_matched);
    }
}
