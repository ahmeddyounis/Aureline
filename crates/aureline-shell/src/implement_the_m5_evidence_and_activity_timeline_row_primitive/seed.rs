//! Canonical seed builders for the M5 evidence / activity row primitive.
//!
//! These builders are the single producer of the checked-in support export and
//! the narrowed fixtures. The headless emitter and the inline tests both call
//! them so the in-code matrix, the artifact, the worked resolutions, and the
//! fixtures never drift.

use super::*;

/// Stable packet id for the canonical evidence-row-primitive packet.
pub const M5_EVIDENCE_ROW_PRIMITIVE_PACKET_ID: &str = "m5-evidence-row-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-30T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds one raw history event.
fn event(
    timestamp_repr: &str,
    actor_repr: &str,
    verb: M5ChronologyVerb,
    object_repr: &str,
    outcome: M5EvidenceOutcome,
    provenance: M5ProvenanceBadge,
    detail_ref: Option<&str>,
) -> M5EvidenceEventItem {
    M5EvidenceEventItem {
        timestamp_repr: timestamp_repr.to_owned(),
        actor_repr: actor_repr.to_owned(),
        verb,
        object_repr: object_repr.to_owned(),
        outcome,
        provenance,
        has_detail: detail_ref.is_some(),
        detail_ref: detail_ref.map(str::to_owned),
    }
}

/// Builds a worked resolution case from a lane, portability, and events.
fn log_case(
    surface_family: M5HistorySurfaceFamily,
    portable_evidence: bool,
    events: Vec<M5EvidenceEventItem>,
) -> M5EvidenceRowResolutionCase {
    M5EvidenceRowResolutionCase::resolved(M5EvidenceRowResolutionInput {
        surface_family,
        portable_evidence,
        events,
    })
}

/// A base row with the shared fields filled in and the full anatomy, verb,
/// provenance, detail-state, focus-behavior, and export-field parity every lane
/// carries. Copy formats follow portability: all three when portable, none
/// otherwise.
fn base_row(
    surface_family: M5HistorySurfaceFamily,
    qualification: M5TrustQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    portable_evidence: bool,
    proof_ref: &str,
    example_logs: Vec<M5EvidenceRowResolutionCase>,
) -> M5EvidenceSurfaceRow {
    M5EvidenceSurfaceRow {
        surface_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        // Evidence / activity timelines live in the bottom panel: the execution,
        // output, problems, terminal, and timeline zone.
        shell_zone_slot: M5ShellZoneSlot::BottomPanel,
        responsive_classes: M5ResponsiveClass::ALL.to_vec(),
        window_classes: M5WindowClass::ALL.to_vec(),
        anatomy_parts: M5EvidenceRowAnatomyPart::ALL.to_vec(),
        chronology_verbs: M5ChronologyVerb::ALL.to_vec(),
        provenance_badges: M5ProvenanceBadge::ALL.to_vec(),
        detail_states: M5ChronologyDetailState::ALL.to_vec(),
        portable_evidence,
        copy_formats: if portable_evidence {
            M5EvidenceCopyFormat::ALL.to_vec()
        } else {
            Vec::new()
        },
        focus_behaviors: M5EvidenceRowFocusBehavior::ALL.to_vec(),
        export_fields: M5ChronologyExportField::ALL.to_vec(),
        accessibility_routes: M5TrustAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ShellConsumerSurface::ShellFrame,
            M5ShellConsumerSurface::Layout,
            M5ShellConsumerSurface::DocsHelp,
            M5ShellConsumerSurface::SupportExport,
            M5ShellConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5TrustComponentDowngradeTrigger::VerbVocabularyDrift,
            M5TrustComponentDowngradeTrigger::ProvenanceBadgeMissing,
            M5TrustComponentDowngradeTrigger::ChronologyDetailNotReopenable,
            M5TrustComponentDowngradeTrigger::ExportFieldDropped,
            M5TrustComponentDowngradeTrigger::AuditTruthLostOffPrimarySurface,
            M5TrustComponentDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_EVIDENCE_ROW_SCHEMA_REF,
            M5_EVIDENCE_ROW_ACTIVITY_ROW_REF,
            M5_EVIDENCE_ROW_TASK_EVENT_REF,
            M5_EVIDENCE_ROW_PROVENANCE_REF,
        ]),
        example_logs,
        drifts_from_verb_vocabulary: false,
        drops_provenance_badge: false,
        detail_not_reopenable: false,
        drops_export_or_audit_truth: false,
    }
}

