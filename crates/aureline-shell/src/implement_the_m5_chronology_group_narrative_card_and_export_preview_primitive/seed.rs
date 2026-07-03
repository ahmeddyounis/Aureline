//! Canonical seed builders for the M5 chronology-group primitive.
//!
//! These builders are the single producer of the checked-in support export and
//! the narrowed fixtures. The headless emitter and the inline tests both call
//! them so the in-code matrix, the artifact, the worked resolutions, and the
//! fixtures never drift.

use super::*;

/// Stable packet id for the canonical chronology-group-primitive packet.
pub const M5_CHRONOLOGY_GROUP_PRIMITIVE_PACKET_ID: &str =
    "m5-chronology-group-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-30T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds one raw chronology event.
#[allow(clippy::too_many_arguments)]
fn event(
    phase: M5ChronologyPhase,
    sequence: u32,
    absolute_timestamp: &str,
    relative_label: &str,
    verb: M5ChronologyVerb,
    provenance: M5ProvenanceBadge,
    outcome: M5ChronologyOutcome,
    object_repr: &str,
    consequential: bool,
    detail_ref: Option<&str>,
) -> M5ChronologyEventItem {
    M5ChronologyEventItem {
        phase,
        sequence,
        absolute_timestamp: absolute_timestamp.to_owned(),
        relative_label: relative_label.to_owned(),
        verb,
        provenance,
        outcome,
        object_repr: object_repr.to_owned(),
        consequential,
        detail_ref: detail_ref.map(str::to_owned),
    }
}

/// Builds one export request.
fn export_request(
    redaction_class: M5ChronologyRedactionClass,
    output_format: M5ChronologyExportFormat,
    range_start: &str,
    range_end: &str,
) -> M5ChronologyExportRequest {
    M5ChronologyExportRequest {
        selected_range_start: range_start.to_owned(),
        selected_range_end: range_end.to_owned(),
        time_zone_repr: "UTC".to_owned(),
        redaction_class,
        output_format,
        included_fields: M5ChronologyExportField::ALL.to_vec(),
    }
}

/// Builds a worked resolution case from a lane, events, and an export request.
fn chronology_case(
    history_lane: M5ChronologyHistoryLane,
    events: Vec<M5ChronologyEventItem>,
    export_request: M5ChronologyExportRequest,
) -> M5ChronologyResolutionCase {
    M5ChronologyResolutionCase::resolved(M5ChronologyResolutionInput {
        history_lane,
        events,
        export_request,
    })
}

/// A base row with the shared fields filled in and the full anatomy, phase, verb,
/// provenance, outcome, detail-state, next-action, redaction-class, export-format,
/// focus-behavior, and export-field parity every lane carries.
fn base_row(
    history_lane: M5ChronologyHistoryLane,
    qualification: M5TrustQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    example_chronologies: Vec<M5ChronologyResolutionCase>,
) -> M5ChronologySurfaceRow {
    M5ChronologySurfaceRow {
        history_lane,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        // Chronology surfaces live in the bottom panel: the execution, output,
        // problems, terminal, and timeline zone.
        shell_zone_slot: M5ShellZoneSlot::BottomPanel,
        responsive_classes: M5ResponsiveClass::ALL.to_vec(),
        window_classes: M5WindowClass::ALL.to_vec(),
        anatomy_parts: M5ChronologySurfaceAnatomyPart::ALL.to_vec(),
        phases: M5ChronologyPhase::ALL.to_vec(),
        chronology_verbs: M5ChronologyVerb::ALL.to_vec(),
        provenance_badges: M5ProvenanceBadge::ALL.to_vec(),
        outcomes: M5ChronologyOutcome::ALL.to_vec(),
        detail_states: M5ChronologyDetailState::ALL.to_vec(),
        next_actions: M5NextAction::ALL.to_vec(),
        redaction_classes: M5ChronologyRedactionClass::ALL.to_vec(),
        export_formats: M5ChronologyExportFormat::ALL.to_vec(),
        export_fields: M5ChronologyExportField::ALL.to_vec(),
        focus_behaviors: M5ChronologyFocusBehavior::ALL.to_vec(),
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
            M5TrustComponentDowngradeTrigger::ChronologyDetailNotReopenable,
            M5TrustComponentDowngradeTrigger::ExportFieldDropped,
            M5TrustComponentDowngradeTrigger::AuditTruthLostOffPrimarySurface,
            M5TrustComponentDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_CHRONOLOGY_GROUP_SCHEMA_REF,
            M5_CHRONOLOGY_GROUP_EVIDENCE_TIMELINE_REF,
            M5_CHRONOLOGY_GROUP_REDACTION_PROFILE_REF,
            M5_CHRONOLOGY_GROUP_LINEAGE_REF,
        ]),
        example_chronologies,
        flattens_causal_ordering: false,
        drops_absolute_timestamp: false,
        drops_redaction_intent: false,
        drops_export_or_audit_truth: false,
    }
}

