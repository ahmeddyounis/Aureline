//! Protected tests for published supportability runbooks, field playbooks, and
//! incident/advisory packet integration for the stable line.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_support::publish_supportability_runbooks_field_playbooks_and_incident_advisory::{
    current_supportability_runbook_catalog, load_catalog_entry,
    ApprovalRequirementClass, AuthoritativePosture, RunbookSourceClass, StepClass,
    TargetSelectorScope, DEVIATION_NOTE_RECORD_KIND, PUBLISHED_SUPPORTABILITY_ARTIFACT_REF,
    PUBLISHED_SUPPORTABILITY_DOC_REF, PUBLISHED_SUPPORTABILITY_FIXTURE_DIR,
    PUBLISHED_SUPPORTABILITY_FIXTURE_MANIFEST_REF, PUBLISHED_SUPPORTABILITY_SCHEMA_REF,
    PUBLISHED_SUPPORTABILITY_SCHEMA_VERSION, SUPPORTABILITY_RUNBOOK_CATALOG_ENTRY_RECORD_KIND,
    SUPPORTABILITY_RUNBOOK_CATALOG_RECORD_KIND,
};
use aureline_support::publish_supportability_runbooks_field_playbooks_and_incident_advisory::doctor_projection::{
    current_doctor_runbook_projection,
    DOCTOR_RUNBOOK_PROJECTION_RECORD_KIND, DOCTOR_RUNBOOK_PROJECTION_ROW_RECORD_KIND,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

#[test]
fn corpus_validates_and_covers_required_source_classes() {
    let catalog = current_supportability_runbook_catalog().expect("corpus parses");
    assert_eq!(
        catalog.record_kind,
        SUPPORTABILITY_RUNBOOK_CATALOG_RECORD_KIND
    );
    assert_eq!(
        catalog.schema_version,
        PUBLISHED_SUPPORTABILITY_SCHEMA_VERSION
    );

    let violations = catalog.validate();
    assert_eq!(violations, Vec::new(), "{violations:#?}");

    let mut covered_sources = BTreeSet::new();
    for entry in &catalog.entries {
        assert_eq!(
            entry.record_kind,
            SUPPORTABILITY_RUNBOOK_CATALOG_ENTRY_RECORD_KIND
        );
        covered_sources.insert(entry.source_class);
    }

    for required in RunbookSourceClass::REQUIRED {
        assert!(
            covered_sources.contains(&required),
            "missing required runbook source class: {}",
            required.as_str()
        );
    }
}

#[test]
fn fixtures_round_trip_from_disk() {
    let catalog = current_supportability_runbook_catalog().expect("corpus parses");
    let root = repo_root();
    for entry in &catalog.entries {
        let yaml = std::fs::read_to_string(root.join(&entry.fixture_ref))
            .unwrap_or_else(|err| panic!("read {}: {err}", entry.fixture_ref));
        let parsed = load_catalog_entry(&yaml)
            .unwrap_or_else(|err| panic!("parse {}: {err}", entry.fixture_ref));
        assert_eq!(
            parsed, *entry,
            "round-trip mismatch for {}",
            entry.fixture_ref
        );
    }
}

#[test]
fn declared_docs_schema_and_fixture_paths_exist() {
    let root = repo_root();
    for path in [
        PUBLISHED_SUPPORTABILITY_SCHEMA_REF,
        PUBLISHED_SUPPORTABILITY_DOC_REF,
        PUBLISHED_SUPPORTABILITY_ARTIFACT_REF,
        PUBLISHED_SUPPORTABILITY_FIXTURE_MANIFEST_REF,
    ] {
        assert!(root.join(path).is_file(), "{path} missing");
    }
    assert!(
        root.join(PUBLISHED_SUPPORTABILITY_FIXTURE_DIR).is_dir(),
        "{PUBLISHED_SUPPORTABILITY_FIXTURE_DIR} missing"
    );
}

#[test]
fn every_playbook_packet_is_export_safe() {
    let catalog = current_supportability_runbook_catalog().expect("corpus parses");
    for packet in catalog.playbook_packets() {
        assert!(
            packet.raw_private_material_excluded,
            "{} must exclude raw private material",
            packet.packet_id
        );
        assert!(
            packet.ambient_authority_excluded,
            "{} must exclude ambient authority",
            packet.packet_id
        );
        assert!(
            !packet.steps.is_empty(),
            "{} must have at least one step",
            packet.packet_id
        );
    }
}

#[test]
fn mutating_steps_have_approval_ticket_or_explicit_waiver() {
    let catalog = current_supportability_runbook_catalog().expect("corpus parses");
    for packet in catalog.playbook_packets() {
        for step in &packet.steps {
            if step.step_class.is_mutating() {
                assert!(
                    step.approval_ticket_ref.is_some()
                        || step.approval_requirement
                            == ApprovalRequirementClass::NoApprovalRequired
                        || step.approval_requirement == ApprovalRequirementClass::ApprovalForbidden,
                    "mutating step {} in {} must carry approval_ticket_ref or explicit waiver",
                    step.step_id,
                    packet.packet_id
                );
            }
        }
    }
}

#[test]
fn external_target_steps_carry_handoff_metadata() {
    let catalog = current_supportability_runbook_catalog().expect("corpus parses");
    for packet in catalog.playbook_packets() {
        for step in &packet.steps {
            if step.target_selector_scope.requires_external_handoff() {
                assert!(
                    step.handoff_metadata.is_some(),
                    "step {} in {} with external target scope must carry handoff_metadata",
                    step.step_id,
                    packet.packet_id
                );
            }
        }
    }
}

#[test]
fn browser_only_sources_are_reference_only() {
    let catalog = current_supportability_runbook_catalog().expect("corpus parses");
    for entry in &catalog.entries {
        if entry.source_class == RunbookSourceClass::BrowserOnlyVendorDocs {
            assert_eq!(
                entry.playbook_packet.source_document.authoritative_posture,
                AuthoritativePosture::ReferenceOnly,
                "browser_only_vendor_docs entry {} must have reference_only posture",
                entry.entry_id
            );
        }
    }
}

#[test]
fn incident_advisory_packets_are_export_safe_and_join_deviation_notes() {
    let catalog = current_supportability_runbook_catalog().expect("corpus parses");
    for packet in catalog.incident_advisory_packets() {
        assert!(
            packet.raw_private_material_excluded,
            "{} must exclude raw private material",
            packet.packet_id
        );
        assert!(
            packet.ambient_authority_excluded,
            "{} must exclude ambient authority",
            packet.packet_id
        );
        for note in &packet.deviation_notes {
            assert_eq!(
                note.record_kind, DEVIATION_NOTE_RECORD_KIND,
                "deviation note {} must have correct record_kind",
                note.deviation_note_id
            );
            assert!(
                !note.summary.trim().is_empty(),
                "deviation note {} must have non-empty summary",
                note.deviation_note_id
            );
        }
    }
}

#[test]
fn step_ordinals_are_contiguous_from_zero() {
    let catalog = current_supportability_runbook_catalog().expect("corpus parses");
    for packet in catalog.playbook_packets() {
        let mut ordinals: Vec<u32> = packet.steps.iter().map(|s| s.ordinal).collect();
        ordinals.sort();
        for (expected, actual) in ordinals.iter().enumerate() {
            assert_eq!(
                *actual, expected as u32,
                "step ordinals in {} must be contiguous from 0",
                packet.packet_id
            );
        }
    }
}

#[test]
fn all_required_step_classes_are_represented_across_catalog() {
    let catalog = current_supportability_runbook_catalog().expect("corpus parses");
    let mut covered = BTreeSet::new();
    for packet in catalog.playbook_packets() {
        for step in &packet.steps {
            covered.insert(step.step_class);
        }
    }
    for required in StepClass::REQUIRED {
        assert!(
            covered.contains(&required),
            "missing required step class: {}",
            required.as_str()
        );
    }
}

#[test]
fn all_required_target_scopes_are_represented_across_catalog() {
    let catalog = current_supportability_runbook_catalog().expect("corpus parses");
    let mut covered = BTreeSet::new();
    for packet in catalog.playbook_packets() {
        for scope in &packet.supported_target_scopes {
            covered.insert(*scope);
        }
    }
    for required in [
        TargetSelectorScope::LocalWorkspace,
        TargetSelectorScope::RuntimeTarget,
        TargetSelectorScope::BrowserConsoleExternal,
    ] {
        assert!(
            covered.contains(&required),
            "missing required target scope: {}",
            required.as_str()
        );
    }
}

// ---------------------------------------------------------------------------
// Doctor projection tests
// ---------------------------------------------------------------------------

#[test]
fn doctor_projection_loads_and_is_safe_for_diagnosis_context() {
    let projection = current_doctor_runbook_projection().expect("projection loads");
    assert_eq!(
        projection.record_kind,
        DOCTOR_RUNBOOK_PROJECTION_RECORD_KIND
    );
    assert_eq!(
        projection.schema_version,
        PUBLISHED_SUPPORTABILITY_SCHEMA_VERSION
    );
    assert!(projection.raw_private_material_excluded);
    assert!(projection.ambient_authority_excluded);
    assert!(
        projection.is_safe_for_diagnosis_context(),
        "projection must be safe for diagnosis context"
    );
    assert!(
        !projection.rows.is_empty(),
        "projection must contain at least one row"
    );
}

#[test]
fn every_doctor_projection_row_has_correct_record_kind_and_non_empty_fields() {
    let projection = current_doctor_runbook_projection().expect("projection loads");
    for row in &projection.rows {
        assert_eq!(
            row.record_kind, DOCTOR_RUNBOOK_PROJECTION_ROW_RECORD_KIND,
            "row {} must have correct record_kind",
            row.row_id
        );
        assert!(!row.row_id.is_empty(), "row_id must not be empty");
        assert!(
            !row.catalog_entry_id.is_empty(),
            "catalog_entry_id must not be empty"
        );
        assert!(
            !row.runbook_packet_id.is_empty(),
            "runbook_packet_id must not be empty"
        );
        assert!(!row.step_title.is_empty(), "step_title must not be empty");
        assert!(
            !row.repair_hook_class.is_empty(),
            "repair_hook_class must not be empty"
        );
    }
}

#[test]
fn doctor_projection_mutating_flags_match_step_classes() {
    let projection = current_doctor_runbook_projection().expect("projection loads");
    for row in &projection.rows {
        match row.step_class {
            StepClass::Observe | StepClass::Verify | StepClass::Communicate => {
                assert!(
                    !row.is_mutating,
                    "row {} for step class {:?} must not be mutating",
                    row.row_id, row.step_class
                );
            }
            StepClass::Mitigate | StepClass::Rollback => {
                assert!(
                    row.is_mutating,
                    "row {} for step class {:?} must be mutating",
                    row.row_id, row.step_class
                );
            }
        }
    }
}

#[test]
fn doctor_projection_external_handoff_matches_target_scope() {
    let projection = current_doctor_runbook_projection().expect("projection loads");
    for row in &projection.rows {
        assert_eq!(
            row.requires_external_handoff,
            row.target_selector_scope.requires_external_handoff(),
            "row {} external_handoff must match target_selector_scope",
            row.row_id
        );
    }
}

#[test]
fn doctor_projection_covers_all_step_classes() {
    let projection = current_doctor_runbook_projection().expect("projection loads");
    let covered: BTreeSet<StepClass> = projection.rows.iter().map(|r| r.step_class).collect();
    for required in StepClass::REQUIRED {
        assert!(
            covered.contains(&required),
            "missing step class in doctor projection: {}",
            required.as_str()
        );
    }
}

#[test]
fn doctor_projection_browser_only_sources_are_safe_but_not_authoritative() {
    let projection = current_doctor_runbook_projection().expect("projection loads");
    for row in &projection.rows {
        if row.source_class == RunbookSourceClass::BrowserOnlyVendorDocs {
            assert!(
                row.safe_for_diagnosis_context,
                "browser_only_vendor_docs row {} must be metadata-safe for diagnosis context",
                row.row_id
            );
            assert_eq!(
                row.authoritative_posture,
                AuthoritativePosture::ReferenceOnly,
                "browser_only_vendor_docs row {} must have reference_only posture",
                row.row_id
            );
        }
    }
}
