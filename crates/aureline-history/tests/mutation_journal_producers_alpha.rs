use aureline_history::{
    emit_ai_apply_record, emit_build_output_record, emit_formatter_record, emit_lockfile_record,
    emit_preview_record, emit_refactor_record, producer_binding, validate_producer_registry,
    ActorClass, ActorRef, ApprovalRef, CheckpointDurabilityClass, CheckpointKind, CheckpointRef,
    DurableVsDisposable, MutationJournalEntryRecord, MutationProducerClass, MutationProducerInput,
    PreviewKind, PreviewRef, RedactionClass, ReversalClass, ScopeClass, ScopeRef, SourceClass,
    TargetKind, TargetRef, MUTATION_JOURNAL_ENTRY_RECORD_KIND, MUTATION_PRODUCER_REGISTRY,
    REQUIRED_MUTATION_PRODUCER_CLASSES,
};
use aureline_records::RecordClassId;

fn actor(label: &str) -> ActorRef {
    ActorRef {
        display_name: label.to_owned(),
        stable_id: Some(format!("producer:{label}")),
        role: Some("automation".to_owned()),
    }
}

fn workspace_scope() -> ScopeRef {
    ScopeRef {
        class: ScopeClass::Workspace,
        id: "workspace:producer-alpha".to_owned(),
    }
}

fn target(kind: TargetKind, logical_ref: &str) -> TargetRef {
    TargetRef {
        target_kind: kind,
        filesystem_identity: None,
        logical_ref: Some(logical_ref.to_owned()),
        affected_range: None,
    }
}

fn checkpoint(
    kind: CheckpointKind,
    id: &str,
    durability: CheckpointDurabilityClass,
) -> CheckpointRef {
    CheckpointRef {
        checkpoint_kind: kind,
        checkpoint_id: id.to_owned(),
        durability_class: Some(durability),
    }
}

fn input_for(class: MutationProducerClass) -> MutationProducerInput {
    let binding = producer_binding(class).expect("registered binding");
    let mut input = MutationProducerInput::new(
        format!("m-{}", class.as_str()),
        format!("{}.apply", binding.command_id_prefix),
        actor(class.as_str()),
        workspace_scope(),
        vec![target(
            binding.primary_target_kind,
            &format!("target:{}", class.as_str()),
        )],
        "mono:producer-alpha:0001",
        format!("diff:identity:{}", class.as_str()),
        format!(
            "{} producer emitted metadata-only mutation lineage.",
            class.as_str()
        ),
    );
    input.files_touched = Some(1);
    input.bytes_written = Some(128);
    input.checkpoint_refs = vec![checkpoint(
        CheckpointKind::LocalHistorySnapshot,
        &format!("ckpt-{}", class.as_str()),
        CheckpointDurabilityClass::Durable,
    )];

    match class {
        MutationProducerClass::Refactor => {
            input.group_id = Some("group:refactor:rename-symbol".to_owned());
            input.preview_ref = Some(PreviewRef {
                preview_id: "preview:refactor:rename-symbol".to_owned(),
                preview_kind: Some(PreviewKind::RefactorPreview),
            });
        }
        MutationProducerClass::Formatter => {
            input.save_manifest_ref = Some("save-manifest:formatter:src-lib".to_owned());
        }
        MutationProducerClass::AiApply => {
            input.group_id = Some("group:ai:apply-reviewed-patch".to_owned());
            input.preview_ref = Some(PreviewRef {
                preview_id: "ai-evidence-packet:reviewed-patch-0001".to_owned(),
                preview_kind: Some(PreviewKind::AiPatchPreview),
            });
            input.approval_ref = Some(ApprovalRef {
                approval_id: "approval-ticket:ai-apply-0001".to_owned(),
                approval_policy: Some("ai.evidence.required".to_owned()),
            });
        }
        MutationProducerClass::BuildOutput => {
            input.generated_artifact_lineage_ref =
                Some("gal:build-output:target-release".to_owned());
        }
        MutationProducerClass::Lockfile => {
            input.generated_artifact_lineage_ref = Some("gal:lockfile:cargo".to_owned());
            input.save_manifest_ref = Some("save-manifest:lockfile:cargo".to_owned());
        }
        MutationProducerClass::Preview => {
            input.generated_artifact_lineage_ref =
                Some("gal:preview-snapshot:storybook-button".to_owned());
            input.preview_ref = Some(PreviewRef {
                preview_id: "preview:runtime:storybook-button".to_owned(),
                preview_kind: Some(PreviewKind::GeneratedArtifactRefreshPreview),
            });
        }
    }

    input
}

fn emit(class: MutationProducerClass) -> MutationJournalEntryRecord {
    match class {
        MutationProducerClass::Refactor => emit_refactor_record(input_for(class)),
        MutationProducerClass::Formatter => emit_formatter_record(input_for(class)),
        MutationProducerClass::AiApply => emit_ai_apply_record(input_for(class)),
        MutationProducerClass::BuildOutput => emit_build_output_record(input_for(class)),
        MutationProducerClass::Lockfile => emit_lockfile_record(input_for(class)),
        MutationProducerClass::Preview => emit_preview_record(input_for(class)),
    }
    .expect("producer emits record")
}