fn surface_rows() -> Vec<M5ChronologySurfaceRow> {
    use M5ChronologyOutcome as O;
    use M5ChronologyPhase as Ph;
    use M5ChronologyVerb as V;
    use M5ProvenanceBadge as P;

    let mut rows = Vec::new();

    // 1. AI evidence — an AI run: initiated, executed, and resolved. Three phases,
    //    three groups, AI-initiated provenance, absolute + relative parity.
    rows.push(base_row(
        M5ChronologyHistoryLane::AiEvidence,
        M5TrustQualificationClass::Stable,
        "AI evidence owner",
        "The AI-evidence lane groups a run into Initiation / Execution / Resolution phases, summarizes the current state in one sentence, and previews a metadata-only JSON export that keeps causal order",
        "chronology:m5-chronology-ai:001",
        vec![chronology_case(
            M5ChronologyHistoryLane::AiEvidence,
            vec![
                event(
                    Ph::Initiation,
                    1,
                    "2026-06-30T09:00:00Z",
                    "3h ago",
                    V::Created,
                    P::AiInitiated,
                    O::Succeeded,
                    "ai-run:code-search",
                    true,
                    Some("chronology:ai-detail:001"),
                ),
                event(
                    Ph::Execution,
                    2,
                    "2026-06-30T09:02:00Z",
                    "3h ago",
                    V::Ran,
                    P::AiInitiated,
                    O::Succeeded,
                    "workspace-index-scan",
                    true,
                    Some("chronology:ai-detail:002"),
                ),
                event(
                    Ph::Resolution,
                    3,
                    "2026-06-30T09:05:00Z",
                    "3h ago",
                    V::Updated,
                    P::AiInitiated,
                    O::Succeeded,
                    "ai-run:code-search",
                    true,
                    None,
                ),
            ],
            export_request(
                M5ChronologyRedactionClass::MetadataOnly,
                M5ChronologyExportFormat::Json,
                "2026-06-30T09:00:00Z",
                "2026-06-30T09:05:00Z",
            ),
        )],
    ));

    // 2. Policy changes — an approval and a denial, both in the Review phase (one
    //    contiguous group of two). Exercises `approved` / `rejected`, `denied`
    //    outcome, and pseudonymized-actor redaction.
    rows.push(base_row(
        M5ChronologyHistoryLane::PolicyChanges,
        M5TrustQualificationClass::Stable,
        "Policy governance owner",
        "The policy-changes lane groups an approval and a denial into one Review phase, keeps their causal order, and previews a pseudonymized-actor Markdown export",
        "chronology:m5-chronology-policy:001",
        vec![chronology_case(
            M5ChronologyHistoryLane::PolicyChanges,
            vec![
                event(
                    Ph::Review,
                    1,
                    "2026-06-30T09:10:00Z",
                    "2h ago",
                    V::Approved,
                    P::SystemInitiated,
                    O::Succeeded,
                    "policy:network-egress",
                    true,
                    Some("chronology:policy-detail:001"),
                ),
                event(
                    Ph::Review,
                    2,
                    "2026-06-30T09:11:00Z",
                    "2h ago",
                    V::Rejected,
                    P::SystemInitiated,
                    O::Denied,
                    "policy:local-exec",
                    true,
                    Some("chronology:policy-detail:002"),
                ),
            ],
            export_request(
                M5ChronologyRedactionClass::PseudonymizedActors,
                M5ChronologyExportFormat::Markdown,
                "2026-06-30T09:10:00Z",
                "2026-06-30T09:11:00Z",
            ),
        )],
    ));

    // 3. Task events — a task created then run, still pending. Two phases, two
    //    groups. Exercises `pending` outcome, automation-initiated provenance, and a
    //    metadata-only CSV export.
    rows.push(base_row(
        M5ChronologyHistoryLane::TaskEvents,
        M5TrustQualificationClass::Stable,
        "Task lifecycle owner",
        "The task-events lane groups a task's Initiation and Execution phases, summarizes that the run is still pending, proposes awaiting completion, and previews a CSV export",
        "chronology:m5-chronology-task:001",
        vec![chronology_case(
            M5ChronologyHistoryLane::TaskEvents,
            vec![
                event(
                    Ph::Initiation,
                    1,
                    "2026-06-30T09:15:00Z",
                    "90m ago",
                    V::Created,
                    P::HumanInitiated,
                    O::Succeeded,
                    "task:build-web",
                    true,
                    None,
                ),
                event(
                    Ph::Execution,
                    2,
                    "2026-06-30T09:16:00Z",
                    "89m ago",
                    V::Ran,
                    P::AutomationInitiated,
                    O::Pending,
                    "task:build-web",
                    true,
                    None,
                ),
            ],
            export_request(
                M5ChronologyRedactionClass::MetadataOnly,
                M5ChronologyExportFormat::Csv,
                "2026-06-30T09:15:00Z",
                "2026-06-30T09:16:00Z",
            ),
        )],
    ));

    // 4. Remote reconnects — a failure then a recovery. Execution then Recovery
    //    phase. Exercises `failed` outcome, the Recovery phase, remote-actor
    //    provenance, and an aggregate-counts NDJSON export.
    rows.push(base_row(
        M5ChronologyHistoryLane::RemoteReconnects,
        M5TrustQualificationClass::Stable,
        "Remote-connector trust owner",
        "The remote-reconnects lane groups a connection failure and its recovery into Execution and Recovery phases, keeping the causal order, and previews an aggregate-counts NDJSON export",
        "chronology:m5-chronology-remote:001",
        vec![chronology_case(
            M5ChronologyHistoryLane::RemoteReconnects,
            vec![
                event(
                    Ph::Execution,
                    1,
                    "2026-06-30T09:20:00Z",
                    "70m ago",
                    V::Failed,
                    P::RemoteActor,
                    O::Failed,
                    "remote-host:build-farm",
                    true,
                    Some("chronology:remote-detail:001"),
                ),
                event(
                    Ph::Recovery,
                    2,
                    "2026-06-30T09:22:00Z",
                    "68m ago",
                    V::Recovered,
                    P::RemoteActor,
                    O::Succeeded,
                    "remote-host:build-farm",
                    true,
                    None,
                ),
            ],
            export_request(
                M5ChronologyRedactionClass::AggregateCountsOnly,
                M5ChronologyExportFormat::NdjsonStream,
                "2026-06-30T09:20:00Z",
                "2026-06-30T09:22:00Z",
            ),
        )],
    ));

    // 5. Update history — a channel created, an update that failed, then recovered
    //    by replaying from history and reverting. Three phases. Exercises
    //    `replayed_from_history` provenance and `reverted` outcome.
    rows.push(base_row(
        M5ChronologyHistoryLane::UpdateHistory,
        M5TrustQualificationClass::Stable,
        "Update channel owner",
        "The update-history lane groups a channel's Initiation, a failed Execution, and a Recovery that replayed from history and reverted the change, and previews a metadata-only JSON export",
        "chronology:m5-chronology-update:001",
        vec![chronology_case(
            M5ChronologyHistoryLane::UpdateHistory,
            vec![
                event(
                    Ph::Initiation,
                    1,
                    "2026-06-30T09:25:00Z",
                    "55m ago",
                    V::Created,
                    P::AutomationInitiated,
                    O::Succeeded,
                    "update-channel:stable",
                    false,
                    None,
                ),
                event(
                    Ph::Execution,
                    2,
                    "2026-06-30T09:26:00Z",
                    "54m ago",
                    V::Failed,
                    P::AutomationInitiated,
                    O::Failed,
                    "update-channel:stable",
                    true,
                    Some("chronology:update-detail:001"),
                ),
                event(
                    Ph::Recovery,
                    3,
                    "2026-06-30T09:28:00Z",
                    "52m ago",
                    V::Recovered,
                    P::ReplayedFromHistory,
                    O::Reverted,
                    "update-channel:stable",
                    true,
                    Some("chronology:update-detail:002"),
                ),
            ],
            export_request(
                M5ChronologyRedactionClass::MetadataOnly,
                M5ChronologyExportFormat::Json,
                "2026-06-30T09:25:00Z",
                "2026-06-30T09:28:00Z",
            ),
        )],
    ));

    // 6. Support exports — a bundle exported by a human, one Resolution phase.
    //    Exercises the `exported` verb and the `no_action_needed` next action, and a
    //    pseudonymized-actor Markdown export.
    rows.push(base_row(
        M5ChronologyHistoryLane::SupportExports,
        M5TrustQualificationClass::Stable,
        "Support export owner",
        "The support-exports lane groups a bundle export into one Resolution phase, summarizes that the export completed with no further action needed, and previews a pseudonymized-actor Markdown export",
        "chronology:m5-chronology-support:001",
        vec![chronology_case(
            M5ChronologyHistoryLane::SupportExports,
            vec![event(
                Ph::Resolution,
                1,
                "2026-06-30T09:30:00Z",
                "40m ago",
                V::Exported,
                P::HumanInitiated,
                O::Succeeded,
                "support-bundle:diagnostics",
                true,
                Some("chronology:support-detail:001"),
            )],
            export_request(
                M5ChronologyRedactionClass::PseudonymizedActors,
                M5ChronologyExportFormat::Markdown,
                "2026-06-30T09:30:00Z",
                "2026-06-30T09:30:00Z",
            ),
        )],
    ));

    rows
}

