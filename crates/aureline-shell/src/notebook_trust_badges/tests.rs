//! Unit and fixture tests for the bounded notebook-trust-badge and
//! representation-state wedge.
//!
//! These tests cover:
//!
//! - the protected walk on a fully trusted local notebook (no honesty
//!   markers, no invariant violations, no row autoexecutes on open);
//! - the protected walk on a mixed-trust imported notebook (axes remain
//!   visibly distinct; escape hatches are present; the code cell renders
//!   escaped and the widget output renders as a tombstone fallback);
//! - the named failure drill (a buggy caller flips
//!   `will_autoexecute_on_open` on a code cell — the typed
//!   `AutoexecuteOnOpen` invariant fires and the card's chip lights);
//! - adjacent drills covering active content on untrusted rungs, missing
//!   escape hatches, widget rows with `widget_trust_state =
//!   not_applicable`, live outputs without a kernel, and trust-axes
//!   collapse.
//! - serde round-trip and deterministic plaintext rendering.

use std::path::Path;

use serde::Deserialize;

use super::*;

fn fully_trusted_wedge() -> NotebookTrustBadgeWedge {
    NotebookTrustBadgeWedge::new("ws-nb-trust", "nb.doc.ml.experiment.01")
        .with_workspace_trust(WorkspaceTrustState::TrustedWorkspace)
        .with_notebook_trust_rung(NotebookTrustRung::FullyTrustedUser)
        .with_kernel_availability(KernelAvailability::LocalManagedAvailable)
        .with_output_trust(OutputTrustState::LiveFromCurrentSession)
        .with_widget_trust(WidgetTrustState::NotApplicable)
}

fn untrusted_wedge() -> NotebookTrustBadgeWedge {
    NotebookTrustBadgeWedge::new("ws-nb-trust", "nb.doc.imported.public_sample.01")
        .with_workspace_trust(WorkspaceTrustState::RestrictedWorkspace)
        .with_notebook_trust_rung(NotebookTrustRung::UntrustedTainted)
        .with_kernel_availability(KernelAvailability::LocalManagedUnavailable)
        .with_output_trust(OutputTrustState::CapturedFromPriorSession)
        .with_widget_trust(WidgetTrustState::WidgetDeniedByDefault)
}

#[test]
fn protected_walk_fully_trusted_local_renders_clean_card() {
    let mut wedge = fully_trusted_wedge();
    wedge.add_row(NotebookTrustBadgeRowBuilder::new(
        "row.markdown.intro",
        "nb.cell.intro",
        CellContentClass::MarkdownCell,
        RepresentationState::Sanitized,
    ));
    wedge.add_row(
        NotebookTrustBadgeRowBuilder::new(
            "row.code.imports",
            "nb.cell.imports",
            CellContentClass::CodeCell,
            RepresentationState::Sanitized,
        )
        .with_escape_hatches([EscapeHatch::SafePreview, EscapeHatch::ExportRawSource]),
    );
    wedge.add_row(NotebookTrustBadgeRowBuilder::new(
        "row.output.imports.stdout",
        "nb.output.imports.stdout",
        CellContentClass::RichOutput,
        RepresentationState::Sanitized,
    ));

    let card = wedge.card();
    assert_eq!(card.record_kind, NOTEBOOK_TRUST_BADGE_CARD_RECORD_KIND);
    assert_eq!(
        card.schema_version,
        NOTEBOOK_TRUST_BADGE_CARD_SCHEMA_VERSION
    );
    assert_eq!(
        card.prototype_label_token,
        "m1_prototype_notebook_trust_badges_and_representation_state"
    );
    assert_eq!(card.workspace_trust_state_token, "trusted_workspace");
    assert_eq!(card.notebook_trust_rung_token, "fully_trusted_user");
    assert_eq!(card.kernel_availability_token, "local_managed_available");
    assert_eq!(card.output_trust_state_token, "live_from_current_session");
    assert_eq!(card.widget_trust_state_token, "not_applicable");
    assert_eq!(card.rows.len(), 3);
    assert!(card.invariants.is_empty());
    assert!(!card.has_invariant_violations);
    assert!(!card.any_row_claims_autoexecute_on_open);
    assert!(card.is_clean_trusted_local());

    // Canonical claim-limit set in stable order.
    let tokens: Vec<&str> = card
        .claim_limits
        .iter()
        .map(|row| row.token.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "single_bounded_wedge_only",
            "no_autoexecute_on_open",
            "trust_axes_remain_distinct",
            "no_kernel_or_transport_orchestration",
            "no_widget_admission_pipeline",
        ]
    );

    for row in &card.rows {
        assert!(!row.will_autoexecute_on_open);
    }
}

