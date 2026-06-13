use super::*;

const PACKET_ID: &str = "prompt-session-artifact:stable:0001";
const MINTED_AT: &str = "2026-06-13T00:00:00Z";

fn proof_stale_to(narrowed_to: M5AiWorkflowQualificationClass) -> SessionArtifactDowngradeRule {
    SessionArtifactDowngradeRule {
        trigger: M5AiWorkflowDowngradeTrigger::ProofStale,
        narrowed_to,
        auto_enforced: true,
        rationale: "Stale proof narrows the public claim".to_owned(),
    }
}

fn trust_narrowing_to(narrowed_to: M5AiWorkflowQualificationClass) -> SessionArtifactDowngradeRule {
    SessionArtifactDowngradeRule {
        trigger: M5AiWorkflowDowngradeTrigger::TrustNarrowing,
        narrowed_to,
        auto_enforced: true,
        rationale: "Scope or consent revocation narrows the claim".to_owned(),
    }
}

fn resolved_mention(id: &str, kind: MentionKind, target: &str) -> MentionProvenanceRow {
    MentionProvenanceRow {
        mention_id: id.to_owned(),
        mention_kind: kind,
        resolution_state: MentionResolutionState::Resolved,
        resolved_target_ref: Some(target.to_owned()),
        display_label: "Resolved mention".to_owned(),
        in_scope: true,
    }
}

fn scope_excluded_mention(id: &str) -> MentionProvenanceRow {
    MentionProvenanceRow {
        mention_id: id.to_owned(),
        mention_kind: MentionKind::FileMention,
        resolution_state: MentionResolutionState::UnresolvedScopeExcluded,
        resolved_target_ref: None,
        display_label: "Mention outside the workspace scope".to_owned(),
        in_scope: false,
    }
}

fn primary_attachment(id: &str) -> AttachmentProvenanceRow {
    AttachmentProvenanceRow {
        attachment_id: id.to_owned(),
        stable_object_ref: format!("object:{id}"),
        origin_label: "Workspace file slice".to_owned(),
        source_class: SourceClass::WorkspaceFileSlice,
        semantic_role: AttachmentSemanticRoleClass::PrimaryContext,
        provenance_class: AttachmentProvenanceClass::DirectWorkspace,
        trust_posture: TrustPosture::TrustedFirstParty,
        context_state: ContextItemStateClass::Included,
    }
}

fn docs_attachment(id: &str) -> AttachmentProvenanceRow {
    AttachmentProvenanceRow {
        attachment_id: id.to_owned(),
        stable_object_ref: format!("object:{id}"),
        origin_label: "Docs pack excerpt".to_owned(),
        source_class: SourceClass::DocsPackExcerpt,
        semantic_role: AttachmentSemanticRoleClass::ReferenceMaterial,
        provenance_class: AttachmentProvenanceClass::DocsKnowledgePack,
        trust_posture: TrustPosture::ReviewedDerived,
        context_state: ContextItemStateClass::Included,
    }
}

fn added_receipt(id: &str, source: &str) -> ContextChangeReceipt {
    ContextChangeReceipt {
        receipt_id: id.to_owned(),
        source_ref: source.to_owned(),
        source_class: SourceClass::WorkspaceFileSlice,
        change_kind: ContextChangeKindClass::AddedByUser,
        change_reason: ContextChangeReasonClass::UserAction,
        omission_reason: None,
        prior_state: ContextItemStateClass::NotRequested,
        new_state: ContextItemStateClass::Included,
        reversible: true,
        restore_action_ref: format!("action:context.restore:{id}"),
        inspect_action_ref: format!("action:context.inspect:{id}"),
        replay_visible: true,
    }
}

fn omitted_receipt(id: &str, source: &str) -> ContextChangeReceipt {
    ContextChangeReceipt {
        receipt_id: id.to_owned(),
        source_ref: source.to_owned(),
        source_class: SourceClass::WorkspaceSearchResult,
        change_kind: ContextChangeKindClass::OmittedUnderBudget,
        change_reason: ContextChangeReasonClass::BudgetPressure,
        omission_reason: Some(ContextOmissionReasonClass::Budget),
        prior_state: ContextItemStateClass::Included,
        new_state: ContextItemStateClass::Omitted,
        reversible: true,
        restore_action_ref: format!("action:context.restore:{id}"),
        inspect_action_ref: format!("action:context.inspect:{id}"),
        replay_visible: true,
    }
}

