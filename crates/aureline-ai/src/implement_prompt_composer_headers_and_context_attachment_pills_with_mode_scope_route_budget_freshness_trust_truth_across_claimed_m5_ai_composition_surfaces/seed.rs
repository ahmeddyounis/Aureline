//! Canonical seed builders for the M5 prompt-composer-header / context-attachment-pill
//! primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical header/pill-primitive packet.
pub const M5_PROMPT_COMPOSER_HEADER_PILL_PACKET_ID: &str =
    "m5-prompt-composer-header-context-attachment-pill-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked header resolution case from a full composer-header state.
#[allow(clippy::too_many_arguments)]
fn header_case(
    composer_mode: M5ComposerMode,
    composer_scope: M5ComposerScope,
    route_class: M5ComposerRouteClass,
    provider_model_label: &str,
    budget_posture: M5BudgetPosture,
    route_blocked: bool,
    review_context_available: bool,
) -> M5PromptComposerHeaderResolutionCase {
    M5PromptComposerHeaderResolutionCase::resolved(M5PromptComposerHeaderResolutionInput {
        composer_mode,
        composer_scope,
        route_class,
        provider_model_label: provider_model_label.to_owned(),
        budget_posture,
        route_blocked,
        review_context_available,
    })
}

/// Builds a worked attachment-pill resolution case from a full attachment state.
#[allow(clippy::too_many_arguments)]
fn pill_case(
    attachment_id: &str,
    attachment_label: &str,
    attachment_kind: M5AttachmentKind,
    trust_state: M5AttachmentTrustState,
    is_stale: bool,
    staleness_reason: Option<M5StalenessReason>,
    source_removed: bool,
    in_scope: bool,
) -> M5ContextAttachmentPillResolutionCase {
    M5ContextAttachmentPillResolutionCase::resolved(M5ContextAttachmentPillResolutionInput {
        attachment_id: attachment_id.to_owned(),
        attachment_label: attachment_label.to_owned(),
        attachment_kind,
        trust_state,
        is_stale,
        staleness_reason,
        source_removed,
        in_scope,
    })
}

/// A base row with the shared fields filled in and the full header / pill anatomy, mode,
/// scope, route, budget, kind, trust, posture, action, export-field, and accessibility
/// parity every consumer carries.
fn base_row(
    consumer_surface: M5PromptComposerHeaderPillConsumerSurface,
    qualification: M5ComposerQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    header_examples: Vec<M5PromptComposerHeaderResolutionCase>,
    pill_examples: Vec<M5ContextAttachmentPillResolutionCase>,
) -> M5PromptComposerHeaderPillRow {
    M5PromptComposerHeaderPillRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5ComposerSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ComposerDeploymentLine::ALL.to_vec(),
        header_anatomy_parts: M5ComposerHeaderAnatomyPart::ALL.to_vec(),
        pill_anatomy_parts: M5AttachmentPillAnatomyPart::ALL.to_vec(),
        composer_modes: M5ComposerMode::ALL.to_vec(),
        composer_scopes: M5ComposerScope::ALL.to_vec(),
        route_classes: M5ComposerRouteClass::ALL.to_vec(),
        budget_postures: M5BudgetPosture::ALL.to_vec(),
        header_postures: M5ComposerHeaderPosture::ALL.to_vec(),
        attachment_kinds: M5AttachmentKind::ALL.to_vec(),
        attachment_trust_states: M5AttachmentTrustState::ALL.to_vec(),
        pill_postures: M5AttachmentPillPosture::ALL.to_vec(),
        pill_actions: M5AttachmentPillAction::ALL.to_vec(),
        header_export_fields: M5ComposerHeaderExportField::ALL.to_vec(),
        pill_export_fields: M5AttachmentPillExportField::ALL.to_vec(),
        accessibility_routes: M5ComposerAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ComposerConsumerSurface::InlineComposerUi,
            M5ComposerConsumerSurface::ComposerPanelUi,
            M5ComposerConsumerSurface::PatchReviewUi,
            M5ComposerConsumerSurface::SupportExport,
            M5ComposerConsumerSurface::CliInspect,
            M5ComposerConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5ComposerDowngradeTrigger::ComposerModeUnstated,
            M5ComposerDowngradeTrigger::RouteOrProviderMasked,
            M5ComposerDowngradeTrigger::AttachmentIdentityUnstated,
            M5ComposerDowngradeTrigger::AttachmentFreshnessMasked,
            M5ComposerDowngradeTrigger::BudgetOverrunHidden,
            M5ComposerDowngradeTrigger::SendReviewGateBypassed,
            M5ComposerDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_PROMPT_COMPOSER_HEADER_PILL_SCHEMA_REF,
            M5_PROMPT_COMPOSER_HEADER_PILL_RICHER_COMPOSER_REF,
            M5_PROMPT_COMPOSER_HEADER_PILL_ATTACHMENT_PROVENANCE_REF,
        ]),
        header_examples,
        pill_examples,
        masks_mode_or_route: false,
        hides_attachment_freshness_or_trust: false,
        invents_private_composer_grammar: false,
        bypasses_review_before_send: false,
    }
}

