//! Canonical seed builders for the M5 test-tree-row primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical tree-row primitive packet.
pub const M5_TEST_TREE_ROW_PACKET_ID: &str = "m5-test-tree-row-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked test-tree-row resolution case from a full item state.
#[allow(clippy::too_many_arguments)]
fn tree_case(
    item_class: M5TestTreeItemClass,
    identity_class: M5TestIdentityClass,
    result_origin: M5TestResultOrigin,
    result_freshness: M5TestResultFreshness,
    current_verdict: M5InlineMarkerVerdict,
    target_class: M5TestTargetClass,
    environment_lane: M5TestEnvironmentLane,
    quarantine_ownership: M5QuarantineOwnership,
    release_impact: M5TestReleaseImpact,
    parameterized_case_count: u32,
    item_muted: bool,
    item_label: &str,
    item_identity_ref: &str,
) -> M5TestTreeRowResolutionCase {
    M5TestTreeRowResolutionCase::resolved(M5TestTreeRowResolutionInput {
        item_class,
        identity_class,
        result_origin,
        result_freshness,
        current_verdict,
        target_class,
        environment_lane,
        quarantine_ownership,
        release_impact,
        parameterized_case_count,
        item_muted,
        item_label: item_label.to_owned(),
        item_identity_ref: item_identity_ref.to_owned(),
    })
}

/// A base row with the shared fields filled in and the full tree-row anatomy, item class,
/// identity class, result origin, posture, rerun scope, action, export-field, and
/// accessibility parity every consumer carries.
fn base_row(
    consumer_surface: M5TestTreeConsumerSurface,
    qualification: M5TestQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    tree_examples: Vec<M5TestTreeRowResolutionCase>,
) -> M5TestTreeConsumerRow {
    M5TestTreeConsumerRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5TestSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5TestDeploymentLine::ALL.to_vec(),
        anatomy_parts: M5TestTreeRowAnatomyPart::ALL.to_vec(),
        item_classes: M5TestTreeItemClass::ALL.to_vec(),
        identity_classes: M5TestIdentityClass::ALL.to_vec(),
        result_origins: M5TestResultOrigin::ALL.to_vec(),
        row_postures: M5TestTreeRowPosture::ALL.to_vec(),
        rerun_scopes: M5TestTreeRerunScope::ALL.to_vec(),
        row_actions: M5TestTreeRowAction::ALL.to_vec(),
        export_fields: M5TestTreeRowExportField::ALL.to_vec(),
        accessibility_routes: M5TestAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5TestConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5TestDowngradeTrigger::IdentityClassUnstated,
            M5TestDowngradeTrigger::ResultOriginUnstated,
            M5TestDowngradeTrigger::RerunScopeWidened,
            M5TestDowngradeTrigger::QuarantineReleaseImpactHidden,
            M5TestDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_TEST_TREE_ROW_SCHEMA_REF,
            M5_TEST_TREE_ROW_TEST_ITEM_IDENTITY_REF,
            M5_TEST_TREE_ROW_QUARANTINE_RECORD_REF,
        ]),
        tree_examples,
        masks_identity_or_origin: false,
        hides_quarantine_release_impact: false,
        overstates_imported_certainty: false,
        widens_rerun_scope_silently: false,
    }
}

