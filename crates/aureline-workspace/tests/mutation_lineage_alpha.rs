use aureline_workspace::{
    MutationActorClass, MutationActorRef, MutationAiApplyLineage,
    MutationCheckpointDurabilityClass, MutationCheckpointKind, MutationCheckpointRef,
    MutationDurabilityClass, MutationGeneratedArtifactCue, MutationGroupKind, MutationGroupRecord,
    MutationGroupResolution, MutationJournalEntryRecord, MutationLineageAlphaPacket,
    MutationLineageAlphaValidationError, MutationLineageConsumerSurface, MutationLineageEnvelope,
    MutationPathClass, MutationRedactionClass, MutationReversalClass, MutationScopeClass,
    MutationScopeRef, MutationSideEffectSummary, MutationSourceClass, MutationTargetKind,
    MutationTargetRef,
};

fn actor(display_name: &str, stable_id: &str) -> MutationActorRef {
    MutationActorRef {
        display_name: display_name.to_owned(),
        stable_id: Some(stable_id.to_owned()),
        role: Some("automation".to_owned()),
    }
}

fn workspace_scope() -> MutationScopeRef {
    MutationScopeRef {
        class: MutationScopeClass::Workspace,
        id: "ws-lineage-alpha".to_owned(),
    }
}

fn checkpoint(
    checkpoint_kind: MutationCheckpointKind,
    checkpoint_id: &str,
    durability_class: MutationCheckpointDurabilityClass,
) -> MutationCheckpointRef {
    MutationCheckpointRef {
        checkpoint_kind,
        checkpoint_id: checkpoint_id.to_owned(),
        durability_class: Some(durability_class),
    }
}

fn target(target_kind: MutationTargetKind, logical_ref: &str) -> MutationTargetRef {
    MutationTargetRef {
        target_kind,
        filesystem_identity: None,
        logical_ref: Some(logical_ref.to_owned()),
        affected_range: None,
    }
}

fn side_effect(summary: &str, bytes_written: u64, files_touched: u64) -> MutationSideEffectSummary {
    let mut side_effect = MutationSideEffectSummary::new(summary);
    side_effect.bytes_written = Some(bytes_written);
    side_effect.files_touched = Some(files_touched);
    side_effect
}

#[allow(clippy::too_many_arguments)]
fn entry(
    mutation_id: &str,
    command_id: &str,
    actor_class: MutationActorClass,
    source_class: MutationSourceClass,
    target_kind: MutationTargetKind,
    target_ref: &str,
    undo_class: &str,
    reversal_class: MutationReversalClass,
    redaction_class: MutationRedactionClass,
    durable_vs_disposable: MutationDurabilityClass,
    side_effect_summary: MutationSideEffectSummary,
    checkpoint_ref: MutationCheckpointRef,
) -> MutationJournalEntryRecord {
    MutationJournalEntryRecord::new(
        mutation_id.to_owned(),
        command_id.to_owned(),
        actor_class,
        source_class,
        actor(actor_class.as_str(), command_id),
        workspace_scope(),
        vec![target(target_kind, target_ref)],
        "mono:lineage:started".to_owned(),
        "mono:lineage:committed".to_owned(),
        undo_class.to_owned(),
        reversal_class,
        redaction_class,
        durable_vs_disposable,
        side_effect_summary,
        vec![checkpoint_ref],
    )
}

#[allow(clippy::too_many_arguments)]
fn group(
    group_id: &str,
    group_kind: MutationGroupKind,
    command_id: &str,
    actor_class: MutationActorClass,
    source_class: MutationSourceClass,
    member_mutation_ids: Vec<String>,
    reversal_class: MutationReversalClass,
    redaction_class: MutationRedactionClass,
    durable_vs_disposable: MutationDurabilityClass,
    side_effect_summary: MutationSideEffectSummary,
    checkpoint_ref: MutationCheckpointRef,
) -> MutationGroupRecord {
    MutationGroupRecord::new(
        group_id.to_owned(),
        group_kind,
        command_id.to_owned(),
        actor_class,
        source_class,
        actor(actor_class.as_str(), command_id),
        workspace_scope(),
        "mono:lineage:opened".to_owned(),
        "mono:lineage:resolved".to_owned(),
        MutationGroupResolution::Applied,
        member_mutation_ids,
        reversal_class,
        redaction_class,
        durable_vs_disposable,
        side_effect_summary,
        vec![checkpoint_ref],
    )
}

