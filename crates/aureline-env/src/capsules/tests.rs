use super::*;
use crate::m5_env_governance::{ClaimMaturity, EvidenceState, RowVerdict, WarmStartPosture};

fn local_capsule() -> EnvironmentCapsule {
    seeded_environment_capsules()
        .into_iter()
        .find(|capsule| capsule.identity.capsule_id == "env.capsule.local")
        .expect("local capsule exists")
}

fn container_capsule() -> EnvironmentCapsule {
    seeded_environment_capsules()
        .into_iter()
        .find(|capsule| capsule.identity.capsule_id == "env.capsule.container")
        .expect("container capsule exists")
}

#[test]
fn every_seeded_capsule_validates() {
    for capsule in seeded_environment_capsules() {
        validate_environment_capsule(&capsule).unwrap_or_else(|err| {
            panic!(
                "capsule {} must validate: {err}",
                capsule.identity.capsule_id
            )
        });
    }
}

#[test]
fn seeded_corpus_covers_every_target_class() {
    let fixtures = seeded_environment_capsule_fixtures();
    let mut classes = BTreeSet::new();
    for fixture in &fixtures {
        classes.insert(fixture.target_class);
    }
    for required in CapsuleTargetClass::ALL {
        assert!(
            classes.contains(&required),
            "fixtures must cover target class {}",
            required.as_str()
        );
    }
}

#[test]
fn every_seeded_fixture_validates() {
    for fixture in seeded_environment_capsule_fixtures() {
        validate_environment_capsule_fixture(&fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn current_capsule_certifies_on_inspection() {
    let inspection = inspect_environment(&local_capsule());
    assert_eq!(inspection.verdict, RowVerdict::Certified);
    assert_eq!(inspection.effective_maturity, ClaimMaturity::Stable);
    assert!(inspection.narrow_reason_tokens.is_empty());
    assert!(!inspection.warm_start_downgraded);
    assert_eq!(inspection.reasons.len(), 7);
}

#[test]
fn stale_fingerprint_narrows_and_forces_cold_build() {
    // The marquee guardrail: a stale prebuild fingerprint on the capsule
    // narrows the claim AND drops warm-full-reuse to a cold build.
    let mut capsule = container_capsule();
    capsule.compatibility_fingerprint.coverage = EvidenceState::Stale;
    let inspection = inspect_environment(&capsule);
    assert_eq!(inspection.verdict, RowVerdict::Narrowed);
    assert_eq!(inspection.effective_maturity, ClaimMaturity::Preview);
    assert_eq!(
        inspection.effective_warm_start_posture,
        WarmStartPosture::ColdBuild
    );
    assert!(inspection.warm_start_downgraded);
    assert_eq!(
        inspection.warm_start_downgrade_tokens,
        vec!["prebuild_fingerprint_stale".to_owned()]
    );
}

#[test]
fn ungated_hook_withholds_the_capsule() {
    let mut capsule = local_capsule();
    capsule.trust_hooks[0].gate_state = TrustGateState::Ungated;
    let inspection = inspect_environment(&capsule);
    assert_eq!(inspection.verdict, RowVerdict::Withheld);
    assert_eq!(inspection.effective_maturity, ClaimMaturity::Withdrawn);
    assert!(
        inspection
            .stale_or_missing_dimension_tokens
            .contains(&"trust_hooks".to_owned()),
        "trust_hooks must be flagged missing"
    );
}

#[test]
fn desktop_headless_and_support_share_one_object() {
    // Acceptance: every surface reuses the same inspection object.
    let capsule = container_capsule();
    let desktop = desktop_environment_inspection(&capsule);
    let headless = headless_environment_inspection(&capsule);
    let support = support_environment_inspection(&capsule);
    assert_eq!(desktop, headless, "desktop and headless must be identical");
    assert_eq!(
        support.inspection, desktop,
        "support export must wrap the same inspection object"
    );
}

#[test]
fn export_is_metadata_first_and_carries_no_bodies() {
    let capsule = local_capsule();
    let export = export_capsule_metadata(&capsule);
    assert_eq!(export.redaction_class, RedactionClass::MetadataOnly);
    // Every digest is a 64-hex token, never a body.
    for exported in &export.source_digests {
        assert_eq!(exported.digest.value.len(), 64);
    }
    // Hook commands never cross the boundary — only ids, phases, states.
    assert_eq!(export.trust_hook_states.len(), capsule.trust_hooks.len());
    // Env bindings export names only.
    assert_eq!(export.declared_env_names, vec!["APP_ENV".to_owned()]);
}

#[test]
fn diff_detects_a_digest_and_version_change() {
    let base = local_capsule();
    let mut target = base.clone();
    target.identity.capsule_version = 2;
    target.identity.capsule_digest = CapsuleDigest {
        algorithm: "sha256".to_owned(),
        value: "f".repeat(64),
    };
    if let Some(source) = target.source_refs.first_mut() {
        source.digest = CapsuleDigest {
            algorithm: "sha256".to_owned(),
            value: "a".repeat(64),
        };
    }
    let diff = diff_capsules(&base, &target);
    assert!(!diff.identical);
    let paths: Vec<&str> = diff.changes.iter().map(|c| c.path.as_str()).collect();
    assert!(paths.contains(&"identity.capsule_version"));
    assert!(paths.contains(&"identity.capsule_digest"));
    assert!(paths.iter().any(|p| p.starts_with("source_refs.")));
}

#[test]
fn diff_of_identical_capsules_is_empty() {
    let capsule = container_capsule();
    let diff = diff_capsules(&capsule, &capsule);
    assert!(diff.identical);
    assert!(diff.changes.is_empty());
}

#[test]
fn capsule_round_trips_through_json() {
    let capsule = container_capsule();
    let json = serde_json::to_string(&capsule).expect("capsule serializes");
    let back: EnvironmentCapsule = serde_json::from_str(&json).expect("capsule deserializes");
    assert_eq!(capsule, back);
}

#[test]
fn fixtures_cover_certified_narrowed_and_withheld() {
    let fixtures = seeded_environment_capsule_fixtures();
    let mut verdicts = BTreeSet::new();
    let mut saw_warm_downgrade = false;
    for fixture in &fixtures {
        verdicts.insert(fixture.expected_verdict);
        if !fixture.expected_warm_start_downgrade_tokens.is_empty() {
            saw_warm_downgrade = true;
        }
    }
    for required in [
        RowVerdict::Certified,
        RowVerdict::Narrowed,
        RowVerdict::Withheld,
    ] {
        assert!(
            verdicts.contains(&required),
            "fixtures must cover {required:?}"
        );
    }
    assert!(
        saw_warm_downgrade,
        "fixtures must cover a warm-start downgrade"
    );
}
