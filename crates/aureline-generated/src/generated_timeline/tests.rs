use super::*;

#[test]
fn full_unredacted_snapshot_claims_exact_continuity() {
    let outcome = classify_generated_history(
        CaptureMode::FullSnapshot,
        RedactionClass::Unredacted,
        DriftState::InSync,
    );
    assert_eq!(outcome.restore_fidelity, RestoreFidelity::ExactSnapshot);
    assert!(outcome.exact_byte_continuity_claimed);
    assert_eq!(outcome.compare_basis, CompareBasis::ByteSnapshot);
    assert_eq!(outcome.restore_availability, RestoreAvailability::Available);
    assert!(outcome.writes_new_checkpoint_on_restore);
    assert!(outcome.block_reason_tokens.is_empty());
    assert!(outcome.byte_provenance.captured_directly);
    assert!(!outcome.byte_provenance.reconstructed_from_source);
}

#[test]
fn metadata_plus_reference_never_claims_exact_continuity() {
    // The marquee guardrail: a metadata-plus-reference capture restores by
    // regeneration and can never claim exact byte continuity.
    let outcome = classify_generated_history(
        CaptureMode::MetadataPlusReference,
        RedactionClass::Unredacted,
        DriftState::InSync,
    );
    assert_eq!(
        outcome.restore_fidelity,
        RestoreFidelity::CompatibleRegeneration
    );
    assert!(!outcome.exact_byte_continuity_claimed);
    assert_ne!(outcome.compare_basis, CompareBasis::ByteSnapshot);
    assert_eq!(
        outcome.restore_availability,
        RestoreAvailability::ReviewRequired
    );
    assert!(outcome
        .block_reason_tokens
        .contains(&"capture_metadata_plus_reference".to_owned()));
    assert!(outcome.byte_provenance.reconstructed_from_source);
    assert!(!outcome.byte_provenance.captured_directly);
}

#[test]
fn redacted_full_snapshot_drops_exact_continuity() {
    let outcome = classify_generated_history(
        CaptureMode::FullSnapshot,
        RedactionClass::SecretsRedacted,
        DriftState::InSync,
    );
    assert_eq!(
        outcome.restore_fidelity,
        RestoreFidelity::CompatibleRegeneration
    );
    assert!(!outcome.exact_byte_continuity_claimed);
    assert!(outcome.byte_provenance.redacted);
    assert!(outcome
        .block_reason_tokens
        .contains(&"redaction_secrets".to_owned()));
}

#[test]
fn full_snapshot_ignores_source_drift() {
    // A full snapshot holds the original bytes locally, so a drifting or
    // missing canonical source does not weaken its exact restore.
    for drift in [
        DriftState::Drifting,
        DriftState::SourceMissing,
        DriftState::Unknown,
    ] {
        let source = if drift == DriftState::SourceMissing {
            // a source-missing divergence still implies the bytes are local.
            DriftState::SourceMissing
        } else {
            drift
        };
        let outcome = classify_generated_history(
            CaptureMode::FullSnapshot,
            RedactionClass::Unredacted,
            source,
        );
        assert_eq!(
            outcome.restore_fidelity,
            RestoreFidelity::ExactSnapshot,
            "full snapshot must stay exact under drift {source:?}"
        );
        assert!(outcome.exact_byte_continuity_claimed);
        // Drift block tokens only apply on the regeneration path.
        assert!(outcome.block_reason_tokens.is_empty());
    }
}

#[test]
fn omitted_bytes_are_evidence_only() {
    let outcome = classify_generated_history(
        CaptureMode::OmittedBytes,
        RedactionClass::Unredacted,
        DriftState::InSync,
    );
    assert_eq!(outcome.restore_fidelity, RestoreFidelity::EvidenceOnly);
    assert!(!outcome.exact_byte_continuity_claimed);
    assert_eq!(outcome.compare_basis, CompareBasis::EvidenceManifest);
    assert_eq!(
        outcome.restore_availability,
        RestoreAvailability::DisabledExportOnly
    );
    assert!(!outcome.writes_new_checkpoint_on_restore);
    assert!(outcome.byte_provenance.bytes_omitted);
}

