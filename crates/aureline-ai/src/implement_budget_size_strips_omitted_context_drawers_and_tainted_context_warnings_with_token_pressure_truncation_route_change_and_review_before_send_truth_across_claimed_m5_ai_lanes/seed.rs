//! Canonical seed builders for the M5 budget-strip / tainted-context-warning primitive.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix,
//! the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical budget/taint-primitive packet.
pub const M5_BUDGET_TAINT_PACKET_ID: &str =
    "m5-budget-size-strip-tainted-context-warning-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds an omitted-context drawer entry.
fn omit(
    context_class: M5ContextClass,
    reason: M5OmittedContextReason,
    detail: &str,
) -> M5OmittedContextEntry {
    M5OmittedContextEntry {
        context_class,
        reason,
        detail: detail.to_owned(),
    }
}

/// Builds a worked budget-strip resolution case from a full strip state.
#[allow(clippy::too_many_arguments)]
fn budget_case(
    strip_id: &str,
    strip_label: &str,
    included: &[M5ContextClass],
    omitted: Vec<M5OmittedContextEntry>,
    unmetered_local: bool,
    hard_ceiling_hit: bool,
    over_budget: bool,
    truncation_pending: bool,
    near_limit: bool,
    route_before: Option<M5ComposerRouteClass>,
    route_after: M5ComposerRouteClass,
) -> M5BudgetSizeStripResolutionCase {
    M5BudgetSizeStripResolutionCase::resolved(M5BudgetSizeStripResolutionInput {
        strip_id: strip_id.to_owned(),
        strip_label: strip_label.to_owned(),
        included_context_classes: included.to_vec(),
        omitted_entries: omitted,
        unmetered_local,
        hard_ceiling_hit,
        over_budget,
        truncation_pending,
        near_limit,
        route_before,
        route_after,
    })
}

/// Builds a worked tainted-context-warning resolution case from a full warning state.
#[allow(clippy::too_many_arguments)]
fn taint_case(
    warning_id: &str,
    context_label: &str,
    taint_source: M5TaintSource,
    taint_severity: M5TaintSeverity,
    treated_as_data: bool,
    side_effecting_route: bool,
    acknowledged: bool,
    quarantine_note: Option<&str>,
) -> M5TaintedContextWarningResolutionCase {
    M5TaintedContextWarningResolutionCase::resolved(M5TaintedContextWarningResolutionInput {
        warning_id: warning_id.to_owned(),
        context_label: context_label.to_owned(),
        taint_source,
        taint_severity,
        treated_as_data,
        side_effecting_route,
        acknowledged,
        quarantine_note: quarantine_note.map(str::to_owned),
    })
}

/// A base row with the shared fields filled in and the full budget / taint anatomy, posture,
/// band, reason, class, route-switch, action, export-field, and accessibility parity every
/// consumer carries.
fn base_row(
    consumer_surface: M5BudgetTaintConsumerSurface,
    qualification: M5ComposerQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    budget_examples: Vec<M5BudgetSizeStripResolutionCase>,
    taint_examples: Vec<M5TaintedContextWarningResolutionCase>,
) -> M5BudgetTaintRow {
    M5BudgetTaintRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5ComposerSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ComposerDeploymentLine::ALL.to_vec(),
        budget_anatomy_parts: M5BudgetStripAnatomyPart::ALL.to_vec(),
        taint_anatomy_parts: M5TaintWarningAnatomyPart::ALL.to_vec(),
        budget_postures: M5BudgetPosture::ALL.to_vec(),
        pressure_bands: M5BudgetPressureBand::ALL.to_vec(),
        omitted_reasons: M5OmittedContextReason::ALL.to_vec(),
        context_classes: M5ContextClass::ALL.to_vec(),
        route_switch_consequences: M5RouteSwitchConsequence::ALL.to_vec(),
        budget_actions: M5BudgetStripAction::ALL.to_vec(),
        taint_sources: M5TaintSource::ALL.to_vec(),
        taint_severities: M5TaintSeverity::ALL.to_vec(),
        taint_warning_postures: M5TaintWarningPosture::ALL.to_vec(),
        taint_actions: M5TaintWarningAction::ALL.to_vec(),
        budget_export_fields: M5BudgetStripExportField::ALL.to_vec(),
        taint_export_fields: M5TaintWarningExportField::ALL.to_vec(),
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
            M5ComposerDowngradeTrigger::OmittedContextUndisclosed,
            M5ComposerDowngradeTrigger::BudgetOverrunHidden,
            M5ComposerDowngradeTrigger::TaintStateHidden,
            M5ComposerDowngradeTrigger::SendReviewGateBypassed,
            M5ComposerDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_BUDGET_TAINT_SCHEMA_REF,
            M5_BUDGET_TAINT_CONTEXT_ASSEMBLY_REF,
            M5_BUDGET_TAINT_TAINTED_CONTEXT_REF,
        ]),
        budget_examples,
        taint_examples,
        masks_budget_or_omission_truth: false,
        downplays_taint_source_or_severity: false,
        invents_private_context_grammar: false,
        bypasses_review_before_side_effecting_send: false,
    }
}

