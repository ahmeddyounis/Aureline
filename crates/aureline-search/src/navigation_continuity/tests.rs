use super::*;

#[test]
fn seeded_canonical_packet_validates() {
    let packet = seeded_navigation_continuity_packet();
    assert_eq!(
        packet.record_kind,
        NAVIGATION_CONTINUITY_BINDING_PACKET_RECORD_KIND
    );
    assert_eq!(packet.packet_id, NAVIGATION_CONTINUITY_BINDING_PACKET_ID);
    let findings = packet.validate();
    assert!(findings.is_empty(), "unexpected findings: {findings:?}");
    assert!(packet.is_export_safe());
}

#[test]
fn covers_every_surface_once() {
    let packet = seeded_navigation_continuity_packet();
    assert_eq!(packet.surfaces.len(), REQUIRED_CONTINUITY_SURFACES.len());
    for surface in REQUIRED_CONTINUITY_SURFACES {
        assert!(packet.surface_for(surface).is_some(), "missing {surface:?}");
    }
}

#[test]
fn realizes_all_artifact_kinds_and_drift_states() {
    let packet = seeded_navigation_continuity_packet();
    for kind in COVERED_ARTIFACT_KINDS {
        assert!(
            packet
                .realized_artifact_kind_tokens()
                .contains(&kind.as_str()),
            "missing artifact kind {}",
            kind.as_str()
        );
    }
    for state in REQUIRED_DRIFT_STATES {
        assert!(
            packet
                .realized_drift_state_tokens()
                .contains(&state.as_str()),
            "missing drift state {}",
            state.as_str()
        );
    }
}

#[test]
fn realizes_back_forward_and_recent_history() {
    // Acceptance: back/forward history preserves origin/destination refs and is
    // a real, attributable set of states — not collapsed into a single bucket.
    let packet = seeded_navigation_continuity_packet();
    for role in HistoryRole::ALL {
        assert!(
            packet
                .realized_history_role_tokens()
                .contains(&role.as_str()),
            "missing history role {}",
            role.as_str()
        );
    }
    for artifact in packet.all_artifacts().filter(|a| a.is_history_entry()) {
        assert!(artifact.history_role.is_some());
        let origin = artifact
            .origin_target_ref
            .as_ref()
            .expect("history keeps an origin anchor");
        assert_ne!(origin, &artifact.canonical_target_ref);
    }
}

#[test]
fn drift_states_stay_visible_and_recoverable() {
    // Acceptance: drifted, missing-target, scope-unavailable, and archived
    // artifacts never carry a resolved target and always keep a visible reason
    // plus recovery choices — never a silent relocation.
    let packet = seeded_navigation_continuity_packet();
    let unresolved: Vec<_> = packet
        .all_artifacts()
        .filter(|a| a.requires_visible_reason())
        .collect();
    assert!(!unresolved.is_empty());
    for artifact in unresolved {
        assert!(artifact.resolved_target_ref.is_none());
        assert!(artifact
            .drift_reason
            .as_ref()
            .is_some_and(|reason| !reason.trim().is_empty()));
        assert!(!artifact.recovery_choices.is_empty());
        assert!(!artifact.used_nearby_fallback);
    }
}

#[test]
fn bookmarks_bind_first_and_remap_with_stable_evidence_only() {
    // Guardrail: a remap follows stable evidence to a different target and never
    // a nearest-target fallback.
    let packet = seeded_navigation_continuity_packet();
    let remapped: Vec<_> = packet
        .all_artifacts()
        .filter(|a| a.drift_state == NavigationDriftState::Remapped)
        .collect();
    assert!(!remapped.is_empty());
    for artifact in remapped {
        assert!(!artifact.used_nearby_fallback);
        assert!(!artifact.remap_evidence_refs.is_empty());
        let resolved = artifact
            .resolved_target_ref
            .as_ref()
            .expect("remapped artifact resolves");
        assert_ne!(resolved, &artifact.canonical_target_ref);
    }
}

