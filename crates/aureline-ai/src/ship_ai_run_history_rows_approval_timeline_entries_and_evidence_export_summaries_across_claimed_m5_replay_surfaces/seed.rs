//! Canonical seed builders for the M5 AI run-history-row / approval-timeline-entry /
//! evidence-export-summary primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical run-history / approval-timeline / evidence-export
/// packet.
pub const M5_AI_RUN_HISTORY_EXPORT_PRIMITIVE_PACKET_ID: &str =
    "m5-ai-run-history-row-approval-timeline-entry-evidence-export-summary-primitive:stable:0001";

/// The canonical run identity threaded through a run-history example, an approval example,
/// and an evidence example so the same AI run appears consistently across surfaces.
const SHARED_RUN_ID: &str = "run-2026-07-06-0007";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// A pinned expiry timestamp for grants that carry one.
const EXPIRY_TIMESTAMP: &str = "2026-07-20T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked run-history resolution case from a full run state.
#[allow(clippy::too_many_arguments)]
fn rh_case(
    canonical_run_id: &str,
    task_label: &str,
    occurred_at_label: &str,
    provider_label: &str,
    model_label: &str,
    execution_mode: M5AiExecutionMode,
    run_outcome: M5AiRunOutcome,
    support_linked: bool,
    has_approvals: bool,
) -> M5AiRunHistoryResolutionCase {
    M5AiRunHistoryResolutionCase::resolved(M5AiRunHistoryResolutionInput {
        canonical_run_id: canonical_run_id.to_owned(),
        task_label: task_label.to_owned(),
        occurred_at_label: occurred_at_label.to_owned(),
        provider_label: provider_label.to_owned(),
        model_label: model_label.to_owned(),
        execution_mode,
        run_outcome,
        support_linked,
        has_approvals,
    })
}

/// Builds a worked approval-timeline resolution case from a full grant state.
#[allow(clippy::too_many_arguments)]
fn ap_case(
    approval_id: &str,
    run_id_label: &str,
    actor_label: &str,
    actor_class: M5AiApprovalActorClass,
    grant_scope: M5AiApprovalGrantScope,
    policy_epoch_label: &str,
    gate: M5AiApprovalGate,
    expiry_label: &str,
    has_expiry: bool,
    is_revoked: bool,
    is_single_use: bool,
    single_use_consumed: bool,
    is_expired: bool,
    expiring_soon: bool,
) -> M5AiApprovalTimelineResolutionCase {
    M5AiApprovalTimelineResolutionCase::resolved(M5AiApprovalTimelineResolutionInput {
        approval_id: approval_id.to_owned(),
        run_id_label: run_id_label.to_owned(),
        actor_label: actor_label.to_owned(),
        actor_class,
        grant_scope,
        policy_epoch_label: policy_epoch_label.to_owned(),
        gate,
        expiry_label: expiry_label.to_owned(),
        has_expiry,
        is_revoked,
        is_single_use,
        single_use_consumed,
        is_expired,
        expiring_soon,
        inspectable: true,
    })
}

/// Builds a worked evidence / export summary resolution case from a full packet state.
fn ev_case(
    packet_id: &str,
    run_id_label: &str,
    artifact_classes: &[M5AiEvidenceArtifactClass],
    redaction_posture: M5AiRedactionPosture,
    support_linkage: M5AiSupportLinkage,
    export_formats: &[M5AiExportFormat],
) -> M5AiEvidenceSummaryResolutionCase {
    M5AiEvidenceSummaryResolutionCase::resolved(M5AiEvidenceSummaryResolutionInput {
        packet_id: packet_id.to_owned(),
        run_id_label: run_id_label.to_owned(),
        artifact_classes: artifact_classes.to_vec(),
        redaction_posture,
        support_linkage,
        export_formats: export_formats.to_vec(),
        offers_structured_summary: true,
    })
}