fn claimed_evidence(id: &str, safety: ReplaySafetyClass) -> ReplaySafeEvidence {
    ReplaySafeEvidence {
        evidence_id: format!("evidence:{id}"),
        replay_lineage_ref: format!("replay:{id}"),
        redaction_manifest_ref: format!("redaction:{id}"),
        route_receipt_ref: format!("route:{id}"),
        spend_receipt_ref: format!("spend:{id}"),
        operator_packet_ref: format!("operator:{id}"),
        support_packet_ref: format!("support:{id}"),
        compliance_packet_ref: format!("compliance:{id}"),
        replay_safety: safety,
        requires_raw_prompt_for_replay: false,
    }
}

fn unclaimed_evidence(id: &str) -> ReplaySafeEvidence {
    ReplaySafeEvidence {
        evidence_id: format!("evidence:{id}"),
        replay_lineage_ref: format!("replay:{id}"),
        redaction_manifest_ref: format!("redaction:{id}"),
        route_receipt_ref: String::new(),
        spend_receipt_ref: String::new(),
        operator_packet_ref: String::new(),
        support_packet_ref: String::new(),
        compliance_packet_ref: String::new(),
        replay_safety: ReplaySafetyClass::ReplayDegradedLabelsOnly,
        requires_raw_prompt_for_replay: false,
    }
}

fn action_refs(id: &str) -> (String, String, String) {
    (
        format!("action:artifact.inspect:{id}"),
        format!("action:artifact.delete:{id}"),
        format!("action:artifact.export:{id}"),
    )
}