// Keep the numbered contract cases beside their explanatory comments.
#[allow(clippy::vec_init_then_push)]
fn rows() -> Vec<M5BudgetTaintRow> {
    use M5ComposerRouteClass as Route;
    use M5ContextClass as Ctx;
    use M5OmittedContextReason as Why;
    use M5TaintSeverity as Sev;
    use M5TaintSource as Src;

    let all = M5ContextClass::ALL;

    let mut rows = Vec::new();

    // 1. Inline composer — a within-budget strip with nothing omitted and an unchanged route,
    //    and a near-limit strip that truncates retrieved snippets for size; a pasted-external
    //    injection-suspected warning that blocks a side-effecting send and a pasted-external
    //    informational warning flagged as data.
    rows.push(base_row(
        M5BudgetTaintConsumerSurface::InlineComposer,
        M5ComposerQualificationClass::Stable,
        "Inline composer owner",
        "The inline composer renders the shared budget strip and tainted-context warning so a within-budget request shows every included context class with nothing omitted, a near-limit request names the retrieved snippets it truncated for size and offers an inspect-omitted path before send, a pasted-external injection-suspected input is blocked before a side-effecting route runs, and a pasted-external informational input is flagged as data rather than trusted instruction",
        "evidence:m5-budget-taint-inline:001",
        vec![
            budget_case(
                "strip.inline.within",
                "Inline composition budget",
                &all,
                vec![],
                false,
                false,
                false,
                false,
                false,
                Some(Route::ManagedRoute),
                Route::ManagedRoute,
            ),
            budget_case(
                "strip.inline.near",
                "Inline composition budget",
                &all,
                vec![omit(
                    Ctx::RetrievedSnippets,
                    Why::SizeTruncated,
                    "trimmed the lowest-ranked retrieved snippets to fit the size band",
                )],
                false,
                false,
                false,
                false,
                true,
                Some(Route::ManagedRoute),
                Route::ManagedRoute,
            ),
        ],
        vec![
            taint_case(
                "warn.inline.injection",
                "pasted block from an external chat",
                Src::PastedExternalText,
                Sev::InjectionSuspected,
                true,
                true,
                false,
                Some("held: the pasted text tries to redirect tool use; review before any side effect"),
            ),
            taint_case(
                "warn.inline.info",
                "pasted release-note snippet",
                Src::PastedExternalText,
                Sev::Informational,
                true,
                false,
                false,
                None,
            ),
        ],
    ));

    // 2. Side panel — an over-budget strip that caps conversation history and truncates
    //    retrieved snippets while the route moves on-device to managed (locality changed), and
    //    a truncation-pending strip that trims attached objects while the provider class
    //    changes; a promoted tool-output quarantine-required warning that blocks a
    //    side-effecting send and a promoted tool-output elevated warning that requires review.
    rows.push(base_row(
        M5BudgetTaintConsumerSurface::SidePanel,
        M5ComposerQualificationClass::Stable,
        "Side panel owner",
        "The side panel renders the same budget strip and tainted-context warning so an over-budget request names the conversation history it capped and the retrieved snippets it truncated and makes the on-device-to-managed route change explicit, a truncation-pending request names the attached objects it trimmed and the provider-class route change, a promoted tool-output quarantine-required input is held before a side-effecting route runs, and a promoted tool-output elevated input requires review before send",
        "evidence:m5-budget-taint-side-panel:001",
        vec![
            budget_case(
                "strip.side.over",
                "Side panel budget",
                &all,
                vec![
                    omit(
                        Ctx::ConversationHistory,
                        Why::BudgetCapped,
                        "capped older turns beyond the conversation-history budget",
                    ),
                    omit(
                        Ctx::RetrievedSnippets,
                        Why::SizeTruncated,
                        "trimmed retrieved snippets past the size band",
                    ),
                ],
                false,
                false,
                true,
                false,
                false,
                Some(Route::LocalModel),
                Route::ManagedRoute,
            ),
            budget_case(
                "strip.side.truncating",
                "Side panel budget",
                &all,
                vec![omit(
                    Ctx::AttachedObjects,
                    Why::SizeTruncated,
                    "trimmed the largest attached object to fit",
                )],
                false,
                false,
                false,
                true,
                false,
                Some(Route::ByokDirect),
                Route::ManagedRoute,
            ),
        ],
        vec![
            taint_case(
                "warn.side.quarantine",
                "promoted shell-tool output",
                Src::ToolOutput,
                Sev::QuarantineRequired,
                true,
                true,
                false,
                Some("held: promoted tool output carries untrusted directives; quarantine before send"),
            ),
            taint_case(
                "warn.side.elevated",
                "promoted linter output",
                Src::ToolOutput,
                Sev::Elevated,
                true,
                true,
                false,
                None,
            ),
        ],
    ));

    // 3. Patch draft — a hard-blocked strip that policy-excludes conversation history (cannot
    //    send), and an unmetered-local strip with nothing omitted and an unchanged local route;
    //    a fetched-url elevated warning acknowledged as data and proceedable, and an untainted
    //    prior-model-output warning that is trusted.
    rows.push(base_row(
        M5BudgetTaintConsumerSurface::PatchDraft,
        M5ComposerQualificationClass::Stable,
        "Patch draft owner",
        "The patch draft renders the same budget strip and tainted-context warning so a hard-blocked request names the conversation history it policy-excluded and refuses to send, an unmetered local request shows nothing omitted on an unchanged on-device route, a fetched-url elevated input can be acknowledged as data and proceed, and an untainted prior-model-output context reads as trusted with a proceed path",
        "evidence:m5-budget-taint-patch-draft:001",
        vec![
            budget_case(
                "strip.patch.hard",
                "Patch draft budget",
                &all,
                vec![omit(
                    Ctx::ConversationHistory,
                    Why::PolicyExcluded,
                    "excluded prior turns from a different workspace by policy",
                )],
                false,
                true,
                false,
                false,
                false,
                Some(Route::ManagedRoute),
                Route::ManagedRoute,
            ),
            budget_case(
                "strip.patch.local",
                "Patch draft budget",
                &all,
                vec![],
                true,
                false,
                false,
                false,
                false,
                Some(Route::LocalModel),
                Route::LocalModel,
            ),
        ],
        vec![
            taint_case(
                "warn.patch.ack",
                "fetched documentation page",
                Src::FetchedUrlContent,
                Sev::Elevated,
                true,
                false,
                true,
                None,
            ),
            taint_case(
                "warn.patch.trusted",
                "prior assistant turn in this thread",
                Src::PriorModelOutput,
                Sev::None,
                false,
                false,
                false,
                None,
            ),
        ],
    ));

    // 4. CLI / headless — a within-budget strip that dedup-collapses tool output while the route
    //    moves on-device to self-hosted (locality changed), and a near-limit strip that drops a
    //    stale active selection while the reach widens self-hosted to byok; an untrusted-file
    //    injection-suspected warning blocked, and a third-party-connector informational warning
    //    acknowledged and proceedable.
    rows.push(base_row(
        M5BudgetTaintConsumerSurface::CliHeadless,
        M5ComposerQualificationClass::Stable,
        "CLI / headless owner",
        "The CLI / headless surface renders the same budget strip and tainted-context warning so a within-budget request names the duplicate tool output it collapsed and the on-device-to-self-hosted route change, a near-limit request names the stale active selection it dropped and the widened self-hosted-to-byok reach, an untrusted-file injection-suspected input is blocked, and a third-party-connector informational input can be acknowledged and proceed — the same truth a headless reviewer reads elsewhere",
        "evidence:m5-budget-taint-cli:001",
        vec![
            budget_case(
                "strip.cli.dedup",
                "Headless budget",
                &all,
                vec![omit(
                    Ctx::ToolOutput,
                    Why::DedupCollapsed,
                    "collapsed identical tool output emitted twice in the run",
                )],
                false,
                false,
                false,
                false,
                false,
                Some(Route::LocalModel),
                Route::SelfHostedRoute,
            ),
            budget_case(
                "strip.cli.stale",
                "Headless budget",
                &all,
                vec![omit(
                    Ctx::ActiveSelection,
                    Why::StaleDropped,
                    "dropped the active selection after the buffer moved on",
                )],
                false,
                false,
                false,
                false,
                true,
                Some(Route::SelfHostedRoute),
                Route::ByokDirect,
            ),
        ],
        vec![
            taint_case(
                "warn.cli.injection",
                "untrusted file added to context",
                Src::UntrustedFile,
                Sev::InjectionSuspected,
                true,
                false,
                false,
                Some("held: the file body contains embedded instructions; treat as data only"),
            ),
            taint_case(
                "warn.cli.connector",
                "third-party connector payload",
                Src::ThirdPartyConnector,
                Sev::Informational,
                true,
                false,
                true,
                None,
            ),
        ],
    ));

    // 5. Support export — an over-budget strip that policy-excludes instructions and truncates
    //    retrieved snippets while the reach narrows byok to self-hosted, and a truncation-pending
    //    strip that budget-caps retrieved snippets on an unchanged route; a prior-model-output
    //    elevated warning requiring review, and a fetched-url quarantine-required warning held
    //    before a side-effecting route.
    rows.push(base_row(
        M5BudgetTaintConsumerSurface::SupportExport,
        M5ComposerQualificationClass::Stable,
        "Support export owner",
        "The support export renders the same budget strip and tainted-context warning so an over-budget request's excluded instructions, truncated retrieved snippets, and narrowed byok-to-self-hosted route are reconstructable from the export alone, a truncation-pending request names the retrieved snippets it budget-capped, a prior-model-output elevated input requires review, and a fetched-url quarantine-required input is held before a side-effecting route runs",
        "evidence:m5-budget-taint-support:001",
        vec![
            budget_case(
                "strip.support.over",
                "Support export budget",
                &all,
                vec![
                    omit(
                        Ctx::Instructions,
                        Why::PolicyExcluded,
                        "excluded repo instructions blocked on this deployment line",
                    ),
                    omit(
                        Ctx::RetrievedSnippets,
                        Why::SizeTruncated,
                        "trimmed retrieved snippets past the size band",
                    ),
                ],
                false,
                false,
                true,
                false,
                false,
                Some(Route::ByokDirect),
                Route::SelfHostedRoute,
            ),
            budget_case(
                "strip.support.capped",
                "Support export budget",
                &all,
                vec![omit(
                    Ctx::RetrievedSnippets,
                    Why::BudgetCapped,
                    "capped retrieved snippets at the retrieval budget",
                )],
                false,
                false,
                false,
                true,
                false,
                Some(Route::ManagedRoute),
                Route::ManagedRoute,
            ),
        ],
        vec![
            taint_case(
                "warn.support.elevated",
                "prior model output promoted to context",
                Src::PriorModelOutput,
                Sev::Elevated,
                true,
                false,
                false,
                None,
            ),
            taint_case(
                "warn.support.quarantine",
                "fetched page pulled into the draft",
                Src::FetchedUrlContent,
                Sev::QuarantineRequired,
                true,
                true,
                false,
                Some("held: fetched page carries untrusted directives; quarantine before send"),
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5BudgetTaintGovernanceReview {
    M5BudgetTaintGovernanceReview {
        one_primitive_carries_budget_and_taint_truth: true,
        budget_strip_names_included_and_omitted_context: true,
        omitted_context_always_names_reason_and_detail: true,
        truncation_reason_always_disclosed: true,
        route_switch_consequence_always_explicit: true,
        taint_source_and_severity_always_shown: true,
        untrusted_content_treated_as_data: true,
        taint_preserves_review_before_side_effecting_send: true,
        omission_and_route_change_exportable: true,
        every_row_declares_accessibility_route: true,
        descriptors_stable_across_ui_export_support: true,
    }
}

fn consumer_projection() -> M5BudgetTaintConsumerProjection {
    M5BudgetTaintConsumerProjection {
        send_capable_surfaces_consume_shared_primitive: true,
        budget_posture_reads_single_source: true,
        omitted_context_reads_single_source: true,
        taint_state_reads_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5BudgetTaintProofFreshness {
    M5BudgetTaintProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5BudgetTaintReleasePosture {
    M5BudgetTaintReleasePosture {
        release_packet_ref: M5_BUDGET_TAINT_ARTIFACT_REF.to_owned(),
        ai_audit_ref: M5_BUDGET_TAINT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_BUDGET_TAINT_SCHEMA_REF,
        M5_BUDGET_TAINT_DOC_REF,
        M5_BUDGET_TAINT_COMPONENT_MATRIX_REF,
        M5_BUDGET_TAINT_CONTEXT_ASSEMBLY_REF,
        M5_BUDGET_TAINT_TAINTED_CONTEXT_REF,
    ])
}

/// Builds the canonical M5 budget-strip / tainted-context-warning packet.
pub fn seeded_m5_budget_taint_packet() -> M5BudgetTaintPacket {
    M5BudgetTaintPacket::new(M5BudgetTaintPacketInput {
        packet_id: M5_BUDGET_TAINT_PACKET_ID.to_owned(),
        matrix_label:
            "M5 budget-size strip and tainted-context warning primitive: included and omitted context classes, budget posture, token/size pressure band, truncation reason, route-switch consequence, taint source, taint severity, warning posture, data-treatment, review path, and bounded inspect/adjust/review/reduce/proceed and review/quarantine/remove/acknowledge/proceed actions"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5BudgetTaintVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the patch draft is narrowed to Preview pending omitted-context drawer
/// parity proof across every patch-apply path; every consumer stays visible.
pub fn seeded_m5_budget_taint_patch_draft_preview_narrowed() -> M5BudgetTaintPacket {
    let mut packet = seeded_m5_budget_taint_packet();
    packet.packet_id =
        "m5-budget-size-strip-tainted-context-warning-primitive:patch-draft-preview:0001"
            .to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5BudgetTaintConsumerSurface::PatchDraft)
        .expect("patch-draft row present");
    row.qualification = M5ComposerQualificationClass::Preview;
    packet
}

/// Narrowed variant: the CLI / headless surface is held at Beta because a slice of headless
/// paths do not yet render the pressure-band cue on every profile; every consumer stays
/// visible.
pub fn seeded_m5_budget_taint_cli_headless_beta_narrowed() -> M5BudgetTaintPacket {
    let mut packet = seeded_m5_budget_taint_packet();
    packet.packet_id =
        "m5-budget-size-strip-tainted-context-warning-primitive:cli-headless-beta:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5BudgetTaintConsumerSurface::CliHeadless)
        .expect("cli-headless row present");
    row.qualification = M5ComposerQualificationClass::Beta;
    packet
}
