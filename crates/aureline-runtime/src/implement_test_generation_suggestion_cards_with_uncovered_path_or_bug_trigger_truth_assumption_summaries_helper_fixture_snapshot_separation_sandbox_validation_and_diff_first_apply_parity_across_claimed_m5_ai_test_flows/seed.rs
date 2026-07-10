//! Canonical seed builders for the M5 test-generation-suggestion-card primitive.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix, the
//! artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical suggestion-card-components primitive packet.
pub const M5_SUGGESTION_CARD_COMPONENTS_PACKET_ID: &str =
    "m5-test-generation-suggestion-card-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Named-field seed for one worked suggestion-card resolution case.
struct SuggestionSeed<'a> {
    trigger_source: M5GenerationTriggerSource,
    target_refs: &'a [&'a str],
    trigger_context_ref: &'a str,
    assumption_classes: &'a [M5GeneratedAssumptionClass],
    review_classes: &'a [M5GeneratedReviewClass],
    apply_scope: M5GeneratedApplyScope,
    generated_file_count: u32,
    provenance_class: M5TestIntelligenceProvenanceClass,
    offers_sandbox_run: bool,
    offers_diff_preview: bool,
    offers_rollback: bool,
    suggestion_identity_ref: &'a str,
}

/// Builds a worked suggestion-card resolution case from a full suggestion state.
fn suggestion_case(seed: SuggestionSeed) -> M5SuggestionCardResolutionCase {
    M5SuggestionCardResolutionCase::resolved(M5SuggestionCardResolutionInput {
        trigger_source: seed.trigger_source,
        target_refs: strings(seed.target_refs),
        trigger_context_ref: seed.trigger_context_ref.to_owned(),
        assumption_classes: seed.assumption_classes.to_vec(),
        review_classes: seed.review_classes.to_vec(),
        apply_scope: seed.apply_scope,
        generated_file_count: seed.generated_file_count,
        provenance_class: seed.provenance_class,
        offers_sandbox_run: seed.offers_sandbox_run,
        offers_diff_preview: seed.offers_diff_preview,
        offers_rollback: seed.offers_rollback,
        suggestion_identity_ref: seed.suggestion_identity_ref.to_owned(),
    })
}

/// A base row with the shared fields filled in and the full suggestion anatomy, trigger source,
/// review class, assumption class, apply scope, suggestion posture, provenance, action,
/// export-field, and accessibility parity every consumer carries.
fn base_row(
    consumer_surface: M5SuggestionCardConsumerSurface,
    qualification: M5TestIntelligenceQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    suggestion_examples: Vec<M5SuggestionCardResolutionCase>,
) -> M5SuggestionCardConsumerRow {
    M5SuggestionCardConsumerRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5TestIntelligenceSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5TestIntelligenceDeploymentLine::ALL.to_vec(),
        suggestion_anatomy_parts: M5SuggestionCardAnatomyPart::ALL.to_vec(),
        trigger_sources: M5GenerationTriggerSource::ALL.to_vec(),
        review_classes: M5GeneratedReviewClass::ALL.to_vec(),
        assumption_classes: M5GeneratedAssumptionClass::ALL.to_vec(),
        apply_scopes: M5GeneratedApplyScope::ALL.to_vec(),
        suggestion_postures: M5SuggestionPosture::ALL.to_vec(),
        provenance_classes: M5TestIntelligenceProvenanceClass::ALL.to_vec(),
        suggestion_actions: M5SuggestionCardAction::ALL.to_vec(),
        suggestion_export_fields: M5SuggestionCardExportField::ALL.to_vec(),
        accessibility_routes: M5TestIntelligenceAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5TestIntelligenceConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5TestIntelligenceDowngradeTrigger::GeneratedAssumptionHidden,
            M5TestIntelligenceDowngradeTrigger::OpaqueApplyBundle,
            M5TestIntelligenceDowngradeTrigger::ProvenanceClassUnstated,
            M5TestIntelligenceDowngradeTrigger::FreshnessClassUndisclosed,
            M5TestIntelligenceDowngradeTrigger::AlternateStateLabelInvented,
            M5TestIntelligenceDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_SUGGESTION_CARD_COMPONENTS_SCHEMA_REF,
            M5_SUGGESTION_CARD_COMPONENTS_TEST_GENERATION_REF,
        ]),
        suggestion_examples,
        bundles_assumption_fixture_or_snapshot_into_opaque_apply: false,
        hides_trigger_source_or_target_symbols: false,
        hides_assumption_summary_or_generated_file_count: false,
        invents_alternate_suggestion_or_apply_state_label: false,
    }
}

