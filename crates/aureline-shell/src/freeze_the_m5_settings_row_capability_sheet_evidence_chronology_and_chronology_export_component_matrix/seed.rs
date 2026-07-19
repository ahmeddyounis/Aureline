// Sequential pushes keep each contract scenario adjacent to its rationale.
#![allow(clippy::vec_init_then_push)]

//! Canonical seed builders for the frozen M5 settings-row, capability-sheet,
//! evidence-chronology, and chronology-export component matrix.
//!
//! These builders are the single producer of the checked-in support export and
//! the narrowed fixtures. The headless emitter and the inline tests both call
//! them so the in-code matrix, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical trust-chronology-component matrix.
pub const M5_TRUST_COMPONENTS_MATRIX_PACKET_ID: &str = "m5-trust-chronology-components:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-30T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every component must be able to show.
fn mandatory_labels() -> Vec<M5TrustRequiredLabel> {
    M5TrustRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a component carries.
fn labels_with(extra: &[M5TrustRequiredLabel]) -> Vec<M5TrustRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every component filled in and every
/// family-specific vocabulary left empty for the caller to populate.
fn base_row(
    component_family: M5TrustComponentFamily,
    qualification: M5TrustQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    shell_zone_slot: M5ShellZoneSlot,
    proof_ref: &str,
) -> M5TrustComponentRow {
    M5TrustComponentRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        shell_zone_slot,
        responsive_classes: M5ResponsiveClass::ALL.to_vec(),
        window_classes: M5WindowClass::ALL.to_vec(),
        surface_families: M5ShellSurfaceFamily::ALL.to_vec(),
        required_labels: mandatory_labels(),
        settings_row_states: vec![],
        source_pills: vec![],
        consequence_classes: vec![],
        capability_scope_states: vec![],
        chronology_verbs: vec![],
        provenance_badges: vec![],
        chronology_detail_states: vec![],
        chronology_export_fields: vec![],
        accessibility_routes: M5TrustAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ShellConsumerSurface::ShellFrame,
            M5ShellConsumerSurface::SupportExport,
            M5ShellConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5TrustComponentDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_TRUST_COMPONENTS_SCHEMA_REF,
            M5_TRUST_COMPONENTS_SHELL_ZONE_REF,
        ]),
        conflates_effective_and_configured: false,
        hides_permission_scope: false,
        invents_private_row_grammar: false,
        drops_audit_or_support_truth: false,
    }
}

