//! Canonical seed builders for the frozen M5 AI-review-assist matrix.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code matrix, the artifact, and the
//! fixtures never drift.

use super::*;

/// Stable packet id for the canonical AI-review-assist matrix.
pub const M5_AI_REVIEW_ASSIST_MATRIX_PACKET_ID: &str = "m5-ai-review-assist:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-16T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn mandatory_labels() -> Vec<M5AiReviewAssistRequiredLabel> {
    M5AiReviewAssistRequiredLabel::MANDATORY.to_vec()
}

fn labels_with(extra: &[M5AiReviewAssistRequiredLabel]) -> Vec<M5AiReviewAssistRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

#[allow(clippy::too_many_arguments)]
fn base_row(
    object_class: M5AiReviewAssistObject,
    qualification: M5AiReviewAssistQualificationClass,
    owner_role: &str,
    backup_owner_role: &str,
    scope_summary: &str,
    closure_ref: &str,
    source_refs: &[&str],
    required_visible_state: M5AiReviewAssistVisibleState,
) -> M5AiReviewAssistRow {
    M5AiReviewAssistRow {
        object_class,
        qualification,
        publish_state: M5AiReviewAssistPublishState::LocalDraft,
        owner_role: owner_role.to_owned(),
        backup_owner_role: backup_owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        required_visible_state,
        surface_families: M5AiReviewAssistSurfaceFamily::ALL.to_vec(),
        classification_stages: M5AiReviewAssistClassificationStage::ALL.to_vec(),
        required_labels: mandatory_labels(),
        semantic_roles: vec![],
        finding_row_roles: vec![],
        scope_selector_roles: vec![],
        publish_sheet_roles: vec![],
        resolution_memory_roles: vec![],
        degraded_reasons: M5AiReviewAssistDegradedReason::ALL.to_vec(),
        accessibility_routes: M5AiReviewAssistAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5AiReviewAssistConsumerSurface::ReviewDetail,
            M5AiReviewAssistConsumerSurface::SupportExportPacket,
        ],
        downgrade_triggers: vec![M5AiReviewAssistDowngradeTrigger::AiReviewAssistMatrixStale],
        required_closure_artifact_refs: strings(&[closure_ref]),
        source_contract_refs: strings(source_refs),
        lets_ai_review_results_publish_or_merge_implicitly: false,
        hides_whether_output_stays_local_or_becomes_a_provider_comment_suggested_patch_or_check_annotation: false,
        keeps_stale_findings_looking_current_after_diff_or_instruction_drift: false,
        loses_local_drafts_or_evidence_when_provider_write_scope_is_missing_or_publish_fails: false,
        presents_an_ai_review_finding_without_its_analyzed_scope_publish_destination_or_lifecycle_state: false,
    }
}

fn txn(f: [&str; 7]) -> M5AiReviewAssistVisibleState {
    M5AiReviewAssistVisibleState {
        finding_label: f[0].to_owned(),
        finding_class_and_severity: f[1].to_owned(),
        analyzed_scope: f[2].to_owned(),
        publish_destination: f[3].to_owned(),
        local_versus_provider_state: f[4].to_owned(),
        lifecycle_state: f[5].to_owned(),
        publish_export_fallback: f[6].to_owned(),
    }
}

