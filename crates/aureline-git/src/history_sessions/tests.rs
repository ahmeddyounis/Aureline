use super::*;

const CANONICAL_MAP: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/git/m5/history_sessions/history_session_first_consumers.json"
));

const STASH_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/git/m5/history-sessions/stash_distinct_verbs.json"
));

const PUBLISH_BLOCKED_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/git/m5/history-sessions/publish_blocked_invalidated.json"
));

const CONFLICT_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/git/m5/history-sessions/conflict_reopen_identity.json"
));

fn baseline() -> HistorySessionConsumerMap {
    serde_json::from_str(CANONICAL_MAP).expect("canonical map deserializes")
}

fn session<'a>(map: &'a HistorySessionConsumerMap, id: &str) -> &'a HistorySession {
    map.sessions
        .iter()
        .find(|session| session.session_id == id)
        .expect("session present")
}

fn session_mut<'a>(map: &'a mut HistorySessionConsumerMap, id: &str) -> &'a mut HistorySession {
    map.sessions
        .iter_mut()
        .find(|session| session.session_id == id)
        .expect("session present")
}

fn binding<'a>(
    map: &'a HistorySessionConsumerMap,
    surface: SessionConsumerSurface,
    session_ref: &str,
) -> &'a SessionConsumerBinding {
    map.consumer_bindings
        .iter()
        .find(|binding| binding.surface == surface && binding.session_ref == session_ref)
        .expect("binding present")
}

#[test]
fn checked_artifact_validates() {
    let map = current_history_session_first_consumers_map().expect("checked map validates clean");
    assert_eq!(map.map_id, "git-history-session-first-consumers:0001");
}

#[test]
fn canonical_map_validates_clean() {
    let map = baseline();
    assert!(map.validate().is_empty(), "{:?}", map.validate());
}

#[test]
fn canonical_map_round_trips() {
    let map = baseline();
    let reparsed = HistorySessionConsumerMap::parse_json(&map.export_safe_json())
        .expect("export round-trips through parse_json");
    assert_eq!(map, reparsed);
}

#[test]
fn fixtures_validate() {
    for raw in [STASH_FIXTURE, PUBLISH_BLOCKED_FIXTURE, CONFLICT_FIXTURE] {
        let map = HistorySessionConsumerMap::parse_json(raw).expect("fixture parses and validates");
        assert!(map.validate().is_empty(), "{:?}", map.validate());
    }
}