fn composer_stable() -> SessionArtifactRow {
    let id = "composer-inline-edit";
    let (inspect, delete, export) = action_refs(id);
    SessionArtifactRow {
        artifact_id: id.to_owned(),
        session_ref: "session:composer-inline-edit".to_owned(),
        draft_ref: "draft:composer-inline-edit".to_owned(),
        context_snapshot_ref: "snapshot:composer-inline-edit".to_owned(),
        artifact_label: "Inline assist edit session".to_owned(),
        surface: M5AiWorkflowConsumerSurface::DesktopComposer,
        artifact_class: SessionArtifactClass::ActiveSessionArtifact,
        claimed_qualification: M5AiWorkflowQualificationClass::Stable,
        scope: MemoryScopeClass::Workspace,
        locality: MemoryLocalityClass::WorkspaceLocal,
        retention: MemoryRetentionClass::DurableUntilDeleted,
        delete_export_posture: MemoryDeleteExportPosture::WorkspaceScoped,
        attachment_provenance: vec![
            primary_attachment("att-edit-file"),
            docs_attachment("att-edit-docs"),
        ],
        mention_provenance: vec![
            resolved_mention(
                "men-edit-symbol",
                MentionKind::SymbolMention,
                "symbol:target",
            ),
            scope_excluded_mention("men-edit-excluded"),
        ],
        context_receipts: vec![
            added_receipt("ctx-edit-add", "source:edit-file"),
            omitted_receipt("ctx-edit-omit", "source:edit-search"),
        ],
        evidence: claimed_evidence(id, ReplaySafetyClass::FullyReplaySafe),
        downgrade_rules: vec![
            proof_stale_to(M5AiWorkflowQualificationClass::Beta),
            trust_narrowing_to(M5AiWorkflowQualificationClass::Experimental),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::FullyReversible,
        rollback_verified: true,
        inspect_action_ref: inspect,
        delete_action_ref: delete,
        export_action_ref: export,
    }
}

fn review_beta() -> SessionArtifactRow {
    let id = "review-patch-session";
    let (inspect, delete, export) = action_refs(id);
    SessionArtifactRow {
        artifact_id: id.to_owned(),
        session_ref: "session:review-patch".to_owned(),
        draft_ref: "draft:review-patch".to_owned(),
        context_snapshot_ref: "snapshot:review-patch".to_owned(),
        artifact_label: "Patch review session artifact".to_owned(),
        surface: M5AiWorkflowConsumerSurface::DesktopReviewWorkspace,
        artifact_class: SessionArtifactClass::ActiveSessionArtifact,
        claimed_qualification: M5AiWorkflowQualificationClass::Beta,
        scope: MemoryScopeClass::Workspace,
        locality: MemoryLocalityClass::WorkspaceLocal,
        retention: MemoryRetentionClass::UntilUserRevoked,
        delete_export_posture: MemoryDeleteExportPosture::UserScoped,
        attachment_provenance: vec![primary_attachment("att-review-diff")],
        mention_provenance: vec![resolved_mention(
            "men-review-run",
            MentionKind::RunMention,
            "run:target",
        )],
        context_receipts: vec![
            added_receipt("ctx-review-add", "source:review-diff"),
            omitted_receipt("ctx-review-omit", "source:review-search"),
        ],
        evidence: claimed_evidence(id, ReplaySafetyClass::ReplaySafeRedacted),
        downgrade_rules: vec![
            proof_stale_to(M5AiWorkflowQualificationClass::Preview),
            trust_narrowing_to(M5AiWorkflowQualificationClass::Experimental),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::CheckpointReversible,
        rollback_verified: true,
        inspect_action_ref: inspect,
        delete_action_ref: delete,
        export_action_ref: export,
    }
}

fn browser_draft_preview() -> SessionArtifactRow {
    let id = "browser-docs-draft";
    let (inspect, delete, export) = action_refs(id);
    SessionArtifactRow {
        artifact_id: id.to_owned(),
        session_ref: "session:browser-docs".to_owned(),
        draft_ref: "draft:browser-docs".to_owned(),
        context_snapshot_ref: "snapshot:browser-docs".to_owned(),
        artifact_label: "Docs companion draft".to_owned(),
        surface: M5AiWorkflowConsumerSurface::BrowserCompanion,
        artifact_class: SessionArtifactClass::DraftInProgress,
        claimed_qualification: M5AiWorkflowQualificationClass::Preview,
        scope: MemoryScopeClass::Thread,
        locality: MemoryLocalityClass::WorkspaceLocal,
        retention: MemoryRetentionClass::SessionOnly,
        delete_export_posture: MemoryDeleteExportPosture::EphemeralAutoExpire,
        attachment_provenance: vec![docs_attachment("att-docs-excerpt")],
        mention_provenance: vec![resolved_mention(
            "men-docs-anchor",
            MentionKind::DocsAnchorMention,
            "docs:anchor",
        )],
        context_receipts: vec![added_receipt("ctx-docs-add", "source:docs-excerpt")],
        evidence: claimed_evidence(id, ReplaySafetyClass::ReplaySafeRedacted),
        downgrade_rules: vec![
            proof_stale_to(M5AiWorkflowQualificationClass::Experimental),
            trust_narrowing_to(M5AiWorkflowQualificationClass::Held),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::NotApplicable,
        rollback_verified: false,
        inspect_action_ref: inspect,
        delete_action_ref: delete,
        export_action_ref: export,
    }
}

fn deleted_tombstone_held() -> SessionArtifactRow {
    let id = "composer-deleted-session";
    let (inspect, delete, export) = action_refs(id);
    SessionArtifactRow {
        artifact_id: id.to_owned(),
        session_ref: "session:composer-deleted".to_owned(),
        draft_ref: "draft:composer-deleted".to_owned(),
        context_snapshot_ref: "snapshot:composer-deleted".to_owned(),
        artifact_label: "Deleted composer session tombstone".to_owned(),
        surface: M5AiWorkflowConsumerSurface::DesktopComposer,
        artifact_class: SessionArtifactClass::DeletedTombstone,
        claimed_qualification: M5AiWorkflowQualificationClass::Held,
        scope: MemoryScopeClass::Workspace,
        locality: MemoryLocalityClass::WorkspaceLocal,
        retention: MemoryRetentionClass::DurableUntilDeleted,
        delete_export_posture: MemoryDeleteExportPosture::NotApplicable,
        attachment_provenance: vec![],
        mention_provenance: vec![],
        context_receipts: vec![ContextChangeReceipt {
            receipt_id: "ctx-deleted-revoke".to_owned(),
            source_ref: "source:deleted-session".to_owned(),
            source_class: SourceClass::WorkspaceFileSlice,
            change_kind: ContextChangeKindClass::PolicyFiltered,
            change_reason: ContextChangeReasonClass::ScopeRevoked,
            omission_reason: Some(ContextOmissionReasonClass::ScopeExcluded),
            prior_state: ContextItemStateClass::Included,
            new_state: ContextItemStateClass::Blocked,
            reversible: false,
            restore_action_ref: String::new(),
            inspect_action_ref: "action:context.inspect:ctx-deleted-revoke".to_owned(),
            replay_visible: true,
        }],
        evidence: unclaimed_evidence(id),
        downgrade_rules: vec![
            proof_stale_to(M5AiWorkflowQualificationClass::Unavailable),
            trust_narrowing_to(M5AiWorkflowQualificationClass::Unavailable),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::EvidencePreservedNoRevert,
        rollback_verified: false,
        inspect_action_ref: inspect,
        delete_action_ref: delete,
        export_action_ref: export,
    }
}

fn source_contracts() -> Vec<String> {
    vec![
        SESSION_ARTIFACT_SCHEMA_REF.to_owned(),
        SESSION_ARTIFACT_DOC_REF.to_owned(),
        RICHER_PROMPT_COMPOSER_SCHEMA_REF.to_owned(),
        M5_AI_WORKFLOW_MATRIX_SCHEMA_REF.to_owned(),
        MEMORY_CLASS_MATERIALIZATION_SCHEMA_REF.to_owned(),
        AI_RUN_RECEIPT_SCHEMA_REF.to_owned(),
    ]
}

fn packet() -> PromptSessionArtifactPacket {
    PromptSessionArtifactPacket::new(PromptSessionArtifactPacketInput {
        packet_id: PACKET_ID.to_owned(),
        catalogue_label: "M5 prompt-composer draft and session-artifact records".to_owned(),
        artifacts: vec![
            composer_stable(),
            review_beta(),
            browser_draft_preview(),
            deleted_tombstone_held(),
        ],
        proof_freshness: SessionArtifactProofFreshness {
            proof_freshness_slo_hours: 24,
            last_proof_refresh: MINTED_AT.to_owned(),
            auto_narrow_on_stale: true,
        },
        source_contract_refs: source_contracts(),
        redaction_class_token: "ai_session_artifact_review_safe".to_owned(),
        minted_at: MINTED_AT.to_owned(),
    })
}

#[test]
fn canonical_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.artifacts.len(), 4);
    assert_eq!(packet.claimed_artifact_count(), 3);
    assert_eq!(packet.draft_artifact_count(), 1);
}

#[test]
fn no_artifacts_fails() {
    let mut packet = packet();
    packet.artifacts.clear();
    assert!(packet
        .validate()
        .contains(&SessionArtifactViolation::NoArtifacts));
}

#[test]
fn duplicate_artifact_fails() {
    let mut packet = packet();
    let first = packet.artifacts[0].clone();
    packet.artifacts.push(first);
    assert!(packet
        .validate()
        .contains(&SessionArtifactViolation::DuplicateArtifact));
}

#[test]
fn surface_coverage_incomplete_fails() {
    let mut packet = packet();
    // Drop every browser-companion row, leaving the surface uncovered.
    packet
        .artifacts
        .retain(|a| a.surface != M5AiWorkflowConsumerSurface::BrowserCompanion);
    assert!(packet
        .validate()
        .contains(&SessionArtifactViolation::SurfaceCoverageIncomplete));
}

#[test]
fn artifact_row_incomplete_fails() {
    let mut packet = packet();
    packet.artifacts[0].artifact_label.clear();
    assert!(packet
        .validate()
        .contains(&SessionArtifactViolation::ArtifactRowIncomplete));
}

#[test]
fn draft_drifted_to_durable_memory_fails() {
    let mut packet = packet();
    // The browser draft is index 2; promoting it to durable retention drifts it
    // into hidden memory.
    packet.artifacts[2].retention = MemoryRetentionClass::DurableUntilDeleted;
    assert!(packet
        .validate()
        .contains(&SessionArtifactViolation::DraftDriftedToDurableMemory));
}

#[test]
fn durable_artifact_not_deletable_fails() {
    let mut packet = packet();
    packet.artifacts[0].delete_export_posture = MemoryDeleteExportPosture::EphemeralAutoExpire;
    assert!(packet
        .validate()
        .contains(&SessionArtifactViolation::DurableArtifactNotDeletable));
}

#[test]
fn scope_locality_mismatch_fails() {
    let mut packet = packet();
    // A workspace artifact must not sit in a tenant-wide pinned store.
    packet.artifacts[0].locality = MemoryLocalityClass::TenantRegionPinned;
    assert!(packet
        .validate()
        .contains(&SessionArtifactViolation::ScopeLocalityMismatch));
}

#[test]
fn org_scope_local_locality_fails() {
    let mut packet = packet();
    packet.artifacts[0].scope = MemoryScopeClass::Org;
    packet.artifacts[0].locality = MemoryLocalityClass::LocalDeviceOnly;
    assert!(packet
        .validate()
        .contains(&SessionArtifactViolation::ScopeLocalityMismatch));
}

#[test]
fn attachment_provenance_incomplete_fails() {
    let mut packet = packet();
    packet.artifacts[0].attachment_provenance[0]
        .origin_label
        .clear();
    assert!(packet
        .validate()
        .contains(&SessionArtifactViolation::AttachmentProvenanceIncomplete));
}

#[test]
fn mention_resolution_inconsistent_fails() {
    let mut packet = packet();
    // A resolved mention with no target is inconsistent.
    packet.artifacts[0].mention_provenance[0].resolved_target_ref = None;
    assert!(packet
        .validate()
        .contains(&SessionArtifactViolation::MentionResolutionInconsistent));
}

#[test]
fn scope_excluded_mention_in_scope_fails() {
    let mut packet = packet();
    // A scope-excluded mention may never be silently in scope.
    packet.artifacts[0].mention_provenance[1].in_scope = true;
    assert!(packet
        .validate()
        .contains(&SessionArtifactViolation::MentionResolutionInconsistent));
}

#[test]
fn context_receipt_incomplete_fails() {
    let mut packet = packet();
    // A reversible change must name its restore action.
    packet.artifacts[0].context_receipts[0]
        .restore_action_ref
        .clear();
    assert!(packet
        .validate()
        .contains(&SessionArtifactViolation::ContextReceiptIncomplete));
}

#[test]
fn context_change_not_replay_visible_fails() {
    let mut packet = packet();
    packet.artifacts[0].context_receipts[1].replay_visible = false;
    assert!(packet
        .validate()
        .contains(&SessionArtifactViolation::ContextChangeNotReplayVisible));
}

#[test]
fn context_change_reason_too_generic_fails() {
    let mut packet = packet();
    // An omission may not collapse into an unspecified reason.
    packet.artifacts[0].context_receipts[1].change_reason = ContextChangeReasonClass::Unspecified;
    assert!(packet
        .validate()
        .contains(&SessionArtifactViolation::ContextChangeReasonTooGeneric));
}

#[test]
fn evidence_lineage_incomplete_fails() {
    let mut packet = packet();
    packet.artifacts[0].evidence.replay_lineage_ref.clear();
    assert!(packet
        .validate()
        .contains(&SessionArtifactViolation::EvidenceLineageIncomplete));
}

#[test]
fn evidence_requires_raw_prompt_fails() {
    let mut packet = packet();
    packet.artifacts[0].evidence.requires_raw_prompt_for_replay = true;
    assert!(packet
        .validate()
        .contains(&SessionArtifactViolation::EvidenceRequiresRawPrompt));
}

#[test]
fn evidence_not_replay_safe_fails() {
    let mut packet = packet();
    packet.artifacts[0].evidence.replay_safety = ReplaySafetyClass::NotReplaySafe;
    assert!(packet
        .validate()
        .contains(&SessionArtifactViolation::EvidenceNotReplaySafe));
}

#[test]
fn claimed_artifact_missing_evidence_fails() {
    let mut packet = packet();
    packet.artifacts[0].evidence.compliance_packet_ref.clear();
    assert!(packet
        .validate()
        .contains(&SessionArtifactViolation::ClaimedArtifactMissingEvidence));
}

#[test]
fn claimed_rollback_unverified_fails() {
    let mut packet = packet();
    packet.artifacts[0].rollback_verified = false;
    assert!(packet
        .validate()
        .contains(&SessionArtifactViolation::ClaimedRollbackUnverified));
}

#[test]
fn downgrade_rules_missing_fails() {
    let mut packet = packet();
    packet.artifacts[0].downgrade_rules.clear();
    assert!(packet
        .validate()
        .contains(&SessionArtifactViolation::DowngradeRulesMissing));
}

#[test]
fn downgrade_rule_missing_proof_stale_fails() {
    let mut packet = packet();
    packet.artifacts[0]
        .downgrade_rules
        .retain(|rule| rule.trigger != M5AiWorkflowDowngradeTrigger::ProofStale);
    assert!(packet
        .validate()
        .contains(&SessionArtifactViolation::DowngradeRuleMissingProofStale));
}

#[test]
fn downgrade_rule_missing_trust_narrowing_fails() {
    let mut packet = packet();
    packet.artifacts[0]
        .downgrade_rules
        .retain(|rule| rule.trigger != M5AiWorkflowDowngradeTrigger::TrustNarrowing);
    assert!(packet
        .validate()
        .contains(&SessionArtifactViolation::DowngradeRuleMissingTrustNarrowing));
}

#[test]
fn downgrade_rule_not_narrowing_fails() {
    let mut packet = packet();
    packet.artifacts[0].downgrade_rules[0].narrowed_to = M5AiWorkflowQualificationClass::Stable;
    assert!(packet
        .validate()
        .contains(&SessionArtifactViolation::DowngradeRuleNotNarrowing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&SessionArtifactViolation::MissingSourceContracts));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&SessionArtifactViolation::ProofFreshnessIncomplete));
}

