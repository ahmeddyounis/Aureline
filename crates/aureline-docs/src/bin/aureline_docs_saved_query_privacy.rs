//! Headless emitter for the saved-query-privacy packet and its fixture corpus.
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_saved_query_privacy -- packet
//! cargo run -q -p aureline-docs --bin aureline_docs_saved_query_privacy -- support-export
//! cargo run -q -p aureline-docs --bin aureline_docs_saved_query_privacy -- summary
//! cargo run -q -p aureline-docs --bin aureline_docs_saved_query_privacy -- fixture privacy_narrowed_rerun_narrowed
//! cargo run -q -p aureline-docs --bin aureline_docs_saved_query_privacy -- validate
//! ```

use aureline_docs::{
    seeded_stable_saved_query_privacy_input, QueryPrivacyClass, QueryRedactionClass,
    RetentionPosture, SavedQueryDegradation, SavedQueryDegradationClass, SavedQueryFindingSeverity,
    SavedQueryPrivacyPacket, SavedQueryPrivacyPacketInput,
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
    let packet = SavedQueryPrivacyPacket::materialize(seeded_stable_saved_query_privacy_input());
    print_json(&packet)
}

fn emit_support_export() -> Result<(), Box<dyn std::error::Error>> {
    let packet = SavedQueryPrivacyPacket::materialize(seeded_stable_saved_query_privacy_input());
    let export = packet.support_export(
        "support-export:saved_query_privacy:001",
        "2026-06-10T00:00:10Z",
    );
    print_json(&export)
}

fn emit_summary() {
    let packet = SavedQueryPrivacyPacket::materialize(seeded_stable_saved_query_privacy_input());
    print!("{}", packet.render_markdown_summary());
}

fn emit_fixture(name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let name = name.ok_or("fixture name is required")?;
    let fixture = match name {
        "privacy_narrowed_rerun_narrowed" => privacy_narrowed_fixture(),
        "retention_leak_blocks_stable" => retention_leak_fixture(),
        "support_export_unsafe_blocks_stable" => support_export_unsafe_fixture(),
        other => return Err(format!("unknown fixture: {other}").into()),
    };
    print_json(&fixture)
}

fn validate_packet() {
    let packet = SavedQueryPrivacyPacket::materialize(seeded_stable_saved_query_privacy_input());
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
struct SavedQueryPrivacyFixture {
    record_kind: &'static str,
    schema_version: u32,
    case_name: &'static str,
    scenario: &'static str,
    input: SavedQueryPrivacyPacketInput,
    expect: ExpectedFixture,
}

#[derive(Serialize)]
struct ExpectedFixture {
    promotion_state: &'static str,
    expected_finding_kinds: Vec<&'static str>,
}

fn fixture_input(packet_id: &str) -> SavedQueryPrivacyPacketInput {
    let mut input = seeded_stable_saved_query_privacy_input();
    input.packet_id = packet_id.to_owned();
    for projection in input.consumer_projections.iter_mut() {
        projection.packet_id_ref = packet_id.to_owned();
    }
    input
}

fn privacy_narrowed_fixture() -> SavedQueryPrivacyFixture {
    let mut input = fixture_input("packet:m5:saved_query_privacy:privacy_narrowed_rerun");
    input.query_degradations.push(SavedQueryDegradation {
        degradation_class: SavedQueryDegradationClass::PrivacyNarrowed,
        severity: SavedQueryFindingSeverity::Narrowing,
        summary: "the shared-team pinned query was narrowed back to private after a policy change; the set narrows below stable".to_owned(),
        entry_id_ref: Some("entry:pinned_query:retry_backoff_team_query".to_owned()),
        evidence_ref: Some("evidence:saved-query-privacy:privacy-narrow-state".to_owned()),
    });
    SavedQueryPrivacyFixture {
        record_kind: "saved_query_privacy_controls_case",
        schema_version: 1,
        case_name: "privacy_narrowed_rerun_narrowed",
        scenario: "The shared-team pinned query was narrowed back to private after a policy change, so the set records a narrowing degradation. The entries stay valid and attributable but narrow below Stable instead of hiding them.",
        input,
        expect: ExpectedFixture {
            promotion_state: "narrowed_below_stable",
            expected_finding_kinds: vec![],
        },
    }
}

fn retention_leak_fixture() -> SavedQueryPrivacyFixture {
    let mut input = fixture_input("packet:m5:saved_query_privacy:retention_leak");
    // Move the private-local saved query into a shared store, which exposes it
    // to the team beyond its owner-only privacy ceiling.
    let mut entry_id = String::new();
    for entry in input.entries.iter_mut() {
        if entry.privacy_class == QueryPrivacyClass::PrivateLocal {
            entry.retention.posture = RetentionPosture::SharedStore;
            entry.retention.disclosed = true;
            entry.retention.note = "incorrectly retained in the shared team store".to_owned();
            entry_id = entry.entry_id.clone();
        }
    }
    for row in input.export.rows.iter_mut() {
        if row.entry_id_ref == entry_id {
            row.retention_posture = RetentionPosture::SharedStore;
            row.retention_disclosed = true;
        }
    }
    SavedQueryPrivacyFixture {
        record_kind: "saved_query_privacy_controls_case",
        schema_version: 1,
        case_name: "retention_leak_blocks_stable",
        scenario: "A private-local saved query is retained in a shared team store, exposing it to the team beyond its owner-only privacy ceiling. Local-versus-shared retention truth is mandatory, so the validator blocks promotion with retention_privacy_mismatch.",
        input,
        expect: ExpectedFixture {
            promotion_state: "blocks_stable",
            expected_finding_kinds: vec!["retention_privacy_mismatch"],
        },
    }
}

fn support_export_unsafe_fixture() -> SavedQueryPrivacyFixture {
    let mut input = fixture_input("packet:m5:saved_query_privacy:support_export_unsafe");
    // Mark the synced history entry export-safe while its redaction class still
    // needs redaction.
    let mut entry_id = String::new();
    for entry in input.entries.iter_mut() {
        if entry.privacy_class == QueryPrivacyClass::PrivateSynced {
            entry.export_safety.redaction_class = QueryRedactionClass::NeedsRedaction;
            entry.export_safety.export_safe = true;
            entry_id = entry.entry_id.clone();
        }
    }
    for row in input.export.rows.iter_mut() {
        if row.entry_id_ref == entry_id {
            row.redaction_class = QueryRedactionClass::NeedsRedaction;
            row.export_safe = true;
        }
    }
    SavedQueryPrivacyFixture {
        record_kind: "saved_query_privacy_controls_case",
        schema_version: 1,
        case_name: "support_export_unsafe_blocks_stable",
        scenario: "A synced history entry is marked export-safe while its redaction class still needs redaction. A search-history entry may never travel in a support export unless it is actually redaction-safe, so the validator blocks promotion with support_export_unsafe.",
        input,
        expect: ExpectedFixture {
            promotion_state: "blocks_stable",
            expected_finding_kinds: vec!["support_export_unsafe"],
        },
    }
}

fn print_json<T: Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