#[test]
fn every_kind_is_present_in_the_canonical_map() {
    let map = baseline();
    for kind in HistorySurgerySession::ALL {
        assert!(
            map.sessions
                .iter()
                .any(|session| session.session_kind == kind),
            "canonical map missing session kind {}",
            kind.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_is_bound_for_each_session() {
    let map = baseline();
    for session in &map.sessions {
        for surface in SessionConsumerSurface::ALL {
            assert!(
                map.consumer_bindings.iter().any(|binding| {
                    binding.surface == surface && binding.session_ref == session.session_id
                }),
                "missing {} binding for session {}",
                surface.as_str(),
                session.session_id
            );
        }
    }
}

#[test]
fn stash_verbs_stay_distinct_on_every_surface() {
    let map = HistorySessionConsumerMap::parse_json(STASH_FIXTURE).expect("stash fixture parses");
    for binding in map
        .consumer_bindings
        .iter()
        .filter(|binding| binding.session_ref == "stash-0001")
    {
        // Every surface discloses all four distinct verbs.
        for verb in HISTORY_SESSION_STASH_VERBS {
            assert!(
                binding.disclosed_verbs.iter().any(|v| v == verb),
                "{} did not disclose stash verb {verb}",
                binding.surface.as_str()
            );
        }
        // Only mutation surfaces mark them actionable.
        if binding.surface.is_mutation_surface() {
            assert_eq!(binding.actionable_verbs, binding.disclosed_verbs);
        } else {
            assert!(binding.actionable_verbs.is_empty());
        }
    }
}

#[test]
fn publish_ready_allows_network_mutation_only_on_mutation_surfaces() {
    let map = baseline();
    for binding in map
        .consumer_bindings
        .iter()
        .filter(|binding| binding.session_ref == "publish-0001")
    {
        assert_eq!(
            binding.network_mutation_allowed,
            binding.surface.is_mutation_surface(),
            "{} network mutation gate is wrong",
            binding.surface.as_str()
        );
    }
}

#[test]
fn blocked_publish_never_allows_network_mutation() {
    let map = HistorySessionConsumerMap::parse_json(PUBLISH_BLOCKED_FIXTURE)
        .expect("publish-blocked fixture parses");
    assert!(map
        .consumer_bindings
        .iter()
        .all(|binding| !binding.network_mutation_allowed));
}

#[test]
fn identity_and_recovery_survive_every_binding() {
    let map = baseline();
    for binding in &map.consumer_bindings {
        assert!(
            binding.identity_preserved && binding.reopen_safe,
            "binding {} drops identity/reopen safety",
            binding.binding_id
        );
        // Every mutating session keeps a visible recovery path; the checkpoint is
        // itself the recovery surface.
        assert!(
            binding.recovery_visible,
            "binding {} hides recovery",
            binding.binding_id
        );
    }
}

#[test]
fn conflict_preserves_provenance_and_source_text_across_surfaces() {
    let map =
        HistorySessionConsumerMap::parse_json(CONFLICT_FIXTURE).expect("conflict fixture parses");
    let conflict = session(&map, "conflict-0001");
    assert_eq!(conflict.target_refs.len(), 3, "base/ours/theirs preserved");
    assert!(conflict.raw_source_text_ref.is_some());
    assert!(conflict.structured_cards_ref.is_some());
    // Support export and provider overlay never hydrate the raw body.
    for surface in [
        SessionConsumerSurface::SupportExport,
        SessionConsumerSurface::ProviderOverlay,
    ] {
        let b = binding(&map, surface, "conflict-0001");
        assert!(!b.raw_body_export_allowed);
        assert!(b.identity_preserved);
    }
    // An inspection surface may hydrate the raw body.
    assert!(binding(&map, SessionConsumerSurface::Review, "conflict-0001").raw_body_export_allowed);
}

#[test]
fn support_export_covers_every_session() {
    let map = baseline();
    for session in &map.sessions {
        assert!(
            map.support_export
                .session_refs
                .contains(&session.session_id),
            "support export omits {}",
            session.session_id
        );
    }
}

#[test]
fn tampered_binding_fails_validation() {
    let mut map = baseline();
    let target = map
        .consumer_bindings
        .iter_mut()
        .find(|binding| {
            binding.session_ref == "stash-0001" && binding.surface == SessionConsumerSurface::Search
        })
        .expect("search stash binding present");
    // Forge an actionable verb on a read-only surface.
    target.actionable_verbs = vec!["drop".to_owned()];
    let violations = map.validate();
    assert!(violations.iter().any(|error| matches!(
        error,
        HistorySessionValidationError::BindingDoesNotMatchDescriptor { .. }
    )));
}

#[test]
fn network_mutation_without_preconditions_fails() {
    let mut map = baseline();
    // Force the publish proposal into a blocked state but leave a binding
    // claiming the network mutation is allowed.
    session_mut(&mut map, "publish-0001").check_invalidation_state =
        Some("checks_invalidated_blocks_publish".to_owned());
    let violations = map.validate();
    assert!(violations.iter().any(|error| matches!(
        error,
        HistorySessionValidationError::NetworkMutationWithoutPreconditions { .. }
            | HistorySessionValidationError::BindingDoesNotMatchDescriptor { .. }
    )));
}

#[test]
fn missing_stash_verb_fails() {
    let mut map = HistorySessionConsumerMap::parse_json(STASH_FIXTURE).expect("stash fixture");
    session_mut(&mut map, "stash-0001")
        .available_actions
        .retain(|action| action != "create_branch");
    assert!(map.validate().iter().any(|error| matches!(
        error,
        HistorySessionValidationError::StashVerbsNotDistinct { .. }
            | HistorySessionValidationError::MissingRequiredAction { .. }
    )));
}

#[test]
fn conflict_without_source_text_fails() {
    let mut map =
        HistorySessionConsumerMap::parse_json(CONFLICT_FIXTURE).expect("conflict fixture");
    session_mut(&mut map, "conflict-0001").raw_source_text_ref = None;
    assert!(map.validate().iter().any(|error| matches!(
        error,
        HistorySessionValidationError::SourceTextNotPreserved { .. }
    )));
}

#[test]
fn mutating_session_without_recovery_fails() {
    let mut map = baseline();
    let stash = session_mut(&mut map, "stash-0001");
    stash.checkpoint_lineage_refs.clear();
    stash.reflog_only_fallback = false;
    assert!(map.validate().iter().any(|error| matches!(
        error,
        HistorySessionValidationError::MutationMissingRecovery { .. }
    )));
}

#[test]
fn canonical_record_kind_mismatch_fails() {
    let mut map = baseline();
    session_mut(&mut map, "conflict-0001").canonical_record_kind = "wrong_kind".to_owned();
    assert!(map.validate().iter().any(|error| matches!(
        error,
        HistorySessionValidationError::CanonicalRecordKindMismatch { .. }
    )));
}

#[test]
fn duplicate_session_fails() {
    let mut map = baseline();
    let dup = session(&map, "stash-0001").clone();
    map.sessions.push(dup);
    assert!(map.validate().iter().any(|error| matches!(
        error,
        HistorySessionValidationError::DuplicateSessionId { .. }
    )));
}

#[test]
fn unknown_binding_session_fails() {
    let mut map = baseline();
    map.consumer_bindings[0].session_ref = "ghost".to_owned();
    assert!(map.validate().iter().any(|error| matches!(
        error,
        HistorySessionValidationError::UnknownBindingSession { .. }
    )));
}

#[test]
fn support_export_missing_field_fails() {
    let mut map = baseline();
    map.support_export
        .reconstruction_fields
        .retain(|field| field != "checkpoint_lineage_refs");
    assert!(map.validate().iter().any(|error| matches!(
        error,
        HistorySessionValidationError::SupportExportMissingField { .. }
    )));
}

#[test]
fn support_export_unredacted_fails() {
    let mut map = baseline();
    map.support_export.raw_patch_bodies_redacted = false;
    assert!(map
        .validate()
        .contains(&HistorySessionValidationError::SupportExportEmbedsRawMaterial));
}

#[test]
fn raw_boundary_material_in_export_fails() {
    let mut map = baseline();
    session_mut(&mut map, "stash-0001").summary_label = "leak bearer abc123".to_owned();
    assert!(map
        .validate()
        .contains(&HistorySessionValidationError::RawBoundaryMaterialInExport));
}

#[test]
fn markdown_summary_lists_every_session() {
    let summary = baseline().render_markdown_summary();
    for id in [
        "conflict-0001",
        "sequence-0001",
        "stash-0001",
        "publish-0001",
        "checkpoint-0001",
    ] {
        assert!(summary.contains(id), "summary missing session {id}");
    }
}