#[test]
fn result_bearing_surfaces_bind_to_durable_identities() {
    let packet = seeded_navigation_continuity_packet();
    for surface in [
        NavigationContinuitySurface::Search,
        NavigationContinuitySurface::Docs,
        NavigationContinuitySurface::Topology,
    ] {
        for artifact in &packet.surface_for(surface).unwrap().artifacts {
            let result_id = artifact
                .result_id_ref
                .as_ref()
                .unwrap_or_else(|| panic!("{surface:?} artifact must bind a result id"));
            assert!(result_id.contains(':'));
            assert!(result_id.parse::<u64>().is_err());
        }
    }
}

#[test]
fn restore_preserves_unresolved_artifacts_with_reasons() {
    // Acceptance: restore reopens continuity artifacts with visible
    // drift/missing-target reasons instead of dropping them.
    let packet = seeded_navigation_continuity_packet();
    let preserved = packet.restore.preserved_unresolved_count();
    assert!(
        preserved >= 3,
        "expected several preserved drifted artifacts"
    );
    for restored in &packet.restore.artifacts {
        // Every restore row points at a real artifact in the packet.
        assert!(packet.artifact_for(&restored.artifact_id_ref).is_some());
        if !restored.target_resolves_under_current_scope {
            assert!(restored.artifact_preserved);
            assert!(restored
                .restore_reason
                .as_ref()
                .is_some_and(|reason| !reason.trim().is_empty()));
            assert!(!restored.recovery_choices.is_empty());
        }
    }
}

#[test]
fn all_four_consumers_reuse_one_continuity_object() {
    // Acceptance: search, docs, graph, notebook, and diff surfaces share one
    // continuity vocabulary and export model across the first consumers.
    let packet = seeded_navigation_continuity_packet();
    for required in ContinuityConsumerClass::ALL {
        let projection = packet
            .consumer_projections
            .iter()
            .find(|projection| projection.consumer == required)
            .unwrap_or_else(|| panic!("missing consumer {}", required.as_str()));
        assert_eq!(projection.ingested_packet_id, packet.packet_id);
        assert!(projection.preserves_export_safe_ids);
        assert!(projection.preserves_drift_vocabulary);
        assert!(projection.preserves_drift_reasons);
        assert!(projection.preserves_origin_destination);
        assert!(projection.reuses_same_continuity_objects);
        assert!(!projection.widens_authority);
    }
}

#[test]
fn workset_drift_variant_drifts_more_but_preserves_identity_and_vocabulary() {
    let canonical = seeded_navigation_continuity_packet();
    let drifted = seeded_workset_drift_navigation_continuity_packet();
    assert!(drifted.validate().is_empty());
    assert!(drifted.is_export_safe());

    // The full surface, artifact-kind, and drift-state vocabulary is preserved.
    assert_eq!(
        canonical.realized_surface_tokens(),
        drifted.realized_surface_tokens()
    );
    assert_eq!(
        canonical.realized_artifact_kind_tokens(),
        drifted.realized_artifact_kind_tokens()
    );
    assert_eq!(
        canonical.realized_drift_state_tokens(),
        drifted.realized_drift_state_tokens()
    );

    // Under the narrowed workset, strictly more artifacts drift.
    assert!(
        drifted.unresolved_artifact_count() > canonical.unresolved_artifact_count(),
        "narrowed workset should drift more"
    );

    // The bookmark identity is preserved; only the drift state changed.
    let canonical_bookmark = canonical
        .artifact_for(&format!(
            "{NAVIGATION_CONTINUITY_BINDING_PACKET_ID}:search:bookmark"
        ))
        .unwrap();
    let drifted_bookmark = drifted
        .artifact_for(&format!(
            "{NAVIGATION_CONTINUITY_BINDING_PACKET_ID}:search:bookmark"
        ))
        .unwrap();
    assert_eq!(canonical_bookmark.drift_state, NavigationDriftState::Bound);
    assert_eq!(drifted_bookmark.drift_state, NavigationDriftState::Drifted);
    assert_eq!(
        canonical_bookmark.result_id_ref,
        drifted_bookmark.result_id_ref
    );
    assert_eq!(
        canonical_bookmark.canonical_target_ref,
        drifted_bookmark.canonical_target_ref
    );
    // The drifted bookmark never relocates and stays recoverable.
    assert!(drifted_bookmark.resolved_target_ref.is_none());
    assert!(!drifted_bookmark.used_nearby_fallback);
    assert!(!drifted_bookmark.recovery_choices.is_empty());

    // The drifted bookmark survives restore with a visible reason.
    let restored = drifted
        .restore
        .artifacts
        .iter()
        .find(|row| {
            row.artifact_id_ref
                == format!("{NAVIGATION_CONTINUITY_BINDING_PACKET_ID}:search:bookmark")
        })
        .expect("workset-drift restore preserves the search bookmark");
    assert!(!restored.target_resolves_under_current_scope);
    assert!(restored.artifact_preserved);
    assert!(restored.restore_reason.is_some());
}