#[test]
fn protected_walk_mixed_trust_untrusted_notebook_keeps_axes_distinct() {
    let mut wedge = untrusted_wedge();
    wedge.add_row(NotebookTrustBadgeRowBuilder::new(
        "row.markdown.intro",
        "nb.cell.intro",
        CellContentClass::MarkdownCell,
        RepresentationState::Sanitized,
    ));
    wedge.add_row(
        NotebookTrustBadgeRowBuilder::new(
            "row.code.untrusted",
            "nb.cell.script",
            CellContentClass::CodeCell,
            RepresentationState::Escaped,
        )
        .with_honesty_marker(true)
        .with_degraded(DegradedStateToken::Limited)
        .with_escape_hatches([EscapeHatch::SafePreview, EscapeHatch::ExportRawSource]),
    );
    wedge.add_row(
        NotebookTrustBadgeRowBuilder::new(
            "row.widget.scatter",
            "nb.output.widget.scatter.01",
            CellContentClass::WidgetOutput,
            RepresentationState::TombstoneStaticFallback,
        )
        .with_honesty_marker(true)
        .with_degraded(DegradedStateToken::PolicyBlocked)
        .with_escape_hatches([EscapeHatch::SafePreview, EscapeHatch::KeepStaticFallback]),
    );

    let card = wedge.card();
    assert!(
        !card.has_invariant_violations,
        "card should be clean: {:?}",
        card.invariants
    );
    assert!(!card.any_row_claims_autoexecute_on_open);

    // Axes are visibly distinct: notebook untrusted, workspace
    // restricted, kernel unavailable, outputs captured (not live),
    // widget denied. None of these collapse onto one chip.
    assert_eq!(card.notebook_trust_rung_token, "untrusted_tainted");
    assert_eq!(card.workspace_trust_state_token, "restricted_workspace");
    assert_eq!(card.kernel_availability_token, "local_managed_unavailable");
    assert_eq!(card.output_trust_state_token, "captured_from_prior_session");
    assert_eq!(card.widget_trust_state_token, "widget_denied_by_default");

    // The mixed-trust code cell renders Escaped, the widget output
    // renders as a tombstone fallback, and both rows surface an honesty
    // marker with a degraded chip.
    let code_row = card
        .rows
        .iter()
        .find(|r| r.row_id == "row.code.untrusted")
        .expect("code row");
    assert_eq!(code_row.representation_state_token, "escaped");
    assert!(code_row.honesty_marker_present);
    assert_eq!(code_row.degraded_token.as_deref(), Some("Limited"));
    assert!(code_row.escape_hatches.contains(&"safe_preview".to_owned()));

    let widget_row = card
        .rows
        .iter()
        .find(|r| r.row_id == "row.widget.scatter")
        .expect("widget row");
    assert_eq!(
        widget_row.representation_state_token,
        "tombstone_static_fallback"
    );
    assert!(widget_row.honesty_marker_present);
    assert_eq!(widget_row.degraded_token.as_deref(), Some("PolicyBlocked"));
    assert!(widget_row
        .escape_hatches
        .contains(&"keep_static_fallback".to_owned()));
}