#[test]
fn missing_source_floors_regeneration_to_evidence_only() {
    let outcome = classify_generated_history(
        CaptureMode::MetadataPlusReference,
        RedactionClass::Unredacted,
        DriftState::SourceMissing,
    );
    assert_eq!(outcome.restore_fidelity, RestoreFidelity::EvidenceOnly);
    assert!(outcome
        .block_reason_tokens
        .contains(&"regeneration_source_missing".to_owned()));
}

#[test]
fn policy_withheld_floors_to_evidence_only() {
    let outcome = classify_generated_history(
        CaptureMode::MetadataPlusReference,
        RedactionClass::PolicyWithheld,
        DriftState::InSync,
    );
    assert_eq!(outcome.restore_fidelity, RestoreFidelity::EvidenceOnly);
    assert!(!outcome.exact_byte_continuity_claimed);
}

#[test]
fn fidelity_only_narrows_under_compounding_floors() {
    // Regenerated candidate (compatible) plus a size cap (compatible) stays
    // compatible; adding a missing source narrows to evidence only.
    let compatible = classify_generated_history(
        CaptureMode::RegeneratedCandidate,
        RedactionClass::SizeCapped,
        DriftState::Drifting,
    );
    assert_eq!(
        compatible.restore_fidelity,
        RestoreFidelity::CompatibleRegeneration
    );
}

#[test]
fn seeded_packet_validates_and_covers_modes_and_fidelities() {
    let packet = seeded_generated_timeline_packet();
    validate_generated_timeline_packet(&packet)
        .expect("seeded packet must satisfy the frozen contract");

    let captures: BTreeSet<_> = packet.entries.iter().map(|e| e.capture_mode).collect();
    for required in CaptureMode::ALL {
        assert!(captures.contains(&required), "missing capture {required:?}");
    }
    let fidelities: BTreeSet<_> = packet
        .entries
        .iter()
        .map(|e| e.outcome.restore_fidelity)
        .collect();
    for required in RestoreFidelity::ALL {
        assert!(
            fidelities.contains(&required),
            "missing fidelity {required:?}"
        );
    }
}

#[test]
fn seeded_fixtures_validate_and_cover_the_guardrail() {
    let fixtures = seeded_generated_timeline_fixtures();
    assert!(!fixtures.is_empty());
    let mut saw_exact = false;
    let mut saw_blocked_exact = false;
    let mut fidelities = BTreeSet::new();
    for fixture in &fixtures {
        validate_generated_timeline_entry_fixture(fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
        fidelities.insert(fixture.expected_restore_fidelity);
        if fixture.expected_exact_byte_continuity_claimed {
            saw_exact = true;
            assert_eq!(
                fixture.entry.capture_mode,
                CaptureMode::FullSnapshot,
                "only a full snapshot may claim exact continuity"
            );
            assert_eq!(fixture.entry.redaction_class, RedactionClass::Unredacted);
        } else if fixture.entry.capture_mode != CaptureMode::FullSnapshot {
            saw_blocked_exact = true;
        }
    }
    for required in RestoreFidelity::ALL {
        assert!(
            fidelities.contains(&required),
            "fixtures must cover {required:?}"
        );
    }
    assert!(saw_exact, "fixtures must cover an exact-continuity entry");
    assert!(
        saw_blocked_exact,
        "fixtures must cover a withheld exact-continuity entry"
    );
}

#[test]
fn copy_line_is_stable_and_self_consistent() {
    let packet = seeded_generated_timeline_packet();
    for entry in &packet.entries {
        assert_eq!(entry.copy_line, timeline_copy_line(entry));
        assert!(entry.export_projection.is_export_safe());
    }
}

#[test]
fn packet_round_trips_through_json() {
    let packet = seeded_generated_timeline_packet();
    let json = serde_json::to_string(&packet).expect("packet serializes");
    let back: GeneratedTimelinePacket = serde_json::from_str(&json).expect("packet deserializes");
    assert_eq!(packet, back);
}

#[test]
fn fixtures_round_trip_through_json() {
    for fixture in seeded_generated_timeline_fixtures() {
        let json = serde_json::to_string(&fixture).expect("fixture serializes");
        let back: GeneratedTimelineEntryFixture =
            serde_json::from_str(&json).expect("fixture deserializes");
        assert_eq!(fixture, back);
    }
}
