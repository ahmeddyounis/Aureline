use std::path::{Path, PathBuf};

use aureline_search::RetrievalInspectorPacket;

use crate::context_inspector::{
    AiContextRetrievalExport, ComposerContextAlphaInput, ComposerContextAlphaSnapshot,
    ComposerContextItem, ComposerContextReviewLock, ContextFreshnessClass, ContextGroupClass,
    ContextItemStateClass, ContextLocalityClass, ContextOmissionReasonClass, ContextTrustClass,
    ExecutionBoundaryClass, IntentModeClass, ReviewLockClass,
};
use crate::evidence::{
    AiMutationEvidencePacket, AiMutationEvidencePacketInput, ApplyOutcomeClass, ApprovalActorClass,
    ApprovalDecisionClass, ApprovalLineageEntry, CitationVisibilityClass, CitedSourceClass,
    CitedSourceReference, EvidenceFreshnessClass, EvidenceRedactionClass, EvidenceSourcePosture,
    MutationEvidenceState, MutationIntentClass, MutationReviewLineage, RouteSpendLineage,
    ValidationOutcomeClass,
};
use crate::registry::ProviderModelRegistryPacket;
use crate::routing::RoutingRunStateClass;
use crate::routing_policy::SpendReceiptRecord;
use crate::{
    ComposerDraft, CostRoutingBetaViolation, SourceClass, COMPOSER_CONTEXT_ALPHA_SCHEMA_VERSION,
};

use super::*;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root canonicalizes")
}

fn fixture_registry() -> ProviderModelRegistryPacket {
    serde_json::from_str(include_str!(
        "../../../../../fixtures/ai/provider_model_registry_beta/registry_packet.json"
    ))
    .expect("provider/model registry fixture parses")
}

fn routing_packet() -> crate::routing::AiRoutingPacket {
    let mut packet = fixture_registry()
        .routing_packet_for_surface(
            "surface:review-chat-cheapest",
            "routing-packet:composer-context-evidence:beta:0001",
            "request-workspace:composer-context-evidence:beta:0001",
            "2026-05-17T12:45:00Z",
        )
        .expect("routing packet builds");
    packet.run_state_class = RoutingRunStateClass::PostRunCompleted;
    packet
}

fn spend_receipt(routing: &crate::routing::AiRoutingPacket) -> SpendReceiptRecord {
    SpendReceiptRecord::from_routing_packet(
        routing,
        "spend-receipt:composer-context-evidence:beta:0001",
        "route-receipt:composer-context-evidence:beta:0001",
        "assembly:composer-context-evidence:beta:0001",
        "2026-05-17T12:45:10Z",
    )
}

fn context_item(
    id: &str,
    group_class: ContextGroupClass,
    state_class: ContextItemStateClass,
    source_class: SourceClass,
    omission_reason_class: Option<ContextOmissionReasonClass>,
) -> ComposerContextItem {
    ComposerContextItem {
        context_item_id: id.to_owned(),
        group_class,
        state_class,
        source_class,
        stable_identity_ref: format!("stable:{id}"),
        display_label: format!("Context row {id}"),
        freshness_class: if state_class == ContextItemStateClass::Stale {
            ContextFreshnessClass::Stale
        } else {
            ContextFreshnessClass::AuthoritativeLive
        },
        trust_class: if group_class == ContextGroupClass::ExternalToolResults {
            ContextTrustClass::UntrustedExternal
        } else {
            ContextTrustClass::TrustedFirstParty
        },
        locality_class: if group_class == ContextGroupClass::RuntimeArtifacts {
            ContextLocalityClass::RemoteRuntime
        } else {
            ContextLocalityClass::LocalWorkspace
        },
        estimated_byte_size: 512,
        omission_reason_class,
        source_attachment_ref: None,
        source_mention_ref: None,
        docs_identity: None,
    }
}

