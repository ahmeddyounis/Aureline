//! Canonical seed builders for the frozen M5 editor-inline component matrix.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix, the
//! artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical editor-inline component matrix.
pub const M5_EDITOR_INLINE_COMPONENT_MATRIX_PACKET_ID: &str =
    "m5-editor-inline-components:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-12T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every component must be able to show.
fn mandatory_labels() -> Vec<M5EditorInlineRequiredLabel> {
    M5EditorInlineRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a component carries.
fn labels_with(extra: &[M5EditorInlineRequiredLabel]) -> Vec<M5EditorInlineRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every component filled in and every family-specific
/// vocabulary left empty for the caller to populate.
fn base_row(
    component_family: M5EditorInlineComponentFamily,
    qualification: M5EditorInlineQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5EditorInlineComponentRow {
    M5EditorInlineComponentRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5EditorInlineSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5EditorInlineDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        dispositions: vec![],
        tab_states: vec![],
        gutter_marker_kinds: vec![],
        diagnostic_severities: vec![],
        fix_postures: vec![],
        diff_change_kinds: vec![],
        anchor_durabilities: vec![],
        ai_confidences: vec![],
        evidence_disclosures: vec![],
        degraded_reasons: M5EditorInlineDegradedReason::ALL.to_vec(),
        accessibility_routes: M5EditorInlineAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5EditorInlineConsumerSurface::SupportExport,
            M5EditorInlineConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5EditorInlineDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        encodes_tab_marker_or_diagnostic_state_by_color_alone: false,
        lets_comment_anchor_or_evidence_pointer_silently_drift: false,
        blurs_outdated_and_resolved_review_state: false,
        presents_inferred_fix_as_exact: false,
        hides_evidence_timeline_in_opaque_log: false,
    }
}

fn component_rows() -> Vec<M5EditorInlineComponentRow> {
    use M5AiConfidence as AI;
    use M5AnchorDurability as AN;
    use M5DiagnosticSeverity as SV;
    use M5DiffChangeKind as DC;
    use M5EditorInlineComponentFamily as F;
    use M5EditorInlineConsumerSurface as C;
    use M5EditorInlineDisposition as BD;
    use M5EditorInlineDowngradeTrigger as D;
    use M5EditorInlineQualificationClass as Q;
    use M5EditorInlineRequiredLabel as L;
    use M5EditorTabState as TS;
    use M5EvidenceDisclosure as EV;
    use M5FixPosture as FP;
    use M5GutterMarkerKind as GM;

    let mut rows = Vec::new();

    // 1. Editor tab.
    let mut row = base_row(
        F::EditorTab,
        Q::Stable,
        "Editor surface owner",
        "One editor-tab model naming the open-document context (active-current, background, or unpinned preview) and its per-tab item state (modified, preview, pinned, read-only, shared, generated, remote), so a background or preview tab never reads as the active saved document and state is never encoded by color alone",
        "evidence:m5-editor-tab-parity:001",
        &[M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF, M5_EDITOR_TAB_SCHEMA_REF],
    );
    row.tab_states = TS::ALL.to_vec();
    row.dispositions = vec![
        BD::Modified,
        BD::Preview,
        BD::Pinned,
        BD::ReadOnly,
        BD::Shared,
        BD::Generated,
        BD::Remote,
    ];
    row.required_labels = labels_with(&[L::AnchorAndFreshness]);
    row.consumer_surfaces = vec![
        C::EditorUi,
        C::NotebookUi,
        C::ReviewUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::TabMarkerDiagnosticColorOnly,
        D::AnchorStateUnstated,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Gutter.
    let mut row = base_row(
        F::Gutter,
        Q::Stable,
        "Editor surface owner",
        "One gutter model naming breakpoint, change-marker (added, modified, removed), and fold layering next to the code, plus the diagnostic severity a gutter glyph reflects, so layered gutter state is never encoded by color alone and a change marker is never confused with a breakpoint",
        "evidence:m5-gutter-marker-parity:001",
        &[M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF, M5_GUTTER_MARKER_SCHEMA_REF],
    );
    row.gutter_marker_kinds = GM::ALL.to_vec();
    row.diagnostic_severities = vec![SV::Error, SV::Warning, SV::Info, SV::Hint];
    row.dispositions = vec![BD::Modified, BD::Generated, BD::Remote, BD::BlockedByPolicy];
    row.required_labels = labels_with(&[L::AnchorAndFreshness]);
    row.consumer_surfaces = vec![
        C::EditorUi,
        C::DiffUi,
        C::NotebookUi,
        C::DiagnosticsUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::TabMarkerDiagnosticColorOnly,
        D::DiagnosticFreshnessUnstated,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Diagnostic decoration.
    let mut row = base_row(
        F::DiagnosticDecoration,
        Q::Stable,
        "Diagnostics surface owner",
        "One diagnostic-decoration model naming problem severity (error, warning, info, hint) and freshness (stale-versus-current) anchored to a durable range (anchored-exact, re-anchored, drifted, or outdated), so a stale diagnostic is never presented as current and severity is never encoded by color alone",
        "evidence:m5-diagnostic-decoration-parity:001",
        &[M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF, M5_DIAGNOSTIC_DECORATION_SCHEMA_REF],
    );
    row.diagnostic_severities = SV::ALL.to_vec();
    row.anchor_durabilities = vec![
        AN::AnchoredExact,
        AN::ReAnchored,
        AN::DriftedApproximate,
        AN::OutdatedAnchor,
    ];
    row.dispositions = vec![
        BD::Outdated,
        BD::ReAnchored,
        BD::BlockedByPolicy,
        BD::ReviewRequired,
        BD::Failed,
    ];
    row.required_labels = labels_with(&[L::AnchorAndFreshness]);
    row.consumer_surfaces = vec![
        C::DiagnosticsUi,
        C::EditorUi,
        C::DiffUi,
        C::NotebookUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::DiagnosticFreshnessUnstated,
        D::AnchorStateUnstated,
        D::TabMarkerDiagnosticColorOnly,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Code-action chip.
    let mut row = base_row(
        F::CodeActionChip,
        Q::Stable,
        "AI action-state owner",
        "One code-action-chip model naming the fix posture (exact, inferred, heuristic, multiple-candidate, or not-applicable) and the applied / reverted / review-required / blocked state, so an inferred fix is never presented as an exact one and a failed apply is never read as clean",
        "evidence:m5-code-action-chip-parity:001",
        &[M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF, M5_CODE_ACTION_CHIP_SCHEMA_REF],
    );
    row.fix_postures = FP::ALL.to_vec();
    row.dispositions = vec![
        BD::ExactFix,
        BD::InferredFix,
        BD::Applied,
        BD::Reverted,
        BD::Failed,
        BD::BlockedByPolicy,
        BD::ReviewRequired,
    ];
    row.required_labels = labels_with(&[L::ConfidenceAndSource]);
    row.consumer_surfaces = vec![
        C::EditorUi,
        C::AiUi,
        C::DiagnosticsUi,
        C::ReviewUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::InferredFixShownAsExact,
        D::AiConfidenceUnstated,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Diff view.
    let mut row = base_row(
        F::DiffView,
        Q::Stable,
        "Diff / merge governance owner",
        "One diff-view model naming every change kind (added, removed, modified, moved, conflicted, unchanged-context), so an added, removed, modified, moved, or conflicted region is never collapsed into one ambiguous generic change and a generated or remote side is named",
        "evidence:m5-diff-view-parity:001",
        &[M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF, M5_DIFF_VIEW_SCHEMA_REF],
    );
    row.diff_change_kinds = DC::ALL.to_vec();
    row.dispositions = vec![
        BD::Modified,
        BD::Generated,
        BD::Remote,
        BD::Outdated,
        BD::BlockedByPolicy,
    ];
    row.required_labels = labels_with(&[L::AnchorAndFreshness]);
    row.consumer_surfaces = vec![
        C::DiffUi,
        C::EditorUi,
        C::ReviewUi,
        C::NotebookUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::DiffChangeKindCollapsed,
        D::AnchorStateUnstated,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 6. Review thread.
    let mut row = base_row(
        F::ReviewThread,
        Q::Stable,
        "Hosted-review owner",
        "One review-thread model naming comment-anchor durability (anchored-exact, re-anchored, drifted, outdated, or orphaned) and resolution state (outdated-versus-resolved), so a comment anchor never silently drifts and outdated and resolved review state never blur together",
        "evidence:m5-review-thread-parity:001",
        &[M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF, M5_REVIEW_THREAD_SCHEMA_REF],
    );
    row.anchor_durabilities = AN::ALL.to_vec();
    row.dispositions = vec![
        BD::Outdated,
        BD::Resolved,
        BD::ReAnchored,
        BD::ReviewRequired,
        BD::BlockedByPolicy,
    ];
    row.required_labels = labels_with(&[L::AnchorAndFreshness]);
    row.consumer_surfaces = vec![
        C::ReviewUi,
        C::DiffUi,
        C::EditorUi,
        C::AiUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::CommentAnchorDriftedSilently,
        D::OutdatedAndResolvedBlurred,
        D::AnchorStateUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 7. AI message card.
    let mut row = base_row(
        F::AiMessageCard,
        Q::Stable,
        "AI action-state owner",
        "One AI-message-card model naming source context, confidence (grounded, low, unverified, or streaming), and the available actions, plus the collapsed / expanded / redacted disclosure of its evidence, so an unverified or streaming message is never read as final and evidence pointers never silently drift",
        "evidence:m5-ai-message-card-parity:001",
        &[M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF, M5_AI_MESSAGE_CARD_SCHEMA_REF],
    );
    row.ai_confidences = AI::ALL.to_vec();
    row.evidence_disclosures = vec![
        EV::ExpandedFull,
        EV::CollapsedSummary,
        EV::RedactedExportSafe,
    ];
    row.dispositions = vec![
        BD::Streaming,
        BD::ReviewRequired,
        BD::Applied,
        BD::Reverted,
        BD::Failed,
        BD::BlockedByPolicy,
        BD::Generated,
        BD::ExportSafeEvidence,
    ];
    row.required_labels = labels_with(&[L::ConfidenceAndSource, L::EvidenceLineage]);
    row.consumer_surfaces = vec![
        C::AiUi,
        C::EditorUi,
        C::ReviewUi,
        C::NotebookUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::AiConfidenceUnstated,
        D::EvidencePointerDriftedSilently,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 8. Evidence timeline.
    let mut row = base_row(
        F::EvidenceTimeline,
        Q::Stable,
        "Support / export owner",
        "One evidence-timeline model naming inspectable, collapsible, export-safe evidence lineage (expanded, collapsed-summary, partially-loaded, redacted, or empty), so an evidence timeline is never hidden in an opaque log and a redacted export is never mistaken for a complete one",
        "evidence:m5-evidence-timeline-parity:001",
        &[M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF, M5_EVIDENCE_TIMELINE_SCHEMA_REF],
    );
    row.evidence_disclosures = EV::ALL.to_vec();
    row.dispositions = vec![
        BD::ExportSafeEvidence,
        BD::Outdated,
        BD::Resolved,
        BD::ReAnchored,
        BD::BlockedByPolicy,
    ];
    row.required_labels = labels_with(&[L::EvidenceLineage]);
    row.consumer_surfaces = vec![
        C::SupportExport,
        C::CliExport,
        C::ReviewUi,
        C::AiUi,
        C::DiagnosticsUi,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::EvidenceTimelineOpaqueLog,
        D::EvidencePointerDriftedSilently,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5EditorInlineGovernanceReview {
    M5EditorInlineGovernanceReview {
        editor_tab_shows_state_and_context: true,
        gutter_layers_markers_without_color_only: true,
        diagnostic_decoration_shows_severity_and_freshness: true,
        code_action_chip_distinguishes_exact_from_inferred: true,
        diff_view_names_every_change_kind: true,
        review_thread_shows_anchor_and_resolution_truth: true,
        ai_message_card_shows_source_confidence_and_actions: true,
        evidence_timeline_is_inspectable_not_opaque: true,
        state_never_encoded_by_color_alone: true,
        comment_anchors_never_drift_silently: true,
        evidence_pointers_never_drift_silently: true,
        outdated_and_resolved_never_blurred: true,
        inferred_fix_never_presented_as_exact: true,
        every_component_declares_deployment_lines: true,
        every_component_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_inline_vocabulary: true,
    }
}

fn consumer_projection() -> M5EditorInlineConsumerProjection {
    M5EditorInlineConsumerProjection {
        editor_surfaces_consume_tab_and_gutter_vocabulary: true,
        diff_surfaces_consume_change_kind_vocabulary: true,
        review_consumes_anchor_and_resolution_vocabulary: true,
        ai_surfaces_consume_confidence_and_evidence_vocabulary: true,
        notebook_consumes_inline_component_vocabulary: true,
        support_export_reads_single_inline_source: true,
    }
}

fn proof_freshness() -> M5EditorInlineProofFreshness {
    M5EditorInlineProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5EditorInlineReleasePosture {
    M5EditorInlineReleasePosture {
        proof_packet_ref: M5_EDITOR_INLINE_COMPONENT_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_EDITOR_INLINE_COMPONENT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF,
        M5_EDITOR_INLINE_COMPONENT_DOC_REF,
        M5_EDITOR_TAB_SCHEMA_REF,
        M5_GUTTER_MARKER_SCHEMA_REF,
        M5_DIAGNOSTIC_DECORATION_SCHEMA_REF,
        M5_CODE_ACTION_CHIP_SCHEMA_REF,
        M5_DIFF_VIEW_SCHEMA_REF,
        M5_REVIEW_THREAD_SCHEMA_REF,
        M5_AI_MESSAGE_CARD_SCHEMA_REF,
        M5_EVIDENCE_TIMELINE_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 editor-inline component matrix packet.
pub fn seeded_m5_editor_inline_component_matrix() -> M5EditorInlineComponentMatrixPacket {
    M5EditorInlineComponentMatrixPacket::new(M5EditorInlineComponentMatrixPacketInput {
        packet_id: M5_EDITOR_INLINE_COMPONENT_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 editor-tab, gutter, diagnostic-decoration, code-action-chip, diff-view, review-thread, AI-message-card, and evidence-timeline component matrix"
                .to_owned(),
        component_rows: component_rows(),
        vocabulary_set: M5EditorInlineVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the diff view is held at Beta because moved / conflicted change-kind parity
/// round-trips are not yet proven across every deployment line; every component stays visible.
pub fn seeded_m5_editor_inline_component_matrix_diff_view_beta_narrowed(
) -> M5EditorInlineComponentMatrixPacket {
    let mut packet = seeded_m5_editor_inline_component_matrix();
    packet.packet_id = "m5-editor-inline-components:diff-view-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5EditorInlineComponentFamily::DiffView)
        .expect("diff-view row present");
    row.qualification = M5EditorInlineQualificationClass::Beta;
    packet
}

/// Narrowed variant: the review thread is narrowed to Preview pending orphaned-anchor and resolution
/// parity across every deployment line; every component stays visible.
pub fn seeded_m5_editor_inline_component_matrix_review_thread_preview_narrowed(
) -> M5EditorInlineComponentMatrixPacket {
    let mut packet = seeded_m5_editor_inline_component_matrix();
    packet.packet_id = "m5-editor-inline-components:review-thread-preview:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5EditorInlineComponentFamily::ReviewThread)
        .expect("review-thread row present");
    row.qualification = M5EditorInlineQualificationClass::Preview;
    packet
}