fn rows() -> Vec<M5TestTreeConsumerRow> {
    use M5InlineMarkerVerdict as Verdict;
    use M5QuarantineOwnership as Owner;
    use M5TestEnvironmentLane as Env;
    use M5TestIdentityClass as Identity;
    use M5TestReleaseImpact as Impact;
    use M5TestResultFreshness as Fresh;
    use M5TestResultOrigin as Origin;
    use M5TestTargetClass as Target;
    use M5TestTreeItemClass as Item;

    vec![
        // 1. Test-explorer tree — a durable-keyed unit suite that fans out on rerun, and a
        //    durable-keyed concrete case with a fresh live-local pass: the highest-certainty
        //    live row.
        base_row(
            M5TestTreeConsumerSurface::TestExplorerTree,
            M5TestQualificationClass::Stable,
            "Test explorer tree owner",
            "The test-explorer tree renders the shared test-tree row so a durable-keyed unit suite names its class, identity, and live-local origin with a whole-suite rerun scope, and a durable-keyed concrete case with a fresh live-local pass reads as the highest-certainty live-concrete row exposing single-case rerun and debug",
            "evidence:m5-tree-row-test-explorer:001",
            vec![
                tree_case(
                    Item::Suite,
                    Identity::DurableKeyed,
                    Origin::LiveLocal,
                    Fresh::Fresh,
                    Verdict::Passed,
                    Target::UnitTest,
                    Env::LocalHost,
                    Owner::Unowned,
                    Impact::NoImpact,
                    0,
                    false,
                    "Auth unit suite",
                    "tree:auth-unit-suite",
                ),
                tree_case(
                    Item::ConcreteCase,
                    Identity::DurableKeyed,
                    Origin::LiveLocal,
                    Fresh::Fresh,
                    Verdict::Passed,
                    Target::UnitTest,
                    Env::LocalHost,
                    Owner::Unowned,
                    Impact::NoImpact,
                    1,
                    false,
                    "token refresh returns a fresh token",
                    "tree:auth-unit-suite::token-refresh",
                ),
            ],
        ),
        // 2. Editor-gutter tree — a parameterized template that fans out over 12 variants,
        //    and a notebook-backed item with a fresh live-local pass; both live-certainty
        //    aggregates keep their exact rerun scope.
        base_row(
            M5TestTreeConsumerSurface::EditorGutterTree,
            M5TestQualificationClass::Stable,
            "Editor gutter tree owner",
            "The editor-gutter tree renders the shared test-tree row so a parameterized template names its 12-variant parameterized-group rerun scope without collapsing the count, and a notebook-backed item names its notebook-cells rerun scope and stays debuggable — neither widening its rerun scope",
            "evidence:m5-tree-row-editor-gutter:001",
            vec![
                tree_case(
                    Item::Template,
                    Identity::ParametrizedCase,
                    Origin::LiveLocal,
                    Fresh::Fresh,
                    Verdict::Passed,
                    Target::IntegrationTest,
                    Env::Container,
                    Owner::Unowned,
                    Impact::NoImpact,
                    12,
                    false,
                    "matrix parse cases",
                    "tree:integration::matrix-parse",
                ),
                tree_case(
                    Item::NotebookBackedItem,
                    Identity::PathDerived,
                    Origin::LiveLocal,
                    Fresh::Fresh,
                    Verdict::Passed,
                    Target::IntegrationTest,
                    Env::LocalHost,
                    Owner::Unowned,
                    Impact::NoImpact,
                    1,
                    false,
                    "notebook: data-load smoke",
                    "tree:notebook::data-load-smoke",
                ),
            ],
        ),
        // 3. Run-panel tree — an imported CI result that is replay-only (reduced certainty,
        //    no local rerun), and a stale live-local failure that still reruns and debugs;
        //    proves imported evidence never reads as live.
        base_row(
            M5TestTreeConsumerSurface::RunPanelTree,
            M5TestQualificationClass::Stable,
            "Run panel tree owner",
            "The run-panel tree renders the shared test-tree row so an imported CI result reads as an imported-evidence row that is replay-only and withholds the local rerun/debug it cannot honestly offer, and a stale live-local failure reads as a stale-result row that still exposes single-case rerun and debug — so imported evidence never inherits live certainty",
            "evidence:m5-tree-row-run-panel:001",
            vec![
                tree_case(
                    Item::ImportedResult,
                    Identity::ImportedRecord,
                    Origin::ImportedCi,
                    Fresh::Fresh,
                    Verdict::Failed,
                    Target::EndToEndTest,
                    Env::CiMatrix,
                    Owner::Unowned,
                    Impact::NoImpact,
                    1,
                    false,
                    "checkout flow e2e (from CI)",
                    "tree:e2e::checkout-flow@ci",
                ),
                tree_case(
                    Item::ConcreteCase,
                    Identity::DurableKeyed,
                    Origin::LiveLocal,
                    Fresh::Stale,
                    Verdict::Failed,
                    Target::UnitTest,
                    Env::LocalHost,
                    Owner::Unowned,
                    Impact::NoImpact,
                    1,
                    false,
                    "price rounds half to even",
                    "tree:pricing::round-half-even",
                ),
            ],
        ),
        // 4. Headless / CLI tree — a partial-discovery placeholder that names nothing
        //    concrete to rerun yet, and a durable-keyed flaky-suspected concrete case with a
        //    fresh live-local run; proves the same grammar works without a desktop UI.
        base_row(
            M5TestTreeConsumerSurface::HeadlessCliTree,
            M5TestQualificationClass::Stable,
            "Headless CLI tree owner",
            "The headless / CLI tree renders the shared test-tree row so a partial-discovery placeholder reads as a partial-discovery row with a nothing-concrete-yet rerun scope and no faked rerun, and a durable-keyed concrete case flagged flaky-suspected on a fresh live-local run reads as a live-concrete row exposing single-case rerun and debug — proving the same tree grammar works headless",
            "evidence:m5-tree-row-headless-cli:001",
            vec![
                tree_case(
                    Item::PartialDiscoveryPlaceholder,
                    Identity::DiscoveredOnly,
                    Origin::UnknownOrigin,
                    Fresh::NeverRun,
                    Verdict::NotRun,
                    Target::UnitTest,
                    Env::LocalHost,
                    Owner::Unowned,
                    Impact::NoImpact,
                    0,
                    false,
                    "partial: undiscovered spec module",
                    "tree:discovery::pending-spec-module",
                ),
                tree_case(
                    Item::ConcreteCase,
                    Identity::DurableKeyed,
                    Origin::LiveLocal,
                    Fresh::Fresh,
                    Verdict::FlakySuspected,
                    Target::ContractTest,
                    Env::RemoteRunner,
                    Owner::Unowned,
                    Impact::NoImpact,
                    1,
                    false,
                    "contract: schema stays backward compatible",
                    "tree:contract::schema-back-compat",
                ),
            ],
        ),
        // 5. Test-report export — a muted / quarantined concrete case whose team-owned
        //    quarantine hides it from release gating (its release impact heads the row), and
        //    a durable-keyed benchmark suite that fans out; proves quarantine coverage.
        base_row(
            M5TestTreeConsumerSurface::TestReportExport,
            M5TestQualificationClass::Stable,
            "Test report export owner",
            "The test-report export renders the shared test-tree row so a team-owned quarantined concrete case reads as a quarantined row whose hidden-from-release impact heads it while still exposing rerun, debug, and review-quarantine, and a durable-keyed benchmark suite names its whole-suite rerun scope — the same row a reviewer reads elsewhere",
            "evidence:m5-tree-row-test-report-export:001",
            vec![
                tree_case(
                    Item::ConcreteCase,
                    Identity::DurableKeyed,
                    Origin::LiveLocal,
                    Fresh::Fresh,
                    Verdict::Failed,
                    Target::UnitTest,
                    Env::LocalHost,
                    Owner::TeamOwned,
                    Impact::HiddenFromRelease,
                    1,
                    true,
                    "flaky login redirect",
                    "tree:auth::login-redirect-quarantined",
                ),
                tree_case(
                    Item::Suite,
                    Identity::DurableKeyed,
                    Origin::LiveLocal,
                    Fresh::Fresh,
                    Verdict::Passed,
                    Target::BenchmarkTest,
                    Env::BrowserMatrix,
                    Owner::Unowned,
                    Impact::NoImpact,
                    0,
                    false,
                    "render benchmark suite",
                    "tree:bench::render-suite",
                ),
            ],
        ),
    ]
}

