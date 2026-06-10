//! Headless emitter for the mirrored docs-pack recall packet and its fixture
//! corpus.
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_pack_recall -- packet
//! cargo run -q -p aureline-docs --bin aureline_docs_pack_recall -- support-export
//! cargo run -q -p aureline-docs --bin aureline_docs_pack_recall -- summary
//! cargo run -q -p aureline-docs --bin aureline_docs_pack_recall -- fixture mirror_offline_recall_narrowed
//! cargo run -q -p aureline-docs --bin aureline_docs_pack_recall -- validate
//! ```

use aureline_docs::{
    seeded_stable_docs_pack_recall_input, DocsPackRecallConfidence, DocsPackRecallFindingKind,
    DocsPackRecallFreshness, DocsPackRecallPacket, DocsPackRecallPacketInput,
    DocsPackRecallPromotionState, DocsPackRecallStaleFindingClass, DocsPackRecallVersionMatch,
    DOCS_PACK_RECALL_SCHEMA_VERSION,
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
    let packet = DocsPackRecallPacket::materialize(seeded_stable_docs_pack_recall_input());
    print_json(&packet)
}

fn emit_support_export() -> Result<(), Box<dyn std::error::Error>> {
    let packet = DocsPackRecallPacket::materialize(seeded_stable_docs_pack_recall_input());
    let export = packet.support_export(
        "support-export:docs_pack_recall:001",
        "2026-06-08T00:00:10Z",
    );
    print_json(&export)
}

fn emit_summary() {
    let packet = DocsPackRecallPacket::materialize(seeded_stable_docs_pack_recall_input());
    print!("{}", packet.render_markdown_summary());
}

fn emit_fixture(name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let name = name.ok_or("fixture name is required")?;
    let fixture = match name {
        "mirror_offline_recall_narrowed" => mirror_offline_narrowed_fixture(),
        "live_mirror_over_authoritative_blocks_stable" => live_mirror_over_authoritative_fixture(),
        "stale_state_collapsed_blocks_stable" => stale_state_collapsed_fixture(),
        other => return Err(format!("unknown fixture: {other}").into()),
    };
    print_json(&fixture)
}

fn validate_packet() {
    let packet = DocsPackRecallPacket::materialize(seeded_stable_docs_pack_recall_input());
    if packet.is_clean_stable() {
        println!("ok");
    } else {
        for finding in &packet.validation_findings {
            eprintln!("{}: {}", finding.finding_kind.as_str(), finding.summary);
        }
        std::process::exit(3);
    }
}

fn print_json<T: Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(value)?;
    println!("{json}");
    Ok(())
}

#[derive(Debug, Serialize)]
struct Fixture {
    record_kind: &'static str,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: DocsPackRecallPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Serialize)]
struct ExpectedFixture {
    promotion_state: &'static str,
    expected_finding_kinds: Vec<&'static str>,
}

const FIXTURE_RECORD_KIND: &str = "mirrored_docs_pack_recall_case";

fn mirror_offline_narrowed_fixture() -> Fixture {
    let mut input = seeded_stable_docs_pack_recall_input();
    input.packet_id = "packet:m5:docs_pack_recall:mirror_offline_narrowed".to_owned();
    for projection in input.consumer_projections.iter_mut() {
        projection.packet_id_ref = input.packet_id.clone();
    }
    // The mirror is offline: its row falls back to a pinned offline cache and the
    // recall narrows rather than hiding the result.
    if let Some(row) = input.result_rows.get_mut(1) {
        row.chips.freshness = DocsPackRecallFreshness::RefreshPending;
        row.chips.confidence = DocsPackRecallConfidence::Medium;
        row.ranking_reason =
            "pinned, signed mirror is offline; serving the last verified offline snapshot"
                .to_owned();
    }
    for finding in input.stale_example_findings.iter_mut() {
        if finding.finding_class == DocsPackRecallStaleFindingClass::NearbyVersion {
            finding.severity = aureline_docs::DocsPackRecallFindingSeverity::Narrowing;
            finding.summary =
                "mirror offline: a fresher upstream version may exist but cannot be verified"
                    .to_owned();
        }
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_PACK_RECALL_SCHEMA_VERSION,
        case_name: "mirror_offline_recall_narrowed".to_owned(),
        scenario: "A pinned, signed mirror is offline. Its row falls back to the last verified \
                   offline snapshot with refresh_pending freshness, and the attached \
                   nearby-version finding is narrowing. The recall stays a valid, attributable \
                   packet but narrows below Stable instead of hiding the result."
            .to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsPackRecallPromotionState::NarrowedBelowStable.as_str(),
            expected_finding_kinds: Vec::new(),
        },
    }
}

fn live_mirror_over_authoritative_fixture() -> Fixture {
    let mut input = seeded_stable_docs_pack_recall_input();
    input.packet_id = "packet:m5:docs_pack_recall:live_mirror_over_authoritative".to_owned();
    for projection in input.consumer_projections.iter_mut() {
        projection.packet_id_ref = input.packet_id.clone();
    }
    // A mirrored row claims live authority without a pinned, verified pack.
    if let Some(row) = input.result_rows.get_mut(1) {
        row.pack_pinned = false;
        row.pack_signed_and_verified = false;
        row.chips.freshness = DocsPackRecallFreshness::AuthoritativeLive;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_PACK_RECALL_SCHEMA_VERSION,
        case_name: "live_mirror_over_authoritative_blocks_stable".to_owned(),
        scenario:
            "A mirrored_official_docs row claims authoritative_live freshness while its pack \
                   is neither pinned nor signature-verified, making the mirror look more \
                   authoritative than it is. The validator blocks promotion with \
                   live_mirror_looks_more_authoritative."
                .to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsPackRecallPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsPackRecallFindingKind::LiveMirrorLooksMoreAuthoritative.as_str(),
            ],
        },
    }
}

fn stale_state_collapsed_fixture() -> Fixture {
    let mut input = seeded_stable_docs_pack_recall_input();
    input.packet_id = "packet:m5:docs_pack_recall:stale_state_collapsed".to_owned();
    for projection in input.consumer_projections.iter_mut() {
        projection.packet_id_ref = input.packet_id.clone();
    }
    // A drifted version is presented as a confident, live match — collapsing the
    // version-truth distinction the chips are supposed to keep.
    if let Some(row) = input.result_rows.get_mut(2) {
        row.chips.version_match = DocsPackRecallVersionMatch::IncompatibleDriftDetected;
        row.chips.confidence = DocsPackRecallConfidence::High;
        row.chips.freshness = DocsPackRecallFreshness::AuthoritativeLive;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_PACK_RECALL_SCHEMA_VERSION,
        case_name: "stale_state_collapsed_blocks_stable".to_owned(),
        scenario: "A row whose version drifted incompatibly is presented as a high-confidence, \
                   authoritative_live match, collapsing the version-match chip's truth. The \
                   validator blocks promotion with version_truth_collapsed."
            .to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsPackRecallPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![DocsPackRecallFindingKind::VersionTruthCollapsed.as_str()],
        },
    }
}