#[test]
fn raw_boundary_material_in_export_fails() {
    let mut packet = packet();
    packet.artifacts[0].attachment_provenance[0].origin_label =
        "https://api.vendor.example/object".to_owned();
    assert!(packet
        .validate()
        .contains(&SessionArtifactViolation::RawBoundaryMaterialInExport));
}

#[test]
fn narrowed_qualification_applies_matching_rule() {
    let artifact = composer_stable();
    assert_eq!(
        artifact.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProofStale),
        M5AiWorkflowQualificationClass::Beta
    );
    assert_eq!(
        artifact.narrowed_qualification(M5AiWorkflowDowngradeTrigger::TrustNarrowing),
        M5AiWorkflowQualificationClass::Experimental
    );
    // A trigger with no rule leaves the claim unchanged.
    assert_eq!(
        artifact.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProviderUnavailable),
        M5AiWorkflowQualificationClass::Stable
    );
}

#[test]
fn render_inspector_lists_every_posture() {
    let card = browser_draft_preview().render_inspector();
    assert!(card.contains("browser-docs-draft"));
    assert!(card.contains("draft_in_progress"));
    assert!(card.contains("browser_companion"));
    assert!(card.contains("replay_safe_redacted"));
}

#[test]
fn markdown_summary_lists_every_artifact() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("Artifact inspectors"));
    for artifact in &packet().artifacts {
        assert!(
            summary.contains(&artifact.artifact_id),
            "missing {}",
            artifact.artifact_id
        );
    }
}