fn context_items() -> Vec<ComposerContextItem> {
    vec![
        context_item(
            "ctx.included.file",
            ContextGroupClass::OpenFiles,
            ContextItemStateClass::Included,
            SourceClass::WorkspaceFileSlice,
            None,
        ),
        context_item(
            "ctx.pinned.symbol",
            ContextGroupClass::SymbolsGraphEntities,
            ContextItemStateClass::Pinned,
            SourceClass::WorkspaceSymbol,
            None,
        ),
        context_item(
            "ctx.omitted.history",
            ContextGroupClass::DiffsHistory,
            ContextItemStateClass::Omitted,
            SourceClass::WorkspaceSearchResult,
            Some(ContextOmissionReasonClass::Budget),
        ),
        context_item(
            "ctx.stale.runtime",
            ContextGroupClass::RuntimeArtifacts,
            ContextItemStateClass::Stale,
            SourceClass::TerminalTranscriptExcerpt,
            Some(ContextOmissionReasonClass::Stale),
        ),
        context_item(
            "ctx.trimmed.tool",
            ContextGroupClass::ExternalToolResults,
            ContextItemStateClass::Trimmed,
            SourceClass::TerminalTranscriptExcerpt,
            Some(ContextOmissionReasonClass::Budget),
        ),
    ]
}

fn snapshot(routing: &crate::routing::AiRoutingPacket) -> ComposerContextAlphaSnapshot {
    let draft = ComposerDraft::new(
        "turn-draft:composer-context-evidence:beta:0001",
        "composer-session:composer-context-evidence:beta:0001",
        "request-workspace:composer-context-evidence:beta:0001",
        "Review the payment retry change with the pinned context and omitted history visible.",
    );
    ComposerContextAlphaSnapshot::project(
        &draft,
        routing,
        ComposerContextAlphaInput {
            intent_mode: IntentModeClass::ReviewDiff,
            scope_label: "Selected workset".to_owned(),
            execution_boundary_class: ExecutionBoundaryClass::ManagedHosted,
            action_identity_ref: Some("cmd:ai.review_diff".to_owned()),
            mention_previews: Vec::new(),
            attachment_pills: Vec::new(),
            context_items: context_items(),
            graph_cue_packets: Vec::new(),
            review_lock: ComposerContextReviewLock {
                lock_class: ReviewLockClass::FrozenForReview,
                context_snapshot_ref: "context-snapshot:composer-context-evidence:beta:0001"
                    .to_owned(),
                route_snapshot_ref: routing.routing_packet_id.clone(),
                review_started_at: Some("2026-05-17T12:44:00Z".to_owned()),
            },
        },
    )
}

fn retrieval_export() -> AiContextRetrievalExport {
    let packet: RetrievalInspectorPacket = serde_json::from_str(include_str!(
        "../../../../../artifacts/search/m3/hybrid_retrieval_beta_packet.json"
    ))
    .expect("retrieval artifact parses");
    AiContextRetrievalExport::from_packet(
        "ai-context-export:composer-context-evidence:beta:0001",
        "context-snapshot:composer-context-evidence:beta:0001",
        "request-workspace:composer-context-evidence:beta:0001",
        "2026-05-17T12:45:01Z",
        packet,
    )
}