fn governance_review() -> M5TestTreeRowGovernanceReview {
    M5TestTreeRowGovernanceReview {
        tree_row_shows_item_class_and_identity: true,
        tree_row_shows_state_and_freshness: true,
        tree_row_shows_result_origin: true,
        tree_row_shows_target_and_environment: true,
        tree_row_shows_parameterized_count: true,
        tree_row_shows_mute_and_release_impact: true,
        rerun_scope_explicit_and_never_widened: true,
        imported_or_partial_never_reads_as_live: true,
        tree_rows_stable_across_deployment_lines: true,
        tree_rows_stable_across_consumer_surfaces: true,
        every_row_declares_accessibility_route: true,
        support_export_reconstructs_tree_truth: true,
        later_rows_cannot_invent_parallel_tree_vocabulary: true,
    }
}

fn consumer_projection() -> M5TestTreeRowConsumerProjection {
    M5TestTreeRowConsumerProjection {
        test_and_editor_surfaces_consume_tree_vocabulary: true,
        row_posture_reads_single_source: true,
        rerun_scope_reads_single_source: true,
        support_export_reads_single_source: true,
        headless_and_desktop_read_single_source: true,
    }
}

fn proof_freshness() -> M5TestTreeRowProofFreshness {
    M5TestTreeRowProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5TestTreeRowReleasePosture {
    M5TestTreeRowReleasePosture {
        release_packet_ref: M5_TEST_TREE_ROW_ARTIFACT_REF.to_owned(),
        test_evidence_audit_ref: M5_TEST_TREE_ROW_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_TEST_TREE_ROW_SCHEMA_REF,
        M5_TEST_TREE_ROW_DOC_REF,
        M5_TEST_TREE_ROW_COMPONENT_MATRIX_REF,
        M5_TEST_TREE_ROW_TEST_ITEM_IDENTITY_REF,
        M5_TEST_TREE_ROW_QUARANTINE_RECORD_REF,
    ])
}