fn protected_envelopes(
    include_build_cue: bool,
    leaking_checkpoint_ref: Option<&str>,
) -> Vec<MutationLineageEnvelope> {
    let refactor = entry(
        "m-refactor-0001",
        "language.refactor.rename",
        MutationActorClass::RefactorEngine,
        MutationSourceClass::MachineLocal,
        MutationTargetKind::Buffer,
        "buffer:src/lib.rs",
        "refactor_multi_file",
        MutationReversalClass::CompensatingUndo,
        MutationRedactionClass::CodeAdjacent,
        MutationDurabilityClass::DurableUserAuthored,
        side_effect(
            "Refactor engine renamed one symbol across reviewed edits.",
            224,
            2,
        ),
        checkpoint(
            MutationCheckpointKind::MutationGroupPreview,
            "ckpt-refactor-preview-0001",
            MutationCheckpointDurabilityClass::Durable,
        ),
    );

    let formatter = entry(
        "m-format-0001",
        "editor.format_on_save",
        MutationActorClass::Formatter,
        MutationSourceClass::MachineLocal,
        MutationTargetKind::Buffer,
        "buffer:src/lib.rs",
        "formatter_run",
        MutationReversalClass::CompensatingUndo,
        MutationRedactionClass::CodeAdjacent,
        MutationDurabilityClass::DurableUserAuthored,
        side_effect("Formatter rewrote one staged buffer before save.", 438, 1),
        checkpoint(
            MutationCheckpointKind::SaveManifest,
            leaking_checkpoint_ref.unwrap_or("ckpt-format-save-0001"),
            MutationCheckpointDurabilityClass::Durable,
        ),
    );

    let lockfile = entry(
        "m-lockfile-0001",
        "package.lockfile.refresh",
        MutationActorClass::CodegenRunner,
        MutationSourceClass::MachineLocal,
        MutationTargetKind::GeneratedArtifact,
        "path:Cargo.lock",
        "machine_generated_change",
        MutationReversalClass::RegenerateOrRecompute,
        MutationRedactionClass::EnvironmentAdjacent,
        MutationDurabilityClass::DurableWorkspaceAuthored,
        side_effect("Resolver refreshed Cargo.lock from Cargo.toml.", 812, 1),
        checkpoint(
            MutationCheckpointKind::LocalHistorySnapshot,
            "ckpt-lockfile-0001",
            MutationCheckpointDurabilityClass::Durable,
        ),
    );

    let build = entry(
        "m-build-0001",
        "build.run",
        MutationActorClass::BuildRunner,
        MutationSourceClass::MachineLocal,
        MutationTargetKind::GeneratedArtifact,
        "path:target/release/aureline",
        "machine_generated_change",
        MutationReversalClass::RegenerateOrRecompute,
        MutationRedactionClass::EnvironmentAdjacent,
        MutationDurabilityClass::DisposableDerived,
        side_effect("Build wrote one release binary.", 44_128_256, 1),
        checkpoint(
            MutationCheckpointKind::LocalHistorySnapshot,
            "ckpt-build-output-0001",
            MutationCheckpointDurabilityClass::Disposable,
        ),
    );

    let preview = group(
        "g-preview-0001",
        MutationGroupKind::PreviewRegeneration,
        "preview.regenerate",
        MutationActorClass::PreviewRegenerator,
        MutationSourceClass::MachineLocal,
        vec!["m-preview-0001".to_owned()],
        MutationReversalClass::RegenerateOrRecompute,
        MutationRedactionClass::EnvironmentAdjacent,
        MutationDurabilityClass::DisposableDerived,
        side_effect(
            "Preview runtime regenerated one render snapshot.",
            18_432,
            1,
        ),
        checkpoint(
            MutationCheckpointKind::LocalHistorySnapshot,
            "ckpt-preview-snapshot-0001",
            MutationCheckpointDurabilityClass::Disposable,
        ),
    );

    let ai = group(
        "g-ai-0001",
        MutationGroupKind::AiPatch,
        "ai.apply.patch",
        MutationActorClass::AiApply,
        MutationSourceClass::AiHostedProvider,
        vec!["m-ai-0001".to_owned(), "m-ai-0002".to_owned()],
        MutationReversalClass::CompensatingUndo,
        MutationRedactionClass::CodeAdjacent,
        MutationDurabilityClass::DurableUserAuthored,
        side_effect("AI apply wrote two reviewed source edits.", 112, 2),
        checkpoint(
            MutationCheckpointKind::MutationGroupPreview,
            "ckpt-ai-preview-0001",
            MutationCheckpointDurabilityClass::Durable,
        ),
    )
    .with_ai_apply_lineage(MutationAiApplyLineage::new(
        "evidence-packet:ai-mutation:alpha:0001",
        "vendor_hosted_managed",
        "spend-receipt:ai-mutation:alpha:0001",
        None,
    ));

    let mut build_envelope =
        MutationLineageEnvelope::from_entry("env-build", MutationPathClass::BuildOutput, build)
            .with_generated_artifact_lineage_ref("gal-build-output-0001");
    if include_build_cue {
        build_envelope = build_envelope.with_target_relative_path("target/release/aureline");
    }

    vec![
        MutationLineageEnvelope::from_entry("env-refactor", MutationPathClass::Refactor, refactor)
            .with_target_relative_path("src/lib.rs")
            .with_preview_ref("preview-refactor-0001", "refactor_preview"),
        MutationLineageEnvelope::from_entry("env-format", MutationPathClass::Formatter, formatter)
            .with_target_relative_path("src/lib.rs"),
        MutationLineageEnvelope::from_entry("env-lockfile", MutationPathClass::Lockfile, lockfile)
            .with_target_relative_path("Cargo.lock")
            .with_generated_artifact_lineage_ref("gal-lockfile-cargo-0001"),
        build_envelope,
        MutationLineageEnvelope::from_group(
            "env-preview",
            MutationPathClass::PreviewRegeneration,
            preview,
        )
        .with_generated_artifact_cue(MutationGeneratedArtifactCue::preview_snapshot(
            "preview:snapshots/button.primary.png",
            "ui/components/Button.tsx",
            "producer:preview_runtime_storybook",
            "Storybook preview runtime",
            "in_sync",
            "preview.snapshot.storybook",
        ))
        .with_generated_artifact_lineage_ref("gal-preview-snapshot-0001")
        .with_preview_ref("preview-run-0001", "preview_runtime_refresh"),
        MutationLineageEnvelope::from_group("env-ai", MutationPathClass::AiApply, ai)
            .with_preview_ref("ai-evidence-packet-0001", "ai_patch_preview")
            .with_approval_ref("approval-ai-evidence-0001", "ai.evidence.required"),
    ]
}

