//! Canonical seed builders for the M5 prompt-composer-component-consumer lane.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked bindings, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical prompt-composer-component-consumer packet.
pub const M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_PACKET_ID: &str =
    "m5-prompt-composer-component-consumer:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked binding case for one consumer/family adoption.
fn case(
    consumer: M5ComposerComponentConsumer,
    component_family: M5PromptComposerComponentFamily,
    parity_health: M5ComposerParityHealth,
    export_caveats: &[M5ComposerConsumerExportCaveat],
    note: &str,
) -> M5ComposerBindingCase {
    M5ComposerBindingCase::resolved(M5ComposerBindingInput {
        consumer,
        component_family,
        descriptor_families: M5ComposerParityDescriptor::ALL.to_vec(),
        parity_health,
        export_caveats: export_caveats.to_vec(),
        note_repr: Some(note.to_owned()),
    })
}

/// Builds a component binding that points at its canonical family refs.
fn binding(
    component_family: M5PromptComposerComponentFamily,
    example_bindings: Vec<M5ComposerBindingCase>,
) -> M5ComposerComponentBinding {
    M5ComposerComponentBinding {
        component_family,
        canonical_schema_ref: family_canonical_schema_ref(component_family).to_owned(),
        canonical_artifact_ref: family_canonical_artifact_ref(component_family).to_owned(),
        references_canonical_not_local_prose: true,
        example_bindings,
    }
}

/// A base row with the shared parity vocabulary filled in.
fn base_row(
    consumer: M5ComposerComponentConsumer,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    component_bindings: Vec<M5ComposerComponentBinding>,
) -> M5ComposerComponentConsumerRow {
    M5ComposerComponentConsumerRow {
        consumer,
        qualification: M5ComposerQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5ComposerSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ComposerDeploymentLine::ALL.to_vec(),
        anatomy_parts: M5ComposerConsumerAnatomyPart::ALL.to_vec(),
        descriptor_families: M5ComposerParityDescriptor::ALL.to_vec(),
        parity_health_modes: M5ComposerParityHealth::ALL.to_vec(),
        export_caveats: M5ComposerConsumerExportCaveat::ALL.to_vec(),
        claim_parity_states: M5ComposerClaimParityState::ALL.to_vec(),
        narrowing_reasons: M5ComposerParityNarrowingReason::ALL.to_vec(),
        recovery_actions: M5ComposerParityRecoveryAction::ALL.to_vec(),
        export_fields: M5ComposerConsumerExportField::ALL.to_vec(),
        accessibility_routes: M5ComposerAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5ComposerConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5ComposerDowngradeTrigger::RouteOrProviderMasked,
            M5ComposerDowngradeTrigger::TaintStateHidden,
            M5ComposerDowngradeTrigger::DraftLocalityMasked,
            M5ComposerDowngradeTrigger::SendReviewGateBypassed,
            M5ComposerDowngradeTrigger::ProofStale,
        ],
        component_bindings,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_SCHEMA_REF,
            M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        ]),
        rewords_claims_per_surface: false,
        invents_new_composer_grammar: false,
        drops_locality_route_approval_or_taint_when_narrowed: false,
        inherits_stronger_label_from_healthier_surface: false,
    }
}