fn rows() -> Vec<M5SuggestionCardConsumerRow> {
    use M5GeneratedApplyScope as Scope;
    use M5GeneratedAssumptionClass as Assume;
    use M5GeneratedReviewClass as Class;
    use M5GenerationTriggerSource as Trigger;
    use M5TestIntelligenceProvenanceClass as Prov;

    vec![
        // 1. Suggestion review panel — an assertion-only suggestion for an uncovered line that is
        //    apply-capable precisely because its scope names only assertions and it keeps a
        //    diff-first preview and rollback, and a fixture-and-assertion suggestion for a failing
        //    bug repro that names both its assertion and its fixture churn before offering apply.
        base_row(
            M5SuggestionCardConsumerSurface::SuggestionReviewPanel,
            M5TestIntelligenceQualificationClass::Stable,
            "Suggestion review panel owner",
            "The suggestion review panel renders the shared test-generation suggestion card so an assertion-only proposal for an uncovered line becomes apply-capable only when it names its assertion review class and keeps a diff-first preview and a rollback, and a fixture-and-assertion proposal for a failing bug repro separates its assertion churn from its helper / fixture churn — with its generated-test assumptions summarised — before any apply-capable action is offered",
            "evidence:m5-suggestion-review-panel:001",
            vec![
                suggestion_case(SuggestionSeed {
                    trigger_source: Trigger::UncoveredLine,
                    target_refs: &["src/checkout/cart.rs::apply_discount"],
                    trigger_context_ref: "uncovered:checkout::apply-discount-line-84",
                    assumption_classes: &[Assume::AssertionInferred],
                    review_classes: &[Class::AssertionChange],
                    apply_scope: Scope::AssertionOnly,
                    generated_file_count: 1,
                    provenance_class: Prov::VerifiedCurrentRun,
                    offers_sandbox_run: true,
                    offers_diff_preview: true,
                    offers_rollback: true,
                    suggestion_identity_ref: "suggestion-card:review-panel::uncovered-line",
                }),
                suggestion_case(SuggestionSeed {
                    trigger_source: Trigger::FailingBugRepro,
                    target_refs: &[
                        "src/checkout/tax.rs::compute_tax",
                        "tests/support/tax_fixtures.rs",
                    ],
                    trigger_context_ref: "bug:checkout::tax-rounding-regression-1188",
                    assumption_classes: &[Assume::FixtureAssumed, Assume::AssertionInferred],
                    review_classes: &[Class::AssertionChange, Class::HelperOrFixtureAddition],
                    apply_scope: Scope::FixtureAndAssertion,
                    generated_file_count: 2,
                    provenance_class: Prov::VerifiedCurrentRun,
                    offers_sandbox_run: true,
                    offers_diff_preview: true,
                    offers_rollback: true,
                    suggestion_identity_ref: "suggestion-card:review-panel::bug-repro-fixture",
                }),
            ],
        ),
        // 2. Editor inline suggestion — a snapshot-included suggestion for an uncovered branch that
        //    stays apply-capable only because its scope explicitly names its snapshot / golden
        //    review class alongside its assertion and fixture churn, with its snapshot assumption
        //    summarised and a diff-first preview and rollback preserved.
        base_row(
            M5SuggestionCardConsumerSurface::EditorSuggestionInline,
            M5TestIntelligenceQualificationClass::Stable,
            "Editor inline suggestion owner",
            "The editor inline-suggestion surface renders the shared test-generation suggestion card so a snapshot-included proposal for an uncovered branch stays apply-capable only when its scope names its snapshot / golden review class alongside its assertion and helper / fixture churn — never applying a snapshot through an assertion-only click — with its generated snapshot assumption summarised and a diff-first preview and rollback preserved",
            "evidence:m5-editor-suggestion-inline:001",
            vec![suggestion_case(SuggestionSeed {
                trigger_source: Trigger::UncoveredBranch,
                target_refs: &[
                    "src/report/render.rs::render_invoice",
                    "tests/support/render_fixtures.rs",
                    "tests/snapshots/invoice.snap",
                ],
                trigger_context_ref: "uncovered:report::render-invoice-branch-else",
                assumption_classes: &[Assume::SnapshotGenerated, Assume::AssertionInferred],
                review_classes: &[
                    Class::AssertionChange,
                    Class::HelperOrFixtureAddition,
                    Class::SnapshotOrGoldenUpdate,
                ],
                apply_scope: Scope::SnapshotIncluded,
                generated_file_count: 3,
                provenance_class: Prov::VerifiedCurrentRun,
                offers_sandbox_run: true,
                offers_diff_preview: true,
                offers_rollback: true,
                suggestion_identity_ref: "suggestion-card:editor::uncovered-branch-snapshot",
            })],
        ),
        // 3. Test-tree suggestion — a full-bundle proposal for a regression-guard gap that mixes
        //    assertion, fixture, and snapshot churn and is therefore held to a review-first path
        //    rather than a one-click apply, with all three generated-test assumptions summarised.
        base_row(
            M5SuggestionCardConsumerSurface::TestTreeSuggestion,
            M5TestIntelligenceQualificationClass::Stable,
            "Test-tree suggestion owner",
            "The test-tree suggestion surface renders the shared test-generation suggestion card so a full-bundle proposal for a regression-guard gap that mixes assertion, helper / fixture, and snapshot / golden churn is held to a review-first path — never a one-click apply — so its assumption, fixture, and snapshot churn are separated and reviewed before anything is applied, with a sandbox run and a diff-first preview always offered",
            "evidence:m5-test-tree-suggestion:001",
            vec![suggestion_case(SuggestionSeed {
                trigger_source: Trigger::RegressionGuardGap,
                target_refs: &[
                    "src/pricing/engine.rs::price_order",
                    "tests/support/pricing_fixtures.rs",
                    "tests/snapshots/pricing.snap",
                ],
                trigger_context_ref: "regression:pricing::price-order-guard-gap",
                assumption_classes: &[
                    Assume::FixtureAssumed,
                    Assume::SnapshotGenerated,
                    Assume::UnverifiedBehavior,
                ],
                review_classes: &[
                    Class::AssertionChange,
                    Class::HelperOrFixtureAddition,
                    Class::SnapshotOrGoldenUpdate,
                ],
                apply_scope: Scope::FullBundleApply,
                generated_file_count: 5,
                provenance_class: Prov::ImportedCiArtifact,
                offers_sandbox_run: true,
                offers_diff_preview: true,
                offers_rollback: true,
                suggestion_identity_ref: "suggestion-card:test-tree::regression-full-bundle",
            })],
        ),
        // 4. Headless / CLI suggestion — a review-required proposal for a missing-assertion gap
        //    that mixes assertion and snapshot churn from a cached local result and is therefore
        //    held to review-first without a desktop surface, proving the same grammar works
        //    headless.
        base_row(
            M5SuggestionCardConsumerSurface::HeadlessCliSuggestion,
            M5TestIntelligenceQualificationClass::Stable,
            "Headless / CLI suggestion owner",
            "The headless / CLI suggestion surface renders the shared test-generation suggestion card so a review-required proposal for a missing-assertion gap that mixes assertion and snapshot / golden churn from a cached local result is held to a review-first path without a desktop surface, with its mock and dependency assumptions summarised and a diff-first preview and rollback preserved — proving the same grammar works headless",
            "evidence:m5-headless-cli-suggestion:001",
            vec![suggestion_case(SuggestionSeed {
                trigger_source: Trigger::MissingAssertionGap,
                target_refs: &[
                    "src/api/client.rs::fetch_orders",
                    "tests/snapshots/orders.snap",
                ],
                trigger_context_ref: "missing-assertion:api::fetch-orders-status",
                assumption_classes: &[Assume::MockSynthesized, Assume::DependencyAssumed],
                review_classes: &[Class::AssertionChange, Class::SnapshotOrGoldenUpdate],
                apply_scope: Scope::ReviewRequired,
                generated_file_count: 2,
                provenance_class: Prov::CachedLocalResult,
                offers_sandbox_run: false,
                offers_diff_preview: true,
                offers_rollback: true,
                suggestion_identity_ref: "suggestion-card:headless::missing-assertion-review",
            })],
        ),
        // 5. Suggestion export — an apply-blocked proposal for a manual request that carries no
        //    apply-capable action at all, read elsewhere with the same vocabulary a reviewer sees
        //    in the panel and the editor.
        base_row(
            M5SuggestionCardConsumerSurface::SuggestionExport,
            M5TestIntelligenceQualificationClass::Stable,
            "Suggestion export owner",
            "The suggestion export renders the shared test-generation suggestion card so an apply-blocked proposal for a manual request carries no apply-capable action at all — never presenting a settled apply — and reads with the same trigger, assumption, review-class, and apply-scope vocabulary a reviewer sees in the panel and the editor",
            "evidence:m5-suggestion-export:001",
            vec![suggestion_case(SuggestionSeed {
                trigger_source: Trigger::ManualRequest,
                target_refs: &["src/util/format.rs::format_currency"],
                trigger_context_ref: "manual:util::format-currency-request",
                assumption_classes: &[],
                review_classes: &[Class::AssertionChange],
                apply_scope: Scope::ApplyBlocked,
                generated_file_count: 0,
                provenance_class: Prov::Unknown,
                offers_sandbox_run: false,
                offers_diff_preview: false,
                offers_rollback: false,
                suggestion_identity_ref: "suggestion-card:export::manual-apply-blocked",
            })],
        ),
    ]
}