fn ai_review_assist_rows() -> Vec<M5AiReviewAssistRow> {
    use M5AiReviewAssistConsumerSurface as C;
    use M5AiReviewAssistDowngradeTrigger as D;
    use M5AiReviewAssistObject as O;
    use M5AiReviewAssistQualificationClass as Q;
    use M5AiReviewAssistRequiredLabel as L;
    use M5AiReviewAssistRole as R;

    let mut rows = Vec::new();

    // 1. AiReviewFindingRow.
    let mut row = base_row(
        O::AiReviewFindingRow,
        Q::Stable,
        "AI review finding-row owner",
        "Review-governance backup owner",
        "One reusable AI review finding row shows its finding class, severity, and confidence, names the analyzed diff scope it was produced from, shows its lifecycle state (open, outdated, suppressed), links its durable resolution memory, and never auto-approves, auto-requests changes, or auto-merges from a finding",
        "evidence:m5-ai-review-finding-row-closure:001",
        &[
            M5_AI_REVIEW_ASSIST_MATRIX_SCHEMA_REF,
            M5_AI_REVIEW_FINDING_DOMAIN_SCHEMA_REF,
            M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        ],
        txn([
            "ai review finding",
            "finding class: correctness; severity: high; confidence: medium",
            "analyzed scope: the staged diff hunks under review, checked against the repo review instructions",
            "held as a local draft; no provider comment, suggested patch, or check annotation yet",
            "local draft, not provider-committed; publishing stays an explicit action",
            "open",
            "if publishing is unavailable the finding is kept as a local draft and offered for export or copy",
        ]),
    );
    row.finding_row_roles = M5AiReviewAssistFindingRowRole::ALL.to_vec();
    row.semantic_roles = vec![R::FindingClassification, R::LifecycleStateTracking];
    row.required_labels = labels_with(&[L::LifecycleState]);
    row.consumer_surfaces = vec![
        C::ReviewDetail,
        C::AiReviewPanel,
        C::FindingRow,
        C::PendingReviewTray,
        C::SupportExportPacket,
    ];
    row.publish_state = M5AiReviewAssistPublishState::LocalDraft;
    row.downgrade_triggers = vec![
        D::FindingShownWithoutScope,
        D::FindingClassBadgeMissing,
        D::StaleFindingShownAsCurrent,
        D::LifecycleStateMissing,
        D::AiReviewAssistMatrixStale,
    ];
    rows.push(row);

    // 2. ReviewScopeSelector.
    let mut row = base_row(
        O::ReviewScopeSelector,
        Q::Stable,
        "Review scope-selector owner",
        "AI-review-governance backup owner",
        "One review scope selector names the analyzed diff range plus the repo instruction and enabled check source that bound it, flags scope drift, and offers a rerun-within-scope safe next step so findings never silently outlive the diff they were bound to",
        "evidence:m5-review-scope-selector-closure:001",
        &[
            M5_AI_REVIEW_ASSIST_MATRIX_SCHEMA_REF,
            M5_AI_REVIEW_SCOPE_SELECTOR_DOMAIN_SCHEMA_REF,
            M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
        ],
        txn([
            "review scope selector",
            "finding class filter: all classes; confidence floor shown",
            "analyzed scope: selected diff range plus the repo instruction and enabled check source that bound it",
            "no publish destination; scope selection stays local until findings are published",
            "local scope selection; choosing a scope writes no provider state",
            "rerun recommended when the diff or instruction source drifts from the analyzed scope",
            "if the diff drifts, the prior scope is retained locally and a rerun-within-scope is offered",
        ]),
    );
    row.scope_selector_roles = M5AiReviewAssistScopeSelectorRole::ALL.to_vec();
    row.semantic_roles = vec![R::AnalyzedScopeDisclosure];
    row.required_labels = labels_with(&[L::FindingClassBadge]);
    row.consumer_surfaces = vec![
        C::ReviewDetail,
        C::AiReviewPanel,
        C::ReviewScopeSelector,
        C::SupportExportPacket,
    ];
    row.publish_state = M5AiReviewAssistPublishState::LocalDraft;
    row.downgrade_triggers = vec![
        D::AnalyzedScopeUnstated,
        D::FindingShownWithoutScope,
        D::StaleFindingShownAsCurrent,
        D::AiReviewAssistMatrixStale,
    ];
    rows.push(row);

    // 3. PublishToReviewSheet.
    let mut row = base_row(
        O::PublishToReviewSheet,
        Q::Stable,
        "Publish-to-review-sheet owner",
        "Provider-governance backup owner",
        "One publish-to-review sheet shows the publish mode (local draft, publish now, open in provider), names the provider destination (comment, suggested patch, check annotation), shows local-draft-versus-provider-committed state before mutation, and offers a publish-or-export fallback so AI review output never publishes or merges implicitly",
        "evidence:m5-publish-to-review-sheet-closure:001",
        &[
            M5_AI_REVIEW_ASSIST_MATRIX_SCHEMA_REF,
            M5_AI_REVIEW_PUBLISH_SHEET_DOMAIN_SCHEMA_REF,
            M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
        ],
        txn([
            "publish to review sheet",
            "finding class and severity carried through from the selected findings",
            "analyzed scope carried through so the published review names the diff it covers",
            "publish destination: a provider review comment on the connected provider",
            "provider-committed: this publishes to the provider, distinct from a local draft",
            "published",
            "if provider write scope is missing or publish fails, the draft is preserved and an export or copy fallback is offered",
        ]),
    );
    row.publish_sheet_roles = M5AiReviewAssistPublishSheetRole::ALL.to_vec();
    row.semantic_roles = vec![
        R::PublishDestinationDisclosure,
        R::LocalVersusProviderState,
        R::PublishExportFallback,
    ];
    row.required_labels = labels_with(&[L::PublishDestination]);
    row.consumer_surfaces = vec![
        C::PublishToReviewSheet,
        C::ProviderPublishReview,
        C::ReviewDetail,
        C::PendingReviewTray,
        C::SupportExportPacket,
    ];
    row.publish_state = M5AiReviewAssistPublishState::PublishNowProviderComment;
    row.downgrade_triggers = vec![
        D::AiReviewAutoActioned,
        D::PublishDestinationHidden,
        D::PublishModeUnstated,
        D::LocalDraftLostOnPublishFailure,
        D::PublishExportFallbackMissing,
        D::AiReviewAssistMatrixStale,
    ];
    rows.push(row);

    // 4. ResolutionMemoryRow.
    let mut row = base_row(
        O::ResolutionMemoryRow,
        Q::Stable,
        "Resolution-memory-row owner",
        "Support-governance backup owner",
        "One resolution memory row shows the resolution state (dismissed, published, suppressed), shows finding freshness and outdated state, names a reopen-or-rerun path, and preserves the local draft and evidence when a publish fails so a finding's durable history stays provable and no stale finding resurfaces as current",
        "evidence:m5-resolution-memory-row-closure:001",
        &[
            M5_AI_REVIEW_ASSIST_MATRIX_SCHEMA_REF,
            M5_AI_REVIEW_RESOLUTION_MEMORY_DOMAIN_SCHEMA_REF,
            M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        ],
        txn([
            "resolution memory row",
            "finding class and severity retained with the resolution outcome",
            "analyzed scope retained so a reopened finding names the diff it was found on",
            "no publish destination; the resolution is recorded locally and included in the export packet",
            "local resolution record; reopening does not silently rewrite provider state",
            "dismissed, published, outdated, suppressed, or rerun recommended as the finding's durable state",
            "if a published resolution cannot be refreshed, the local record and evidence are preserved, never dropped",
        ]),
    );
    row.resolution_memory_roles = M5AiReviewAssistResolutionMemoryRole::ALL.to_vec();
    row.semantic_roles = vec![R::ResolutionMemoryDisclosure, R::LifecycleStateTracking];
    row.required_labels = labels_with(&[L::LifecycleState]);
    row.consumer_surfaces = vec![
        C::ResolutionMemoryLedger,
        C::ReviewDetail,
        C::PendingReviewTray,
        C::SupportExportPacket,
    ];
    row.publish_state = M5AiReviewAssistPublishState::ExportFallbackOffline;
    row.downgrade_triggers = vec![
        D::StaleFindingShownAsCurrent,
        D::ResolutionMemoryUnstated,
        D::LifecycleStateMissing,
        D::LocalDraftLostOnPublishFailure,
        D::AiReviewAssistMatrixStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5AiReviewAssistGovernanceReview {
    M5AiReviewAssistGovernanceReview {
        no_ai_review_finding_publishes_or_merges_implicitly: true,
        every_covered_object_class_names_owner_backup_owner_and_first_consumer: true,
        local_draft_state_is_mechanically_distinct_from_provider_committed_state: true,
        every_finding_names_its_class_severity_and_confidence: true,
        every_finding_names_its_analyzed_diff_scope_and_instruction_source: true,
        every_publish_names_its_mode_and_provider_destination_before_mutation: true,
        publish_or_export_fallback_is_named_for_every_finding: true,
        no_stale_or_outdated_finding_is_shown_as_current: true,
        no_local_draft_or_evidence_is_lost_when_publish_fails: true,
        every_object_declares_classification_stages: true,
        every_object_declares_accessibility_route: true,
        support_export_reads_single_ai_review_assist_source: true,
        review_ai_provider_pending_and_support_bind_to_single_source: true,
        later_rows_cannot_invent_parallel_ai_review_assist_vocabulary: true,
        ai_review_assist_truth_survives_zoom_and_high_contrast: true,
        claims_narrow_automatically_when_matrix_row_missing_or_stale: true,
    }
}

fn consumer_projection() -> M5AiReviewAssistConsumerProjection {
    M5AiReviewAssistConsumerProjection {
        review_detail_and_ai_panel_consume_shared_ai_review_finding_truth: true,
        pending_review_and_provider_publish_consume_shared_publish_destination_truth: true,
        help_and_support_export_consume_shared_finding_lifecycle_truth: true,
        docs_help_and_screenshots_read_single_ai_review_assist_source: true,
        findings_bind_to_shared_resolution_memory_relation: true,
        support_export_reads_single_ai_review_assist_source: true,
    }
}

fn proof_freshness() -> M5AiReviewAssistProofFreshness {
    M5AiReviewAssistProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5AiReviewAssistReleasePosture {
    M5AiReviewAssistReleasePosture {
        proof_packet_ref: M5_AI_REVIEW_ASSIST_ARTIFACT_REF.to_owned(),
        ai_review_assist_audit_ref: M5_AI_REVIEW_ASSIST_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_AI_REVIEW_ASSIST_MATRIX_SCHEMA_REF,
        M5_AI_REVIEW_ASSIST_MATRIX_DOC_REF,
        M5_AI_REVIEW_FINDING_DOMAIN_SCHEMA_REF,
        M5_AI_REVIEW_SCOPE_SELECTOR_DOMAIN_SCHEMA_REF,
        M5_AI_REVIEW_PUBLISH_SHEET_DOMAIN_SCHEMA_REF,
        M5_AI_REVIEW_RESOLUTION_MEMORY_DOMAIN_SCHEMA_REF,
        M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 AI-review-assist matrix packet.
pub fn seeded_m5_ai_review_assist_matrix() -> M5AiReviewAssistMatrixPacket {
    M5AiReviewAssistMatrixPacket::new(M5AiReviewAssistMatrixPacketInput {
        packet_id: M5_AI_REVIEW_ASSIST_MATRIX_PACKET_ID.to_owned(),
        matrix_label: "M5 AI-review-finding-row, review-scope-selector, publish-to-review-sheet, and resolution-memory-row matrix"
            .to_owned(),
        ai_review_assist_rows: ai_review_assist_rows(),
        vocabulary_set: M5AiReviewAssistVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the publish-to-review sheet is held at Beta because its provider-linked publish-later
/// and open-in-provider continuity are not yet fully proven; every object class stays visible.
pub fn seeded_m5_ai_review_assist_matrix_publish_sheet_beta_narrowed(
) -> M5AiReviewAssistMatrixPacket {
    let mut packet = seeded_m5_ai_review_assist_matrix();
    packet.packet_id = "m5-ai-review-assist:publish-sheet-beta:0001".to_owned();
    let row = packet
        .ai_review_assist_rows
        .iter_mut()
        .find(|row| row.object_class == M5AiReviewAssistObject::PublishToReviewSheet)
        .expect("publish-to-review-sheet row present");
    row.qualification = M5AiReviewAssistQualificationClass::Beta;
    packet
}

/// Narrowed variant: the resolution memory row is narrowed to Preview pending durable reopen / rerun and
/// outdated-history proof; every object class stays visible.
pub fn seeded_m5_ai_review_assist_matrix_resolution_memory_preview_narrowed(
) -> M5AiReviewAssistMatrixPacket {
    let mut packet = seeded_m5_ai_review_assist_matrix();
    packet.packet_id = "m5-ai-review-assist:resolution-memory-preview:0001".to_owned();
    let row = packet
        .ai_review_assist_rows
        .iter_mut()
        .find(|row| row.object_class == M5AiReviewAssistObject::ResolutionMemoryRow)
        .expect("resolution-memory-row row present");
    row.qualification = M5AiReviewAssistQualificationClass::Preview;
    packet
}