fn governance_review() -> M5ChronologyGroupGovernanceReview {
    M5ChronologyGroupGovernanceReview {
        one_chronology_model_across_lanes: true,
        timeline_groups_retain_ordering: true,
        groups_declare_phase_count_and_outcome: true,
        narrative_explains_state_and_next_action: true,
        relative_and_absolute_time_parity: true,
        export_preview_declares_full_disclosure: true,
        export_preserves_causality: true,
        support_export_keeps_chronology_vocabulary: true,
        every_surface_bound_to_shell_zone: true,
        later_lanes_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5ChronologyGroupConsumerProjection {
    M5ChronologyGroupConsumerProjection {
        history_lanes_consume_shared_chronology: true,
        grouping_reads_single_phase_vocabulary: true,
        narrative_reads_single_source: true,
        export_reads_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5ChronologyGroupProofFreshness {
    M5ChronologyGroupProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ChronologyGroupReleasePosture {
    M5ChronologyGroupReleasePosture {
        release_packet_ref: M5_CHRONOLOGY_GROUP_ARTIFACT_REF.to_owned(),
        chronology_audit_ref: M5_CHRONOLOGY_GROUP_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_CHRONOLOGY_GROUP_SCHEMA_REF,
        M5_CHRONOLOGY_GROUP_DOC_REF,
        M5_CHRONOLOGY_GROUP_SHELL_ZONE_REF,
        M5_CHRONOLOGY_GROUP_COMPONENT_MATRIX_REF,
        M5_CHRONOLOGY_GROUP_EVIDENCE_TIMELINE_REF,
        M5_CHRONOLOGY_GROUP_REDACTION_PROFILE_REF,
        M5_CHRONOLOGY_GROUP_LINEAGE_REF,
    ])
}

/// Builds the canonical M5 chronology-group-primitive packet.
pub fn seeded_m5_chronology_group_primitive_packet() -> M5ChronologyGroupPrimitivePacket {
    M5ChronologyGroupPrimitivePacket::new(M5ChronologyGroupPrimitivePacketInput {
        packet_id: M5_CHRONOLOGY_GROUP_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 chronology-group primitive: grouped phases, narrative summary cards, timezone-safe export previews, and no-lost-causality ordering"
                .to_owned(),
        surface_rows: surface_rows(),
        vocabulary_set: M5ChronologyGroupVocabularySet::canonical(),
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
/// replay-recovery grouping does not yet render on every profile; every lane stays
/// visible.
pub fn seeded_m5_chronology_group_primitive_update_history_beta_narrowed(
) -> M5ChronologyGroupPrimitivePacket {
    let mut packet = seeded_m5_chronology_group_primitive_packet();
    packet.packet_id = "m5-chronology-group-primitive:update-history-beta:0001".to_owned();
    let row = packet
        .surface_rows
        .iter_mut()
        .find(|row| row.history_lane == M5ChronologyHistoryLane::UpdateHistory)
        .expect("update-history row present");
    row.qualification = M5TrustQualificationClass::Beta;
    packet
}

/// Narrowed variant: the support-exports lane is narrowed to Preview pending export
/// preview parity across every output format; every lane stays visible.
pub fn seeded_m5_chronology_group_primitive_support_exports_preview_narrowed(
) -> M5ChronologyGroupPrimitivePacket {
    let mut packet = seeded_m5_chronology_group_primitive_packet();
    packet.packet_id = "m5-chronology-group-primitive:support-exports-preview:0001".to_owned();
    let row = packet
        .surface_rows
        .iter_mut()
        .find(|row| row.history_lane == M5ChronologyHistoryLane::SupportExports)
        .expect("support-exports row present");
    row.qualification = M5TrustQualificationClass::Preview;
    packet
}