#[test]
fn producer_registry_covers_six_registered_record_kinds() {
    validate_producer_registry().expect("producer registry validates");
    assert_eq!(
        MUTATION_PRODUCER_REGISTRY.len(),
        REQUIRED_MUTATION_PRODUCER_CLASSES.len()
    );

    for class in REQUIRED_MUTATION_PRODUCER_CLASSES {
        let binding = producer_binding(class).expect("binding present");
        assert_eq!(binding.producer_class, class);
        assert_eq!(
            binding.emitted_record_kind,
            MUTATION_JOURNAL_ENTRY_RECORD_KIND,
            "producer {} must reuse the canonical mutation-journal entry record kind",
            class.as_str()
        );
        aureline_records::validate_typed(
            binding.emitted_record_kind,
            RecordClassId::DurableWorkspaceState,
        )
        .expect("emitted record kind is registered");
    }
}

#[test]
fn six_surface_producers_emit_expected_actor_lineage() {
    let cases = [
        (
            MutationProducerClass::Refactor,
            ActorClass::RefactorEngine,
            SourceClass::MachineLocal,
            "refactor_multi_file",
            ReversalClass::CompensatingUndo,
            RedactionClass::CodeAdjacent,
            DurableVsDisposable::DurableUserAuthored,
        ),
        (
            MutationProducerClass::Formatter,
            ActorClass::Formatter,
            SourceClass::MachineLocal,
            "formatter_run",
            ReversalClass::CompensatingUndo,
            RedactionClass::CodeAdjacent,
            DurableVsDisposable::DurableUserAuthored,
        ),
        (
            MutationProducerClass::AiApply,
            ActorClass::AiApply,
            SourceClass::AiHostedProvider,
            "machine_generated_change",
            ReversalClass::CompensatingUndo,
            RedactionClass::CodeAdjacent,
            DurableVsDisposable::DurableUserAuthored,
        ),
        (
            MutationProducerClass::BuildOutput,
            ActorClass::BuildRunner,
            SourceClass::MachineLocal,
            "machine_generated_change",
            ReversalClass::RegenerateOrRecompute,
            RedactionClass::EnvironmentAdjacent,
            DurableVsDisposable::DisposableDerived,
        ),
        (
            MutationProducerClass::Lockfile,
            ActorClass::CodegenRunner,
            SourceClass::MachineLocal,
            "machine_generated_change",
            ReversalClass::RegenerateOrRecompute,
            RedactionClass::EnvironmentAdjacent,
            DurableVsDisposable::DurableWorkspaceAuthored,
        ),
        (
            MutationProducerClass::Preview,
            ActorClass::PreviewRegenerator,
            SourceClass::MachineLocal,
            "machine_generated_change",
            ReversalClass::RegenerateOrRecompute,
            RedactionClass::EnvironmentAdjacent,
            DurableVsDisposable::DisposableDerived,
        ),
    ];

    for (
        class,
        actor_class,
        source_class,
        undo_class,
        reversal_class,
        redaction_class,
        durable_vs_disposable,
    ) in cases
    {
        let record = emit(class);
        assert_eq!(record.record_kind, MUTATION_JOURNAL_ENTRY_RECORD_KIND);
        assert_eq!(record.actor_class, actor_class, "{}", class.as_str());
        assert_eq!(record.source_class, source_class, "{}", class.as_str());
        assert_eq!(record.undo_class, undo_class, "{}", class.as_str());
        assert_eq!(record.reversal_class, reversal_class, "{}", class.as_str());
        assert_eq!(
            record.redaction_class,
            redaction_class,
            "{}",
            class.as_str()
        );
        assert_eq!(
            record.durable_vs_disposable,
            durable_vs_disposable,
            "{}",
            class.as_str()
        );
        let expected_diff = format!("diff:identity:{}", class.as_str());
        assert_eq!(
            record.diff_identity_ref.as_deref(),
            Some(expected_diff.as_str())
        );
        assert!(record.side_effect_summary.summary.contains(class.as_str()));
    }
}

#[test]
fn ai_apply_references_evidence_preview_and_approval_lineage() {
    let record = emit(MutationProducerClass::AiApply);

    assert_eq!(
        record
            .preview_ref
            .as_ref()
            .map(|preview| (preview.preview_id.as_str(), preview.preview_kind)),
        Some((
            "ai-evidence-packet:reviewed-patch-0001",
            Some(PreviewKind::AiPatchPreview)
        ))
    );
    assert_eq!(
        record.approval_ref.as_ref().map(|approval| (
            approval.approval_id.as_str(),
            approval.approval_policy.as_deref()
        )),
        Some((
            "approval-ticket:ai-apply-0001",
            Some("ai.evidence.required")
        ))
    );

    let mut missing_approval = input_for(MutationProducerClass::AiApply);
    missing_approval.approval_ref = None;
    let error = emit_ai_apply_record(missing_approval).expect_err("approval lineage is required");
    assert_eq!(error.to_string(), "ai_apply producer has no approval ref");
}

#[test]
fn derived_artifact_producers_reference_generated_lineage() {
    for class in [
        MutationProducerClass::BuildOutput,
        MutationProducerClass::Lockfile,
        MutationProducerClass::Preview,
    ] {
        let record = emit(class);
        assert!(
            record
                .generated_artifact_lineage_ref
                .as_deref()
                .is_some_and(|lineage| lineage.starts_with("gal:")),
            "{} must cite generated-artifact lineage",
            class.as_str()
        );
        assert_eq!(
            record.target_refs[0].target_kind,
            TargetKind::GeneratedArtifact,
            "{} targets a generated artifact",
            class.as_str()
        );
    }
}
