//! Canonical seed builders for the M5 AI execution/replay-component-consumer lane.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so
//! the in-code matrix, the artifact, the worked bindings, and the fixtures never
//! drift.

use super::*;

/// Stable packet id for the canonical execution/replay-component-consumer packet.
pub const M5_AI_EXECUTION_REPLAY_CONSUMER_PACKET_ID: &str =
    "m5-ai-execution-replay-component-consumer:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked binding case for one consumer/family adoption.
fn case(
    consumer: M5AiExecutionReplayConsumer,
    component_family: M5AiSharedComponent,
    replay_health: M5AiReplayHealth,
    export_caveats: &[M5AiExportCaveat],
    note: &str,
) -> M5AiReplayBindingCase {
    M5AiReplayBindingCase::resolved(M5AiReplayBindingInput {
        consumer,
        component_family,
        descriptor_families: M5AiReplayDescriptor::ALL.to_vec(),
        replay_health,
        export_caveats: export_caveats.to_vec(),
        note_repr: Some(note.to_owned()),
    })
}

/// Builds a component binding that points at its canonical family refs.
fn binding(
    component_family: M5AiSharedComponent,
    example_bindings: Vec<M5AiReplayBindingCase>,
) -> M5AiComponentBinding {
    M5AiComponentBinding {
        component_family,
        canonical_schema_ref: component_family.canonical_schema_ref().to_owned(),
        canonical_artifact_ref: component_family.canonical_artifact_ref().to_owned(),
        references_canonical_not_local_prose: true,
        example_bindings,
    }
}

/// A base row with the shared parity vocabulary filled in.
fn base_row(
    consumer: M5AiExecutionReplayConsumer,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    component_bindings: Vec<M5AiComponentBinding>,
) -> M5AiExecutionReplayConsumerRow {
    M5AiExecutionReplayConsumerRow {
        consumer,
        qualification: M5AiQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5AiSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5AiDeploymentLine::ALL.to_vec(),
        anatomy_parts: M5AiConsumerAnatomyPart::ALL.to_vec(),
        descriptor_families: M5AiReplayDescriptor::ALL.to_vec(),
        replay_health_modes: M5AiReplayHealth::ALL.to_vec(),
        export_caveats: M5AiExportCaveat::ALL.to_vec(),
        claim_parity_states: M5AiClaimParityState::ALL.to_vec(),
        narrowing_reasons: M5AiNarrowingReason::ALL.to_vec(),
        recovery_actions: M5AiRecoveryAction::ALL.to_vec(),
        export_fields: M5AiConsumerExportField::ALL.to_vec(),
        accessibility_routes: M5AiAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5AiConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5AiExecutionDowngradeTrigger::RouteOrProviderMasked,
            M5AiExecutionDowngradeTrigger::ApprovalGateHidden,
            M5AiExecutionDowngradeTrigger::CheckpointLineageBroken,
            M5AiExecutionDowngradeTrigger::ReplayCompletenessOverstated,
            M5AiExecutionDowngradeTrigger::ProofStale,
        ],
        component_bindings,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_AI_EXECUTION_REPLAY_CONSUMER_SCHEMA_REF,
            M5_AI_EXECUTION_REPLAY_CONSUMER_OBJECT_MODEL_REF,
        ]),
        rewords_claims_per_surface: false,
        invents_new_execution_grammar: false,
        drops_route_or_approval_when_narrowed: false,
        hides_drift_reason_or_takeover_path: false,
    }
}