#[test]
fn envelopes_emit_required_path_coverage_and_generated_cues() {
    let envelopes = protected_envelopes(true, None);
    let packet = MutationLineageAlphaPacket::support_export(
        "mutation-lineage-alpha:protected",
        "2026-05-13T22:30:00Z",
        &envelopes,
    )
    .expect("support export packet");

    assert!(packet.missing_required_alpha_coverage().is_empty());
    assert_eq!(packet.mutation_lineage_rows.len(), 6);
    assert_eq!(
        packet.consumer_surface,
        MutationLineageConsumerSurface::SupportExport
    );
    assert!(packet
        .export_safety
        .omitted_payload_classes
        .contains(&"raw_secret_material".to_owned()));

    let refactor = packet
        .mutation_lineage_rows
        .iter()
        .find(|row| row.mutation_path_class == MutationPathClass::Refactor)
        .expect("refactor row");
    assert_eq!(refactor.actor_class, "refactor_engine");
    assert_eq!(
        refactor
            .preview_ref
            .as_ref()
            .map(|preview| preview.preview_kind.as_str()),
        Some("refactor_preview")
    );

    let lockfile = packet
        .mutation_lineage_rows
        .iter()
        .find(|row| row.mutation_path_class == MutationPathClass::Lockfile)
        .expect("lockfile row");
    let lockfile_cue = lockfile
        .generated_artifact_cue
        .as_ref()
        .expect("lockfile cue");
    assert_eq!(lockfile_cue.generated_class, "lockfile");
    assert_eq!(
        lockfile_cue.source_canonical_relative_path.as_deref(),
        Some("Cargo.toml")
    );
    assert_eq!(lockfile_cue.producer_id, "producer:cargo_lockfile");

    let build = packet
        .mutation_lineage_rows
        .iter()
        .find(|row| row.mutation_path_class == MutationPathClass::BuildOutput)
        .expect("build row");
    let build_cue = build.generated_artifact_cue.as_ref().expect("build cue");
    assert_eq!(build_cue.generated_class, "build_output");
    assert_eq!(build_cue.default_edit_posture, "inspect_read_only");

    let preview = packet
        .mutation_lineage_rows
        .iter()
        .find(|row| row.mutation_path_class == MutationPathClass::PreviewRegeneration)
        .expect("preview row");
    assert_eq!(
        preview
            .generated_artifact_cue
            .as_ref()
            .expect("preview cue")
            .generated_class,
        "preview_render_snapshot"
    );

    let ai = packet
        .mutation_lineage_rows
        .iter()
        .find(|row| row.mutation_path_class == MutationPathClass::AiApply)
        .expect("AI row");
    assert_eq!(
        ai.approval_ref
            .as_ref()
            .map(|approval| approval.approval_id.as_str()),
        Some("approval-ai-evidence-0001")
    );
    assert_eq!(
        ai.ai_apply_lineage
            .as_ref()
            .map(|lineage| lineage.ai_evidence_packet_ref.as_str()),
        Some("evidence-packet:ai-mutation:alpha:0001")
    );

    let json = serde_json::to_string(&packet).expect("serialize packet");
    let parsed: MutationLineageAlphaPacket = serde_json::from_str(&json).expect("parse packet");
    parsed.validate().expect("round-tripped packet is valid");
}

