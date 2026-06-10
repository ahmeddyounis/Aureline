//! Headless emitter for the retrieval-debug packet and its fixture corpus.
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_retrieval_debug_surfaces -- packet
//! cargo run -q -p aureline-docs --bin aureline_docs_retrieval_debug_surfaces -- support-export
//! cargo run -q -p aureline-docs --bin aureline_docs_retrieval_debug_surfaces -- summary
//! cargo run -q -p aureline-docs --bin aureline_docs_retrieval_debug_surfaces -- fixture index_stale_narrowed
//! cargo run -q -p aureline-docs --bin aureline_docs_retrieval_debug_surfaces -- validate
//! ```

use aureline_docs::{
    seeded_stable_retrieval_debug_input, RetrievalConfidence, RetrievalDebugPacket,
    RetrievalDebugPacketInput, RetrievalDegradation, RetrievalDegradationClass,
    RetrievalDerivationLabel, RetrievalFindingSeverity,
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
    let packet = RetrievalDebugPacket::materialize(seeded_stable_retrieval_debug_input());
    print_json(&packet)
}

fn emit_support_export() -> Result<(), Box<dyn std::error::Error>> {
    let packet = RetrievalDebugPacket::materialize(seeded_stable_retrieval_debug_input());
    let export =
        packet.support_export("support-export:retrieval_debug:001", "2026-06-09T00:00:10Z");
    print_json(&export)
}

fn emit_summary() {
    let packet = RetrievalDebugPacket::materialize(seeded_stable_retrieval_debug_input());
    print!("{}", packet.render_markdown_summary());
}

fn emit_fixture(name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let name = name.ok_or("fixture name is required")?;
    let fixture = match name {
        "index_stale_narrowed" => index_stale_fixture(),
        "uncited_imported_entry_blocks_stable" => uncited_imported_entry_fixture(),
        "heuristic_entry_high_confidence_blocks_stable" => heuristic_high_confidence_fixture(),
        other => return Err(format!("unknown fixture: {other}").into()),
    };
    print_json(&fixture)
}

fn validate_packet() {
    let packet = RetrievalDebugPacket::materialize(seeded_stable_retrieval_debug_input());
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
struct RetrievalFixture {
    record_kind: &'static str,
    schema_version: u32,
    case_name: &'static str,
    scenario: &'static str,
    input: RetrievalDebugPacketInput,
    expect: ExpectedFixture,
}

#[derive(Serialize)]
struct ExpectedFixture {
    promotion_state: &'static str,
    expected_finding_kinds: Vec<&'static str>,
}

fn fixture_input(packet_id: &str) -> RetrievalDebugPacketInput {
    let mut input = seeded_stable_retrieval_debug_input();
    input.packet_id = packet_id.to_owned();
    for projection in input.consumer_projections.iter_mut() {
        projection.packet_id_ref = packet_id.to_owned();
    }
    input
}

fn index_stale_fixture() -> RetrievalFixture {
    let mut input = fixture_input("packet:m5:retrieval_debug:index_stale_narrowed");
    input.retrieval_degradations.push(RetrievalDegradation {
        degradation_class: RetrievalDegradationClass::IndexStale,
        severity: RetrievalFindingSeverity::Narrowing,
        summary: "the recall index is stale relative to the working tree; recall entries narrow below stable".to_owned(),
        entry_id_ref: None,
        evidence_ref: Some("evidence:retrieval-debug:index-status".to_owned()),
    });
    RetrievalFixture {
        record_kind: "retrieval_debug_surfaces_for_docs_recall_and_ai_context_case",
        schema_version: 1,
        case_name: "index_stale_narrowed",
        scenario: "The recall index is stale, so the set records a narrowing degradation. The entries stay valid and attributable but narrow below Stable instead of hiding the results.",
        input,
        expect: ExpectedFixture {
            promotion_state: "narrowed_below_stable",
            expected_finding_kinds: vec![],
        },
    }
}

fn uncited_imported_entry_fixture() -> RetrievalFixture {
    let mut input = fixture_input("packet:m5:retrieval_debug:uncited_imported_entry");
    // Strip the citation off the imported semantic-recall entry on both the
    // entry and its export row.
    let mut entry_id = String::new();
    for entry in input.entries.iter_mut() {
        if entry.derivation_label == RetrievalDerivationLabel::Imported {
            entry.cited = false;
            entry.citation_ref = None;
            entry_id = entry.entry_id.clone();
        }
    }
    for row in input.export.rows.iter_mut() {
        if row.entry_id_ref == entry_id {
            row.cited = false;
        }
    }
    RetrievalFixture {
        record_kind: "retrieval_debug_surfaces_for_docs_recall_and_ai_context_case",
        schema_version: 1,
        case_name: "uncited_imported_entry_blocks_stable",
        scenario: "An imported semantic-recall entry drops its citation. Imported results must stay cited, so the validator blocks promotion with entry_not_cited.",
        input,
        expect: ExpectedFixture {
            promotion_state: "blocks_stable",
            expected_finding_kinds: vec!["entry_not_cited"],
        },
    }
}

fn heuristic_high_confidence_fixture() -> RetrievalFixture {
    let mut input = fixture_input("packet:m5:retrieval_debug:heuristic_high_confidence");
    let mut entry_id = String::new();
    for entry in input.entries.iter_mut() {
        if entry.derivation_label == RetrievalDerivationLabel::Heuristic {
            entry.chips.confidence = RetrievalConfidence::High;
            entry_id = entry.entry_id.clone();
        }
    }
    for row in input.export.rows.iter_mut() {
        if row.entry_id_ref == entry_id {
            row.confidence = RetrievalConfidence::High;
        }
    }
    RetrievalFixture {
        record_kind: "retrieval_debug_surfaces_for_docs_recall_and_ai_context_case",
        schema_version: 1,
        case_name: "heuristic_entry_high_confidence_blocks_stable",
        scenario: "A heuristic AI-context entry is presented as a high-confidence result. A heuristic match may never read as high confidence, so the validator blocks promotion with heuristic_label_looks_authoritative.",
        input,
        expect: ExpectedFixture {
            promotion_state: "blocks_stable",
            expected_finding_kinds: vec!["heuristic_label_looks_authoritative"],
        },
    }
}

fn print_json<T: Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