#[test]
fn failure_drill_autoexecute_on_open_is_rejected_with_typed_invariant() {
    // Named failure drill: a buggy caller flips will_autoexecute_on_open.
    // The wedge MUST surface the typed AutoexecuteOnOpen invariant and
    // light any_row_claims_autoexecute_on_open so the chrome can refuse
    // before any active content runs.
    let mut wedge = fully_trusted_wedge();
    wedge.add_row(
        NotebookTrustBadgeRowBuilder::new(
            "row.code.autoexec.buggy",
            "nb.cell.script.with_side_effects",
            CellContentClass::CodeCell,
            RepresentationState::Sanitized,
        )
        .with_escape_hatches([EscapeHatch::SafePreview, EscapeHatch::ExportRawSource])
        .with_will_autoexecute_on_open(true),
    );

    let card = wedge.card();
    assert!(card.has_invariant_violations);
    assert!(card.any_row_claims_autoexecute_on_open);

    let autoexec_row = card
        .invariants
        .iter()
        .find(|row| row.violation_token == "autoexecute_on_open")
        .expect("typed AutoexecuteOnOpen invariant must surface");
    assert_eq!(
        autoexec_row.addressable_row_id.as_deref(),
        Some("row.code.autoexec.buggy"),
    );
    assert!(autoexec_row
        .violation_label
        .contains("will autoexecute active content on notebook open"));
}

#[test]
fn active_content_on_untrusted_rung_surfaces_typed_invariant() {
    // A code cell rendered as sandboxed_active under untrusted_tainted is
    // a representation-state honesty failure: the chrome would expose
    // live content under an untrusted rung.
    let mut wedge = untrusted_wedge();
    wedge.add_row(
        NotebookTrustBadgeRowBuilder::new(
            "row.code.sandbox.buggy",
            "nb.cell.script",
            CellContentClass::CodeCell,
            RepresentationState::SandboxedActive,
        )
        .with_escape_hatches([EscapeHatch::SafePreview]),
    );
    let card = wedge.card();
    assert!(card.has_invariant_violations);
    let row = card
        .invariants
        .iter()
        .find(|r| r.violation_token == "active_content_on_untrusted_rung")
        .expect("active_content_on_untrusted_rung invariant");
    assert_eq!(
        row.addressable_row_id.as_deref(),
        Some("row.code.sandbox.buggy"),
    );
}

#[test]
fn missing_safe_preview_escape_hatch_on_active_content_is_rejected() {
    let mut wedge = fully_trusted_wedge();
    wedge.add_row(NotebookTrustBadgeRowBuilder::new(
        "row.code.naked",
        "nb.cell.script",
        CellContentClass::CodeCell,
        RepresentationState::Sanitized,
    ));
    let card = wedge.card();
    assert!(card.has_invariant_violations);
    assert!(card
        .invariants
        .iter()
        .any(|r| r.violation_token == "missing_safe_preview_escape_hatch"));
}

#[test]
fn widget_row_without_explicit_widget_trust_is_rejected() {
    let mut wedge = fully_trusted_wedge()
        .with_output_trust(OutputTrustState::WidgetGated)
        .with_widget_trust(WidgetTrustState::NotApplicable);
    wedge.add_row(
        NotebookTrustBadgeRowBuilder::new(
            "row.widget.unlabelled",
            "nb.output.widget.demo",
            CellContentClass::WidgetOutput,
            RepresentationState::TombstoneStaticFallback,
        )
        .with_escape_hatches([EscapeHatch::SafePreview]),
    );
    let card = wedge.card();
    assert!(card.has_invariant_violations);
    assert!(card
        .invariants
        .iter()
        .any(|r| r.violation_token == "widget_trust_not_applicable_for_widget"));
}

#[test]
fn live_outputs_without_kernel_is_rejected() {
    let mut wedge = NotebookTrustBadgeWedge::new("ws-nb-trust", "nb.doc.live_no_kernel")
        .with_workspace_trust(WorkspaceTrustState::TrustedWorkspace)
        .with_notebook_trust_rung(NotebookTrustRung::FullyTrustedUser)
        .with_kernel_availability(KernelAvailability::LocalManagedUnavailable)
        .with_output_trust(OutputTrustState::LiveFromCurrentSession)
        .with_widget_trust(WidgetTrustState::NotApplicable);
    wedge.add_row(NotebookTrustBadgeRowBuilder::new(
        "row.markdown.intro",
        "nb.cell.intro",
        CellContentClass::MarkdownCell,
        RepresentationState::Sanitized,
    ));
    let card = wedge.card();
    assert!(card.has_invariant_violations);
    assert!(card
        .invariants
        .iter()
        .any(|r| r.violation_token == "live_outputs_without_kernel"));
}