#[test]
fn checked_in_packet_matches_seeded_canonical() {
    let checked =
        current_navigation_continuity_packet().expect("checked-in packet parses and validates");
    assert_eq!(checked, seeded_navigation_continuity_packet());
}

#[test]
fn checked_in_workset_drift_matches_seeded() {
    let checked = current_workset_drift_navigation_continuity_packet()
        .expect("checked-in workset-drift packet parses and validates");
    assert_eq!(checked, seeded_workset_drift_navigation_continuity_packet());
}

#[test]
fn support_export_preserves_the_packet_safely() {
    let packet = seeded_navigation_continuity_packet();
    let export = packet.support_export("navigation-continuity-export-1", "2026-06-17T00:00:00Z");
    assert!(export.is_export_safe());
    assert_eq!(export.navigation_continuity_packet, packet);
}

#[test]
fn detects_silent_nearby_relocation() {
    // Guardrail: a nearby-target fallback is never allowed.
    let mut packet = seeded_navigation_continuity_packet();
    packet.surfaces[0].artifacts[1].used_nearby_fallback = true;
    let findings = packet.validate();
    assert!(findings.iter().any(|finding| finding
        .message
        .contains("never relocate to a nearby target")));
}

#[test]
fn detects_dropped_drift_reason() {
    let mut packet = seeded_navigation_continuity_packet();
    let artifact = packet
        .surfaces
        .iter_mut()
        .flat_map(|surface| &mut surface.artifacts)
        .find(|artifact| artifact.requires_visible_reason())
        .unwrap();
    artifact.drift_reason = None;
    let findings = packet.validate();
    assert!(findings
        .iter()
        .any(|finding| finding.message.contains("must keep a visible reason")));
}

#[test]
fn detects_resolved_target_on_missing_artifact() {
    let mut packet = seeded_navigation_continuity_packet();
    let artifact = packet
        .surfaces
        .iter_mut()
        .flat_map(|surface| &mut surface.artifacts)
        .find(|artifact| artifact.drift_state == NavigationDriftState::MissingTarget)
        .unwrap();
    artifact.resolved_target_ref = Some("docs:anchor:something-nearby".to_owned());
    let findings = packet.validate();
    assert!(findings.iter().any(|finding| finding
        .message
        .contains("must not carry a resolved target ref")));
}

#[test]
fn detects_restore_dropping_a_drifted_artifact() {
    let mut packet = seeded_navigation_continuity_packet();
    let restored = packet
        .restore
        .artifacts
        .iter_mut()
        .find(|row| !row.target_resolves_under_current_scope)
        .unwrap();
    restored.artifact_preserved = false;
    let findings = packet.validate();
    assert!(findings.iter().any(|finding| finding
        .message
        .contains("must preserve a non-resolving artifact instead of dropping it")));
}

#[test]
fn detects_history_role_on_non_history_artifact() {
    let mut packet = seeded_navigation_continuity_packet();
    let artifact = packet
        .surfaces
        .iter_mut()
        .flat_map(|surface| &mut surface.artifacts)
        .find(|artifact| !artifact.is_history_entry())
        .unwrap();
    artifact.history_role = Some(HistoryRole::Back);
    let findings = packet.validate();
    assert!(findings.iter().any(|finding| finding
        .message
        .contains("only history entries may declare a history role")));
}

#[test]
fn detects_result_identity_collapsed_into_label() {
    let mut packet = seeded_navigation_continuity_packet();
    let artifact = packet
        .surfaces
        .iter_mut()
        .flat_map(|surface| &mut surface.artifacts)
        .find(|artifact| artifact.result_id_ref.is_some() && artifact.label.is_some())
        .unwrap();
    artifact.label = artifact.result_id_ref.clone();
    let findings = packet.validate();
    assert!(findings
        .iter()
        .any(|finding| finding.message.contains("collapse into the display label")));
}