fn surface_rows() -> Vec<M5EvidenceSurfaceRow> {
    use M5ChronologyVerb as V;
    use M5EvidenceOutcome as O;
    use M5ProvenanceBadge as P;

    let mut rows = Vec::new();

    // 1. AI evidence — an AI run ran an analysis and succeeded, with disclosure-
    //    ready detail into the run's evidence. Portable: copyable as text / JSON /
    //    Markdown, so support / export needs no screenshot.
    rows.push(base_row(
        M5HistorySurfaceFamily::AiEvidence,
        M5TrustQualificationClass::Stable,
        "AI evidence owner",
        "The AI-evidence lane renders the shared row so an AI run reads as `ran` (AI-initiated, succeeded) with expandable detail into the run's evidence, and copies as text / JSON / Markdown without a screenshot",
        true,
        "evidence:m5-evidence-row-ai:001",
        vec![log_case(
            M5HistorySurfaceFamily::AiEvidence,
            true,
            vec![event(
                "2026-06-30T09:00:00Z",
                "ai-run:code-search",
                V::Ran,
                "workspace-index-scan",
                O::Succeeded,
                P::AiInitiated,
                Some("evidence:ai-run-detail:001"),
            )],
        )],
    ));

    // 2. Task events — a task was created and then updated by a human. Terse rows,
    //    no expandable detail, portable copy.
    rows.push(base_row(
        M5HistorySurfaceFamily::TaskEvents,
        M5TrustQualificationClass::Stable,
        "Task lifecycle owner",
        "The task-events lane renders the shared row so the task lifecycle reads with stable verbs — `created` then `updated` (human-initiated, succeeded) — instead of per-feature prose",
        true,
        "evidence:m5-evidence-row-task:001",
        vec![log_case(
            M5HistorySurfaceFamily::TaskEvents,
            true,
            vec![
                event(
                    "2026-06-30T09:05:00Z",
                    "user:maintainer",
                    V::Created,
                    "task:build-web",
                    O::Succeeded,
                    P::HumanInitiated,
                    None,
                ),
                event(
                    "2026-06-30T09:06:00Z",
                    "user:maintainer",
                    V::Updated,
                    "task:build-web",
                    O::Succeeded,
                    P::HumanInitiated,
                    None,
                ),
            ],
        )],
    ));

    // 3. Policy changes — a policy approval and a policy rejection, both system-
    //    initiated with reopenable detail. This exercises `approved` / `rejected`
    //    and the `denied` outcome.
    rows.push(base_row(
        M5HistorySurfaceFamily::PolicyChanges,
        M5TrustQualificationClass::Stable,
        "Policy governance owner",
        "The policy-changes lane renders the shared row so an `approved` change and a `rejected` change (system-initiated, with reopenable detail) read with stable verbs and their provenance, never conflated with a user action",
        true,
        "evidence:m5-evidence-row-policy:001",
        vec![log_case(
            M5HistorySurfaceFamily::PolicyChanges,
            true,
            vec![
                event(
                    "2026-06-30T09:10:00Z",
                    "policy-engine",
                    V::Approved,
                    "policy:network-egress",
                    O::Succeeded,
                    P::SystemInitiated,
                    Some("evidence:policy-change-detail:001"),
                ),
                event(
                    "2026-06-30T09:11:00Z",
                    "policy-engine",
                    V::Rejected,
                    "policy:local-exec",
                    O::Denied,
                    P::SystemInitiated,
                    Some("evidence:policy-change-detail:002"),
                ),
            ],
        )],
    ));

    // 4. Provider mutations — a connected provider's route was updated by the
    //    remote actor. Portable copy, remote-actor provenance.
    rows.push(base_row(
        M5HistorySurfaceFamily::ProviderMutations,
        M5TrustQualificationClass::Stable,
        "Connected-provider registry owner",
        "The provider-mutations lane renders the shared row so a connected-provider route change reads as `updated` (remote-actor, succeeded) — provider-owned state is attributed, never shown as a local change",
        true,
        "evidence:m5-evidence-row-provider:001",
        vec![log_case(
            M5HistorySurfaceFamily::ProviderMutations,
            true,
            vec![event(
                "2026-06-30T09:15:00Z",
                "provider-registry",
                V::Updated,
                "provider-route:hosted-model",
                O::Succeeded,
                P::RemoteActor,
                None,
            )],
        )],
    ));

    // 5. Remote reconnects — a remote host connection recovered. This lane does not
    //    claim portable evidence, so it renders the same row with no copy
    //    renderings — proving the primitive handles non-portable lanes too.
    rows.push(base_row(
        M5HistorySurfaceFamily::RemoteReconnects,
        M5TrustQualificationClass::Stable,
        "Remote-connector trust owner",
        "The remote-reconnects lane renders the shared row so a reconnection reads as `recovered` (remote-actor, succeeded); this lane does not yet claim portable copy, so no copy renderings are emitted while the row grammar stays identical",
        false,
        "evidence:m5-evidence-row-remote:001",
        vec![log_case(
            M5HistorySurfaceFamily::RemoteReconnects,
            false,
            vec![event(
                "2026-06-30T09:20:00Z",
                "remote-connector:build-farm",
                V::Recovered,
                "remote-host:build-farm",
                O::Succeeded,
                P::RemoteActor,
                None,
            )],
        )],
    ));

    // 6. Update history — an automation-initiated update failed and then recovered.
    //    Exercises `failed` / `recovered` verbs and the `failed` outcome.
    rows.push(base_row(
        M5HistorySurfaceFamily::UpdateHistory,
        M5TrustQualificationClass::Stable,
        "Update channel owner",
        "The update-history lane renders the shared row so an update that `failed` and then `recovered` (automation-initiated) reads with stable verbs, the failing event keeps reopenable detail, and the history copies without a screenshot",
        true,
        "evidence:m5-evidence-row-update:001",
        vec![log_case(
            M5HistorySurfaceFamily::UpdateHistory,
            true,
            vec![
                event(
                    "2026-06-30T09:25:00Z",
                    "updater-service",
                    V::Failed,
                    "update-channel:stable",
                    O::Failed,
                    P::AutomationInitiated,
                    Some("evidence:update-failure-detail:001"),
                ),
                event(
                    "2026-06-30T09:26:00Z",
                    "updater-service",
                    V::Recovered,
                    "update-channel:stable",
                    O::Succeeded,
                    P::AutomationInitiated,
                    None,
                ),
            ],
        )],
    ));

    // 7. Support exports — a support bundle was exported by a human. Exercises the
    //    `exported` verb and proves the export self-documents.
    rows.push(base_row(
        M5HistorySurfaceFamily::SupportExports,
        M5TrustQualificationClass::Stable,
        "Support export owner",
        "The support-exports lane renders the shared row so a bundle export reads as `exported` (human-initiated, succeeded) with reopenable detail, so the support flow itself preserves what happened as copyable text / JSON / Markdown",
        true,
        "evidence:m5-evidence-row-support:001",
        vec![log_case(
            M5HistorySurfaceFamily::SupportExports,
            true,
            vec![event(
                "2026-06-30T09:30:00Z",
                "user:support-lead",
                V::Exported,
                "support-bundle:diagnostics",
                O::Succeeded,
                P::HumanInitiated,
                Some("evidence:support-export-detail:001"),
            )],
        )],
    ));

    // 8. Repair flows — a recovery replayed from durable history and reverted a bad
    //    change. Exercises the replayed-from-history provenance and `reverted`
    //    outcome.
    rows.push(base_row(
        M5HistorySurfaceFamily::RepairFlows,
        M5TrustQualificationClass::Stable,
        "Recovery / repair owner",
        "The repair-flows lane renders the shared row so a recovery that replays from durable history reads as `recovered` (replayed-from-history) and reverted the bad change, with reopenable detail and portable copy",
        true,
        "evidence:m5-evidence-row-repair:001",
        vec![log_case(
            M5HistorySurfaceFamily::RepairFlows,
            true,
            vec![event(
                "2026-06-30T09:35:00Z",
                "recovery-service",
                V::Recovered,
                "repair-flow:workspace-restore",
                O::Reverted,
                P::ReplayedFromHistory,
                Some("evidence:repair-flow-detail:001"),
            )],
        )],
    ));

    rows
}