#[test]
fn trust_axes_collapse_when_code_cell_runs_with_not_applicable_kernel() {
    // A code cell row exists but the wedge reports kernel availability as
    // not_applicable — the kernel axis has been collapsed onto the
    // notebook rung.
    let mut wedge = NotebookTrustBadgeWedge::new("ws-nb-trust", "nb.doc.collapsed")
        .with_workspace_trust(WorkspaceTrustState::TrustedWorkspace)
        .with_notebook_trust_rung(NotebookTrustRung::FullyTrustedUser)
        .with_kernel_availability(KernelAvailability::NotApplicable)
        .with_output_trust(OutputTrustState::NotApplicable)
        .with_widget_trust(WidgetTrustState::NotApplicable);
    wedge.add_row(
        NotebookTrustBadgeRowBuilder::new(
            "row.code.collapse",
            "nb.cell.import",
            CellContentClass::CodeCell,
            RepresentationState::Sanitized,
        )
        .with_escape_hatches([EscapeHatch::SafePreview]),
    );
    let card = wedge.card();
    assert!(card.has_invariant_violations);
    let row = card
        .invariants
        .iter()
        .find(|r| r.violation_token == "trust_axes_collapsed")
        .expect("trust_axes_collapsed invariant must surface");
    // Label MUST quote the collapsed axis name verbatim.
    assert!(row.violation_label.contains("kernel_availability"));
}

#[test]
fn render_plaintext_quotes_every_axis_and_row() {
    let mut wedge = untrusted_wedge();
    wedge.add_row(NotebookTrustBadgeRowBuilder::new(
        "row.markdown.intro",
        "nb.cell.intro",
        CellContentClass::MarkdownCell,
        RepresentationState::Sanitized,
    ));
    wedge.add_row(
        NotebookTrustBadgeRowBuilder::new(
            "row.code.untrusted",
            "nb.cell.script",
            CellContentClass::CodeCell,
            RepresentationState::Escaped,
        )
        .with_honesty_marker(true)
        .with_degraded(DegradedStateToken::Limited)
        .with_escape_hatches([EscapeHatch::SafePreview, EscapeHatch::ExportRawSource]),
    );
    let card = wedge.card();
    let text = card.render_plaintext();
    assert!(text.starts_with("[m1_prototype_notebook_trust_badges_and_representation_state]"));
    assert!(text.contains("workspace=restricted_workspace"));
    assert!(text.contains("notebook=untrusted_tainted"));
    assert!(text.contains("kernel=local_managed_unavailable"));
    assert!(text.contains("output=captured_from_prior_session"));
    assert!(text.contains("widget=widget_denied_by_default"));
    assert!(text.contains("representation=escaped"));
    assert!(text.contains("escape_hatches=[safe_preview,export_raw_source]"));
    assert!(text.contains("single_bounded_wedge_only"));
    assert!(text.contains("no_autoexecute_on_open"));
    assert!(text.contains("trust_axes_remain_distinct"));
    assert!(text.contains("no_kernel_or_transport_orchestration"));
    assert!(text.contains("no_widget_admission_pipeline"));
    assert!(text.contains("invariants:\n  - clean"));
}

#[test]
fn record_round_trips_through_serde_json() {
    let mut wedge = fully_trusted_wedge();
    wedge.add_row(
        NotebookTrustBadgeRowBuilder::new(
            "row.code.imports",
            "nb.cell.imports",
            CellContentClass::CodeCell,
            RepresentationState::Sanitized,
        )
        .with_escape_hatches([EscapeHatch::SafePreview]),
    );
    let card = wedge.card();
    let json = serde_json::to_string(&card).expect("serialise");
    let parsed: NotebookTrustBadgeCardRecord =
        serde_json::from_str(&json).expect("round trip parses");
    assert_eq!(parsed, card);
}

