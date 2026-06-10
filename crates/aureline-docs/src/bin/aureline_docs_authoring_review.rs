//! Headless emitter for the docs-authoring-review packet and its fixture corpus.
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_authoring_review -- packet
//! cargo run -q -p aureline-docs --bin aureline_docs_authoring_review -- support-export
//! cargo run -q -p aureline-docs --bin aureline_docs_authoring_review -- summary
//! cargo run -q -p aureline-docs --bin aureline_docs_authoring_review -- fixture stale_example_drift_narrows
//! cargo run -q -p aureline-docs --bin aureline_docs_authoring_review -- validate
//! ```

use aureline_docs::{
    seeded_stable_docs_authoring_review_input, DocsAuthoringReviewPacket,
    DocsAuthoringReviewPacketInput, DocsReviewConfidence, DocsReviewFindingSeverity,
    DocsReviewFreshness, DocsReviewItemKind, DocsReviewTrustClass, ReviewFindingClass,
    SuggestionApplyPosture,
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
        DocsAuthoringReviewPacket::materialize(seeded_stable_docs_authoring_review_input());
    print_json(&packet)
}

fn emit_support_export() -> Result<(), Box<dyn std::error::Error>> {
    let packet =
        DocsAuthoringReviewPacket::materialize(seeded_stable_docs_authoring_review_input());
    let export = packet.support_export(
        "support-export:docs_authoring_review:001",
        "2026-06-10T00:00:10Z",
    );
    print_json(&export)
}

fn emit_summary() {
    let packet =
        DocsAuthoringReviewPacket::materialize(seeded_stable_docs_authoring_review_input());
    print!("{}", packet.render_markdown_summary());
}

fn emit_fixture(name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let name = name.ok_or("fixture name is required")?;
    let fixture = match name {
        "stale_example_drift_narrows" => stale_example_drift_fixture(),
        "broken_link_blocks_stable" => broken_link_fixture(),
        "unverified_suggestion_apply_blocks_stable" => unverified_suggestion_apply_fixture(),
        other => return Err(format!("unknown fixture: {other}").into()),
    };
    print_json(&fixture)
}