fn rows() -> Vec<M5PromptComposerHeaderPillRow> {
    use M5AttachmentKind as Kind;
    use M5AttachmentTrustState as Trust;
    use M5BudgetPosture as Budget;
    use M5ComposerMode as Mode;
    use M5ComposerRouteClass as Route;
    use M5ComposerScope as Scope;
    use M5StalenessReason as Stale;

    let mut rows = Vec::new();

    // 1. Inline assistant — a ready managed-route header and a local-only inline-edit
    //    header; a fresh-trusted file pill and a tainted external-paste pill.
    rows.push(base_row(
        M5PromptComposerHeaderPillConsumerSurface::InlineAssistant,
        M5ComposerQualificationClass::Stable,
        "Inline assistant owner",
        "The inline assistant renders the shared composer header and attachment pill so a ready managed-route chat header names its mode, scope, route, provider/model, and budget band, a local-only inline-edit header discloses that the request stays on device, and each attachment pill names its exact object identity and freshness/trust state — a fresh-trusted file that is openable and removable, and a tainted external paste that reads as tainted with review-trust and remove before send",
        "evidence:m5-composer-header-pill-inline:001",
        vec![
            header_case(
                Mode::ChatAsk,
                Scope::ActiveFile,
                Route::ManagedRoute,
                "managed:code-model-a",
                Budget::WithinBudget,
                false,
                true,
            ),
            header_case(
                Mode::InlineEdit,
                Scope::Selection,
                Route::LocalModel,
                "local:on-device-small",
                Budget::WithinBudget,
                false,
                true,
            ),
        ],
        vec![
            pill_case(
                "attach.file.readme",
                "README.md",
                Kind::File,
                Trust::TrustedFresh,
                false,
                None,
                false,
                true,
            ),
            pill_case(
                "attach.paste.stacktrace",
                "pasted stack trace",
                Kind::ExternalPaste,
                Trust::TaintedExternal,
                false,
                None,
                false,
                true,
            ),
        ],
    ));

    // 2. Side panel — a review-first header and a budget-constrained over-budget header; a
    //    stale symbol pill and an unverified URL-reference pill.
    rows.push(base_row(
        M5PromptComposerHeaderPillConsumerSurface::SidePanel,
        M5ComposerQualificationClass::Stable,
        "Side panel owner",
        "The side panel renders the shared composer header and attachment pill so a review-first workspace header requires review before send, an over-budget repository header reads as budget-constrained rather than plainly ready, a trusted-but-stale symbol pill reads as stale with a refresh action, and an unverified URL-reference pill reads as unverified with review-trust before send",
        "evidence:m5-composer-header-pill-side-panel:001",
        vec![
            header_case(
                Mode::ReviewFirst,
                Scope::Workspace,
                Route::ByokDirect,
                "byok:reasoning-model",
                Budget::WithinBudget,
                false,
                true,
            ),
            header_case(
                Mode::ChatAsk,
                Scope::Repository,
                Route::ManagedRoute,
                "managed:code-model-b",
                Budget::OverBudget,
                false,
                true,
            ),
        ],
        vec![
            pill_case(
                "attach.symbol.parse_config",
                "fn parse_config",
                Kind::Symbol,
                Trust::TrustedStale,
                true,
                Some(Stale::SourceEdited),
                false,
                true,
            ),
            pill_case(
                "attach.url.rfc",
                "linked reference",
                Kind::UrlReference,
                Trust::UnverifiedSource,
                false,
                None,
                false,
                true,
            ),
        ],
    ));

    // 3. Patch draft — a route-blocked policy-pinned header and a budget-blocked
    //    hard-ceiling header; a redacted selection pill and a deleted-source
    //    out-of-scope pill that is no longer openable but still removable.
    rows.push(base_row(
        M5PromptComposerHeaderPillConsumerSurface::PatchDraft,
        M5ComposerQualificationClass::Stable,
        "Patch draft owner",
        "The patch draft renders the shared composer header and attachment pill so a policy-pinned guided-patch header reads as route-blocked rather than ready, a hard-ceiling header reads as budget-blocked, a redacted selection pill reads as redacted with a reveal-scope action, and a deleted-source evidence pill reads as out-of-scope, is no longer openable, but still offers remove before send",
        "evidence:m5-composer-header-pill-patch-draft:001",
        vec![
            header_case(
                Mode::GuidedPatch,
                Scope::ActiveFile,
                Route::PolicyPinnedRoute,
                "policy:pinned-model",
                Budget::WithinBudget,
                true,
                true,
            ),
            header_case(
                Mode::GuidedPatch,
                Scope::OpenFiles,
                Route::ManagedRoute,
                "managed:code-model-c",
                Budget::HardBlocked,
                false,
                true,
            ),
        ],
        vec![
            pill_case(
                "attach.selection.range-1",
                "selected range",
                Kind::SelectionRange,
                Trust::RedactedScope,
                false,
                None,
                false,
                true,
            ),
            pill_case(
                "attach.evidence.deleted-run",
                "evidence packet",
                Kind::EvidencePacket,
                Trust::OutOfScope,
                true,
                Some(Stale::SourceDeleted),
                true,
                false,
            ),
        ],
    ));

    // 4. Handoff surface — a ready self-hosted background-agent header and a local-only
    //    headless-automation header; a fresh-trusted evidence pill and a moved-source
    //    stale file pill.
    rows.push(base_row(
        M5PromptComposerHeaderPillConsumerSurface::HandoffSurface,
        M5ComposerQualificationClass::Stable,
        "Handoff surface owner",
        "The handoff surface renders the shared composer header and attachment pill without changing meaning, so a self-hosted background-agent header reads as ready on a route that leaves the shell, a local-only headless-automation header discloses that the request stays on device, a fresh-trusted evidence pill is openable and removable, and a moved-source file pill reads as stale with a refresh action",
        "evidence:m5-composer-header-pill-handoff:001",
        vec![
            header_case(
                Mode::BackgroundAgent,
                Scope::Workspace,
                Route::SelfHostedRoute,
                "self-hosted:agent-model",
                Budget::WithinBudget,
                false,
                true,
            ),
            header_case(
                Mode::HeadlessAutomation,
                Scope::Repository,
                Route::LocalModel,
                "local:headless-runner",
                Budget::UnmeteredLocal,
                false,
                true,
            ),
        ],
        vec![
            pill_case(
                "attach.evidence.run-42",
                "evidence packet",
                Kind::EvidencePacket,
                Trust::TrustedFresh,
                false,
                None,
                false,
                true,
            ),
            pill_case(
                "attach.file.moved-mod",
                "src/moved_mod.rs",
                Kind::File,
                Trust::TrustedFresh,
                true,
                Some(Stale::SourceMoved),
                false,
                true,
            ),
        ],
    ));

    // 5. CLI / support export — a budget-constrained near-limit header and a ready
    //    mirrored-route header; an unverified external-paste pill and an out-of-scope
    //    symbol pill that is still openable — the same header/pill vocabulary a support or
    //    CLI reviewer reads elsewhere.
    rows.push(base_row(
        M5PromptComposerHeaderPillConsumerSurface::CliSupportExport,
        M5ComposerQualificationClass::Stable,
        "CLI / support export owner",
        "The CLI / support export renders the shared composer header and attachment pill so a near-limit managed-org header reads as budget-constrained, a ready mirrored-route header keeps its route explicit, and each attachment pill's identity, kind, trust state, freshness, and remove/open behavior are reconstructable from the support export alone",
        "evidence:m5-composer-header-pill-cli:001",
        vec![
            header_case(
                Mode::ChatAsk,
                Scope::ManagedOrg,
                Route::ManagedRoute,
                "managed:org-model",
                Budget::NearLimit,
                false,
                true,
            ),
            header_case(
                Mode::InlineEdit,
                Scope::ActiveFile,
                Route::MirroredRoute,
                "mirror:offline-safe-model",
                Budget::WithinBudget,
                false,
                true,
            ),
        ],
        vec![
            pill_case(
                "attach.paste.snippet",
                "pasted snippet",
                Kind::ExternalPaste,
                Trust::UnverifiedSource,
                false,
                None,
                false,
                true,
            ),
            pill_case(
                "attach.symbol.out-of-scope",
                "fn other_module_symbol",
                Kind::Symbol,
                Trust::OutOfScope,
                false,
                None,
                false,
                true,
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5PromptComposerHeaderPillGovernanceReview {
    M5PromptComposerHeaderPillGovernanceReview {
        one_primitive_carries_header_and_pill_truth: true,
        mode_scope_route_budget_always_shown: true,
        header_posture_never_masks_blocked: true,
        local_only_route_always_disclosed: true,
        attachment_identity_always_preserved: true,
        attachment_freshness_and_trust_never_masked: true,
        remove_action_always_offered: true,
        support_export_reconstructs_header_and_pill_truth: true,
        no_surface_invents_parallel_vocabulary: true,
        every_row_declares_accessibility_route: true,
        descriptors_stable_across_ui_export_support: true,
    }
}

fn consumer_projection() -> M5PromptComposerHeaderPillConsumerProjection {
    M5PromptComposerHeaderPillConsumerProjection {
        composition_surfaces_consume_shared_primitive: true,
        header_posture_reads_single_source: true,
        pill_posture_reads_single_source: true,
        pill_actions_read_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5PromptComposerHeaderPillProofFreshness {
    M5PromptComposerHeaderPillProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5PromptComposerHeaderPillReleasePosture {
    M5PromptComposerHeaderPillReleasePosture {
        release_packet_ref: M5_PROMPT_COMPOSER_HEADER_PILL_ARTIFACT_REF.to_owned(),
        ai_audit_ref: M5_PROMPT_COMPOSER_HEADER_PILL_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_PROMPT_COMPOSER_HEADER_PILL_SCHEMA_REF,
        M5_PROMPT_COMPOSER_HEADER_PILL_DOC_REF,
        M5_PROMPT_COMPOSER_HEADER_PILL_COMPONENT_MATRIX_REF,
        M5_PROMPT_COMPOSER_HEADER_PILL_RICHER_COMPOSER_REF,
        M5_PROMPT_COMPOSER_HEADER_PILL_ATTACHMENT_PROVENANCE_REF,
    ])
}

/// Builds the canonical M5 prompt-composer-header / context-attachment-pill packet.
pub fn seeded_m5_prompt_composer_header_pill_packet() -> M5PromptComposerHeaderPillPacket {
    M5PromptComposerHeaderPillPacket::new(M5PromptComposerHeaderPillPacketInput {
        packet_id: M5_PROMPT_COMPOSER_HEADER_PILL_PACKET_ID.to_owned(),
        matrix_label:
            "M5 prompt composer header and context attachment pill primitive: mode, scope, route/provider/model, budget band, header posture, attachment identity, kind, trust state, freshness, pill posture, and bounded open/remove/refresh/review/reveal actions"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5PromptComposerHeaderPillVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the patch draft is narrowed to Preview pending header-posture parity
/// proof across every headless export path; every consumer stays visible.
pub fn seeded_m5_prompt_composer_header_pill_patch_draft_preview_narrowed(
) -> M5PromptComposerHeaderPillPacket {
    let mut packet = seeded_m5_prompt_composer_header_pill_packet();
    packet.packet_id =
        "m5-prompt-composer-header-context-attachment-pill-primitive:patch-draft-preview:0001"
            .to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5PromptComposerHeaderPillConsumerSurface::PatchDraft)
        .expect("patch-draft row present");
    row.qualification = M5ComposerQualificationClass::Preview;
    packet
}

/// Narrowed variant: the handoff surface is held at Beta because a slice of handoff pills
/// do not yet render the freshness cue on every profile; every consumer stays visible.
pub fn seeded_m5_prompt_composer_header_pill_handoff_beta_narrowed(
) -> M5PromptComposerHeaderPillPacket {
    let mut packet = seeded_m5_prompt_composer_header_pill_packet();
    packet.packet_id =
        "m5-prompt-composer-header-context-attachment-pill-primitive:handoff-beta:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| {
            row.consumer_surface == M5PromptComposerHeaderPillConsumerSurface::HandoffSurface
        })
        .expect("handoff-surface row present");
    row.qualification = M5ComposerQualificationClass::Beta;
    packet
}