fn evidence_packet(
    routing: &crate::routing::AiRoutingPacket,
    snapshot: &ComposerContextAlphaSnapshot,
) -> AiMutationEvidencePacket {
    let packet = AiMutationEvidencePacket::new(AiMutationEvidencePacketInput {
        evidence_packet_id: "evidence-packet:composer-context-evidence:beta:0001".to_owned(),
        mutation_wedge_ref: "ai-mutation-wedge:composer-context-evidence:beta".to_owned(),
        composer_session_ref: snapshot.composer_session_id.clone(),
        turn_draft_ref: snapshot.composer_draft_id.clone(),
        request_workspace_ref: snapshot.request_workspace_id.clone(),
        assembly_ref: "assembly:composer-context-evidence:beta:0001".to_owned(),
        packet_state: MutationEvidenceState::Applied,
        intent_class: MutationIntentClass::LocalReversibleEdit,
        route_spend_lineage: RouteSpendLineage::from_routing_packet(
            routing,
            "route-receipt:composer-context-evidence:beta:0001",
            "spend-receipt:composer-context-evidence:beta:0001",
        ),
        approval_lineage: vec![ApprovalLineageEntry {
            approval_lineage_id: "approval-lineage:composer-context-evidence:beta:0001".to_owned(),
            approval_ticket_ref: "approval-ticket:composer-context-evidence:beta:0001".to_owned(),
            decision_class: ApprovalDecisionClass::Approved,
            actor_class: ApprovalActorClass::LocalUser,
            preview_ref: "review-surface:composer-context-evidence:beta:0001".to_owned(),
            policy_epoch_ref: routing.policy_context.policy_epoch_ref.clone(),
            decided_at: "2026-05-17T12:45:05Z".to_owned(),
            summary_label: "User approved the reviewed AI patch proposal after context inspection."
                .to_owned(),
        }],
        cited_sources: vec![CitedSourceReference {
            source_reference_id: "source-ref:composer-context-evidence:beta:symbol".to_owned(),
            source_class: CitedSourceClass::WorkspaceSymbol,
            source_identity_ref: "symbol:payments.retry.policy".to_owned(),
            source_revision_ref: Some(
                "workspace-revision:composer-context-evidence:beta".to_owned(),
            ),
            docs_pack_ref: None,
            docs_pack_revision_ref: None,
            exact_anchor_ref: None,
            docs_node_identity: None,
            docs_node_provenance: None,
            citation_anchor: None,
            citation_visibility_class: CitationVisibilityClass::NotCitationBearing,
            hidden_or_omitted_citation_note: None,
            source_posture: EvidenceSourcePosture::TrustedFirstParty,
            freshness_class: EvidenceFreshnessClass::AuthoritativeLive,
        }],
        derived_explanations: Vec::new(),
        tainted_context_fences: Vec::new(),
        tool_call_lineage_refs: vec![
            "tool-call-lineage:composer-context-evidence:beta:0001".to_owned()
        ],
        context_handoff: Some(
            snapshot.evidence_handoff("context-handoff:composer-context-evidence:beta:0001"),
        ),
        review_lineage: MutationReviewLineage {
            review_surface_ref: "review-surface:composer-context-evidence:beta:0001".to_owned(),
            patch_review_summary_ref: "patch-review:composer-context-evidence:beta:0001".to_owned(),
            produced_artifact_refs: vec![
                "patch-artifact:composer-context-evidence:beta:0001".to_owned()
            ],
            changed_file_count: 2,
            generated_artifact_count: 0,
            validation_summary_refs: vec![
                "validation:composer-context-evidence:beta:0001".to_owned()
            ],
            validation_outcome_class: ValidationOutcomeClass::Passed,
            rollback_checkpoint_ref: Some(
                "checkpoint:composer-context-evidence:beta:0001".to_owned(),
            ),
            mutation_journal_ref: Some(
                "mutation-journal:composer-context-evidence:beta:0001".to_owned(),
            ),
            apply_outcome_class: ApplyOutcomeClass::AppliedSuccess,
        },
        source_contract_refs: source_contract_refs(),
        policy_context: routing.policy_context.clone(),
        running_build_identity_ref: "build-identity:aureline:beta:2026-05-17".to_owned(),
        redaction_class: EvidenceRedactionClass::MetadataSafeDefault,
        minted_at: "2026-05-17T12:45:00Z".to_owned(),
        completed_at: Some("2026-05-17T12:45:20Z".to_owned()),
    });
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    packet
}