// Keep the numbered contract cases beside their explanatory comments.
#[allow(clippy::vec_init_then_push)]
fn consumer_rows() -> Vec<M5AiExecutionReplayConsumerRow> {
    use M5AiExecutionReplayConsumer as Consumer;
    use M5AiExportCaveat as Caveat;
    use M5AiReplayHealth as Health;
    use M5AiSharedComponent as Family;

    let mut rows = Vec::new();

    // 1. Patch review — the action-state banner, approval sheet, and tool-call
    //    timeline at full replay, plus the replay / rerun-review sheet auto-narrowed
    //    when the route/provider/model drifts from what the run recorded.
    rows.push(base_row(
        Consumer::PatchReview,
        "Patch-review surface owner",
        "Patch review adopts the action-state banner, approval sheet, and tool-call timeline row at full replay, and the replay / rerun-review sheet auto-narrowed under route/provider/model drift, pointing at the canonical component schemas so route, approval, checkpoint, and replay-completeness language matches what the evidence inspector, branch queue, support export, and docs/help read",
        "evidence:m5-ai-replay-consumer-patch-review:001",
        vec![
            binding(
                Family::AiActionStateBanner,
                vec![case(
                    Consumer::PatchReview,
                    Family::AiActionStateBanner,
                    Health::FullReplay,
                    &[],
                    "patch-review action-state banner at full replay",
                )],
            ),
            binding(
                Family::ApprovalSheet,
                vec![case(
                    Consumer::PatchReview,
                    Family::ApprovalSheet,
                    Health::FullReplay,
                    &[],
                    "patch-review approval sheet at full replay",
                )],
            ),
            binding(
                Family::ToolCallTimelineRow,
                vec![case(
                    Consumer::PatchReview,
                    Family::ToolCallTimelineRow,
                    Health::FullReplay,
                    &[],
                    "patch-review tool-call timeline at full replay",
                )],
            ),
            binding(
                Family::ReplayReview,
                vec![case(
                    Consumer::PatchReview,
                    Family::ReplayReview,
                    Health::RouteProviderModelDrift,
                    &[Caveat::RouteMirroredNotLive],
                    "patch-review rerun sheet narrowed by route drift",
                )],
            ),
        ],
    ));

    // 2. Evidence inspector — the connector detail row auto-narrowed when a
    //    connector output is missing, plus the run-history row, replay-review sheet,
    //    and local-model pack card at full replay.
    rows.push(base_row(
        Consumer::EvidenceInspector,
        "Evidence-inspector surface owner",
        "The evidence inspector adopts the connector detail row auto-narrowed under a missing connector output, and the run-history row, replay / rerun-review sheet, and local-model pack card at full replay, referencing the canonical schemas so missing evidence narrows the claim instead of inheriting full replay language from healthier runs",
        "evidence:m5-ai-replay-consumer-evidence-inspector:001",
        vec![
            binding(
                Family::ConnectorDetailRow,
                vec![case(
                    Consumer::EvidenceInspector,
                    Family::ConnectorDetailRow,
                    Health::MissingConnectorOutput,
                    &[Caveat::PartialReplayOnly],
                    "evidence-inspector connector row narrowed by missing output",
                )],
            ),
            binding(
                Family::RunHistoryRow,
                vec![case(
                    Consumer::EvidenceInspector,
                    Family::RunHistoryRow,
                    Health::FullReplay,
                    &[],
                    "evidence-inspector run-history row at full replay",
                )],
            ),
            binding(
                Family::ReplayReview,
                vec![case(
                    Consumer::EvidenceInspector,
                    Family::ReplayReview,
                    Health::FullReplay,
                    &[],
                    "evidence-inspector replay-review sheet at full replay",
                )],
            ),
            binding(
                Family::LocalModelPackCard,
                vec![case(
                    Consumer::EvidenceInspector,
                    Family::LocalModelPackCard,
                    Health::FullReplay,
                    &[],
                    "evidence-inspector local-model pack card at full replay",
                )],
            ),
        ],
    ));

    // 3. Branch / worktree queue — the agent-status card, action-state banner, and
    //    run-history row at full replay, plus the approval sheet auto-narrowed when
    //    the authorising approval has gone stale.
    rows.push(base_row(
        Consumer::BranchWorktreeQueue,
        "Branch/worktree-queue surface owner",
        "The branch/worktree agent queue adopts the agent-status card, action-state banner, and run-history row at full replay, and the approval sheet auto-narrowed under a stale approval, keeping the manual-takeover path and drift reason explicit so an interrupted background agent never appears reusable by implication",
        "evidence:m5-ai-replay-consumer-branch-queue:001",
        vec![
            binding(
                Family::AgentStatus,
                vec![case(
                    Consumer::BranchWorktreeQueue,
                    Family::AgentStatus,
                    Health::FullReplay,
                    &[],
                    "branch-queue agent-status card at full replay",
                )],
            ),
            binding(
                Family::AiActionStateBanner,
                vec![case(
                    Consumer::BranchWorktreeQueue,
                    Family::AiActionStateBanner,
                    Health::FullReplay,
                    &[],
                    "branch-queue action-state banner at full replay",
                )],
            ),
            binding(
                Family::ApprovalSheet,
                vec![case(
                    Consumer::BranchWorktreeQueue,
                    Family::ApprovalSheet,
                    Health::StaleApproval,
                    &[Caveat::ApprovalReverificationRequired],
                    "branch-queue approval sheet narrowed by stale approval",
                )],
            ),
            binding(
                Family::RunHistoryRow,
                vec![case(
                    Consumer::BranchWorktreeQueue,
                    Family::RunHistoryRow,
                    Health::FullReplay,
                    &[],
                    "branch-queue run-history row at full replay",
                )],
            ),
        ],
    ));

    // 4. Support export — the run-history row, replay-review sheet, agent-status
    //    card, and tool-call timeline row, all at full replay, reconstructing
    //    consumer parity from the shared model.
    rows.push(base_row(
        Consumer::SupportExport,
        "Support-export owner",
        "The support export adopts the run-history row, replay / rerun-review sheet, agent-status card, and tool-call timeline row at full replay, reconstructing consumer parity from the shared model so a support reviewer reads the same run IDs, route truth, and drift reasons every product surface shows",
        "evidence:m5-ai-replay-consumer-support:001",
        vec![
            binding(
                Family::RunHistoryRow,
                vec![case(
                    Consumer::SupportExport,
                    Family::RunHistoryRow,
                    Health::FullReplay,
                    &[],
                    "support run-history row at full replay",
                )],
            ),
            binding(
                Family::ReplayReview,
                vec![case(
                    Consumer::SupportExport,
                    Family::ReplayReview,
                    Health::FullReplay,
                    &[],
                    "support replay-review sheet at full replay",
                )],
            ),
            binding(
                Family::AgentStatus,
                vec![case(
                    Consumer::SupportExport,
                    Family::AgentStatus,
                    Health::FullReplay,
                    &[],
                    "support agent-status card at full replay",
                )],
            ),
            binding(
                Family::ToolCallTimelineRow,
                vec![case(
                    Consumer::SupportExport,
                    Family::ToolCallTimelineRow,
                    Health::FullReplay,
                    &[],
                    "support tool-call timeline row at full replay",
                )],
            ),
        ],
    ));

    // 5. Docs / help — the connector detail row, action-state banner, and
    //    agent-status card at full replay, plus the local-model pack card
    //    auto-narrowed behind a redaction fence.
    rows.push(base_row(
        Consumer::DocsHelp,
        "Docs/help surface owner",
        "The docs/help surface adopts the connector detail row, action-state banner, and agent-status card at full replay, and the local-model pack card auto-narrowed behind a redaction fence, referencing the canonical component schemas so its prose can never drift from the product truth and the redaction caveat stays disclosed",
        "evidence:m5-ai-replay-consumer-docs-help:001",
        vec![
            binding(
                Family::ConnectorDetailRow,
                vec![case(
                    Consumer::DocsHelp,
                    Family::ConnectorDetailRow,
                    Health::FullReplay,
                    &[],
                    "docs/help connector detail row at full replay",
                )],
            ),
            binding(
                Family::AiActionStateBanner,
                vec![case(
                    Consumer::DocsHelp,
                    Family::AiActionStateBanner,
                    Health::FullReplay,
                    &[],
                    "docs/help action-state banner at full replay",
                )],
            ),
            binding(
                Family::AgentStatus,
                vec![case(
                    Consumer::DocsHelp,
                    Family::AgentStatus,
                    Health::FullReplay,
                    &[],
                    "docs/help agent-status card at full replay",
                )],
            ),
            binding(
                Family::LocalModelPackCard,
                vec![case(
                    Consumer::DocsHelp,
                    Family::LocalModelPackCard,
                    Health::RedactionFenced,
                    &[Caveat::RedactedFieldsWithheld],
                    "docs/help local-model card narrowed behind a redaction fence",
                )],
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5AiExecutionReplayConsumerGovernanceReview {
    M5AiExecutionReplayConsumerGovernanceReview {
        consumers_adopt_shared_primitives: true,
        consumers_reference_canonical_schema: true,
        descriptor_vocabulary_shared_not_reworded: true,
        no_consumer_invents_new_grammar: true,
        descriptors_explicit_on_every_surface: true,
        weakened_replay_auto_narrows_claim: true,
        narrowed_rendering_always_shows_self_contained_banner: true,
        banner_names_exact_reason_and_recovery_action: true,
        help_support_export_present_same_run_and_route_truth: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5AiExecutionReplayConsumerProjection {
    M5AiExecutionReplayConsumerProjection {
        all_consumers_adopt_shared_components: true,
        route_reads_single_source: true,
        approval_reads_single_source: true,
        checkpoint_reads_single_source: true,
        replay_completeness_reads_single_source: true,
    }
}

fn proof_freshness() -> M5AiExecutionReplayConsumerProofFreshness {
    M5AiExecutionReplayConsumerProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5AiExecutionReplayConsumerReleasePosture {
    M5AiExecutionReplayConsumerReleasePosture {
        release_packet_ref: M5_AI_EXECUTION_REPLAY_CONSUMER_ARTIFACT_REF.to_owned(),
        ai_audit_ref: M5_AI_EXECUTION_REPLAY_CONSUMER_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_AI_EXECUTION_REPLAY_CONSUMER_SCHEMA_REF,
        M5_AI_EXECUTION_REPLAY_CONSUMER_DOC_REF,
        M5_AI_EXECUTION_REPLAY_CONSUMER_COMPONENT_MATRIX_REF,
        M5_AI_EXECUTION_REPLAY_CONSUMER_OBJECT_MODEL_REF,
        M5AiSharedComponent::AiActionStateBanner.canonical_schema_ref(),
        M5AiSharedComponent::ConnectorDetailRow.canonical_schema_ref(),
        M5AiSharedComponent::ApprovalSheet.canonical_schema_ref(),
        M5AiSharedComponent::RunHistoryRow.canonical_schema_ref(),
        M5AiSharedComponent::ReplayReview.canonical_schema_ref(),
    ])
}

/// Builds the canonical M5 AI execution/replay-component-consumer packet.
pub fn seeded_m5_ai_execution_replay_consumer_packet() -> M5AiExecutionReplayConsumerPacket {
    M5AiExecutionReplayConsumerPacket::new(M5AiExecutionReplayConsumerPacketInput {
        packet_id: M5_AI_EXECUTION_REPLAY_CONSUMER_PACKET_ID.to_owned(),
        matrix_label:
            "M5 AI execution/replay-component consumers: patch review, evidence inspector, branch/worktree queue, support export, and docs/help keep route, approval, checkpoint-lineage, and replay-completeness parity"
                .to_owned(),
        consumer_rows: consumer_rows(),
        vocabulary_set: M5AiExecutionReplayConsumerVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the branch/worktree queue is held at Beta because a slice of
/// branch-agent renderings do not yet expose the auto-narrow banner on every
/// stale-approval path; every consumer stays visible.
pub fn seeded_m5_ai_execution_replay_consumer_branch_queue_beta_narrowed(
) -> M5AiExecutionReplayConsumerPacket {
    let mut packet = seeded_m5_ai_execution_replay_consumer_packet();
    packet.packet_id =
        "m5-ai-execution-replay-component-consumer:branch-queue-beta:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5AiExecutionReplayConsumer::BranchWorktreeQueue)
        .expect("branch/worktree-queue row present");
    row.qualification = M5AiQualificationClass::Beta;
    packet
}

/// Narrowed variant: the docs/help surface is narrowed to Preview pending redaction
/// caveat-parity proof across every fenced path; every consumer stays visible.
pub fn seeded_m5_ai_execution_replay_consumer_docs_help_preview_narrowed(
) -> M5AiExecutionReplayConsumerPacket {
    let mut packet = seeded_m5_ai_execution_replay_consumer_packet();
    packet.packet_id =
        "m5-ai-execution-replay-component-consumer:docs-help-preview:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5AiExecutionReplayConsumer::DocsHelp)
        .expect("docs/help row present");
    row.qualification = M5AiQualificationClass::Preview;
    packet
}
