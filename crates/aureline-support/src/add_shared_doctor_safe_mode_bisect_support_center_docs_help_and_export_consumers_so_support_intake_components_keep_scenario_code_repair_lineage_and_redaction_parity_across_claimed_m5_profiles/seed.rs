//! Canonical seed builders for the M5 support-intake / escalation component-consumer
//! lane.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked bindings, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical support-intake / escalation component-consumer
/// packet.
pub const M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_PACKET_ID: &str =
    "m5-support-intake-escalation-component-consumer:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked binding case for one consumer/family adoption.
fn case(
    consumer: M5SupportIntakeComponentConsumer,
    component_family: M5SupportIntakeEscalationComponentFamily,
    parity_health: M5SupportIntakeConsumerParityHealth,
    export_caveats: &[M5SupportIntakeConsumerExportCaveat],
    note: &str,
) -> M5SupportIntakeBindingCase {
    M5SupportIntakeBindingCase::resolved(M5SupportIntakeBindingInput {
        consumer,
        component_family,
        descriptor_families: M5SupportIntakeComponentDescriptor::ALL.to_vec(),
        parity_health,
        export_caveats: export_caveats.to_vec(),
        note_repr: Some(note.to_owned()),
    })
}

/// Builds a component binding that points at its canonical family refs.
fn binding(
    component_family: M5SupportIntakeEscalationComponentFamily,
    example_bindings: Vec<M5SupportIntakeBindingCase>,
) -> M5SupportIntakeComponentBinding {
    M5SupportIntakeComponentBinding {
        component_family,
        canonical_schema_ref: family_canonical_schema_ref(component_family).to_owned(),
        canonical_artifact_ref: family_canonical_artifact_ref(component_family).to_owned(),
        references_canonical_not_local_prose: true,
        example_bindings,
    }
}

/// A base row with the shared parity vocabulary filled in.
fn base_row(
    consumer: M5SupportIntakeComponentConsumer,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    component_bindings: Vec<M5SupportIntakeComponentBinding>,
) -> M5SupportIntakeComponentConsumerRow {
    M5SupportIntakeComponentConsumerRow {
        consumer,
        qualification: M5SupportQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5SupportSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5SupportDeploymentLine::ALL.to_vec(),
        anatomy_parts: M5SupportIntakeConsumerAnatomyPart::ALL.to_vec(),
        descriptor_families: M5SupportIntakeComponentDescriptor::ALL.to_vec(),
        parity_health_modes: M5SupportIntakeConsumerParityHealth::ALL.to_vec(),
        export_caveats: M5SupportIntakeConsumerExportCaveat::ALL.to_vec(),
        claim_parity_states: M5SupportIntakeClaimParityState::ALL.to_vec(),
        narrowing_reasons: M5SupportIntakeConsumerNarrowingReason::ALL.to_vec(),
        recovery_actions: M5SupportIntakeConsumerRecoveryAction::ALL.to_vec(),
        export_fields: M5SupportIntakeConsumerExportField::ALL.to_vec(),
        accessibility_routes: M5SupportAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5SupportConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5SupportDowngradeTrigger::ScenarioOrScopeUnstated,
            M5SupportDowngradeTrigger::DoctorFindingLineageUnstated,
            M5SupportDowngradeTrigger::EvidenceClassMasked,
            M5SupportDowngradeTrigger::PacketDestinationUnstated,
            M5SupportDowngradeTrigger::ProofStale,
        ],
        component_bindings,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_SCHEMA_REF,
            M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        ]),
        rewords_claims_per_surface: false,
        invents_new_escalation_grammar: false,
        drops_scenario_packet_redaction_or_repair_when_narrowed: false,
        inherits_stronger_label_from_healthier_profile: false,
    }
}

