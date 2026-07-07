//! Canonical seed builders for the frozen M5 prompt-composer-component matrix.
//!
//! These builders are the single producer of the checked-in support export and
//! the narrowed fixtures. The headless emitter and the inline tests both call them
//! so the in-code matrix, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical prompt-composer-component matrix.
pub const M5_PROMPT_COMPOSER_COMPONENT_MATRIX_PACKET_ID: &str =
    "m5-prompt-composer-components:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every component must be able to show.
fn mandatory_labels() -> Vec<M5ComposerRequiredLabel> {
    M5ComposerRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a component carries.
fn labels_with(extra: &[M5ComposerRequiredLabel]) -> Vec<M5ComposerRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every component filled in and every
/// family-specific vocabulary left empty for the caller to populate.
fn base_row(
    component_family: M5PromptComposerComponentFamily,
    qualification: M5ComposerQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5PromptComposerComponentRow {
    M5PromptComposerComponentRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5ComposerSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ComposerDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        composer_modes: vec![],
        composer_scopes: vec![],
        route_classes: vec![],
        attachment_kinds: vec![],
        attachment_trust_states: vec![],
        mention_resolutions: vec![],
        slash_command_states: vec![],
        budget_postures: vec![],
        omitted_context_reasons: vec![],
        taint_sources: vec![],
        taint_severities: vec![],
        draft_localities: vec![],
        staleness_reasons: vec![],
        send_postures: vec![],
        review_requirements: vec![],
        accessibility_routes: M5ComposerAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ComposerConsumerSurface::InlineComposerUi,
            M5ComposerConsumerSurface::SupportExport,
            M5ComposerConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5ComposerDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        masks_mode_or_route: false,
        hides_taint_or_trust_state: false,
        invents_private_composer_grammar: false,
        bypasses_send_review_gate: false,
    }
}

fn component_rows() -> Vec<M5PromptComposerComponentRow> {
    use M5AttachmentKind as AK;
    use M5AttachmentTrustState as AT;
    use M5BudgetPosture as BP;
    use M5ComposerConsumerSurface as C;
    use M5ComposerDowngradeTrigger as D;
    use M5ComposerMode as CM;
    use M5ComposerQualificationClass as Q;
    use M5ComposerRequiredLabel as L;
    use M5ComposerRouteClass as RC;
    use M5ComposerScope as SC;
    use M5DraftLocality as DL;
    use M5MentionResolution as MR;
    use M5OmittedContextReason as OC;
    use M5PromptComposerComponentFamily as F;
    use M5ReviewRequirement as RV;
    use M5SendPosture as SP;
    use M5SlashCommandState as SL;
    use M5StalenessReason as ST;
    use M5TaintSeverity as TV;
    use M5TaintSource as TS;

    let mut rows = Vec::new();

    // 1. Prompt-composer header.
    let mut row = base_row(
        F::PromptComposerHeader,
        Q::Stable,
        "Prompt-composer header owner",
        "One prompt-composer-header model naming the intent mode — chat/ask, inline edit, guided patch, background agent, review-first, or headless — the scope the request reaches, and the route/provider/model it will run under, so a user never has to infer what will be sent, how wide it reaches, or whether the request stays local or crosses a managed boundary",
        "evidence:m5-prompt-composer-header-parity:001",
        &[
            M5_PROMPT_COMPOSER_COMPONENT_SCHEMA_REF,
            M5_PROMPT_COMPOSER_COMPONENT_ASSEMBLY_REF,
        ],
    );
    row.composer_modes = CM::ALL.to_vec();
    row.composer_scopes = SC::ALL.to_vec();
    row.route_classes = RC::ALL.to_vec();
    row.required_labels = labels_with(&[L::ComposerMode, L::RouteProviderModel]);
    row.consumer_surfaces = vec![
        C::InlineComposerUi,
        C::ComposerPanelUi,
        C::PatchReviewUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ComposerModeUnstated,
        D::RouteOrProviderMasked,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Context-attachment pill.
    let mut row = base_row(
        F::ContextAttachmentPill,
        Q::Stable,
        "Context-attachment pill owner",
        "One context-attachment-pill model naming which object is attached — a file, symbol, selection range, evidence packet, external paste, or URL reference — and its trust state, so a stale, unverified, tainted, redacted, or out-of-scope attachment is never shown as trusted and fresh",
        "evidence:m5-context-attachment-pill-parity:001",
        &[
            M5_PROMPT_COMPOSER_COMPONENT_SCHEMA_REF,
            M5_PROMPT_COMPOSER_COMPONENT_ATTACHMENT_REF,
        ],
    );
    row.attachment_kinds = AK::ALL.to_vec();
    row.attachment_trust_states = AT::ALL.to_vec();
    row.required_labels = labels_with(&[L::TrustOrTaint]);
    row.consumer_surfaces = vec![
        C::ComposerPanelUi,
        C::InlineComposerUi,
        C::PatchReviewUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::AttachmentIdentityUnstated,
        D::AttachmentFreshnessMasked,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Mention resolver.
    let mut row = base_row(
        F::MentionResolver,
        Q::Stable,
        "Mention-resolver owner",
        "One mention-resolver model naming whether an @-mention resolved to a unique or pinned object, is ambiguous across candidates, is unresolved / missing, was denied as out-of-scope, or is deferred pending resolution, so an unresolved or ambiguous mention is never sent as if it bound cleanly",
        "evidence:m5-mention-resolver-parity:001",
        &[
            M5_PROMPT_COMPOSER_COMPONENT_SCHEMA_REF,
            M5_PROMPT_COMPOSER_COMPONENT_ATTACHMENT_REF,
        ],
    );
    row.mention_resolutions = MR::ALL.to_vec();
    row.required_labels = labels_with(&[L::TrustOrTaint]);
    row.consumer_surfaces = vec![
        C::InlineComposerUi,
        C::ComposerPanelUi,
        C::HelpComposerUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![D::MentionLeftUnresolved, D::ProofStale];
    rows.push(row);

    // 4. Slash-command row.
    let mut row = base_row(
        F::SlashCommandRow,
        Q::Stable,
        "Slash-command row owner",
        "One slash-command-row model naming whether a command is available, disabled by an unmet precondition, requires approval, is deprecated / aliased, is hidden by policy, or is unknown, so a disabled, approval-gated, or policy-hidden command is never shown as a plain ready action",
        "evidence:m5-slash-command-row-parity:001",
        &[
            M5_PROMPT_COMPOSER_COMPONENT_SCHEMA_REF,
            M5_PROMPT_COMPOSER_COMPONENT_ASSEMBLY_REF,
        ],
    );
    row.slash_command_states = SL::ALL.to_vec();
    row.consumer_surfaces = vec![
        C::InlineComposerUi,
        C::ComposerPanelUi,
        C::HelpComposerUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![D::SendReviewGateBypassed, D::ProofStale];
    rows.push(row);

    // 5. Budget / size strip.
    let mut row = base_row(
        F::BudgetSizeStrip,
        Q::Stable,
        "Budget-strip owner",
        "One budget-or-size-strip model naming whether the request is within budget, near the limit, over budget, pending truncation, hard-blocked, or unmetered-local, and why any context was omitted or truncated, so an over-budget request or silently dropped context is never presented as a clean within-budget send",
        "evidence:m5-budget-size-strip-parity:001",
        &[
            M5_PROMPT_COMPOSER_COMPONENT_SCHEMA_REF,
            M5_PROMPT_COMPOSER_COMPONENT_ASSEMBLY_REF,
        ],
    );
    row.budget_postures = BP::ALL.to_vec();
    row.omitted_context_reasons = OC::ALL.to_vec();
    row.required_labels = labels_with(&[L::RouteProviderModel]);
    row.consumer_surfaces = vec![
        C::ComposerPanelUi,
        C::BranchAgentConsoleUi,
        C::InlineComposerUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::OmittedContextUndisclosed,
        D::BudgetOverrunHidden,
        D::ProofStale,
    ];
    rows.push(row);

    // 6. Tainted-context warning.
    let mut row = base_row(
        F::TaintedContextWarning,
        Q::Stable,
        "Tainted-context warning owner",
        "One tainted-context-warning model naming where untrusted context came from — pasted external text, tool output, fetched URL content, an untrusted file, a third-party connector, or prior model output — and how severe the taint is, so injection-suspected or quarantine-required context is never shown as trusted before send",
        "evidence:m5-tainted-context-warning-parity:001",
        &[
            M5_PROMPT_COMPOSER_COMPONENT_SCHEMA_REF,
            M5_PROMPT_COMPOSER_COMPONENT_TAINT_REF,
        ],
    );
    row.taint_sources = TS::ALL.to_vec();
    row.taint_severities = TV::ALL.to_vec();
    row.required_labels = labels_with(&[L::TrustOrTaint]);
    row.consumer_surfaces = vec![
        C::InlineComposerUi,
        C::ComposerPanelUi,
        C::PatchReviewUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![D::TaintStateHidden, D::ProofStale];
    rows.push(row);

    // 7. Draft-state row.
    let mut row = base_row(
        F::DraftStateRow,
        Q::Stable,
        "Draft-state row owner",
        "One draft-state-row model naming whether a composer draft is local-only, workspace-synced, account-synced, shared to a thread, ephemeral / unsaved, or retained pending purge, so a local-only draft is never shown as synced and a retained draft is never shown as purged before send",
        "evidence:m5-draft-state-row-parity:001",
        &[
            M5_PROMPT_COMPOSER_COMPONENT_SCHEMA_REF,
            M5_PROMPT_COMPOSER_COMPONENT_DRAFT_REF,
        ],
    );
    row.draft_localities = DL::ALL.to_vec();
    row.consumer_surfaces = vec![
        C::InlineComposerUi,
        C::ComposerPanelUi,
        C::CompanionComposerUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![D::DraftLocalityMasked, D::ProofStale];
    rows.push(row);

    // 8. Attachment-stale banner.
    let mut row = base_row(
        F::AttachmentStaleBanner,
        Q::Stable,
        "Attachment-stale banner owner",
        "One attachment-stale-banner model naming why an attachment is stale — the source was edited, moved, or deleted, a newer revision superseded it, permission was revoked, or the index was rebuilt — so a moved, deleted, or permission-revoked attachment is never left silently attached before send",
        "evidence:m5-attachment-stale-banner-parity:001",
        &[
            M5_PROMPT_COMPOSER_COMPONENT_SCHEMA_REF,
            M5_PROMPT_COMPOSER_COMPONENT_ATTACHMENT_REF,
        ],
    );
    row.staleness_reasons = ST::ALL.to_vec();
    row.required_labels = labels_with(&[L::TrustOrTaint]);
    row.consumer_surfaces = vec![
        C::ComposerPanelUi,
        C::PatchReviewUi,
        C::InlineComposerUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![D::AttachmentStalenessUndisclosed, D::ProofStale];
    rows.push(row);

    // 9. Split-send / review-before-send control.
    let mut row = base_row(
        F::SendReviewControl,
        Q::Stable,
        "Send-review control owner",
        "One split-send / review-before-send control model naming whether a request is ready to send, needs a split-send review, needs review before send, or is blocked by policy, budget, or taint, and which acknowledgement it demands first, so a request that needs review or is blocked never sends as a plain ready action",
        "evidence:m5-send-review-control-parity:001",
        &[
            M5_PROMPT_COMPOSER_COMPONENT_SCHEMA_REF,
            M5_PROMPT_COMPOSER_COMPONENT_DRAFT_REF,
        ],
    );
    row.send_postures = SP::ALL.to_vec();
    row.review_requirements = RV::ALL.to_vec();
    row.required_labels = labels_with(&[L::ComposerMode, L::RouteProviderModel]);
    row.consumer_surfaces = vec![
        C::InlineComposerUi,
        C::ComposerPanelUi,
        C::BranchAgentConsoleUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::SendReviewGateBypassed,
        D::RouteOrProviderMasked,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5PromptComposerComponentGovernanceReview {
    M5PromptComposerComponentGovernanceReview {
        header_shows_mode_scope_and_route: true,
        attachment_pill_shows_identity_and_trust: true,
        mention_resolver_shows_resolution_state: true,
        slash_command_row_shows_availability_and_gate: true,
        budget_strip_shows_budget_and_omitted_context: true,
        tainted_warning_shows_source_and_severity: true,
        draft_state_row_shows_locality_and_retention: true,
        attachment_stale_banner_shows_staleness_reason: true,
        send_review_control_shows_posture_and_review: true,
        local_only_draft_never_shown_as_synced: true,
        tainted_context_never_shown_as_trusted: true,
        no_component_invents_second_composer_grammar: true,
        every_component_declares_deployment_lines: true,
        every_component_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5PromptComposerComponentConsumerProjection {
    M5PromptComposerComponentConsumerProjection {
        inline_and_panel_surfaces_consume_mode_vocabulary: true,
        attachment_and_mention_surfaces_consume_trust_vocabulary: true,
        budget_surfaces_consume_omitted_context_vocabulary: true,
        send_surfaces_consume_review_gate_vocabulary: true,
        support_export_reads_single_source: true,
        help_and_companion_surfaces_read_single_source: true,
    }
}

fn proof_freshness() -> M5PromptComposerComponentProofFreshness {
    M5PromptComposerComponentProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5PromptComposerComponentReleasePosture {
    M5PromptComposerComponentReleasePosture {
        proof_packet_ref: M5_PROMPT_COMPOSER_COMPONENT_ARTIFACT_REF.to_owned(),
        ai_audit_ref: M5_PROMPT_COMPOSER_COMPONENT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_PROMPT_COMPOSER_COMPONENT_SCHEMA_REF,
        M5_PROMPT_COMPOSER_COMPONENT_DOC_REF,
        M5_PROMPT_COMPOSER_COMPONENT_DRAFT_REF,
        M5_PROMPT_COMPOSER_COMPONENT_ATTACHMENT_REF,
        M5_PROMPT_COMPOSER_COMPONENT_TAINT_REF,
        M5_PROMPT_COMPOSER_COMPONENT_ASSEMBLY_REF,
    ])
}

/// Builds the canonical frozen M5 prompt-composer-component matrix packet.
pub fn seeded_m5_prompt_composer_component_matrix() -> M5PromptComposerComponentMatrixPacket {
    M5PromptComposerComponentMatrixPacket::new(M5PromptComposerComponentMatrixPacketInput {
        packet_id: M5_PROMPT_COMPOSER_COMPONENT_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 prompt-composer-header, context-attachment-pill, mention-resolver, slash-command-row, budget-strip, tainted-context-warning, draft-state-row, attachment-stale-banner, and send-review-control component matrix"
                .to_owned(),
        component_rows: component_rows(),
        vocabulary_set: M5PromptComposerComponentVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the tainted-context warning is held at Beta because a slice of
/// taint severities does not yet round-trip across every composer surface; every
/// component stays visible.
pub fn seeded_m5_prompt_composer_component_matrix_tainted_context_warning_beta_narrowed(
) -> M5PromptComposerComponentMatrixPacket {
    let mut packet = seeded_m5_prompt_composer_component_matrix();
    packet.packet_id = "m5-prompt-composer-components:tainted-context-warning-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5PromptComposerComponentFamily::TaintedContextWarning)
        .expect("tainted-context-warning row present");
    row.qualification = M5ComposerQualificationClass::Beta;
    packet
}

/// Narrowed variant: the send-review control is narrowed to Preview pending
/// split-send review parity proof across every composer surface; every component
/// stays visible.
pub fn seeded_m5_prompt_composer_component_matrix_send_review_control_preview_narrowed(
) -> M5PromptComposerComponentMatrixPacket {
    let mut packet = seeded_m5_prompt_composer_component_matrix();
    packet.packet_id = "m5-prompt-composer-components:send-review-control-preview:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5PromptComposerComponentFamily::SendReviewControl)
        .expect("send-review-control row present");
    row.qualification = M5ComposerQualificationClass::Preview;
    packet
}