/// A base row with the shared fields filled in and the full run-history, approval-timeline,
/// and evidence-summary anatomy, entry-point, vocabulary, export-field, and accessibility
/// parity every surface carries.
#[allow(clippy::too_many_arguments)]
fn base_row(
    replay_surface: M5AiReplaySurface,
    qualification: M5AiQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    run_history_examples: Vec<M5AiRunHistoryResolutionCase>,
    approval_timeline_examples: Vec<M5AiApprovalTimelineResolutionCase>,
    evidence_summary_examples: Vec<M5AiEvidenceSummaryResolutionCase>,
) -> M5AiRunHistoryExportRow {
    M5AiRunHistoryExportRow {
        replay_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5AiSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5AiDeploymentLine::ALL.to_vec(),
        run_history_anatomy_parts: M5AiRunHistoryAnatomyPart::ALL.to_vec(),
        approval_timeline_anatomy_parts: M5AiApprovalTimelineAnatomyPart::ALL.to_vec(),
        evidence_summary_anatomy_parts: M5AiEvidenceSummaryAnatomyPart::ALL.to_vec(),
        entry_points: M5AiRunHistoryEntryPoint::ALL.to_vec(),
        execution_modes: M5AiExecutionMode::ALL.to_vec(),
        run_outcomes: M5AiRunOutcome::ALL.to_vec(),
        approval_actor_classes: M5AiApprovalActorClass::ALL.to_vec(),
        approval_grant_scopes: M5AiApprovalGrantScope::ALL.to_vec(),
        approval_expiry_states: M5AiApprovalExpiryState::ALL.to_vec(),
        approval_gates: M5AiApprovalGate::ALL.to_vec(),
        artifact_classes: M5AiEvidenceArtifactClass::ALL.to_vec(),
        redaction_postures: M5AiRedactionPosture::ALL.to_vec(),
        support_linkages: M5AiSupportLinkage::ALL.to_vec(),
        export_formats: M5AiExportFormat::ALL.to_vec(),
        run_history_export_fields: M5AiRunHistoryExportField::ALL.to_vec(),
        approval_timeline_export_fields: M5AiApprovalTimelineExportField::ALL.to_vec(),
        evidence_summary_export_fields: M5AiEvidenceSummaryExportField::ALL.to_vec(),
        accessibility_routes: M5AiAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5AiConsumerSurface::RunHistoryUi,
            M5AiConsumerSurface::ReplayReviewUi,
            M5AiConsumerSurface::SupportExport,
            M5AiConsumerSurface::CliInspect,
            M5AiConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5AiExecutionDowngradeTrigger::RouteOrProviderMasked,
            M5AiExecutionDowngradeTrigger::ReplayCompletenessOverstated,
            M5AiExecutionDowngradeTrigger::RerunReviewReasonUnstated,
            M5AiExecutionDowngradeTrigger::CheckpointLineageBroken,
            M5AiExecutionDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_AI_RUN_HISTORY_EXPORT_RUN_HISTORY_REF,
            M5_AI_RUN_HISTORY_EXPORT_APPROVAL_REF,
            M5_AI_RUN_HISTORY_EXPORT_EVIDENCE_REF,
        ]),
        run_history_examples,
        approval_timeline_examples,
        evidence_summary_examples,
        masks_run_identity_across_surfaces: false,
        collapses_multiple_grants_into_one_badge: false,
        offers_raw_download_links_only: false,
        invents_parallel_history_or_export_grammar: false,
    }
}

