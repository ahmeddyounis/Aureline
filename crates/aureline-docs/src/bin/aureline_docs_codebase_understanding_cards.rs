//! Headless emitter for the topology/ownership/explainer card packet and its
//! fixture corpus.
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_codebase_understanding_cards -- packet
//! cargo run -q -p aureline-docs --bin aureline_docs_codebase_understanding_cards -- support-export
//! cargo run -q -p aureline-docs --bin aureline_docs_codebase_understanding_cards -- summary
//! cargo run -q -p aureline-docs --bin aureline_docs_codebase_understanding_cards -- fixture graph_index_stale_narrowed
//! cargo run -q -p aureline-docs --bin aureline_docs_codebase_understanding_cards -- validate
//! ```

use aureline_docs::{
    seeded_stable_codebase_understanding_cards_input, CodebaseUnderstandingCardsPacket,
    CodebaseUnderstandingCardsPacketInput, EvidenceDerivation, UnderstandingCardKind,
    UnderstandingConfidence, UnderstandingDegradation, UnderstandingDegradationClass,
    UnderstandingFindingSeverity,
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
    let packet = CodebaseUnderstandingCardsPacket::materialize(
        seeded_stable_codebase_understanding_cards_input(),
    );
    print_json(&packet)
}

fn emit_support_export() -> Result<(), Box<dyn std::error::Error>> {
    let packet = CodebaseUnderstandingCardsPacket::materialize(
        seeded_stable_codebase_understanding_cards_input(),
    );
    let export = packet.support_export(
        "support-export:understanding_cards:001",
        "2026-06-09T00:00:10Z",
    );
    print_json(&export)
}

fn emit_summary() {
    let packet = CodebaseUnderstandingCardsPacket::materialize(
        seeded_stable_codebase_understanding_cards_input(),
    );
    print!("{}", packet.render_markdown_summary());
}

fn emit_fixture(name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let name = name.ok_or("fixture name is required")?;
    let fixture = match name {
        "graph_index_stale_narrowed" => graph_index_stale_fixture(),
        "uncited_topology_card_blocks_stable" => uncited_topology_card_fixture(),
        "inferred_explainer_over_authoritative_blocks_stable" => {
            inferred_explainer_over_authoritative_fixture()
        }
        other => return Err(format!("unknown fixture: {other}").into()),
    };
    print_json(&fixture)
}

fn validate_packet() {
    let packet = CodebaseUnderstandingCardsPacket::materialize(
        seeded_stable_codebase_understanding_cards_input(),
    );
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
struct CardsFixture {
    record_kind: &'static str,
    schema_version: u32,
    case_name: &'static str,
    scenario: &'static str,
    input: CodebaseUnderstandingCardsPacketInput,
    expect: ExpectedFixture,
}

#[derive(Serialize)]
struct ExpectedFixture {
    promotion_state: &'static str,
    expected_finding_kinds: Vec<&'static str>,
}

fn fixture_input(packet_id: &str) -> CodebaseUnderstandingCardsPacketInput {
    let mut input = seeded_stable_codebase_understanding_cards_input();
    input.packet_id = packet_id.to_owned();
    for projection in input.consumer_projections.iter_mut() {
        projection.packet_id_ref = packet_id.to_owned();
    }
    input
}

fn graph_index_stale_fixture() -> CardsFixture {
    let mut input = fixture_input("packet:m5:understanding_cards:graph_index_stale_narrowed");
    input.understanding_degradations.push(UnderstandingDegradation {
        degradation_class: UnderstandingDegradationClass::GraphIndexStale,
        severity: UnderstandingFindingSeverity::Narrowing,
        summary: "the workspace graph index is stale relative to the working tree; topology edges narrow below stable".to_owned(),
        card_id_ref: None,
        evidence_ref: Some("evidence:retrieval-debug:graph-index-status".to_owned()),
    });
    CardsFixture {
        record_kind: "topology_ownership_and_codebase_explainer_cards_case",
        schema_version: 1,
        case_name: "graph_index_stale_narrowed",
        scenario: "The workspace graph index is stale, so the card set records a narrowing degradation. The cards stay valid and attributable but narrow below Stable instead of hiding the topology.",
        input,
        expect: ExpectedFixture {
            promotion_state: "narrowed_below_stable",
            expected_finding_kinds: vec![],
        },
    }
}

fn uncited_topology_card_fixture() -> CardsFixture {
    let mut input = fixture_input("packet:m5:understanding_cards:uncited_topology_card");
    // The topology card is a derived summary; strip its citation on both the
    // card and the export so the topology card is no longer cited.
    let card_id = input.cards[0].card_id.clone();
    input.cards[0].provenance.cited = false;
    input.cards[0].provenance.citation_ref = None;
    for row in input.evidence_export.rows.iter_mut() {
        if row.card_id_ref == card_id {
            row.cited = false;
        }
    }
    CardsFixture {
        record_kind: "topology_ownership_and_codebase_explainer_cards_case",
        schema_version: 1,
        case_name: "uncited_topology_card_blocks_stable",
        scenario: "A derived topology card drops its citation. A topology card must stay cited, so the validator blocks promotion with card_not_cited.",
        input,
        expect: ExpectedFixture {
            promotion_state: "blocks_stable",
            expected_finding_kinds: vec!["card_not_cited"],
        },
    }
}

fn inferred_explainer_over_authoritative_fixture() -> CardsFixture {
    let mut input =
        fixture_input("packet:m5:understanding_cards:inferred_explainer_over_authoritative");
    let card_id = "card:explainer:retry_with_backoff".to_owned();
    for card in input.cards.iter_mut() {
        if card.card_kind == UnderstandingCardKind::CodebaseExplainer {
            card.chips.confidence = UnderstandingConfidence::High;
        }
    }
    for row in input.evidence_export.rows.iter_mut() {
        if row.card_id_ref == card_id {
            row.confidence = UnderstandingConfidence::High;
            row.derivation = EvidenceDerivation::InferredExplanation;
        }
    }
    CardsFixture {
        record_kind: "topology_ownership_and_codebase_explainer_cards_case",
        schema_version: 1,
        case_name: "inferred_explainer_over_authoritative_blocks_stable",
        scenario: "An inferred codebase explainer is presented as a high-confidence card. Inference must preserve confidence, so the validator blocks promotion with inferred_card_looks_authoritative.",
        input,
        expect: ExpectedFixture {
            promotion_state: "blocks_stable",
            expected_finding_kinds: vec!["inferred_card_looks_authoritative"],
        },
    }
}

fn print_json<T: Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
