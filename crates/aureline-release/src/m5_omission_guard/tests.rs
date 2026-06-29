//! Inline tests for the M5 omission-guard lane.

use super::*;

fn registry() -> M5OmissionGuardRegistry {
    seeded_m5_omission_guard_registry()
}

#[test]
fn canonical_registry_validates() {
    let registry = registry();
    assert!(registry.validate().is_empty(), "{:?}", registry.validate());
    assert_eq!(registry.registry_id, M5_OMISSION_GUARD_REGISTRY_ID);
    assert_eq!(registry.record_kind, M5_OMISSION_GUARD_REGISTRY_RECORD_KIND);
    assert_eq!(registry.schema_version, M5_OMISSION_GUARD_SCHEMA_VERSION);
    assert_eq!(registry.cases.len(), 9);
    assert!(registry.conformance.all_hold());
    assert!(registry.vocabulary.matches_canonical());
}

#[test]
fn every_case_validates_and_has_non_empty_present_set() {
    for case in registry().cases {
        assert!(
            case.validate().is_empty(),
            "{}: {:?}",
            case.case_id,
            case.validate()
        );
        assert!(
            !case.present_states.is_empty(),
            "{} has an empty present set",
            case.case_id
        );
        assert!(case.guard.all_hold(), "{} guard failed", case.case_id);
    }
}

#[test]
fn present_states_are_in_canonical_order() {
    for case in registry().cases {
        let positions: Vec<usize> = case
            .present_states
            .iter()
            .map(|r| {
                WeakerEvidenceState::ALL
                    .iter()
                    .position(|s| *s == r.state)
                    .unwrap()
            })
            .collect();
        let mut sorted = positions.clone();
        sorted.sort_unstable();
        assert_eq!(positions, sorted, "{} not in canonical order", case.case_id);
    }
}

fn case_states(registry: &M5OmissionGuardRegistry, case_id: &str) -> Vec<WeakerEvidenceState> {
    registry
        .case(case_id)
        .unwrap_or_else(|| panic!("missing case {case_id}"))
        .present_states
        .iter()
        .map(|r| r.state)
        .collect()
}

#[test]
fn mirror_offline_side_loaded_surface_when_present() {
    let registry = registry();
    assert_eq!(
        case_states(&registry, "omission-guard:mirrored"),
        vec![WeakerEvidenceState::Mirrored]
    );
    assert_eq!(
        case_states(&registry, "omission-guard:offline"),
        vec![WeakerEvidenceState::Offline]
    );
    assert_eq!(
        case_states(&registry, "omission-guard:side-loaded"),
        vec![
            WeakerEvidenceState::SideLoaded,
            WeakerEvidenceState::Unverified
        ]
    );
    assert!(registry.conformance.mirror_offline_side_loaded_first_class);
}

#[test]
fn official_anchor_is_explicit_and_unweakened() {
    let registry = registry();
    let official = registry.case("omission-guard:official").unwrap();
    assert_eq!(
        official
            .present_states
            .iter()
            .map(|r| r.state)
            .collect::<Vec<_>>(),
        vec![WeakerEvidenceState::Official]
    );
    assert!(official.is_fully_official());
    assert!(!official.weakening_present);
    assert_eq!(official.claim_state, NarrowedClaimState::FullySupported);
    assert!(registry.conformance.official_anchor_explicit);
}

#[test]
fn official_anchor_coexists_with_weakening() {
    // A first-party signed but stale surface still states `official` alongside `stale`.
    let registry = registry();
    let stale = registry.case("omission-guard:stale").unwrap();
    assert!(stale
        .present_states
        .iter()
        .any(|r| r.state == WeakerEvidenceState::Official));
    assert!(stale
        .present_states
        .iter()
        .any(|r| r.state == WeakerEvidenceState::Stale));
    assert!(stale.weakening_present);
}

#[test]
fn not_provided_and_partial_surface() {
    let registry = registry();
    let blocked = registry
        .case("omission-guard:not-provided-blocked")
        .unwrap();
    assert!(blocked
        .present_states
        .iter()
        .any(|r| r.state == WeakerEvidenceState::NotProvided));
    assert!(blocked
        .present_states
        .iter()
        .any(|r| r.state == WeakerEvidenceState::Missing));
    assert!(registry.conformance.not_provided_never_hidden);
    assert!(registry.conformance.partial_states_surfaced);
    assert!(case_states(&registry, "omission-guard:partial-evidence")
        .contains(&WeakerEvidenceState::Partial));
}

#[test]
fn weakening_aligns_with_claim_narrowing_in_every_case() {
    for case in registry().cases {
        assert_eq!(
            case.weakening_present,
            !case.claim_state.is_fully_supported(),
            "{} weakening/claim mismatch",
            case.case_id
        );
    }
    assert!(registry().conformance.weakening_aligns_with_claim_narrowing);
}