fn governance_review() -> M5SuggestionCardGovernanceReview {
    M5SuggestionCardGovernanceReview {
        card_shows_trigger_source_and_targets: true,
        card_shows_trigger_context: true,
        card_shows_assumption_summary: true,
        card_separates_review_classes: true,
        card_shows_generated_file_count: true,
        card_offers_sandbox_run_and_open_diff: true,
        apply_scope_never_understates_churn: true,
        generated_never_hides_assumption_fixture_or_snapshot_churn: true,
        ai_proposals_preserve_preview_rollback_evidence_parity: true,
        components_stable_across_deployment_lines: true,
        components_stable_across_consumer_surfaces: true,
        every_component_declares_accessibility_route: true,
        support_export_reconstructs_suggestion_truth: true,
        later_components_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5SuggestionCardConsumerProjection {
    M5SuggestionCardConsumerProjection {
        suggestion_surfaces_consume_shared_vocabulary: true,
        suggestion_posture_reads_single_source: true,
        apply_scope_reads_single_source: true,
        ci_and_support_read_same_suggestion_vocabulary: true,
        headless_and_desktop_read_single_source: true,
    }
}

fn proof_freshness() -> M5SuggestionCardProofFreshness {
    M5SuggestionCardProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5SuggestionCardReleasePosture {
    M5SuggestionCardReleasePosture {
        release_packet_ref: M5_SUGGESTION_CARD_COMPONENTS_ARTIFACT_REF.to_owned(),
        test_evidence_audit_ref: M5_SUGGESTION_CARD_COMPONENTS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SUGGESTION_CARD_COMPONENTS_SCHEMA_REF,
        M5_SUGGESTION_CARD_COMPONENTS_DOC_REF,
        M5_SUGGESTION_CARD_COMPONENTS_COMPONENT_MATRIX_REF,
        M5_SUGGESTION_CARD_COMPONENTS_TEST_GENERATION_REF,
    ])
}

/// Builds the canonical M5 suggestion-card-components packet.
pub fn seeded_m5_suggestion_card_components_packet() -> M5SuggestionCardComponentsPacket {
    M5SuggestionCardComponentsPacket::new(M5SuggestionCardComponentsPacketInput {
        packet_id: M5_SUGGESTION_CARD_COMPONENTS_PACKET_ID.to_owned(),
        matrix_label:
            "M5 test-generation-suggestion-card primitive: controlled trigger sources (uncovered line/branch, failing bug repro, regression-guard gap, missing-assertion gap, manual request), target symbol/file refs, uncovered-path/bug context, generated-test assumption summaries, distinct assertion/helper-fixture/snapshot-golden review classes, controlled apply scopes, distinct assertion-only/fixture-and-assertion/snapshot-included/full-bundle/review-required/apply-blocked suggestion postures, generated file counts, a required review-class separation before any apply-capable action, a required assumption summary for an apply-capable generated card, a required diff-first preview and rollback for every apply-capable proposal, and bounded reveal/run-in-sandbox/open-diff/apply-reviewed-classes/rollback/export actions"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5SuggestionCardVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the suggestion-review-panel consumer is narrowed to Preview pending
/// apply-capability-versus-review-first parity proof across every deployment line; every consumer
/// stays visible.
pub fn seeded_m5_suggestion_card_components_suggestion_review_panel_preview_narrowed(
) -> M5SuggestionCardComponentsPacket {
    let mut packet = seeded_m5_suggestion_card_components_packet();
    packet.packet_id =
        "m5-test-generation-suggestion-card-primitive:suggestion-review-panel-preview:0001"
            .to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5SuggestionCardConsumerSurface::SuggestionReviewPanel)
        .expect("suggestion-review-panel row present");
    row.qualification = M5TestIntelligenceQualificationClass::Preview;
    packet
}

/// Narrowed variant: the editor inline-suggestion consumer is held at Beta because a slice of
/// editor surfaces do not yet render the snapshot review-class cue on every profile; every consumer
/// stays visible.
pub fn seeded_m5_suggestion_card_components_editor_suggestion_inline_beta_narrowed(
) -> M5SuggestionCardComponentsPacket {
    let mut packet = seeded_m5_suggestion_card_components_packet();
    packet.packet_id =
        "m5-test-generation-suggestion-card-primitive:editor-suggestion-inline-beta:0001"
            .to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5SuggestionCardConsumerSurface::EditorSuggestionInline)
        .expect("editor-suggestion-inline row present");
    row.qualification = M5TestIntelligenceQualificationClass::Beta;
    packet
}