#[test]
fn session_artifact_fixture_validates() {
    let packet: PromptSessionArtifactPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/ship_prompt_composer_draft_and_session_artifact_records_attachment_and_mention_provenance_context_add_or_remove_receipts/session_artifacts.json"
    )))
    .expect("session artifact fixture parses");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    // The browser draft stays scoped and replay-safe rather than drifting into
    // durable memory.
    let draft = packet
        .artifact("browser-docs-draft")
        .expect("browser draft present");
    assert!(draft.is_draft());
    assert!(!draft.retention.is_durable());
    assert!(draft.evidence.replay_safety.is_replay_safe());
    assert!(!draft.evidence.requires_raw_prompt_for_replay);

    // The deleted session is a tombstone whose context change is still inspectable
    // in replay.
    let tombstone = packet
        .artifact("composer-deleted-session")
        .expect("tombstone present");
    assert_eq!(
        tombstone.artifact_class,
        SessionArtifactClass::DeletedTombstone
    );
    assert!(tombstone.context_receipts.iter().all(|r| r.replay_visible));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_prompt_session_artifact_export()
        .expect("checked session artifact export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
    assert!(!packet.artifacts.is_empty());
    // Every required consumer surface is covered by at least one artifact.
    for surface in super::REQUIRED_SURFACES {
        assert!(
            packet.artifacts.iter().any(|a| a.surface == surface),
            "surface {} uncovered",
            surface.as_str()
        );
    }
}

/// Regenerates the checked-in support export and fixture from the canonical
/// builder. Gated behind `AURELINE_REGEN_SESSION_ARTIFACT=1` so normal test runs
/// never write to the working tree.
#[test]
fn regenerate_checked_artifacts() {
    if std::env::var("AURELINE_REGEN_SESSION_ARTIFACT").as_deref() != Ok("1") {
        return;
    }
    let manifest = env!("CARGO_MANIFEST_DIR");
    let packet = packet();
    let json = packet.export_safe_json();
    let artifact_path = format!(
        "{manifest}/../../artifacts/ai/m5/ship_prompt_composer_draft_and_session_artifact_records_attachment_and_mention_provenance_context_add_or_remove_receipts/support_export.json"
    );
    let fixture_path = format!(
        "{manifest}/../../fixtures/ai/m5/ship_prompt_composer_draft_and_session_artifact_records_attachment_and_mention_provenance_context_add_or_remove_receipts/session_artifacts.json"
    );
    std::fs::write(&artifact_path, format!("{json}\n")).expect("write support export");
    std::fs::write(&fixture_path, format!("{json}\n")).expect("write fixture");
}