/// Builds the canonical M5 test-tree-row packet.
pub fn seeded_m5_test_tree_row_packet() -> M5TestTreeRowPacket {
    M5TestTreeRowPacket::new(M5TestTreeRowPacketInput {
        packet_id: M5_TEST_TREE_ROW_PACKET_ID.to_owned(),
        matrix_label:
            "M5 test-tree-row primitive: suite/template/concrete-case/notebook-backed/imported-result/partial-discovery item classes, stable identity, current state, last-result freshness, imported/live origin, target/environment shorthand, parameterized-case count, mute/quarantine and release impact, derived row posture, exact rerun scope, and bounded reveal/rerun/debug/review-quarantine/export actions"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5TestTreeRowVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the run-panel tree consumer is narrowed to Preview pending
/// imported-versus-live certainty parity proof across every deployment; every consumer
/// stays visible.
pub fn seeded_m5_test_tree_row_run_panel_tree_preview_narrowed() -> M5TestTreeRowPacket {
    let mut packet = seeded_m5_test_tree_row_packet();
    packet.packet_id = "m5-test-tree-row-primitive:run-panel-tree-preview:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5TestTreeConsumerSurface::RunPanelTree)
        .expect("run-panel-tree row present");
    row.qualification = M5TestQualificationClass::Preview;
    packet
}

/// Narrowed variant: the headless / CLI tree consumer is held at Beta because a slice of
/// headless rows do not yet render the keyboard route cue on every profile; every consumer
/// stays visible.
pub fn seeded_m5_test_tree_row_headless_cli_tree_beta_narrowed() -> M5TestTreeRowPacket {
    let mut packet = seeded_m5_test_tree_row_packet();
    packet.packet_id = "m5-test-tree-row-primitive:headless-cli-tree-beta:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5TestTreeConsumerSurface::HeadlessCliTree)
        .expect("headless-cli-tree row present");
    row.qualification = M5TestQualificationClass::Beta;
    packet
}