#[test]
fn every_consumer_renders_the_same_present_set() {
    for case in registry().cases {
        let projected: Vec<PublicTruthConsumer> = case
            .consumer_projections
            .iter()
            .map(|p| p.consumer)
            .collect();
        assert_eq!(projected, PublicTruthConsumer::ALL.to_vec());
        for projection in &case.consumer_projections {
            let rendered: Vec<WeakerEvidenceState> =
                projection.rendered_states.iter().map(|r| r.state).collect();
            let present: Vec<WeakerEvidenceState> =
                case.present_states.iter().map(|r| r.state).collect();
            assert_eq!(rendered, present, "{} consumer diverged", case.case_id);
            assert!(projection.omits_no_present_state);
        }
    }
}

#[test]
fn labels_and_explanations_are_one_vocabulary_across_consumers() {
    for case in registry().cases {
        for projection in &case.consumer_projections {
            for rendered in &projection.rendered_states {
                assert_eq!(rendered.label, rendered.state.label());
                assert_eq!(
                    rendered.explanation_message_id,
                    format!(
                        "{M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX}omission.state.{}.explanation",
                        rendered.state.as_str()
                    )
                );
            }
        }
    }
    assert!(registry().conformance.one_vocabulary_across_consumers);
}

#[test]
fn dropping_a_present_state_from_a_consumer_is_a_silent_omission() {
    let mut case = seeded_mirrored_case();
    assert!(case.validate().is_empty());
    // Hand-edit one consumer to drop the mirrored state — the guard must catch it.
    case.consumer_projections[2].rendered_states.clear();
    case.consumer_projections[2].omits_no_present_state = true;
    let violations = case.validate();
    assert!(
        violations.contains(&M5OmissionGuardViolation::SilentOmission),
        "{violations:?}"
    );
}

#[test]
fn inventing_an_absent_state_on_a_consumer_is_caught() {
    let mut case = seeded_official_case();
    assert!(case.validate().is_empty());
    let invented = RenderedState::new(
        WeakerEvidenceState::Mirrored,
        vec!["source_class:mirror".to_owned()],
    );
    case.consumer_projections[0].rendered_states.push(invented);
    let violations = case.validate();
    assert!(
        violations.contains(&M5OmissionGuardViolation::StateInvented),
        "{violations:?}"
    );
}

#[test]
fn relabeling_a_state_breaks_the_shared_vocabulary() {
    let mut case = seeded_mirrored_case();
    case.consumer_projections[1].rendered_states[0].label = "From a mirror (maybe ok)".to_owned();
    let violations = case.validate();
    assert!(
        violations.contains(&M5OmissionGuardViolation::VocabularyDrift),
        "{violations:?}"
    );
}

#[test]
fn vocabulary_is_frozen() {
    let vocab = OmissionGuardVocabulary::canonical();
    assert!(vocab.matches_canonical());
    assert_eq!(vocab.states.len(), WeakerEvidenceState::ALL.len());
    assert_eq!(vocab.consumers.len(), PublicTruthConsumer::ALL.len());
    for needle in [
        "official",
        "mirrored",
        "offline",
        "side_loaded",
        "not_provided",
        "partial",
    ] {
        assert!(
            vocab.states.iter().any(|s| s.state == needle),
            "missing {needle}"
        );
    }
    // The official anchor is the only non-weakening entry.
    assert_eq!(vocab.states.iter().filter(|s| !s.is_weakening).count(), 1);
    let official = vocab.states.iter().find(|s| s.state == "official").unwrap();
    assert!(!official.is_weakening);
}

#[test]
fn export_carries_no_raw_material() {
    let json = registry().export_safe_json();
    for needle in [
        "credential",
        "secret",
        "password",
        "api_key",
        "raw_payload",
        "bearer_token",
    ] {
        assert!(!json.contains(needle), "found {needle} in export");
    }
}

#[test]
fn registry_round_trips_through_json() {
    let registry = registry();
    let json = registry.export_safe_json();
    let restored: M5OmissionGuardRegistry = serde_json::from_str(&json).unwrap();
    assert_eq!(registry, restored);
    assert!(restored.validate().is_empty());
}

#[test]
fn case_round_trips_through_json() {
    for case in registry().cases {
        let json = case.export_safe_json();
        let restored: OmissionGuardCase = serde_json::from_str(&json).unwrap();
        assert_eq!(case, restored);
    }
}

#[test]
fn summary_counts_match() {
    let registry = registry();
    let s = &registry.summary;
    assert_eq!(s.total_cases, 9);
    assert_eq!(s.fully_official_cases, 1);
    assert_eq!(s.cases_with_weakening, 8);
    // Every case projects its present set onto all eight consumers.
    let expected_renderings: u32 = registry
        .cases
        .iter()
        .map(|c| c.present_states.len() as u32 * PublicTruthConsumer::ALL.len() as u32)
        .sum();
    assert_eq!(s.total_state_renderings, expected_renderings);
    assert!(s.distinct_states_exercised >= 9);
}

#[test]
fn markdown_summary_is_deterministic() {
    let registry = registry();
    assert_eq!(
        registry.render_markdown_summary(),
        registry.render_markdown_summary()
    );
    assert!(registry
        .render_markdown_summary()
        .contains("no-silent-omission guard parity"));
}
