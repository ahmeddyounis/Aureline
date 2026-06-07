//! Fixture-replay and invariant tests for the stable structured-input corpus.

use std::collections::BTreeSet;

use aureline_shell::forms_parameter_source_and_staged_apply::{
    forms_parameter_source_and_staged_apply_corpus, validate_form_truth_packet, ApplyTiming,
    ClientScope, FieldKind, FormTruthPacketRecord, SecretExportBehavior, SourcePrecedence,
    ValidationClass, ValidationState,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/forms/m4/forms-parameter-source-and-staged-apply",
);

fn load_record(filename: &str) -> FormTruthPacketRecord {
    let path = format!("{FIXTURE_DIR}/{filename}");
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn every_scenario_fixture_matches_in_code_projection() {
    for scenario in forms_parameter_source_and_staged_apply_corpus() {
        let on_disk = load_record(scenario.fixture_filename);
        let in_code = scenario.record();
        assert_eq!(
            on_disk, in_code,
            "{} fixture drifted; re-emit with `cargo run -q -p aureline-shell --bin aureline_shell_forms_parameter_source_and_staged_apply -- emit-fixtures fixtures/forms/m4/forms-parameter-source-and-staged-apply`",
            scenario.fixture_filename,
        );
    }
}

#[test]
fn every_packet_validates_against_shared_contract() {
    for scenario in forms_parameter_source_and_staged_apply_corpus() {
        let record = load_record(scenario.fixture_filename);
        validate_form_truth_packet(&record).unwrap_or_else(|errors| {
            panic!("{} failed validation: {errors:#?}", scenario.scenario_id)
        });
    }
}

#[test]
fn corpus_covers_required_apply_timing_and_client_contexts() {
    let mut timings = BTreeSet::new();
    let mut clients = BTreeSet::new();
    for scenario in forms_parameter_source_and_staged_apply_corpus() {
        let record = load_record(scenario.fixture_filename);
        timings.insert(record.staged_apply.apply_timing);
        clients.insert(record.client_scope);
    }
    assert!(timings.contains(&ApplyTiming::Immediate));
    assert!(timings.contains(&ApplyTiming::Staged));
    assert!(timings.contains(&ApplyTiming::PreviewFirst));
    assert!(timings.contains(&ApplyTiming::PolicyLocked));
    assert!(clients.contains(&ClientScope::Desktop));
    assert!(clients.contains(&ClientScope::RemoteWorkspace));
    assert!(clients.contains(&ClientScope::ManagedWorkspace));
    assert!(clients.contains(&ClientScope::OfflineDegraded));
    assert!(clients.contains(&ClientScope::BrowserCompanion));
    assert!(clients.contains(&ClientScope::RestrictedClient));
}

#[test]
fn precedence_audits_keep_required_source_classes_distinct() {
    let mut sources = BTreeSet::new();
    for scenario in forms_parameter_source_and_staged_apply_corpus() {
        let record = load_record(scenario.fixture_filename);
        for audit in &record.precedence_audits {
            for candidate in &audit.candidates {
                sources.insert(candidate.source_class);
            }
        }
    }
    for source in [
        SourcePrecedence::Default,
        SourcePrecedence::Detected,
        SourcePrecedence::Imported,
        SourcePrecedence::Workspace,
        SourcePrecedence::Policy,
        SourcePrecedence::UserOverride,
        SourcePrecedence::SecretReference,
    ] {
        assert!(sources.contains(&source), "missing source {source:?}");
    }
}

#[test]
fn validation_classes_cover_async_and_preview_truth() {
    let mut classes = BTreeSet::new();
    let mut states = BTreeSet::new();
    for scenario in forms_parameter_source_and_staged_apply_corpus() {
        let record = load_record(scenario.fixture_filename);
        for field in &record.fields {
            classes.insert(field.validation.validation_class);
            states.insert(field.validation.state);
            if field.validation.state == ValidationState::Pending {
                assert!(field.validation.last_known_state.is_some());
                assert!(field.validation.invalidates_on_target_change);
            }
            if field.validation.state == ValidationState::Stale {
                assert!(field.validation.invalidates_on_target_change);
                assert!(!field.validation.dependent_inputs.is_empty());
            }
        }
    }
    for class in [
        ValidationClass::LocalSyntax,
        ValidationClass::SchemaModel,
        ValidationClass::EnvironmentProbe,
        ValidationClass::RemoteAuth,
        ValidationClass::PolicyEntitlement,
        ValidationClass::DryRunPreview,
    ] {
        assert!(
            classes.contains(&class),
            "missing validation class {class:?}"
        );
    }
    assert!(states.contains(&ValidationState::Pending));
    assert!(states.contains(&ValidationState::Stale));
    assert!(states.contains(&ValidationState::Blocked));
}

#[test]
fn secret_path_reference_and_code_backed_fields_are_export_safe() {
    let mut kinds = BTreeSet::new();
    for scenario in forms_parameter_source_and_staged_apply_corpus() {
        let record = load_record(scenario.fixture_filename);
        for field in &record.fields {
            kinds.insert(field.field_kind);
            match field.field_kind {
                FieldKind::Secret => {
                    let truth = field.secret_truth.as_ref().expect("secret truth");
                    assert!(matches!(
                        truth.export_behavior,
                        SecretExportBehavior::ReferenceOnly
                            | SecretExportBehavior::RedactedByDefault
                            | SecretExportBehavior::PolicyOwnedOmitted
                    ));
                    assert!(!truth.reveal_friction.is_empty());
                    assert!(!truth.copy_warning.is_empty());
                }
                FieldKind::Path => {
                    let truth = field.path_truth.as_ref().expect("path truth");
                    assert!(!truth.displayed_path.is_empty());
                    assert!(!truth.basis_ref.is_empty());
                }
                FieldKind::ObjectReference => {
                    let truth = field.reference_truth.as_ref().expect("reference truth");
                    assert!(!truth.display_label.is_empty());
                    assert!(!truth.stable_id_path.is_empty());
                }
                FieldKind::CodeBacked => {
                    let truth = field.code_backed_truth.as_ref().expect("code-backed truth");
                    assert!(truth.preserves_comments);
                    assert!(truth.preserves_unknown_fields);
                    assert!(!truth.diff_preview_ref.is_empty());
                }
                FieldKind::Plain => {}
            }
        }
    }
    assert!(kinds.contains(&FieldKind::Secret));
    assert!(kinds.contains(&FieldKind::Path));
    assert!(kinds.contains(&FieldKind::ObjectReference));
    assert!(kinds.contains(&FieldKind::CodeBacked));
}

#[test]
fn staged_and_preview_packets_preserve_review_checkpoint_and_exact_submit_labels() {
    for scenario in forms_parameter_source_and_staged_apply_corpus() {
        let record = load_record(scenario.fixture_filename);
        let apply = &record.staged_apply;
        assert_ne!(apply.final_submit_label, "Continue");
        match apply.apply_timing {
            ApplyTiming::Staged => {
                assert!(apply.dirty);
                assert!(apply.checkpoint_ref.is_some());
                assert!(apply.revert_action_label.is_some());
                assert!(apply.save_and_resume_disclosed);
                for event in ["tab_change", "reconnect", "client_handoff"] {
                    assert!(apply
                        .dirty_state_persisted_across
                        .iter()
                        .any(|v| v == event));
                }
            }
            ApplyTiming::PreviewFirst => {
                assert!(apply.review_sheet_ref.is_some());
                assert!(apply.checkpoint_ref.is_some());
                assert!(!apply.side_effects.is_empty());
            }
            ApplyTiming::PolicyLocked => {
                assert!(apply.review_sheet_ref.is_some());
            }
            ApplyTiming::Immediate => {
                assert!(apply.revert_action_label.is_some());
            }
        }
    }
}

#[test]
fn restricted_client_limitations_are_disclosed_before_final_step() {
    for scenario in forms_parameter_source_and_staged_apply_corpus() {
        let record = load_record(scenario.fixture_filename);
        if matches!(
            record.client_scope,
            ClientScope::BrowserCompanion | ClientScope::RestrictedClient
        ) {
            assert!(!record.client_limitations.is_empty());
            assert!(record
                .client_limitations
                .iter()
                .all(|limitation| limitation.disclosed_before_final_step));
        }
    }
}