#[test]
fn fixture_protected_walk_fully_trusted_local_replays_into_the_wedge() {
    let fixture: WedgeFixture = load_fixture("protected_walk_fully_trusted_local.json");
    let card = build_card_from_fixture(&fixture);
    assert_fixture_matches(&card, &fixture);
    assert!(card.is_clean_trusted_local());
}

#[test]
fn fixture_protected_walk_mixed_trust_untrusted_replays_into_the_wedge() {
    let fixture: WedgeFixture = load_fixture("protected_walk_mixed_trust_untrusted.json");
    let card = build_card_from_fixture(&fixture);
    assert_fixture_matches(&card, &fixture);
    // Trust axes remain visibly distinct on this fixture.
    assert_eq!(card.workspace_trust_state_token, "restricted_workspace");
    assert_eq!(card.notebook_trust_rung_token, "untrusted_tainted");
    assert_eq!(card.kernel_availability_token, "local_managed_unavailable");
    assert_eq!(card.output_trust_state_token, "captured_from_prior_session");
    assert_eq!(card.widget_trust_state_token, "widget_denied_by_default");
}

#[test]
fn fixture_failure_drill_autoexecute_on_open_surfaces_typed_invariant() {
    let fixture: WedgeFixture = load_fixture("failure_drill_autoexecute_on_open.json");
    let card = build_card_from_fixture(&fixture);
    assert_fixture_matches(&card, &fixture);
    assert!(card.has_invariant_violations);
    assert!(card.any_row_claims_autoexecute_on_open);
    let expected = fixture
        .expect
        .expected_violation_tokens
        .as_ref()
        .expect("failure drill must list expected violation tokens");
    for token in expected {
        assert!(
            card.invariants
                .iter()
                .any(|row| &row.violation_token == token),
            "expected invariant {token} to fire on failure drill card"
        );
    }
}

// ---------------------------------------------------------------------------
// Fixture helpers
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct WedgeFixture {
    workspace_id: String,
    notebook_ref: String,
    #[serde(default)]
    wedge_id: Option<String>,
    trust_axes: WedgeFixtureTrustAxes,
    rows: Vec<WedgeFixtureRow>,
    expect: WedgeFixtureExpect,
}

#[derive(Debug, Deserialize)]
struct WedgeFixtureTrustAxes {
    workspace_trust_state: String,
    notebook_trust_rung: String,
    kernel_availability: String,
    output_trust_state: String,
    widget_trust_state: String,
}

#[derive(Debug, Deserialize)]
struct WedgeFixtureRow {
    row_id: String,
    cell_or_output_ref: String,
    content_class: String,
    representation_state: String,
    honesty_marker_present: bool,
    #[serde(default)]
    degraded_token: Option<String>,
    #[serde(default)]
    escape_hatches: Vec<String>,
    #[serde(default)]
    widget_trust_override: Option<String>,
    will_autoexecute_on_open: bool,
}

#[derive(Debug, Deserialize)]
struct WedgeFixtureExpect {
    has_invariant_violations: bool,
    any_row_claims_autoexecute_on_open: bool,
    #[serde(default)]
    summary_contains: Option<String>,
    #[serde(default)]
    expected_violation_tokens: Option<Vec<String>>,
    rows: Vec<WedgeFixtureRowExpect>,
}

#[derive(Debug, Deserialize)]
struct WedgeFixtureRowExpect {
    row_id: String,
    representation_state: String,
    honesty_marker_present: bool,
    will_autoexecute_on_open: bool,
}

