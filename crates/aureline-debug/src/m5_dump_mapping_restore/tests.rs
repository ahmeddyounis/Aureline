//! Unit tests for the M5 dump/mapping/restore set.

use super::*;

#[test]
fn canonical_set_validates_and_is_export_safe() {
    let set = m5_dump_mapping_restore_set();
    set.validate().expect("canonical set validates");
    assert!(set.is_support_export_safe());
    assert!(set.all_invariants_hold());
    assert!(set.raw_payload_excluded);
}

#[test]
fn canonical_set_round_trips_through_serde() {
    let set = m5_dump_mapping_restore_set();
    let json = serde_json::to_string(&set).expect("serializes");
    let back: DumpMappingRestoreSet = serde_json::from_str(&json).expect("round-trips");
    assert_eq!(set, back);
}

#[test]
fn every_mapping_fidelity_is_materialized() {
    let set = m5_dump_mapping_restore_set();
    for fidelity in DebugMappingFidelity::ALL {
        assert!(
            set.artifact_in_fidelity(fidelity).is_some(),
            "missing artifact for fidelity {}",
            fidelity.as_str()
        );
    }
}

#[test]
fn every_kind_entrypoint_and_source_is_materialized() {
    let set = m5_dump_mapping_restore_set();
    for kind in DebugArtifactKind::ALL {
        assert!(
            set.artifacts.iter().any(|a| a.artifact_kind == kind),
            "missing kind {}",
            kind.as_str()
        );
    }
    for entrypoint in DebugArtifactEntrypoint::ALL {
        assert!(
            set.artifacts.iter().any(|a| a.entrypoint == entrypoint),
            "missing entrypoint {}",
            entrypoint.as_str()
        );
    }
    for source in ArtifactSourceClass::ALL {
        assert!(
            set.artifacts.iter().any(|a| a.source_class() == source),
            "missing source class {}",
            source.as_str()
        );
    }
}

#[test]
fn every_restore_posture_is_materialized() {
    let set = m5_dump_mapping_restore_set();
    for posture in RestorePosture::ALL {
        assert!(
            set.restored_layout_in_posture(posture).is_some(),
            "missing restore posture {}",
            posture.as_str()
        );
    }
}

#[test]
fn only_exact_with_verified_build_shows_precise_source_link() {
    let set = m5_dump_mapping_restore_set();
    for a in &set.artifacts {
        let expected =
            a.fidelity().preserves_exact_source() && a.build_match().proves_exact_build();
        assert_eq!(
            a.pill.shows_exact_source_link, expected,
            "strip {} exact link disagrees with its evidence",
            a.strip_id
        );
        assert_eq!(a.pill.requires_disclosure, !a.pill.shows_exact_source_link);
    }
}

#[test]
fn imported_and_mismatch_never_show_exact_link() {
    let set = m5_dump_mapping_restore_set();
    for a in &set.artifacts {
        if a.fidelity().is_imported() {
            assert!(
                a.source_class().is_imported(),
                "imported needs imported source"
            );
            assert!(!a.pill.shows_exact_source_link);
        }
        if a.fidelity().is_build_mismatch() {
            assert_eq!(a.build_match(), ArtifactBuildMatch::MismatchedRejected);
            assert!(!a.pill.shows_exact_source_link);
        }
    }
}

#[test]
fn session_entrypoints_are_distinct_and_inspect_only() {
    let set = m5_dump_mapping_restore_set();
    for entrypoint in DebugArtifactEntrypoint::SESSION_ENTRYPOINTS {
        let matching: Vec<&DebugArtifactStrip> = set
            .artifacts
            .iter()
            .filter(|a| a.entrypoint == entrypoint)
            .collect();
        assert!(
            !matching.is_empty(),
            "missing session entrypoint {}",
            entrypoint.as_str()
        );
        assert!(matching.iter().all(|a| a.opens_inspect_only_session));
    }
    // The import entrypoint never opens a session.
    for a in set
        .artifacts
        .iter()
        .filter(|a| a.entrypoint == DebugArtifactEntrypoint::ImportSymbolsOrSourceMap)
    {
        assert!(!a.opens_inspect_only_session);
    }
}

#[test]
fn restored_layouts_never_imply_live_authority() {
    let set = m5_dump_mapping_restore_set();
    for r in &set.restored_layouts {
        assert!(
            !r.pill.implies_live_continuity,
            "{} implies live continuity",
            r.layout_id
        );
        assert!(
            !r.pill.implies_process_authority,
            "{} implies process authority",
            r.layout_id
        );
        assert!(r.pill.requires_disclosure);
        let expected_exact = r.fidelity().preserves_exact_source() && r.exact_build_still_verified;
        assert_eq!(r.pill.implies_exact_build_mapping, expected_exact);
    }
}

#[test]
fn restored_layouts_reference_real_strips() {
    let set = m5_dump_mapping_restore_set();
    for r in &set.restored_layouts {
        assert!(
            set.artifact(&r.restored_strip_ref).is_some(),
            "{} references unknown strip {}",
            r.layout_id,
            r.restored_strip_ref
        );
    }
}

#[test]
fn shared_vocabulary_supersets_frame_fidelity() {
    for ff in FrameMappingFidelity::ALL {
        assert_eq!(
            DebugMappingFidelity::from_frame_fidelity(ff).narrow_to_frame_fidelity(),
            ff
        );
    }
    assert_eq!(
        DebugMappingFidelity::ALL.len(),
        FrameMappingFidelity::ALL.len() + 2
    );
    assert_eq!(
        DebugMappingFidelity::from_symbolication_label(SymbolicationFidelityLabel::SymbolOnly),
        DebugMappingFidelity::SymbolOnly
    );
}

#[test]
fn tampering_with_a_pill_flag_fails_validation() {
    let mut set = m5_dump_mapping_restore_set();
    // Force a strip to claim a precise source link it has not earned.
    let strip = set
        .artifacts
        .iter_mut()
        .find(|a| !a.pill.shows_exact_source_link)
        .expect("a degraded strip exists");
    strip.pill.shows_exact_source_link = true;
    strip.pill.requires_disclosure = false;
    assert!(
        set.validate().is_err(),
        "tampered exact link must fail validation"
    );
}

#[test]
fn restore_claiming_live_continuity_fails_validation() {
    let mut set = m5_dump_mapping_restore_set();
    set.restored_layouts[0].pill.implies_live_continuity = true;
    assert!(
        set.validate().is_err(),
        "a restore implying live continuity must fail validation"
    );
}

#[test]
fn lines_projection_lists_artifacts_and_restores() {
    let set = m5_dump_mapping_restore_set();
    let lines = m5_dump_mapping_restore_lines(&set);
    assert!(lines.iter().any(|l| l.contains("Artifacts:")));
    assert!(lines.iter().any(|l| l.contains("Restored layouts:")));
    assert!(lines.iter().any(|l| l.contains("Invariants:")));
    for a in &set.artifacts {
        assert!(lines.iter().any(|l| l.contains(&a.strip_id)));
    }
}