// Keep the numbered contract cases beside their explanatory comments.
#[allow(clippy::vec_init_then_push)]
fn consumer_rows() -> Vec<M5ComposerComponentConsumerRow> {
    use M5ComposerComponentConsumer as Consumer;
    use M5ComposerConsumerExportCaveat as Caveat;
    use M5ComposerParityHealth as Health;
    use M5PromptComposerComponentFamily as Family;

    let mut rows = Vec::new();

    // 1. Inline / panel composer — the header, attachment pill, mention resolver, and
    //    split-send review control, all at full parity: the authoritative live
    //    composer everyone else keeps parity with.
    rows.push(base_row(
        Consumer::InlinePanel,
        "Inline / panel composer surface owner",
        "The inline / panel composer adopts the prompt-composer header, context-attachment pill, mention resolver, and split-send review control at full parity, pointing at the canonical component schemas so locality, route/provider/model, approval, and taint language matches what patch review, the branch-agent console, docs/help, and the companion composer read",
        "evidence:m5-composer-consumer-inline-panel:001",
        vec![
            binding(
                Family::PromptComposerHeader,
                vec![case(
                    Consumer::InlinePanel,
                    Family::PromptComposerHeader,
                    Health::FullParity,
                    &[],
                    "inline composer header at full parity",
                )],
            ),
            binding(
                Family::ContextAttachmentPill,
                vec![case(
                    Consumer::InlinePanel,
                    Family::ContextAttachmentPill,
                    Health::FullParity,
                    &[],
                    "inline attachment pill at full parity",
                )],
            ),
            binding(
                Family::MentionResolver,
                vec![case(
                    Consumer::InlinePanel,
                    Family::MentionResolver,
                    Health::FullParity,
                    &[],
                    "inline mention resolver at full parity",
                )],
            ),
            binding(
                Family::SendReviewControl,
                vec![case(
                    Consumer::InlinePanel,
                    Family::SendReviewControl,
                    Health::FullParity,
                    &[],
                    "inline split-send review control at full parity",
                )],
            ),
        ],
    ));

    // 2. Patch review — the header, attachment pill, and budget strip at full parity,
    //    plus the split-send review control auto-narrowed because the patch-review
    //    workflow is review-only and the live send path is disabled there.
    rows.push(base_row(
        Consumer::PatchReview,
        "Patch-review surface owner",
        "Patch review adopts the prompt-composer header, context-attachment pill, and budget / size strip at full parity, and the split-send review control auto-narrowed because the workflow is review-only, keeping locality, route, approval, and taint explicit so a review-only surface never inherits the live composer's send label",
        "evidence:m5-composer-consumer-patch-review:001",
        vec![
            binding(
                Family::PromptComposerHeader,
                vec![case(
                    Consumer::PatchReview,
                    Family::PromptComposerHeader,
                    Health::FullParity,
                    &[],
                    "patch-review header at full parity",
                )],
            ),
            binding(
                Family::ContextAttachmentPill,
                vec![case(
                    Consumer::PatchReview,
                    Family::ContextAttachmentPill,
                    Health::FullParity,
                    &[],
                    "patch-review attachment pill at full parity",
                )],
            ),
            binding(
                Family::BudgetSizeStrip,
                vec![case(
                    Consumer::PatchReview,
                    Family::BudgetSizeStrip,
                    Health::FullParity,
                    &[],
                    "patch-review budget strip at full parity",
                )],
            ),
            binding(
                Family::SendReviewControl,
                vec![case(
                    Consumer::PatchReview,
                    Family::SendReviewControl,
                    Health::ReviewOnlyNarrowed,
                    &[Caveat::SendPathDisabledReviewOnly],
                    "patch-review send control narrowed to review-only",
                )],
            ),
        ],
    ));

    // 3. Branch-agent console — the header, slash-command row, and split-send review
    //    control at full parity, plus the draft-state row auto-narrowed because a
    //    handoff-only branch workflow keeps the draft in its originating composer.
    rows.push(base_row(
        Consumer::BranchAgent,
        "Branch-agent console surface owner",
        "The branch-agent console adopts the prompt-composer header, slash-command row, and split-send review control at full parity, and the draft-state row auto-narrowed under a handoff-only workflow, keeping the originating-composer path and draft locality explicit so a handed-off draft never appears live-editable by implication",
        "evidence:m5-composer-consumer-branch-agent:001",
        vec![
            binding(
                Family::PromptComposerHeader,
                vec![case(
                    Consumer::BranchAgent,
                    Family::PromptComposerHeader,
                    Health::FullParity,
                    &[],
                    "branch-agent header at full parity",
                )],
            ),
            binding(
                Family::SlashCommandRow,
                vec![case(
                    Consumer::BranchAgent,
                    Family::SlashCommandRow,
                    Health::FullParity,
                    &[],
                    "branch-agent slash-command row at full parity",
                )],
            ),
            binding(
                Family::DraftStateRow,
                vec![case(
                    Consumer::BranchAgent,
                    Family::DraftStateRow,
                    Health::HandoffOnlyNarrowed,
                    &[Caveat::DraftHandoffOnly],
                    "branch-agent draft-state row narrowed to handoff-only",
                )],
            ),
            binding(
                Family::SendReviewControl,
                vec![case(
                    Consumer::BranchAgent,
                    Family::SendReviewControl,
                    Health::FullParity,
                    &[],
                    "branch-agent split-send review control at full parity",
                )],
            ),
        ],
    ));

    // 4. Docs / help — the mention resolver, slash-command row, tainted-context
    //    warning, and attachment-stale banner, all at full parity, referencing the
    //    canonical schemas so its prose can never drift from the product truth.
    rows.push(base_row(
        Consumer::DocsHelp,
        "Docs/help surface owner",
        "The docs/help surface adopts the mention resolver, slash-command row, tainted-context warning, and attachment-stale banner at full parity, referencing the canonical component schemas so its prose can never drift from the product truth and taint, locality, route, and approval language stays exact",
        "evidence:m5-composer-consumer-docs-help:001",
        vec![
            binding(
                Family::MentionResolver,
                vec![case(
                    Consumer::DocsHelp,
                    Family::MentionResolver,
                    Health::FullParity,
                    &[],
                    "docs/help mention resolver at full parity",
                )],
            ),
            binding(
                Family::SlashCommandRow,
                vec![case(
                    Consumer::DocsHelp,
                    Family::SlashCommandRow,
                    Health::FullParity,
                    &[],
                    "docs/help slash-command row at full parity",
                )],
            ),
            binding(
                Family::TaintedContextWarning,
                vec![case(
                    Consumer::DocsHelp,
                    Family::TaintedContextWarning,
                    Health::FullParity,
                    &[],
                    "docs/help tainted-context warning at full parity",
                )],
            ),
            binding(
                Family::AttachmentStaleBanner,
                vec![case(
                    Consumer::DocsHelp,
                    Family::AttachmentStaleBanner,
                    Health::FullParity,
                    &[],
                    "docs/help attachment-stale banner at full parity",
                )],
            ),
        ],
    ));

    // 5. Companion composer — the attachment pill, tainted-context warning, and
    //    draft-state row at full parity, plus the budget strip auto-narrowed under an
    //    offline / mirrored scope and the attachment-stale banner auto-narrowed under
    //    a companion-scope limit; every descriptor stays disclosed.
    rows.push(base_row(
        Consumer::Companion,
        "Companion composer surface owner",
        "The companion composer adopts the context-attachment pill, tainted-context warning, and draft-state row at full parity, the budget / size strip auto-narrowed under an offline / mirrored scope, and the attachment-stale banner auto-narrowed under a companion-scope limit, keeping locality, route, approval, and taint disclosed so a reduced companion surface narrows visibly instead of borrowing the full composer's labels",
        "evidence:m5-composer-consumer-companion:001",
        vec![
            binding(
                Family::ContextAttachmentPill,
                vec![case(
                    Consumer::Companion,
                    Family::ContextAttachmentPill,
                    Health::FullParity,
                    &[],
                    "companion attachment pill at full parity",
                )],
            ),
            binding(
                Family::BudgetSizeStrip,
                vec![case(
                    Consumer::Companion,
                    Family::BudgetSizeStrip,
                    Health::OfflineMirrorNarrowed,
                    &[Caveat::RouteMirroredNotLive],
                    "companion budget strip narrowed by offline / mirror scope",
                )],
            ),
            binding(
                Family::TaintedContextWarning,
                vec![case(
                    Consumer::Companion,
                    Family::TaintedContextWarning,
                    Health::FullParity,
                    &[],
                    "companion tainted-context warning at full parity",
                )],
            ),
            binding(
                Family::DraftStateRow,
                vec![case(
                    Consumer::Companion,
                    Family::DraftStateRow,
                    Health::FullParity,
                    &[],
                    "companion draft-state row at full parity",
                )],
            ),
            binding(
                Family::AttachmentStaleBanner,
                vec![case(
                    Consumer::Companion,
                    Family::AttachmentStaleBanner,
                    Health::CompanionScopeNarrowed,
                    &[Caveat::CompanionScopeReduced],
                    "companion attachment-stale banner narrowed by companion-scope limit",
                )],
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5ComposerComponentConsumerGovernanceReview {
    M5ComposerComponentConsumerGovernanceReview {
        consumers_adopt_shared_primitives: true,
        consumers_reference_canonical_schema: true,
        descriptor_vocabulary_shared_not_reworded: true,
        no_consumer_invents_new_grammar: true,
        locality_route_approval_taint_explicit_on_every_surface: true,
        degraded_workflow_auto_narrows_claim: true,
        narrowed_rendering_always_shows_self_contained_banner: true,
        banner_names_exact_reason_and_recovery_action: true,
        help_and_companion_present_same_locality_and_route_truth: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5ComposerComponentConsumerProjection {
    M5ComposerComponentConsumerProjection {
        all_consumers_adopt_shared_components: true,
        locality_reads_single_source: true,
        route_reads_single_source: true,
        approval_reads_single_source: true,
        taint_reads_single_source: true,
    }
}

fn proof_freshness() -> M5ComposerComponentConsumerProofFreshness {
    M5ComposerComponentConsumerProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ComposerComponentConsumerReleasePosture {
    M5ComposerComponentConsumerReleasePosture {
        release_packet_ref: M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_ARTIFACT_REF.to_owned(),
        ai_audit_ref: M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_SCHEMA_REF,
        M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_DOC_REF,
        M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF,
        M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        family_canonical_schema_ref(M5PromptComposerComponentFamily::PromptComposerHeader),
        family_canonical_schema_ref(M5PromptComposerComponentFamily::MentionResolver),
        family_canonical_schema_ref(M5PromptComposerComponentFamily::BudgetSizeStrip),
        family_canonical_schema_ref(M5PromptComposerComponentFamily::DraftStateRow),
    ])
}

/// Builds the canonical M5 prompt-composer-component-consumer packet.
pub fn seeded_m5_prompt_composer_component_consumer_packet() -> M5ComposerComponentConsumerPacket {
    M5ComposerComponentConsumerPacket::new(M5ComposerComponentConsumerPacketInput {
        packet_id: M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_PACKET_ID.to_owned(),
        matrix_label:
            "M5 prompt-composer-component consumers: inline / panel composer, patch review, branch-agent console, docs/help, and companion composer keep locality, route, approval, and taint parity"
                .to_owned(),
        consumer_rows: consumer_rows(),
        vocabulary_set: M5ComposerComponentConsumerVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the branch-agent console is held at Beta because a slice of
/// branch renderings do not yet expose the auto-narrow banner on every handoff-only
/// path; every consumer stays visible.
pub fn seeded_m5_prompt_composer_component_consumer_branch_agent_beta_narrowed(
) -> M5ComposerComponentConsumerPacket {
    let mut packet = seeded_m5_prompt_composer_component_consumer_packet();
    packet.packet_id = "m5-prompt-composer-component-consumer:branch-agent-beta:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5ComposerComponentConsumer::BranchAgent)
        .expect("branch-agent row present");
    row.qualification = M5ComposerQualificationClass::Beta;
    packet
}

/// Narrowed variant: the companion composer is narrowed to Preview pending
/// companion-scope caveat-parity proof across every reduced-scope path; every consumer
/// stays visible.
pub fn seeded_m5_prompt_composer_component_consumer_companion_preview_narrowed(
) -> M5ComposerComponentConsumerPacket {
    let mut packet = seeded_m5_prompt_composer_component_consumer_packet();
    packet.packet_id = "m5-prompt-composer-component-consumer:companion-preview:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5ComposerComponentConsumer::Companion)
        .expect("companion row present");
    row.qualification = M5ComposerQualificationClass::Preview;
    packet
}