fn load_fixture(name: &str) -> WedgeFixture {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/notebooks/m1_trust_badge_cases")
        .join(name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {} must parse: {err}", path.display()))
}

fn build_card_from_fixture(fixture: &WedgeFixture) -> NotebookTrustBadgeCardRecord {
    let mut wedge = NotebookTrustBadgeWedge::new(&fixture.workspace_id, &fixture.notebook_ref)
        .with_workspace_trust(parse_workspace_trust(
            &fixture.trust_axes.workspace_trust_state,
        ))
        .with_notebook_trust_rung(parse_notebook_rung(&fixture.trust_axes.notebook_trust_rung))
        .with_kernel_availability(parse_kernel(&fixture.trust_axes.kernel_availability))
        .with_output_trust(parse_output_trust(&fixture.trust_axes.output_trust_state))
        .with_widget_trust(parse_widget_trust(&fixture.trust_axes.widget_trust_state));
    if let Some(id) = &fixture.wedge_id {
        wedge = wedge.with_wedge_id(id);
    }
    for row in &fixture.rows {
        let mut builder = NotebookTrustBadgeRowBuilder::new(
            &row.row_id,
            &row.cell_or_output_ref,
            parse_content_class(&row.content_class),
            parse_representation(&row.representation_state),
        )
        .with_honesty_marker(row.honesty_marker_present)
        .with_escape_hatches(row.escape_hatches.iter().map(|h| parse_escape_hatch(h)))
        .with_will_autoexecute_on_open(row.will_autoexecute_on_open);
        if let Some(token) = &row.degraded_token {
            builder = builder.with_degraded(parse_degraded(token));
        }
        if let Some(override_token) = &row.widget_trust_override {
            builder = builder.with_widget_trust_override(parse_widget_trust(override_token));
        }
        wedge.add_row(builder);
    }
    wedge.card()
}

fn assert_fixture_matches(card: &NotebookTrustBadgeCardRecord, fixture: &WedgeFixture) {
    assert_eq!(
        card.has_invariant_violations, fixture.expect.has_invariant_violations,
        "has_invariant_violations mismatch"
    );
    assert_eq!(
        card.any_row_claims_autoexecute_on_open, fixture.expect.any_row_claims_autoexecute_on_open,
        "any_row_claims_autoexecute_on_open mismatch"
    );
    if let Some(needle) = &fixture.expect.summary_contains {
        assert!(
            card.summary_line.contains(needle),
            "summary_line {:?} should contain {:?}",
            card.summary_line,
            needle,
        );
    }
    assert_eq!(card.rows.len(), fixture.expect.rows.len(), "row count");
    for (row, expect) in card.rows.iter().zip(fixture.expect.rows.iter()) {
        assert_eq!(row.row_id, expect.row_id);
        assert_eq!(row.representation_state_token, expect.representation_state);
        assert_eq!(row.honesty_marker_present, expect.honesty_marker_present);
        assert_eq!(
            row.will_autoexecute_on_open,
            expect.will_autoexecute_on_open
        );
    }
}

fn parse_workspace_trust(token: &str) -> WorkspaceTrustState {
    match token {
        "trusted_workspace" => WorkspaceTrustState::TrustedWorkspace,
        "restricted_workspace" => WorkspaceTrustState::RestrictedWorkspace,
        "unknown_workspace" => WorkspaceTrustState::UnknownWorkspace,
        other => panic!("unknown workspace_trust_state {other}"),
    }
}

fn parse_notebook_rung(token: &str) -> NotebookTrustRung {
    match token {
        "untrusted_tainted" => NotebookTrustRung::UntrustedTainted,
        "untrusted_quarantined_for_review" => NotebookTrustRung::UntrustedQuarantinedForReview,
        "structural_only_trusted" => NotebookTrustRung::StructuralOnlyTrusted,
        "selective_cell_trust" => NotebookTrustRung::SelectiveCellTrust,
        "fully_trusted_user" => NotebookTrustRung::FullyTrustedUser,
        "fully_trusted_workspace_policy" => NotebookTrustRung::FullyTrustedWorkspacePolicy,
        "trust_revoked_pending_review" => NotebookTrustRung::TrustRevokedPendingReview,
        other => panic!("unknown notebook_trust_rung {other}"),
    }
}

fn parse_kernel(token: &str) -> KernelAvailability {
    match token {
        "not_applicable" => KernelAvailability::NotApplicable,
        "local_managed_available" => KernelAvailability::LocalManagedAvailable,
        "local_managed_unavailable" => KernelAvailability::LocalManagedUnavailable,
        "remote_managed_available" => KernelAvailability::RemoteManagedAvailable,
        "remote_managed_unavailable" => KernelAvailability::RemoteManagedUnavailable,
        "kernel_denied_by_policy" => KernelAvailability::KernelDeniedByPolicy,
        other => panic!("unknown kernel_availability {other}"),
    }
}

fn parse_output_trust(token: &str) -> OutputTrustState {
    match token {
        "not_applicable" => OutputTrustState::NotApplicable,
        "live_from_current_session" => OutputTrustState::LiveFromCurrentSession,
        "captured_from_prior_session" => OutputTrustState::CapturedFromPriorSession,
        "replayed_from_snapshot" => OutputTrustState::ReplayedFromSnapshot,
        "orphaned_output" => OutputTrustState::OrphanedOutput,
        "widget_gated" => OutputTrustState::WidgetGated,
        other => panic!("unknown output_trust_state {other}"),
    }
}

fn parse_widget_trust(token: &str) -> WidgetTrustState {
    match token {
        "not_applicable" => WidgetTrustState::NotApplicable,
        "widget_denied_by_default" => WidgetTrustState::WidgetDeniedByDefault,
        "widget_admitted_after_preview" => WidgetTrustState::WidgetAdmittedAfterPreview,
        "widget_suppressed_by_policy" => WidgetTrustState::WidgetSuppressedByPolicy,
        "widget_content_class_blocked" => WidgetTrustState::WidgetContentClassBlocked,
        "widget_runtime_unavailable" => WidgetTrustState::WidgetRuntimeUnavailable,
        other => panic!("unknown widget_trust_state {other}"),
    }
}

fn parse_content_class(token: &str) -> CellContentClass {
    match token {
        "markdown_cell" => CellContentClass::MarkdownCell,
        "code_cell" => CellContentClass::CodeCell,
        "rich_output" => CellContentClass::RichOutput,
        "widget_output" => CellContentClass::WidgetOutput,
        other => panic!("unknown content_class {other}"),
    }
}

fn parse_representation(token: &str) -> RepresentationState {
    match token {
        "raw" => RepresentationState::Raw,
        "sanitized" => RepresentationState::Sanitized,
        "escaped" => RepresentationState::Escaped,
        "sandboxed_active" => RepresentationState::SandboxedActive,
        "tombstone_static_fallback" => RepresentationState::TombstoneStaticFallback,
        "blocked_metadata_only" => RepresentationState::BlockedMetadataOnly,
        other => panic!("unknown representation_state {other}"),
    }
}

fn parse_escape_hatch(token: &str) -> EscapeHatch {
    match token {
        "safe_preview" => EscapeHatch::SafePreview,
        "open_in_browser" => EscapeHatch::OpenInBrowser,
        "open_in_desktop" => EscapeHatch::OpenInDesktop,
        "export_raw_source" => EscapeHatch::ExportRawSource,
        "keep_static_fallback" => EscapeHatch::KeepStaticFallback,
        other => panic!("unknown escape_hatch {other}"),
    }
}

fn parse_degraded(token: &str) -> DegradedStateToken {
    match token {
        "Warming" => DegradedStateToken::Warming,
        "Cached" => DegradedStateToken::Cached,
        "Partial" => DegradedStateToken::Partial,
        "Stale" => DegradedStateToken::Stale,
        "Offline" => DegradedStateToken::Offline,
        "PolicyBlocked" => DegradedStateToken::PolicyBlocked,
        "Limited" => DegradedStateToken::Limited,
        "Unsupported" => DegradedStateToken::Unsupported,
        "Labs" => DegradedStateToken::Labs,
        "Experimental" => DegradedStateToken::Experimental,
        "RetestPending" => DegradedStateToken::RetestPending,
        other => panic!("unknown degraded_token {other}"),
    }
}