fn component_rows() -> Vec<M5TrustComponentRow> {
    use M5CapabilityConsequenceClass as CC;
    use M5CapabilityScopeState as SC;
    use M5ChronologyDetailState as DT;
    use M5ChronologyExportField as EF;
    use M5ChronologyVerb as V;
    use M5ProvenanceBadge as PB;
    use M5SettingSourcePill as SP;
    use M5SettingsRowState as RS;
    use M5ShellConsumerSurface as C;
    use M5ShellZoneSlot as Z;
    use M5TrustComponentDowngradeTrigger as D;
    use M5TrustComponentFamily as F;
    use M5TrustQualificationClass as Q;
    use M5TrustRequiredLabel as L;

    let mut rows = Vec::new();

    // 1. Settings row.
    let mut row = base_row(
        F::SettingsRow,
        Q::Stable,
        "Settings/config component owner",
        "One settings-row model carrying effective-versus-configured truth: it shows the effective value, names the source pill that produced it, explains lock and pending-reload states, holds an invalid value without applying it, and redacts credential-managed values — never conflating effective with configured",
        Z::MainWorkspace,
        "evidence:m5-settings-row-parity:001",
    );
    row.settings_row_states = RS::ALL.to_vec();
    row.source_pills = SP::ALL.to_vec();
    row.required_labels = labels_with(&[L::EffectiveValue, L::Provenance, L::AuditReopenPath]);
    row.consumer_surfaces = vec![
        C::ShellFrame,
        C::Layout,
        C::DocsHelp,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::EffectiveConfiguredConflated,
        D::SourcePillMissing,
        D::LockStateUnexplained,
        D::AuditTruthLostOffPrimarySurface,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Capability sheet.
    let mut row = base_row(
        F::CapabilitySheet,
        Q::Stable,
        "Policy/capability component owner",
        "One capability-sheet model grouping permission requests by consequence class rather than by a flat permission list; it shows transitive downstream scope, supports reduced-scope grants, requires re-consent when scope changes, and keeps revocations in history",
        Z::TransientOverlay,
        "evidence:m5-capability-sheet-parity:001",
    );
    row.consequence_classes = CC::ALL.to_vec();
    row.capability_scope_states = SC::ALL.to_vec();
    row.required_labels = labels_with(&[L::Provenance, L::AuditReopenPath]);
    row.consumer_surfaces = vec![
        C::ShellFrame,
        C::AttentionRouter,
        C::NotificationEnvelope,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ConsequenceGroupingDropped,
        D::TransitiveScopeHidden,
        D::ReConsentSkipped,
        D::AuditTruthLostOffPrimarySurface,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Event/history row.
    let mut row = base_row(
        F::EventHistoryRow,
        Q::Stable,
        "Activity/evidence component owner",
        "One event/history row model using the stable, closed verb vocabulary and attributing a provenance badge on every event so a human, AI, automation, or remote action is never conflated; its detail stays reopenable and its truth stays in the support export",
        Z::BottomPanel,
        "evidence:m5-event-history-row-parity:001",
    );
    row.chronology_verbs = V::ALL.to_vec();
    row.provenance_badges = PB::ALL.to_vec();
    row.required_labels = labels_with(&[L::Provenance, L::AuditReopenPath]);
    row.consumer_surfaces = vec![
        C::ShellFrame,
        C::AttentionRouter,
        C::NotificationEnvelope,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::VerbVocabularyDrift,
        D::ProvenanceBadgeMissing,
        D::AuditTruthLostOffPrimarySurface,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Timeline group.
    let mut row = base_row(
        F::TimelineGroup,
        Q::Stable,
        "Activity/evidence component owner",
        "One timeline-group model that collapses related events under one heading using the same stable verbs and provenance badges; it groups by object or by time, discloses any filter, and keeps every grouped detail reopenable from durable history",
        Z::BottomPanel,
        "evidence:m5-timeline-group-parity:001",
    );
    row.chronology_verbs = V::ALL.to_vec();
    row.provenance_badges = PB::ALL.to_vec();
    row.chronology_detail_states = DT::ALL.to_vec();
    row.required_labels = labels_with(&[L::Provenance, L::AuditReopenPath]);
    row.consumer_surfaces = vec![
        C::ShellFrame,
        C::AttentionRouter,
        C::NotificationEnvelope,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::VerbVocabularyDrift,
        D::ProvenanceBadgeMissing,
        D::ChronologyDetailNotReopenable,
        D::AuditTruthLostOffPrimarySurface,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Narrative summary card.
    let mut row = base_row(
        F::NarrativeSummaryCard,
        Q::Stable,
        "Activity/evidence component owner",
        "One narrative-summary-card model that summarizes a chronology span in prose without inventing new verbs: it reuses the stable verb vocabulary and provenance badges, discloses its grouping, and always keeps a reopen path back into the underlying events",
        Z::RightInspector,
        "evidence:m5-narrative-summary-card-parity:001",
    );
    row.chronology_verbs = vec![
        V::Created,
        V::Updated,
        V::Ran,
        V::Approved,
        V::Failed,
        V::Recovered,
    ];
    row.provenance_badges = PB::ALL.to_vec();
    row.chronology_detail_states = vec![
        DT::Collapsed,
        DT::Expanded,
        DT::GroupedByObject,
        DT::GroupedByTime,
        DT::ReopenableDetail,
    ];
    row.required_labels = labels_with(&[L::Provenance, L::AuditReopenPath]);
    row.consumer_surfaces = vec![
        C::ShellFrame,
        C::AttentionRouter,
        C::DocsHelp,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::VerbVocabularyDrift,
        D::ProvenanceBadgeMissing,
        D::ChronologyDetailNotReopenable,
        D::AuditTruthLostOffPrimarySurface,
        D::ProofStale,
    ];
    rows.push(row);

    // 6. Chronology export preview.
    let mut row = base_row(
        F::ChronologyExportPreview,
        Q::Stable,
        "Support/export component owner",
        "One chronology-export-preview model that shows exactly which fields will leave the trust boundary — the stable verb, provenance, timestamp, object ref, actor role, outcome code, and redaction class — so an export never silently drops a truth-bearing column and every previewed row is reconstructable from the support export",
        Z::TransientOverlay,
        "evidence:m5-chronology-export-preview-parity:001",
    );
    row.chronology_export_fields = EF::ALL.to_vec();
    row.required_labels = labels_with(&[L::Provenance, L::AuditReopenPath]);
    row.consumer_surfaces = vec![
        C::ShellFrame,
        C::ReleaseProof,
        C::DocsHelp,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ExportFieldDropped,
        D::ProvenanceBadgeMissing,
        D::AuditTruthLostOffPrimarySurface,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5TrustComponentGovernanceReview {
    M5TrustComponentGovernanceReview {
        settings_row_carries_effective_versus_configured: true,
        settings_source_pills_and_lock_state_explained: true,
        capability_sheet_groups_by_consequence: true,
        capability_transitive_scope_and_reconsent_preserved: true,
        chronology_uses_stable_verbs_and_provenance: true,
        chronology_detail_and_export_portable: true,
        no_component_invents_second_row_grammar: true,
        no_audit_or_support_truth_dropped: true,
        every_component_bound_to_shell_zone: true,
        every_component_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5TrustComponentConsumerProjection {
    M5TrustComponentConsumerProjection {
        settings_surfaces_consume_matrix: true,
        capability_sheets_consume_scope_vocabulary: true,
        activity_and_evidence_consume_chronology_vocabulary: true,
        chronology_export_reads_single_source: true,
        support_export_reads_single_source: true,
        accessibility_bridge_reads_single_source: true,
    }
}

fn proof_freshness() -> M5TrustComponentProofFreshness {
    M5TrustComponentProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5TrustComponentReleasePosture {
    M5TrustComponentReleasePosture {
        release_packet_ref: "artifacts/release/m5-trust-chronology-proof/support_export.json"
            .to_owned(),
        trust_component_audit_ref: "artifacts/components/m5-trust-chronology-components.md"
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_TRUST_COMPONENTS_SCHEMA_REF,
        M5_TRUST_COMPONENTS_DOC_REF,
        M5_TRUST_COMPONENTS_SHELL_ZONE_REF,
        M5_TRUST_COMPONENTS_SETTINGS_CONTRACT_REF,
        M5_TRUST_COMPONENTS_CAPABILITY_CONTRACT_REF,
    ])
}

/// Builds the canonical frozen M5 trust-chronology-component matrix packet.
pub fn seeded_m5_trust_chronology_component_matrix() -> M5TrustComponentMatrixPacket {
    M5TrustComponentMatrixPacket::new(M5TrustComponentMatrixPacketInput {
        packet_id: M5_TRUST_COMPONENTS_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 settings-row, capability-sheet, evidence-chronology, and chronology-export component matrix"
                .to_owned(),
        component_rows: component_rows(),
        vocabulary_set: M5TrustComponentVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the narrative summary card is held at Beta because a slice
/// of narrative summaries do not yet round-trip their reopen path across every
/// export path; every component stays visible.
pub fn seeded_m5_trust_chronology_component_matrix_narrative_summary_card_beta_narrowed(
) -> M5TrustComponentMatrixPacket {
    let mut packet = seeded_m5_trust_chronology_component_matrix();
    packet.packet_id = "m5-trust-chronology-components:narrative-summary-card-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5TrustComponentFamily::NarrativeSummaryCard)
        .expect("narrative-summary-card row present");
    row.qualification = M5TrustQualificationClass::Beta;
    packet
}

/// Narrowed variant: the chronology export preview is narrowed to Preview pending
/// redaction-class parity proof across every export field; every component stays
/// visible.
pub fn seeded_m5_trust_chronology_component_matrix_chronology_export_preview_preview_narrowed(
) -> M5TrustComponentMatrixPacket {
    let mut packet = seeded_m5_trust_chronology_component_matrix();
    packet.packet_id =
        "m5-trust-chronology-components:chronology-export-preview-preview:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5TrustComponentFamily::ChronologyExportPreview)
        .expect("chronology-export-preview row present");
    row.qualification = M5TrustQualificationClass::Preview;
    packet
}