#[allow(clippy::vec_init_then_push)]
fn rows() -> Vec<M5AiRunHistoryExportRow> {
    use M5AiApprovalActorClass as Actor;
    use M5AiApprovalGate as Gate;
    use M5AiApprovalGrantScope as GrantScope;
    use M5AiEvidenceArtifactClass as Artifact;
    use M5AiExecutionMode as Mode;
    use M5AiExportFormat as Fmt;
    use M5AiRedactionPosture as Redaction;
    use M5AiRunOutcome as Outcome;
    use M5AiSupportLinkage as Linkage;

    let mut rows = Vec::new();

    // 1. Run-history surface — the shared run identity is anchored here (run-history
    //    example, approval example, and evidence example all cite the same canonical id),
    //    and the row keeps the stable open / replay / export entry points.
    rows.push(base_row(
        M5AiReplaySurface::RunHistory,
        M5AiQualificationClass::Stable,
        "Run-history surface owner",
        "The run-history surface renders the shared run-history row, approval-timeline entry, and evidence-export summary so one canonical run id, task label, time, provider/model route, and outcome stay visible with stable open/replay/export entry points, and the same run identity anchors its approval-timeline and evidence summaries",
        "evidence:m5-ai-run-history-export-run-history:001",
        vec![
            rh_case(
                SHARED_RUN_ID,
                "Refactor auth module",
                "2026-07-06T10:00:00Z",
                "provider.managed-a",
                "model.opus-4",
                Mode::ForegroundAssistant,
                Outcome::Succeeded,
                true,
                true,
            ),
            rh_case(
                "run-2026-07-06-0008",
                "Draft migration plan",
                "2026-07-06T10:20:00Z",
                "provider.local-oss",
                "model.local-mixtral",
                Mode::GuidedPatch,
                Outcome::Failed,
                false,
                false,
            ),
        ],
        vec![
            ap_case(
                "appr-0007-a",
                SHARED_RUN_ID,
                "owner.alex",
                Actor::WorkspaceOwner,
                GrantScope::Workspace,
                "policy-epoch-2026-07",
                Gate::OneClickConfirm,
                EXPIRY_TIMESTAMP,
                true,
                false,
                false,
                false,
                false,
                false,
            ),
            ap_case(
                "appr-0007-b",
                "run-2026-07-06-0008",
                "reviewer.blake",
                Actor::DelegatedReviewer,
                GrantScope::Task,
                "policy-epoch-2026-07",
                Gate::HighFrictionTyped,
                EXPIRY_TIMESTAMP,
                true,
                false,
                false,
                false,
                false,
                true,
            ),
        ],
        vec![
            ev_case(
                "evp-0007-a",
                SHARED_RUN_ID,
                &[
                    Artifact::PromptTranscript,
                    Artifact::ToolCallLog,
                    Artifact::ApprovalLineage,
                ],
                Redaction::CredentialsRedacted,
                Linkage::LinkedOpenTicket,
                &[Fmt::JsonBundle, Fmt::MarkdownReport],
            ),
            ev_case(
                "evp-0007-b",
                "run-2026-07-06-0008",
                &[Artifact::DiffPacket, Artifact::ValidationReceipt],
                Redaction::FullyRedacted,
                Linkage::LinkedResolvedTicket,
                &[Fmt::SignedArchive],
            ),
        ],
    ));

    // 2. Evidence-packet surface — a superseded background-agent run and an awaiting-review
    //    review-first run; a two-person tenant grant that has expired and a policy-engine
    //    global grant that was revoked.
    rows.push(base_row(
        M5AiReplaySurface::EvidencePacket,
        M5AiQualificationClass::Stable,
        "Evidence-packet surface owner",
        "The evidence-packet surface renders the shared components so an evidence packet keeps the run identity, discloses its included artifact classes and redaction posture, preserves support-packet linkage, and shows expired or revoked approval grants as no longer effective rather than a vague approved badge",
        "evidence:m5-ai-run-history-export-evidence-packet:001",
        vec![
            rh_case(
                "run-2026-07-06-0009",
                "Summarize incident timeline",
                "2026-07-06T11:00:00Z",
                "provider.managed-b",
                "model.sonnet-4",
                Mode::BackgroundBranchAgent,
                Outcome::Superseded,
                true,
                false,
            ),
            rh_case(
                "run-2026-07-06-0010",
                "Propose patch for parser",
                "2026-07-06T11:30:00Z",
                "provider.self-hosted",
                "model.internal-7b",
                Mode::ReviewFirstPlacement,
                Outcome::AwaitingReview,
                false,
                true,
            ),
        ],
        vec![
            ap_case(
                "appr-0009-a",
                "run-2026-07-06-0009",
                "security.casey",
                Actor::SecurityReviewer,
                GrantScope::Tenant,
                "policy-epoch-2026-06",
                Gate::TwoPersonReview,
                EXPIRY_TIMESTAMP,
                true,
                false,
                false,
                false,
                true,
                false,
            ),
            ap_case(
                "appr-0009-b",
                "run-2026-07-06-0009",
                "policy.engine",
                Actor::PolicyEngine,
                GrantScope::Global,
                "policy-epoch-2026-06",
                Gate::PolicyBlocked,
                "",
                false,
                true,
                false,
                false,
                false,
                false,
            ),
        ],
        vec![
            ev_case(
                "evp-0009-a",
                "run-2026-07-06-0009",
                &[Artifact::RouteReceipt, Artifact::SpendReceipt],
                Redaction::PiiRedacted,
                Linkage::LinkedInternalCase,
                &[Fmt::CsvTable],
            ),
            ev_case(
                "evp-0009-b",
                "run-2026-07-06-0009",
                &[Artifact::RedactionManifest, Artifact::PromptTranscript],
                Redaction::RedactionPending,
                Linkage::LinkagePendingConsent,
                &[Fmt::RedactedPdf],
            ),
        ],
    ));

    // 3. Export surface — a partially-applied headless run and a cancelled foreground run;
    //    a consumed single-use automation grant and a session-scoped auto-approved grant;
    //    an export carrying every artifact class and format, and an unshareable unredacted
    //    and redaction-failed packet.
    rows.push(base_row(
        M5AiReplaySurface::Export,
        M5AiQualificationClass::Stable,
        "Export surface owner",
        "The export surface renders the shared components so an export summary lists the packet id, every included artifact class, its redaction posture, support linkage, and supported export formats, keeps an unredacted or redaction-failed packet out of the shareable state, and never collapses to a raw-file download link",
        "evidence:m5-ai-run-history-export-export:001",
        vec![
            rh_case(
                "run-2026-07-06-0011",
                "Batch lint fixes",
                "2026-07-06T12:00:00Z",
                "provider.managed-a",
                "model.haiku-4",
                Mode::HeadlessAutomation,
                Outcome::PartiallyApplied,
                true,
                true,
            ),
            rh_case(
                "run-2026-07-06-0012",
                "Explain failing test",
                "2026-07-06T12:15:00Z",
                "provider.managed-a",
                "model.opus-4",
                Mode::ForegroundAssistant,
                Outcome::Cancelled,
                false,
                false,
            ),
        ],
        vec![
            ap_case(
                "appr-0011-a",
                "run-2026-07-06-0011",
                "automation.agent-1",
                Actor::AutomationAgent,
                GrantScope::SingleAction,
                "policy-epoch-2026-07",
                Gate::NotifyOnly,
                "",
                false,
                false,
                true,
                true,
                false,
                false,
            ),
            ap_case(
                "appr-0011-b",
                "run-2026-07-06-0011",
                "owner.alex",
                Actor::WorkspaceOwner,
                GrantScope::Session,
                "policy-epoch-2026-07",
                Gate::AutoApproved,
                "",
                false,
                false,
                false,
                false,
                false,
                false,
            ),
        ],
        vec![
            ev_case(
                "evp-0011-a",
                "run-2026-07-06-0011",
                &[
                    Artifact::PromptTranscript,
                    Artifact::ToolCallLog,
                    Artifact::DiffPacket,
                    Artifact::RouteReceipt,
                    Artifact::SpendReceipt,
                    Artifact::ApprovalLineage,
                    Artifact::ValidationReceipt,
                    Artifact::RedactionManifest,
                ],
                Redaction::Unredacted,
                Linkage::NotLinked,
                &[
                    Fmt::JsonBundle,
                    Fmt::MarkdownReport,
                    Fmt::CsvTable,
                    Fmt::SignedArchive,
                    Fmt::RedactedPdf,
                ],
            ),
            ev_case(
                "evp-0011-b",
                "run-2026-07-06-0011",
                &[Artifact::ApprovalLineage],
                Redaction::RedactionFailed,
                Linkage::LinkedOpenTicket,
                &[Fmt::JsonBundle],
            ),
        ],
    ));

    // 4. Support surface — two support-linked runs a support reviewer reconstructs from the
    //    export alone; two more active / expiring grants; two fully-redacted, support-linked
    //    packets proving redaction and support continuity are preserved.
    rows.push(base_row(
        M5AiReplaySurface::Support,
        M5AiQualificationClass::Stable,
        "Support-desk surface owner",
        "The support-desk surface renders the shared components so a support reviewer reconstructs the run identity, its approvals, and its evidence summary — packet id, artifact classes, redaction posture, and support linkage — from the export alone, with redaction and support-continuity state preserved rather than a raw file download",
        "evidence:m5-ai-run-history-export-support:001",
        vec![
            rh_case(
                "run-2026-07-06-0013",
                "Reproduce crash report",
                "2026-07-06T13:00:00Z",
                "provider.managed-b",
                "model.sonnet-4",
                Mode::BackgroundBranchAgent,
                Outcome::Succeeded,
                true,
                true,
            ),
            rh_case(
                "run-2026-07-06-0014",
                "Guided dependency bump",
                "2026-07-06T13:20:00Z",
                "provider.local-oss",
                "model.local-mixtral",
                Mode::GuidedPatch,
                Outcome::Failed,
                true,
                false,
            ),
        ],
        vec![
            ap_case(
                "appr-0013-a",
                "run-2026-07-06-0013",
                "reviewer.dana",
                Actor::DelegatedReviewer,
                GrantScope::Workspace,
                "policy-epoch-2026-07",
                Gate::OneClickConfirm,
                EXPIRY_TIMESTAMP,
                true,
                false,
                false,
                false,
                false,
                false,
            ),
            ap_case(
                "appr-0013-b",
                "run-2026-07-06-0013",
                "security.erin",
                Actor::SecurityReviewer,
                GrantScope::Tenant,
                "policy-epoch-2026-07",
                Gate::TwoPersonReview,
                EXPIRY_TIMESTAMP,
                true,
                false,
                false,
                false,
                false,
                true,
            ),
        ],
        vec![
            ev_case(
                "evp-0013-a",
                "run-2026-07-06-0013",
                &[Artifact::PromptTranscript, Artifact::RedactionManifest],
                Redaction::FullyRedacted,
                Linkage::LinkedResolvedTicket,
                &[Fmt::SignedArchive, Fmt::RedactedPdf],
            ),
            ev_case(
                "evp-0013-b",
                "run-2026-07-06-0013",
                &[Artifact::ToolCallLog],
                Redaction::CredentialsRedacted,
                Linkage::LinkedInternalCase,
                &[Fmt::MarkdownReport],
            ),
        ],
    ));

    // 5. Replay surface — a superseded review-first run and an awaiting-review headless run;
    //    a revoked policy-engine grant and an expired automation grant; two more redacted
    //    packets with pending / open linkages.
    rows.push(base_row(
        M5AiReplaySurface::Replay,
        M5AiQualificationClass::Stable,
        "Replay / rerun-review surface owner",
        "The replay/rerun-review surface renders the shared components so a rerun keeps the same run identity, replays through the stable entry points, and shows which approvals still apply — an expired or revoked grant is never presented as still effective when the run is re-reviewed",
        "evidence:m5-ai-run-history-export-replay:001",
        vec![
            rh_case(
                "run-2026-07-06-0015",
                "Re-review parser patch",
                "2026-07-06T14:00:00Z",
                "provider.self-hosted",
                "model.internal-7b",
                Mode::ReviewFirstPlacement,
                Outcome::Superseded,
                false,
                true,
            ),
            rh_case(
                "run-2026-07-06-0016",
                "Replay batch fixes",
                "2026-07-06T14:30:00Z",
                "provider.managed-a",
                "model.haiku-4",
                Mode::HeadlessAutomation,
                Outcome::AwaitingReview,
                true,
                true,
            ),
        ],
        vec![
            ap_case(
                "appr-0015-a",
                "run-2026-07-06-0015",
                "policy.engine",
                Actor::PolicyEngine,
                GrantScope::Global,
                "policy-epoch-2026-05",
                Gate::PolicyBlocked,
                "",
                false,
                true,
                false,
                false,
                false,
                false,
            ),
            ap_case(
                "appr-0015-b",
                "run-2026-07-06-0015",
                "automation.agent-2",
                Actor::AutomationAgent,
                GrantScope::Task,
                "policy-epoch-2026-05",
                Gate::HighFrictionTyped,
                EXPIRY_TIMESTAMP,
                true,
                false,
                false,
                false,
                true,
                false,
            ),
        ],
        vec![
            ev_case(
                "evp-0015-a",
                "run-2026-07-06-0015",
                &[Artifact::DiffPacket, Artifact::ValidationReceipt],
                Redaction::PiiRedacted,
                Linkage::LinkedOpenTicket,
                &[Fmt::JsonBundle],
            ),
            ev_case(
                "evp-0015-b",
                "run-2026-07-06-0015",
                &[Artifact::RouteReceipt],
                Redaction::FullyRedacted,
                Linkage::LinkagePendingConsent,
                &[Fmt::CsvTable],
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5AiRunHistoryExportGovernanceReview {
    M5AiRunHistoryExportGovernanceReview {
        one_primitive_carries_history_approval_and_evidence_truth: true,
        run_identity_consistent_across_surfaces: true,
        provider_model_route_always_named: true,
        stable_open_replay_export_entry_points: true,
        approval_history_preserves_distinct_grants: true,
        approval_history_never_collapsed_into_one_badge: true,
        export_summaries_preserve_redaction_and_support: true,
        export_summaries_never_raw_download_only: true,
        support_export_reconstructs_history_and_export_truth: true,
        no_surface_invents_parallel_vocabulary: true,
        every_row_declares_accessibility_route: true,
        descriptors_stable_across_ui_export_support: true,
    }
}

fn consumer_projection() -> M5AiRunHistoryExportConsumerProjection {
    M5AiRunHistoryExportConsumerProjection {
        replay_surfaces_consume_shared_primitive: true,
        run_identity_reads_single_source: true,
        approval_expiry_reads_single_source: true,
        redaction_shareability_reads_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5AiRunHistoryExportProofFreshness {
    M5AiRunHistoryExportProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5AiRunHistoryExportReleasePosture {
    M5AiRunHistoryExportReleasePosture {
        release_packet_ref: M5_AI_RUN_HISTORY_EXPORT_ARTIFACT_REF.to_owned(),
        ai_audit_ref: M5_AI_RUN_HISTORY_EXPORT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_AI_RUN_HISTORY_EXPORT_SCHEMA_REF,
        M5_AI_RUN_HISTORY_EXPORT_DOC_REF,
        M5_AI_RUN_HISTORY_EXPORT_COMPONENT_MATRIX_REF,
        M5_AI_RUN_HISTORY_EXPORT_RUN_HISTORY_REF,
        M5_AI_RUN_HISTORY_EXPORT_APPROVAL_REF,
        M5_AI_RUN_HISTORY_EXPORT_EVIDENCE_REF,
    ])
}

/// Builds the canonical M5 AI run-history / approval-timeline / evidence-export primitive
/// packet.
pub fn seeded_m5_ai_run_history_export_primitive_packet() -> M5AiRunHistoryExportPrimitivePacket {
    M5AiRunHistoryExportPrimitivePacket::new(M5AiRunHistoryExportPrimitivePacketInput {
        packet_id: M5_AI_RUN_HISTORY_EXPORT_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 AI run-history row, approval-timeline entry, and evidence-export summary primitive: canonical run id, task label, time, provider/model route, outcome, stable open/replay/export entry points, distinct actor/scope/policy-epoch/expiry approval grants, and packet-id/artifact-class/redaction/support-linkage/export-format summaries"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5AiRunHistoryExportVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the export surface is narrowed to Preview pending redaction-and-support
/// continuity parity proof across every headless export path; every surface stays visible.
pub fn seeded_m5_ai_run_history_export_primitive_evidence_export_preview_narrowed(
) -> M5AiRunHistoryExportPrimitivePacket {
    let mut packet = seeded_m5_ai_run_history_export_primitive_packet();
    packet.packet_id =
        "m5-ai-run-history-row-approval-timeline-entry-evidence-export-summary-primitive:export-preview:0001"
            .to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.replay_surface == M5AiReplaySurface::Export)
        .expect("export row present");
    row.qualification = M5AiQualificationClass::Preview;
    packet
}

/// Narrowed variant: the support surface is held at Beta because a slice of support-desk
/// rows do not yet render the expiry cue on every profile; every surface stays visible.
pub fn seeded_m5_ai_run_history_export_primitive_support_beta_narrowed(
) -> M5AiRunHistoryExportPrimitivePacket {
    let mut packet = seeded_m5_ai_run_history_export_primitive_packet();
    packet.packet_id =
        "m5-ai-run-history-row-approval-timeline-entry-evidence-export-summary-primitive:support-beta:0001"
            .to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.replay_surface == M5AiReplaySurface::Support)
        .expect("support row present");
    row.qualification = M5AiQualificationClass::Beta;
    packet
}