fn governance_review() -> M5EvidenceRowGovernanceReview {
    M5EvidenceRowGovernanceReview {
        one_row_model_across_history_lanes: true,
        stable_verb_vocabulary_enforced: true,
        provenance_badge_always_attributed: true,
        detail_reopenable_from_durable_history: true,
        copy_export_parity_text_json_markdown: true,
        support_export_keeps_chronology_vocabulary: true,
        no_lane_invents_local_prose_verbs: true,
        every_row_bound_to_shell_zone: true,
        every_row_declares_accessibility_route: true,
        later_lanes_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5EvidenceRowConsumerProjection {
    M5EvidenceRowConsumerProjection {
        history_lanes_consume_shared_row: true,
        resolver_reads_single_verb_vocabulary: true,
        provenance_reads_single_source: true,
        detail_reopen_reads_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5EvidenceRowProofFreshness {
    M5EvidenceRowProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5EvidenceRowReleasePosture {
    M5EvidenceRowReleasePosture {
        release_packet_ref: M5_EVIDENCE_ROW_ARTIFACT_REF.to_owned(),
        evidence_row_audit_ref: M5_EVIDENCE_ROW_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_EVIDENCE_ROW_SCHEMA_REF,
        M5_EVIDENCE_ROW_DOC_REF,
        M5_EVIDENCE_ROW_SHELL_ZONE_REF,
        M5_EVIDENCE_ROW_COMPONENT_MATRIX_REF,
        M5_EVIDENCE_ROW_ACTIVITY_ROW_REF,
        M5_EVIDENCE_ROW_TASK_EVENT_REF,
        M5_EVIDENCE_ROW_PROVENANCE_REF,
    ])
}

/// Builds the canonical M5 evidence-row-primitive packet.
pub fn seeded_m5_evidence_row_primitive_packet() -> M5EvidenceRowPrimitivePacket {
    M5EvidenceRowPrimitivePacket::new(M5EvidenceRowPrimitivePacketInput {
        packet_id: M5_EVIDENCE_ROW_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 evidence / activity row primitive: stable verbs, provenance badges, disclosure-ready detail, and text / JSON / Markdown copy parity"
                .to_owned(),
        surface_rows: surface_rows(),
        vocabulary_set: M5EvidenceRowVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the update-history lane is held at Beta because a slice of the
/// failure-detail reopen path does not yet render on every profile; every lane
/// stays visible.
pub fn seeded_m5_evidence_row_primitive_update_history_beta_narrowed(
) -> M5EvidenceRowPrimitivePacket {
    let mut packet = seeded_m5_evidence_row_primitive_packet();
    packet.packet_id = "m5-evidence-row-primitive:update-history-beta:0001".to_owned();
    let row = packet
        .surface_rows
        .iter_mut()
        .find(|row| row.surface_family == M5HistorySurfaceFamily::UpdateHistory)
        .expect("update-history row present");
    row.qualification = M5TrustQualificationClass::Beta;
    packet
}

/// Narrowed variant: the repair-flows lane is narrowed to Preview pending copy
/// parity across every export path; every lane stays visible.
pub fn seeded_m5_evidence_row_primitive_repair_flows_preview_narrowed(
) -> M5EvidenceRowPrimitivePacket {
    let mut packet = seeded_m5_evidence_row_primitive_packet();
    packet.packet_id = "m5-evidence-row-primitive:repair-flows-preview:0001".to_owned();
    let row = packet
        .surface_rows
        .iter_mut()
        .find(|row| row.surface_family == M5HistorySurfaceFamily::RepairFlows)
        .expect("repair-flows row present");
    row.qualification = M5TrustQualificationClass::Preview;
    packet
}