#[test]
fn protected_fixture_packet_is_export_safe() {
    let raw = include_str!(
        "../../../fixtures/workspace/mutation_lineage_alpha/protected_mutation_lineage_packet.json"
    );
    let packet: MutationLineageAlphaPacket = serde_json::from_str(raw).expect("fixture parses");

    packet.validate().expect("fixture is export safe");
    assert!(packet.missing_required_alpha_coverage().is_empty());
    assert_eq!(packet.mutation_lineage_rows.len(), 6);
    assert!(packet
        .mutation_lineage_rows
        .iter()
        .all(|row| !row.raw_payload_exported && row.support_export_safe));
}

#[test]
fn derived_artifact_rows_require_generated_cues() {
    let envelopes = protected_envelopes(false, None);
    let err = MutationLineageAlphaPacket::support_export(
        "mutation-lineage-alpha:missing-cue",
        "2026-05-13T22:31:00Z",
        &envelopes,
    )
    .expect_err("build output without cue must fail");

    assert!(matches!(
        err,
        MutationLineageAlphaValidationError::MissingGeneratedCue {
            mutation_path_class: MutationPathClass::BuildOutput,
            ..
        }
    ));
}

#[test]
fn support_export_rejects_raw_payload_refs() {
    let envelopes = protected_envelopes(true, Some("obj:blake3:raw-body-ref"));
    let err = MutationLineageAlphaPacket::support_export(
        "mutation-lineage-alpha:raw-ref",
        "2026-05-13T22:32:00Z",
        &envelopes,
    )
    .expect_err("raw object ref must fail support export validation");

    assert!(matches!(
        err,
        MutationLineageAlphaValidationError::RawPayloadRefLeaked { .. }
    ));
}