fn source_contract_refs() -> Vec<String> {
    vec![
        COMPOSER_CONTEXT_EVIDENCE_BETA_AI_DOC_REF.to_owned(),
        COMPOSER_CONTEXT_EVIDENCE_BETA_UX_DOC_REF.to_owned(),
        COMPOSER_CONTEXT_EVIDENCE_BETA_SCHEMA_REF.to_owned(),
        "docs/ai/context_assembly_contract.md".to_owned(),
        "docs/ai/evidence_replayability_contract.md".to_owned(),
        "docs/ai/spend_and_route_receipt_contract.md".to_owned(),
        "schemas/ai/spend_receipt.schema.json".to_owned(),
        "schemas/ai/evidence_packet.schema.json".to_owned(),
    ]
}

fn surface_rows(
    snapshot: &ComposerContextAlphaSnapshot,
    evidence_packet: &AiMutationEvidencePacket,
) -> Vec<ComposerContextEvidenceSurfaceRow> {
    let tokens = vec![
        "included".to_owned(),
        "pinned".to_owned(),
        "omitted".to_owned(),
        "stale".to_owned(),
        "trimmed".to_owned(),
    ];
    [
        ComposerContextEvidenceSurfaceClass::Composer,
        ComposerContextEvidenceSurfaceClass::ContextInspector,
        ComposerContextEvidenceSurfaceClass::ReviewWorkspace,
        ComposerContextEvidenceSurfaceClass::DocsHelp,
        ComposerContextEvidenceSurfaceClass::SupportExport,
        ComposerContextEvidenceSurfaceClass::Cli,
    ]
    .into_iter()
    .map(|surface_class| ComposerContextEvidenceSurfaceRow {
        surface_class,
        projection_ref: format!(
            "projection:{}:composer-context-evidence:beta:0001",
            surface_class.as_str()
        ),
        composer_context_snapshot_ref: snapshot.review_lock.context_snapshot_ref.clone(),
        evidence_packet_ref: evidence_packet.evidence_packet_id.clone(),
        route_receipt_ref: evidence_packet
            .route_spend_lineage
            .route_receipt_ref
            .clone(),
        spend_receipt_ref: evidence_packet
            .route_spend_lineage
            .spend_receipt_ref
            .clone(),
        context_state_tokens: tokens.clone(),
        preserves_operator_truth: true,
        raw_private_material_excluded: true,
        supports_json_export: true,
        supports_markdown_summary: true,
    })
    .collect()
}

fn beta_packet() -> ComposerContextEvidenceBetaPacket {
    let routing = routing_packet();
    let snapshot = snapshot(&routing);
    assert!(snapshot.validate().is_empty(), "{:?}", snapshot.validate());
    let retrieval_export = retrieval_export();
    assert!(
        retrieval_export.validate().is_empty(),
        "{:?}",
        retrieval_export.validate()
    );
    let evidence_packet = evidence_packet(&routing, &snapshot);
    let spend_receipt = spend_receipt(&routing);
    assert!(
        spend_receipt.validate().is_empty(),
        "{:?}",
        spend_receipt.validate()
    );
    assert!(!spend_receipt
        .validate()
        .contains(&CostRoutingBetaViolation::SpendReceiptMissingAttributionDimensions));

    ComposerContextEvidenceBetaPacket::from_runtime_parts(
        &snapshot,
        &retrieval_export,
        &evidence_packet,
        &spend_receipt,
        ComposerContextEvidenceBetaInput {
            packet_id: "composer-context-evidence:beta:0001".to_owned(),
            workflow_or_surface_id: "surface:review-chat-cheapest".to_owned(),
            display_label: "Review chat composer context evidence beta".to_owned(),
            surface_rows: surface_rows(&snapshot, &evidence_packet),
            source_contract_refs: source_contract_refs(),
            json_export_ref: COMPOSER_CONTEXT_EVIDENCE_BETA_ARTIFACT_REF.to_owned(),
            markdown_summary_ref: "artifacts/ai/m3/composer_context_evidence_beta_summary.md"
                .to_owned(),
            minted_at: "2026-05-17T12:46:00Z".to_owned(),
        },
    )
}