fn validate_packet() {
    let packet =
        DocsAuthoringReviewPacket::materialize(seeded_stable_docs_authoring_review_input());
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
struct DocsAuthoringReviewFixture {
    record_kind: &'static str,
    schema_version: u32,
    case_name: &'static str,
    scenario: &'static str,
    input: DocsAuthoringReviewPacketInput,
    expect: ExpectedFixture,
}

#[derive(Serialize)]
struct ExpectedFixture {
    promotion_state: &'static str,
    expected_finding_kinds: Vec<&'static str>,
}

fn fixture_input(packet_id: &str) -> DocsAuthoringReviewPacketInput {
    let mut input = seeded_stable_docs_authoring_review_input();
    input.packet_id = packet_id.to_owned();
    for projection in input.consumer_projections.iter_mut() {
        projection.packet_id_ref = packet_id.to_owned();
    }
    input
}

fn stale_example_drift_fixture() -> DocsAuthoringReviewFixture {
    let mut input = fixture_input("packet:m5:docs_authoring_review:stale_example_drift");
    // The stale-example review found a drift; the verdict narrows the claim.
    let mut item_id = String::new();
    for item in input.items.iter_mut() {
        if item.item_kind == DocsReviewItemKind::StaleExampleReview {
            item.review.finding_class = ReviewFindingClass::StaleExampleDrifted;
            item.review.severity = DocsReviewFindingSeverity::Narrowing;
            item.review.note =
                "the example drifted from the retry_with_backoff signature; flagged for an update"
                    .to_owned();
            item.chips.freshness = DocsReviewFreshness::Stale;
            item.chips.confidence = DocsReviewConfidence::Medium;
            item.suggestion.apply_posture = SuggestionApplyPosture::PreviewRequired;
            item.suggestion.note =
                "apply updates the example to the current signature; a preview is required"
                    .to_owned();
            item_id = item.item_id.clone();
        }
    }
    for row in input.export.rows.iter_mut() {
        if row.item_id_ref == item_id {
            row.finding_class = ReviewFindingClass::StaleExampleDrifted;
            row.review_severity = DocsReviewFindingSeverity::Narrowing;
            row.confidence = DocsReviewConfidence::Medium;
            row.apply_posture = SuggestionApplyPosture::PreviewRequired;
        }
    }
    DocsAuthoringReviewFixture {
        record_kind: "docs_authoring_review_controls_case",
        schema_version: 1,
        case_name: "stale_example_drift_narrows",
        scenario: "A stale-example review finds the retry/backoff example drifted from the documented signature. The verdict carries a narrowing severity and the example is shown as stale, so the set narrows below Stable instead of hiding it — the downgrade narrows the claim, it does not hide the item.",
        input,
        expect: ExpectedFixture {
            promotion_state: "narrowed_below_stable",
            expected_finding_kinds: vec![],
        },
    }
}

fn broken_link_fixture() -> DocsAuthoringReviewFixture {
    let mut input = fixture_input("packet:m5:docs_authoring_review:broken_link");
    // A broken link is presented as live-authoritative — a truth collapse that
    // blocks promotion with stale_verdict_freshness_mismatch.
    let mut item_id = String::new();
    for item in input.items.iter_mut() {
        if item.item_kind == DocsReviewItemKind::StaleLinkReview {
            item.review.finding_class = ReviewFindingClass::StaleLinkBroken;
            item.review.severity = DocsReviewFindingSeverity::Blocking;
            item.review.note =
                "the runbook link no longer resolves; the target was removed upstream".to_owned();
            item.chips.freshness = DocsReviewFreshness::AuthoritativeLive;
            item_id = item.item_id.clone();
        }
    }
    for row in input.export.rows.iter_mut() {
        if row.item_id_ref == item_id {
            row.finding_class = ReviewFindingClass::StaleLinkBroken;
            row.review_severity = DocsReviewFindingSeverity::Blocking;
        }
    }
    DocsAuthoringReviewFixture {
        record_kind: "docs_authoring_review_controls_case",
        schema_version: 1,
        case_name: "broken_link_blocks_stable",
        scenario: "A stale-link review finds the runbook link broken but the item still claims live-authoritative freshness. Stale-link review truth is mandatory, so the validator blocks promotion with stale_verdict_freshness_mismatch — a broken link can never read as current.",
        input,
        expect: ExpectedFixture {
            promotion_state: "blocks_stable",
            expected_finding_kinds: vec!["stale_verdict_freshness_mismatch"],
        },
    }
}

fn unverified_suggestion_apply_fixture() -> DocsAuthoringReviewFixture {
    let mut input = fixture_input("packet:m5:docs_authoring_review:unverified_apply");
    // An unverified live-mirror suggestion offers a one-click apply.
    let item = &mut input.items[0];
    item.trust_class = DocsReviewTrustClass::LiveMirrorSuggestion;
    item.trust_disclosure_note =
        "a live-mirror authoring suggestion; not verified at materialization time".to_owned();
    item.suggestion.apply_posture = SuggestionApplyPosture::ApplyAvailable;
    item.chips.confidence = DocsReviewConfidence::Medium;
    item.cited = true;
    let item_id = item.item_id.clone();
    for row in input.export.rows.iter_mut() {
        if row.item_id_ref == item_id {
            row.trust_class = DocsReviewTrustClass::LiveMirrorSuggestion;
            row.apply_posture = SuggestionApplyPosture::ApplyAvailable;
            row.confidence = DocsReviewConfidence::Medium;
        }
    }
    DocsAuthoringReviewFixture {
        record_kind: "docs_authoring_review_controls_case",
        schema_version: 1,
        case_name: "unverified_suggestion_apply_blocks_stable",
        scenario: "An unverified live-mirror authoring suggestion offers a one-click apply. Authoring-suggestion apply-posture truth is mandatory, so the validator blocks promotion with unverified_suggestion_apply_offered — an unverified suggestion may surface a preview but never a one-click apply.",
        input,
        expect: ExpectedFixture {
            promotion_state: "blocks_stable",
            expected_finding_kinds: vec!["unverified_suggestion_apply_offered"],
        },
    }
}

fn print_json<T: Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