// Sequential pushes preserve the numbered consumer-matrix narrative below.
#[allow(clippy::vec_init_then_push)]
fn consumer_rows() -> Vec<M5SupportIntakeComponentConsumerRow> {
    use M5SupportIntakeComponentConsumer as Consumer;
    use M5SupportIntakeConsumerExportCaveat as Caveat;
    use M5SupportIntakeConsumerParityHealth as Health;
    use M5SupportIntakeEscalationComponentFamily as Family;

    let mut rows = Vec::new();

    // 1. Project Doctor results — the support-scenario picker row and unsafe-fix blocked
    //    note at full parity (start diagnosis, review a suggested repair), plus the
    //    escalation-packet summary auto-narrowed because the scenario classification is
    //    still uncertain and cannot escalate to a vendor case yet.
    rows.push(base_row(
        Consumer::DoctorResults,
        "Project Doctor surface owner",
        "Project Doctor results adopt the support-scenario picker row and unsafe-fix blocked note at full parity, pointing at the canonical component schemas so scenario code, packet id, redaction class, and approved-repair guidance match what safe mode, bisect, the support center, Help / docs, and the export desk read; the escalation-packet summary auto-narrows while the scenario classification is uncertain",
        "evidence:m5-support-consumer-doctor-results:001",
        vec![
            binding(
                Family::SupportScenarioPickerRow,
                vec![case(
                    Consumer::DoctorResults,
                    Family::SupportScenarioPickerRow,
                    Health::FullParity,
                    &[],
                    "doctor scenario picker row at full parity",
                )],
            ),
            binding(
                Family::UnsafeFixBlockedNote,
                vec![case(
                    Consumer::DoctorResults,
                    Family::UnsafeFixBlockedNote,
                    Health::FullParity,
                    &[],
                    "doctor unsafe-fix blocked note at full parity",
                )],
            ),
            binding(
                Family::EscalationPacketSummary,
                vec![case(
                    Consumer::DoctorResults,
                    Family::EscalationPacketSummary,
                    Health::ScenarioUncertainNarrowed,
                    &[Caveat::ScenarioUncertainLocalOnly],
                    "doctor escalation-packet summary narrowed by uncertain scenario classification",
                )],
            ),
        ],
    ));

    // 2. Safe-mode recovery flow — the support-scenario picker row, handoff-timeline row,
    //    and unsafe-fix blocked note all at full parity: the reduced-capability recovery
    //    lane keeps the same scenario / packet / repair truth Doctor established.
    rows.push(base_row(
        Consumer::SafeMode,
        "Safe-mode recovery surface owner",
        "The safe-mode recovery flow adopts the support-scenario picker row, handoff-timeline row, and unsafe-fix blocked note at full parity, keeping scenario code, packet id, redaction class, and approved-repair guidance explicit so a reduced-capability recovery lane never re-words the case truth",
        "evidence:m5-support-consumer-safe-mode:001",
        vec![
            binding(
                Family::SupportScenarioPickerRow,
                vec![case(
                    Consumer::SafeMode,
                    Family::SupportScenarioPickerRow,
                    Health::FullParity,
                    &[],
                    "safe-mode scenario picker row at full parity",
                )],
            ),
            binding(
                Family::HandoffTimelineRow,
                vec![case(
                    Consumer::SafeMode,
                    Family::HandoffTimelineRow,
                    Health::FullParity,
                    &[],
                    "safe-mode handoff-timeline row at full parity",
                )],
            ),
            binding(
                Family::UnsafeFixBlockedNote,
                vec![case(
                    Consumer::SafeMode,
                    Family::UnsafeFixBlockedNote,
                    Health::FullParity,
                    &[],
                    "safe-mode unsafe-fix blocked note at full parity",
                )],
            ),
        ],
    ));

    // 3. Extension-bisect recovery flow — the support-scenario picker row and
    //    handoff-timeline row at full parity, plus the issue-report builder step
    //    auto-narrowed because the evidence classes gathered mid-bisect are still
    //    incomplete for a full report.
    rows.push(base_row(
        Consumer::Bisect,
        "Extension-bisect recovery surface owner",
        "The extension-bisect recovery flow adopts the support-scenario picker row and handoff-timeline row at full parity, and the issue-report builder step auto-narrowed because the evidence classes gathered mid-bisect are incomplete, keeping scenario code, packet id, redaction class, and approved-repair guidance disclosed so a partial bisect report narrows visibly instead of inheriting a completed report's label",
        "evidence:m5-support-consumer-bisect:001",
        vec![
            binding(
                Family::SupportScenarioPickerRow,
                vec![case(
                    Consumer::Bisect,
                    Family::SupportScenarioPickerRow,
                    Health::FullParity,
                    &[],
                    "bisect scenario picker row at full parity",
                )],
            ),
            binding(
                Family::IssueReportBuilderStep,
                vec![case(
                    Consumer::Bisect,
                    Family::IssueReportBuilderStep,
                    Health::EvidenceIncompleteNarrowed,
                    &[Caveat::EvidenceIncompleteNotFullReport],
                    "bisect issue-report builder step narrowed by incomplete evidence classes",
                )],
            ),
            binding(
                Family::HandoffTimelineRow,
                vec![case(
                    Consumer::Bisect,
                    Family::HandoffTimelineRow,
                    Health::FullParity,
                    &[],
                    "bisect handoff-timeline row at full parity",
                )],
            ),
        ],
    ));

    // 4. Support center — the issue-report builder step, escalation-packet summary,
    //    handoff-timeline row, and unsafe-fix blocked note, all at full parity: the
    //    authoritative support-center rendering every other surface keeps parity with.
    rows.push(base_row(
        Consumer::SupportCenter,
        "Support-center surface owner",
        "The support center adopts the issue-report builder step, escalation-packet summary, handoff-timeline row, and unsafe-fix blocked note at full parity, referencing the canonical component schemas so scenario code, packet id, redaction class, and approved-repair guidance stay one truth across every claimed support surface",
        "evidence:m5-support-consumer-support-center:001",
        vec![
            binding(
                Family::IssueReportBuilderStep,
                vec![case(
                    Consumer::SupportCenter,
                    Family::IssueReportBuilderStep,
                    Health::FullParity,
                    &[],
                    "support-center issue-report builder step at full parity",
                )],
            ),
            binding(
                Family::EscalationPacketSummary,
                vec![case(
                    Consumer::SupportCenter,
                    Family::EscalationPacketSummary,
                    Health::FullParity,
                    &[],
                    "support-center escalation-packet summary at full parity",
                )],
            ),
            binding(
                Family::HandoffTimelineRow,
                vec![case(
                    Consumer::SupportCenter,
                    Family::HandoffTimelineRow,
                    Health::FullParity,
                    &[],
                    "support-center handoff-timeline row at full parity",
                )],
            ),
            binding(
                Family::UnsafeFixBlockedNote,
                vec![case(
                    Consumer::SupportCenter,
                    Family::UnsafeFixBlockedNote,
                    Health::FullParity,
                    &[],
                    "support-center unsafe-fix blocked note at full parity",
                )],
            ),
        ],
    ));

    // 5. Help / docs — the support-scenario picker row and issue-report builder step at
    //    full parity, plus the escalation-packet summary auto-narrowed because on this
    //    deployment the packet destination is unavailable under current policy, so docs
    //    can only describe local export.
    rows.push(base_row(
        Consumer::DocsHelp,
        "Help / docs surface owner",
        "Help / docs adopt the support-scenario picker row and issue-report builder step at full parity, and the escalation-packet summary auto-narrowed because the packet destination is unavailable under current policy, keeping scenario code, packet id, redaction class, and approved-repair guidance explicit so documentation degrades to local export instead of inheriting a healthier profile's escalation language",
        "evidence:m5-support-consumer-docs-help:001",
        vec![
            binding(
                Family::SupportScenarioPickerRow,
                vec![case(
                    Consumer::DocsHelp,
                    Family::SupportScenarioPickerRow,
                    Health::FullParity,
                    &[],
                    "docs / help scenario picker row at full parity",
                )],
            ),
            binding(
                Family::IssueReportBuilderStep,
                vec![case(
                    Consumer::DocsHelp,
                    Family::IssueReportBuilderStep,
                    Health::FullParity,
                    &[],
                    "docs / help issue-report builder step at full parity",
                )],
            ),
            binding(
                Family::EscalationPacketSummary,
                vec![case(
                    Consumer::DocsHelp,
                    Family::EscalationPacketSummary,
                    Health::DestinationUnavailableNarrowed,
                    &[Caveat::DestinationUnavailableLocalBundleOnly],
                    "docs / help escalation-packet summary narrowed by unavailable packet destination",
                )],
            ),
        ],
    ));

    // 6. Support / export desk — the escalation-packet summary, handoff-timeline row,
    //    issue-report builder step, and unsafe-fix blocked note, referencing the canonical
    //    schemas so its prose can never drift from the product truth; the escalation-packet
    //    summary is auto-narrowed because redaction review is still pending.
    rows.push(base_row(
        Consumer::SupportExport,
        "Support / export desk surface owner",
        "The support / export desk adopts the escalation-packet summary, handoff-timeline row, issue-report builder step, and unsafe-fix blocked note, referencing the canonical component schemas so its prose can never drift from the product truth, and the escalation-packet summary auto-narrowed because redaction review is still pending, keeping scenario code, packet id, redaction class, and approved-repair guidance exact",
        "evidence:m5-support-consumer-support-export:001",
        vec![
            binding(
                Family::EscalationPacketSummary,
                vec![case(
                    Consumer::SupportExport,
                    Family::EscalationPacketSummary,
                    Health::RedactionPendingNarrowed,
                    &[Caveat::RedactionPendingNotShareable],
                    "support / export escalation-packet summary narrowed by pending redaction review",
                )],
            ),
            binding(
                Family::HandoffTimelineRow,
                vec![case(
                    Consumer::SupportExport,
                    Family::HandoffTimelineRow,
                    Health::FullParity,
                    &[],
                    "support / export handoff-timeline row at full parity",
                )],
            ),
            binding(
                Family::IssueReportBuilderStep,
                vec![case(
                    Consumer::SupportExport,
                    Family::IssueReportBuilderStep,
                    Health::FullParity,
                    &[],
                    "support / export issue-report builder step at full parity",
                )],
            ),
            binding(
                Family::UnsafeFixBlockedNote,
                vec![case(
                    Consumer::SupportExport,
                    Family::UnsafeFixBlockedNote,
                    Health::FullParity,
                    &[],
                    "support / export unsafe-fix blocked note at full parity",
                )],
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5SupportIntakeComponentConsumerGovernanceReview {
    M5SupportIntakeComponentConsumerGovernanceReview {
        consumers_adopt_shared_primitives: true,
        consumers_reference_canonical_schema: true,
        descriptor_vocabulary_shared_not_reworded: true,
        no_consumer_invents_new_grammar: true,
        scenario_packet_redaction_repair_explicit_on_every_surface: true,
        degraded_state_auto_narrows_claim: true,
        narrowed_rendering_always_shows_self_contained_banner: true,
        banner_names_exact_reason_and_recovery_action: true,
        support_export_presents_same_scenario_and_repair_truth: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5SupportIntakeComponentConsumerProjection {
    M5SupportIntakeComponentConsumerProjection {
        all_consumers_adopt_shared_components: true,
        scenario_code_reads_single_source: true,
        packet_id_reads_single_source: true,
        redaction_class_reads_single_source: true,
        approved_repair_reads_single_source: true,
    }
}

fn proof_freshness() -> M5SupportIntakeComponentConsumerProofFreshness {
    M5SupportIntakeComponentConsumerProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5SupportIntakeComponentConsumerReleasePosture {
    M5SupportIntakeComponentConsumerReleasePosture {
        release_packet_ref: M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_ARTIFACT_REF.to_owned(),
        support_case_audit_ref: M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_SCHEMA_REF,
        M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_DOC_REF,
        M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF,
        M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        family_canonical_schema_ref(
            M5SupportIntakeEscalationComponentFamily::SupportScenarioPickerRow,
        ),
        family_canonical_schema_ref(
            M5SupportIntakeEscalationComponentFamily::IssueReportBuilderStep,
        ),
        family_canonical_schema_ref(
            M5SupportIntakeEscalationComponentFamily::EscalationPacketSummary,
        ),
        family_canonical_schema_ref(M5SupportIntakeEscalationComponentFamily::UnsafeFixBlockedNote),
    ])
}

/// Builds the canonical M5 support-intake / escalation component-consumer packet.
pub fn seeded_m5_support_intake_escalation_component_consumer_packet(
) -> M5SupportIntakeComponentConsumerPacket {
    M5SupportIntakeComponentConsumerPacket::new(M5SupportIntakeComponentConsumerPacketInput {
        packet_id: M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_PACKET_ID.to_owned(),
        matrix_label:
            "M5 support-intake / escalation component consumers: Project Doctor, safe mode, bisect, the support center, Help / docs, and the export desk keep scenario code, packet id, redaction class, and approved-repair parity"
                .to_owned(),
        consumer_rows: consumer_rows(),
        vocabulary_set: M5SupportIntakeComponentConsumerVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the extension-bisect recovery flow is held at Preview because a slice
/// of mid-bisect renderings still gather incomplete evidence classes; every consumer stays
/// visible.
pub fn seeded_m5_support_intake_escalation_component_consumer_bisect_preview_narrowed(
) -> M5SupportIntakeComponentConsumerPacket {
    let mut packet = seeded_m5_support_intake_escalation_component_consumer_packet();
    packet.packet_id =
        "m5-support-intake-escalation-component-consumer:bisect-preview:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5SupportIntakeComponentConsumer::Bisect)
        .expect("bisect row present");
    row.qualification = M5SupportQualificationClass::Preview;
    packet
}

/// Narrowed variant: the Help / docs surface is held at Beta because a slice of documented
/// escalation paths do not yet expose the auto-narrow banner on every
/// destination-unavailable path; every consumer stays visible.
pub fn seeded_m5_support_intake_escalation_component_consumer_docs_help_beta_narrowed(
) -> M5SupportIntakeComponentConsumerPacket {
    let mut packet = seeded_m5_support_intake_escalation_component_consumer_packet();
    packet.packet_id =
        "m5-support-intake-escalation-component-consumer:docs-help-beta:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5SupportIntakeComponentConsumer::DocsHelp)
        .expect("docs / help row present");
    row.qualification = M5SupportQualificationClass::Beta;
    packet
}
