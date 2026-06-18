//! Headless emitter for the docs-suggestion-panel packet and its fixture corpus.
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_suggestion_panel -- packet
//! cargo run -q -p aureline-docs --bin aureline_docs_suggestion_panel -- support-export
//! cargo run -q -p aureline-docs --bin aureline_docs_suggestion_panel -- summary
//! cargo run -q -p aureline-docs --bin aureline_docs_suggestion_panel -- fixture mirror_offline_narrows
//! cargo run -q -p aureline-docs --bin aureline_docs_suggestion_panel -- validate
//! ```

use aureline_docs::{
    seeded_stable_docs_suggestion_panel_input, DocsSuggestionPanelPacket,
    DocsSuggestionPanelPacketInput, PanelApplyPosture, PanelConfidence, PanelDegradationClass,
    PanelEvidenceProvenance, PanelFindingSeverity, PanelFreshness, PanelProposalKind,
};
use serde::Serialize;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.first().map(String::as_str) {
        Some("packet") | None => emit_packet()?,
        Some("support-export") => emit_support_export()?,
        Some("summary") => emit_summary(),
        Some("fixture") => emit_fixture(args.get(1).map(String::as_str))?,
        Some("validate") => validate_packet(),
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn emit_packet() -> Result<(), Box<dyn std::error::Error>> {
    let packet =
        DocsSuggestionPanelPacket::materialize(seeded_stable_docs_suggestion_panel_input());
    print_json(&packet)
}

fn emit_support_export() -> Result<(), Box<dyn std::error::Error>> {
    let packet =
        DocsSuggestionPanelPacket::materialize(seeded_stable_docs_suggestion_panel_input());
    let export = packet.support_export(
        "support-export:docs_suggestion_panel:001",
        "2026-06-12T00:00:10Z",
    );
    print_json(&export)
}

fn emit_summary() {
    let packet =
        DocsSuggestionPanelPacket::materialize(seeded_stable_docs_suggestion_panel_input());
    print!("{}", packet.render_markdown_summary());
}

fn emit_fixture(name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let name = name.ok_or("fixture name is required")?;
    let fixture = match name {
        "mirror_offline_narrows" => mirror_offline_fixture(),
        "prose_only_card_blocks_stable" => prose_only_fixture(),
        "unverified_apply_blocks_stable" => unverified_apply_fixture(),
        other => return Err(format!("unknown fixture: {other}").into()),
    };
    print_json(&fixture)
}

fn validate_packet() {
    let packet =
        DocsSuggestionPanelPacket::materialize(seeded_stable_docs_suggestion_panel_input());
    if packet.is_clean_stable() {
        println!("ok");
    } else {
        for finding in &packet.validation_findings {
            eprintln!("{}: {}", finding.finding_kind.as_str(), finding.summary);
        }
        std::process::exit(3);
    }
}

#[derive(Serialize)]
struct DocsSuggestionPanelFixture {
    record_kind: &'static str,
    schema_version: u32,
    case_name: &'static str,
    scenario: &'static str,
    input: DocsSuggestionPanelPacketInput,
    expect: ExpectedFixture,
}

#[derive(Serialize)]
struct ExpectedFixture {
    promotion_state: &'static str,
    expected_finding_kinds: Vec<&'static str>,
}

fn fixture_input(packet_id: &str) -> DocsSuggestionPanelPacketInput {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    input.packet_id = packet_id.to_owned();
    for projection in input.consumer_projections.iter_mut() {
        projection.packet_id_ref = packet_id.to_owned();
    }
    input
}

fn mirror_offline_fixture() -> DocsSuggestionPanelFixture {
    let mut input = fixture_input("packet:m5:docs_suggestion_panel:mirror_offline");
    // The mirror is offline; the panel narrows but the suggestions stay visible.
    input
        .panel_degradations
        .push(aureline_docs::PanelDegradation {
            degradation_class: PanelDegradationClass::MirrorOfflineSnapshot,
            severity: PanelFindingSeverity::Narrowing,
            summary:
                "the docs mirror is offline; the imported broken-link suggestion is served from the last snapshot and the panel is narrowed"
                    .to_owned(),
            suggestion_id_ref: Some("suggestion:help:retry_backoff_runbook_link".to_owned()),
            evidence_ref: Some("evidence:docs-suggestion-panel:mirror-offline".to_owned()),
        });
    DocsSuggestionPanelFixture {
        record_kind: "docs_suggestion_panel_case",
        schema_version: 1,
        case_name: "mirror_offline_narrows",
        scenario: "The docs mirror is offline, so the imported broken-link suggestion is served from the last snapshot. The narrowing degradation keeps every suggestion visible and attributable, so the panel narrows below Stable instead of hiding the suggestions — the downgrade narrows the claim, it does not hide the work.",
        input,
        expect: ExpectedFixture {
            promotion_state: "narrowed_below_stable",
            expected_finding_kinds: vec![],
        },
    }
}

fn prose_only_fixture() -> DocsSuggestionPanelFixture {
    let mut input = fixture_input("packet:m5:docs_suggestion_panel:prose_only");
    // The README suggestion is a prose-only "recommended edit" card with no diff.
    let suggestion = &mut input.suggestions[0];
    suggestion.proposal.proposal_kind = PanelProposalKind::ProseOnlyCard;
    suggestion.proposal.hunk_count = 0;
    suggestion.proposal.added_lines = 0;
    suggestion.proposal.removed_lines = 0;
    suggestion.proposal.summary =
        "recommended edit: mention the new max_elapsed parameter somewhere in the README"
            .to_owned();
    // A prose-only card can never offer a one-click apply.
    suggestion.actions.apply_posture = PanelApplyPosture::ApplyUnavailableDisclosed;
    let id = suggestion.suggestion_id.clone();
    for row in input.export.rows.iter_mut() {
        if row.suggestion_id_ref == id {
            row.apply_posture = PanelApplyPosture::ApplyUnavailableDisclosed;
        }
    }
    DocsSuggestionPanelFixture {
        record_kind: "docs_suggestion_panel_case",
        schema_version: 1,
        case_name: "prose_only_card_blocks_stable",
        scenario: "A README suggestion is a prose-only 'recommended edit' card with no diff. Diff-first proposals are mandatory, so the validator blocks promotion with proposal_not_diff_based — docs maintenance may not bypass the shared review/diff model just because the target is prose.",
        input,
        expect: ExpectedFixture {
            promotion_state: "blocks_stable",
            expected_finding_kinds: vec!["proposal_not_diff_based"],
        },
    }
}

fn unverified_apply_fixture() -> DocsSuggestionPanelFixture {
    let mut input = fixture_input("packet:m5:docs_suggestion_panel:unverified_apply");
    // The imported broken-link suggestion offers a one-click apply.
    let suggestion = input
        .suggestions
        .iter_mut()
        .find(|s| s.provenance == PanelEvidenceProvenance::Imported)
        .expect("imported suggestion present");
    suggestion.actions.apply_posture = PanelApplyPosture::ApplyAvailable;
    suggestion.chips.confidence = PanelConfidence::Medium;
    suggestion.chips.freshness = PanelFreshness::WarmCached;
    let id = suggestion.suggestion_id.clone();
    for row in input.export.rows.iter_mut() {
        if row.suggestion_id_ref == id {
            row.apply_posture = PanelApplyPosture::ApplyAvailable;
            row.confidence = PanelConfidence::Medium;
            row.freshness = PanelFreshness::WarmCached;
        }
    }
    DocsSuggestionPanelFixture {
        record_kind: "docs_suggestion_panel_case",
        schema_version: 1,
        case_name: "unverified_apply_blocks_stable",
        scenario: "An imported, mirror-served broken-link suggestion offers a one-click apply. Action-parity apply gating is mandatory, so the validator blocks promotion with unverified_apply_offered — an unverified evidence source may surface a preview but never a one-click apply.",
        input,
        expect: ExpectedFixture {
            promotion_state: "blocks_stable",
            expected_finding_kinds: vec!["unverified_apply_offered"],
        },
    }
}

fn print_json<T: Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