#[test]
fn generated_beta_packet_validates_claimed_context_evidence_and_spend_truth() {
    let packet = beta_packet();

    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.observed_context_state_tokens,
        vec![
            "included".to_owned(),
            "pinned".to_owned(),
            "omitted".to_owned(),
            "stale".to_owned(),
            "trimmed".to_owned()
        ]
    );
    assert_eq!(packet.evidence_packet_state_token, "applied");
    assert_eq!(
        packet.tool_call_lineage_refs,
        vec!["tool-call-lineage:composer-context-evidence:beta:0001"]
    );
    assert_eq!(
        packet.spend_receipt_ref,
        "spend-receipt:composer-context-evidence:beta:0001"
    );
    assert_eq!(packet.retrieval_promotion_state_token, "promotable");
}

#[test]
fn checked_in_support_export_matches_generated_projection() {
    let generated: serde_json::Value =
        serde_json::from_str(&beta_packet().export_safe_json()).expect("generated parses");
    let checked_in: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../../artifacts/ai/m3/composer_context_evidence_beta_support_export.json"
    ))
    .expect("checked-in support export parses");

    assert_eq!(generated, checked_in);
}

#[test]
fn current_checked_in_support_export_validates_and_renders_markdown() {
    let packet = current_beta_composer_context_evidence_support_export()
        .expect("current support export parses and validates");
    let markdown = packet.render_markdown_summary();

    assert!(markdown.contains("composer-context-evidence:beta:0001"));
    assert!(markdown.contains("Tool lineage"));
    assert!(!packet.export_safe_json().contains("://"));
}

#[test]
fn missing_trimmed_context_blocks_beta_claim() {
    let mut packet = beta_packet();
    packet
        .observed_context_state_tokens
        .retain(|token| token != "trimmed");

    assert!(packet
        .validate()
        .contains(&ComposerContextEvidenceBetaViolation::MissingContextStateCoverage));
}

#[test]
fn missing_tool_lineage_or_spend_mismatch_blocks_beta_claim() {
    let mut missing_tool = beta_packet();
    missing_tool.tool_call_lineage_refs.clear();
    assert!(missing_tool
        .validate()
        .contains(&ComposerContextEvidenceBetaViolation::MissingToolLineage));

    let mut spend_mismatch = beta_packet();
    spend_mismatch.spend_receipt_cost_envelope_token =
        "envelope_unknown_unverified_cost".to_owned();
    assert!(spend_mismatch
        .validate()
        .contains(&ComposerContextEvidenceBetaViolation::SpendReceiptMismatch));
}

#[test]
fn surface_projection_drift_blocks_beta_claim() {
    let mut packet = beta_packet();
    packet.surface_rows[0].spend_receipt_ref = "spend-receipt:drifted".to_owned();

    assert!(packet
        .validate()
        .contains(&ComposerContextEvidenceBetaViolation::SurfaceProjectionDrift));
}

#[test]
fn schema_doc_fixture_and_artifact_paths_exist() {
    for rel in [
        COMPOSER_CONTEXT_EVIDENCE_BETA_SCHEMA_REF,
        COMPOSER_CONTEXT_EVIDENCE_BETA_AI_DOC_REF,
        COMPOSER_CONTEXT_EVIDENCE_BETA_UX_DOC_REF,
        COMPOSER_CONTEXT_EVIDENCE_BETA_FIXTURE_DIR,
        COMPOSER_CONTEXT_EVIDENCE_BETA_ARTIFACT_REF,
        "artifacts/ai/m3/composer_context_evidence_beta_summary.md",
    ] {
        assert!(repo_root().join(rel).exists(), "{rel} should exist");
    }
    assert_eq!(COMPOSER_CONTEXT_ALPHA_SCHEMA_VERSION, 1);
}
