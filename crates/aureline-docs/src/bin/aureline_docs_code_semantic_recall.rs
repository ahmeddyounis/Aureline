//! Headless emitter for the docs-and-code semantic-recall packet and its fixture
//! corpus.
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_code_semantic_recall -- packet
//! cargo run -q -p aureline-docs --bin aureline_docs_code_semantic_recall -- support-export
//! cargo run -q -p aureline-docs --bin aureline_docs_code_semantic_recall -- summary
//! cargo run -q -p aureline-docs --bin aureline_docs_code_semantic_recall -- fixture embedder_unavailable_lexical_fallback_narrowed
//! cargo run -q -p aureline-docs --bin aureline_docs_code_semantic_recall -- validate
//! ```

use aureline_docs::{
    seeded_stable_semantic_recall_ledger_input, DerivationClass, RecallDegradation,
    RecallDegradationClass, SemanticRecallConfidence, SemanticRecallFindingSeverity,
    SemanticRecallLedgerPacket, SemanticRecallLedgerPacketInput,
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
        SemanticRecallLedgerPacket::materialize(seeded_stable_semantic_recall_ledger_input());
    print_json(&packet)
}

fn emit_support_export() -> Result<(), Box<dyn std::error::Error>> {
    let packet =
        SemanticRecallLedgerPacket::materialize(seeded_stable_semantic_recall_ledger_input());
    let export = packet.support_export(
        "support-export:semantic_recall_ledger:001",
        "2026-06-08T00:00:10Z",
    );
    print_json(&export)
}

fn emit_summary() {
    let packet =
        SemanticRecallLedgerPacket::materialize(seeded_stable_semantic_recall_ledger_input());
    print!("{}", packet.render_markdown_summary());
}

fn emit_fixture(name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let name = name.ok_or("fixture name is required")?;
    let fixture = match name {
        "embedder_unavailable_lexical_fallback_narrowed" => embedder_unavailable_fixture(),
        "uncited_code_explainer_blocks_stable" => uncited_code_explainer_fixture(),
        "inferred_explanation_over_authoritative_blocks_stable" => {
            inferred_over_authoritative_fixture()
        }
        other => return Err(format!("unknown fixture: {other}").into()),
    };
    print_json(&fixture)
}

fn validate_packet() {
    let packet =
        SemanticRecallLedgerPacket::materialize(seeded_stable_semantic_recall_ledger_input());
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
struct RecallFixture {
    record_kind: &'static str,
    schema_version: u32,
    case_name: &'static str,
    scenario: &'static str,
    input: SemanticRecallLedgerPacketInput,
    expect: ExpectedFixture,
}

#[derive(Serialize)]
struct ExpectedFixture {
    promotion_state: &'static str,
    expected_finding_kinds: Vec<&'static str>,
}

fn fixture_input(packet_id: &str) -> SemanticRecallLedgerPacketInput {
    let mut input = seeded_stable_semantic_recall_ledger_input();
    input.packet_id = packet_id.to_owned();
    for projection in input.consumer_projections.iter_mut() {
        projection.packet_id_ref = packet_id.to_owned();
    }
    input
}

fn embedder_unavailable_fixture() -> RecallFixture {
    let mut input = fixture_input("packet:m5:semantic_recall:embedder_unavailable_narrowed");
    input.recall_degradations.push(RecallDegradation {
        degradation_class: RecallDegradationClass::EmbedderUnavailableLexicalFallback,
        severity: SemanticRecallFindingSeverity::Narrowing,
        summary: "embedder unavailable; ranking fell back to lexical signals only".to_owned(),
        result_id_ref: None,
        evidence_ref: Some("evidence:retrieval-debug:embedder-status".to_owned()),
    });
    RecallFixture {
        record_kind: "docs_and_code_semantic_recall_query_session_ledger_case",
        schema_version: 1,
        case_name: "embedder_unavailable_lexical_fallback_narrowed",
        scenario: "The embedder is unavailable, so the recall falls back to lexical ranking and records a narrowing degradation. The packet stays a valid, attributable recall but narrows below Stable instead of hiding the result.",
        input,
        expect: ExpectedFixture {
            promotion_state: "narrowed_below_stable",
            expected_finding_kinds: vec![],
        },
    }
}

fn uncited_code_explainer_fixture() -> RecallFixture {
    let mut input = fixture_input("packet:m5:semantic_recall:uncited_code_explainer");
    // Row 4 is a derived summary; strip its citation on both the row and the
    // provenance export so the explainer is no longer cited.
    input.result_rows[3].provenance.cited = false;
    input.result_rows[3].provenance.citation_ref = None;
    input.provenance_export.rows[3].cited = false;
    RecallFixture {
        record_kind: "docs_and_code_semantic_recall_query_session_ledger_case",
        schema_version: 1,
        case_name: "uncited_code_explainer_blocks_stable",
        scenario: "A derived code-explainer summary drops its citation. A codebase explainer must stay cited, so the validator blocks promotion with code_result_not_cited.",
        input,
        expect: ExpectedFixture {
            promotion_state: "blocks_stable",
            expected_finding_kinds: vec!["code_result_not_cited"],
        },
    }
}

fn inferred_over_authoritative_fixture() -> RecallFixture {
    let mut input = fixture_input("packet:m5:semantic_recall:inferred_over_authoritative");
    let row = &mut input.result_rows[3];
    row.provenance.derivation = DerivationClass::InferredExplanation;
    row.chips.confidence = SemanticRecallConfidence::High;
    input.provenance_export.rows[3].derivation = DerivationClass::InferredExplanation;
    input.provenance_export.rows[3].confidence = SemanticRecallConfidence::High;
    RecallFixture {
        record_kind: "docs_and_code_semantic_recall_query_session_ledger_case",
        schema_version: 1,
        case_name: "inferred_explanation_over_authoritative_blocks_stable",
        scenario: "An inferred explanation is presented as a high-confidence result. Inference must preserve confidence, so the validator blocks promotion with inferred_result_looks_authoritative.",
        input,
        expect: ExpectedFixture {
            promotion_state: "blocks_stable",
            expected_finding_kinds: vec!["inferred_result_looks_authoritative"],
        },
    }
}

fn print_json<T: Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
