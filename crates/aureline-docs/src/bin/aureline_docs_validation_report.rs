//! Headless emitter for the docs-validation-report packet and its fixture corpus.
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_validation_report -- packet
//! cargo run -q -p aureline-docs --bin aureline_docs_validation_report -- support-export
//! cargo run -q -p aureline-docs --bin aureline_docs_validation_report -- summary
//! cargo run -q -p aureline-docs --bin aureline_docs_validation_report -- fixture mirror_offline_narrows
//! cargo run -q -p aureline-docs --bin aureline_docs_validation_report -- validate
//! ```

use aureline_docs::{
    seeded_stable_docs_validation_report_input, DocsValidationReportPacket,
    DocsValidationReportPacketInput, ValidationDegradationClass, ValidationFindingSeverity,
    ValidationFreshness, ValidationMode, ValidationOutcome,
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
        DocsValidationReportPacket::materialize(seeded_stable_docs_validation_report_input());
    print_json(&packet)
}

fn emit_support_export() -> Result<(), Box<dyn std::error::Error>> {
    let packet =
        DocsValidationReportPacket::materialize(seeded_stable_docs_validation_report_input());
    let export = packet.support_export(
        "support-export:docs_validation_report:001",
        "2026-06-12T00:00:10Z",
    );
    print_json(&export)
}

fn emit_summary() {
    let packet =
        DocsValidationReportPacket::materialize(seeded_stable_docs_validation_report_input());
    print!("{}", packet.render_markdown_summary());
}

fn emit_fixture(name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let name = name.ok_or("fixture name is required")?;
    let fixture = match name {
        "mirror_offline_narrows" => mirror_offline_fixture(),
        "rendered_claims_execution_blocks_stable" => rendered_claims_execution_fixture(),
        "untraced_broken_link_blocks_stable" => untraced_broken_link_fixture(),
        other => return Err(format!("unknown fixture: {other}").into()),
    };
    print_json(&fixture)
}

fn validate_packet() {
    let packet =
        DocsValidationReportPacket::materialize(seeded_stable_docs_validation_report_input());
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
struct DocsValidationReportFixture {
    record_kind: &'static str,
    schema_version: u32,
    case_name: &'static str,
    scenario: &'static str,
    input: DocsValidationReportPacketInput,
    expect: ExpectedFixture,
}

#[derive(Serialize)]
struct ExpectedFixture {
    promotion_state: &'static str,
    expected_finding_kinds: Vec<&'static str>,
}

fn fixture_input(packet_id: &str) -> DocsValidationReportPacketInput {
    let mut input = seeded_stable_docs_validation_report_input();
    input.packet_id = packet_id.to_owned();
    for projection in input.consumer_projections.iter_mut() {
        projection.packet_id_ref = packet_id.to_owned();
    }
    input
}

fn mirror_offline_fixture() -> DocsValidationReportFixture {
    let mut input = fixture_input("packet:m5:docs_validation_report:mirror_offline");
    // The mirror is offline; the report narrows but the rows stay visible.
    input.report_degradations.push(aureline_docs::ValidationDegradation {
        degradation_class: ValidationDegradationClass::MirrorOfflineSnapshot,
        severity: ValidationFindingSeverity::Narrowing,
        summary:
            "the docs mirror is offline; the imported broken-link and mirrored unsupported rows are served from the last snapshot and the report is narrowed"
                .to_owned(),
        row_id_ref: Some("row:tutorial:runbook_broken_link".to_owned()),
        evidence_ref: Some("evidence:docs-validation-report:mirror-offline".to_owned()),
    });
    DocsValidationReportFixture {
        record_kind: "docs_validation_report_case",
        schema_version: 1,
        case_name: "mirror_offline_narrows",
        scenario: "The docs mirror is offline, so the imported broken-link and mirrored unsupported rows are served from the last snapshot. The narrowing degradation keeps every row visible and attributable, so the report narrows below Stable instead of hiding the rows — the downgrade narrows the claim, it does not hide the findings.",
        input,
        expect: ExpectedFixture {
            promotion_state: "narrowed_below_stable",
            expected_finding_kinds: vec![],
        },
    }
}

fn rendered_claims_execution_fixture() -> DocsValidationReportFixture {
    let mut input = fixture_input("packet:m5:docs_validation_report:rendered_claims_execution");
    // The rendered-preview row is presented as an actually executed pass.
    let row = input
        .rows
        .iter_mut()
        .find(|r| r.mode == ValidationMode::Rendered)
        .expect("rendered row present");
    row.outcome = ValidationOutcome::ExecutedPass;
    row.chips.freshness = ValidationFreshness::AuthoritativeLive;
    let id = row.row_id.clone();
    for export in input.export.rows.iter_mut() {
        if export.row_id_ref == id {
            export.outcome = ValidationOutcome::ExecutedPass;
            export.freshness = ValidationFreshness::AuthoritativeLive;
        }
    }
    DocsValidationReportFixture {
        record_kind: "docs_validation_report_case",
        schema_version: 1,
        case_name: "rendered_claims_execution_blocks_stable",
        scenario: "A rendered-preview row is presented as an actually executed pass. The rendered-vs-executed distinction is mandatory, so the validator blocks promotion with execution_claim_without_run — a harmless rendered preview may never be presented as an actually executed example.",
        input,
        expect: ExpectedFixture {
            promotion_state: "blocks_stable",
            expected_finding_kinds: vec!["execution_claim_without_run"],
        },
    }
}

fn untraced_broken_link_fixture() -> DocsValidationReportFixture {
    let mut input = fixture_input("packet:m5:docs_validation_report:untraced_broken_link");
    // The broken-link finding drops its source/evidence trace, leaving a
    // decorative badge with no failing source to open.
    let row = input
        .rows
        .iter_mut()
        .find(|r| r.mode == ValidationMode::BrokenLink)
        .expect("broken-link row present");
    row.source_trace_ref = String::new();
    DocsValidationReportFixture {
        record_kind: "docs_validation_report_case",
        schema_version: 1,
        case_name: "untraced_broken_link_blocks_stable",
        scenario: "A broken-link finding drops its source/evidence trace, leaving a decorative badge. Actionable findings must carry a trace, so the validator blocks promotion with finding_not_traced — broken-link and stale-example findings are actionable review items, not decorative badges.",
        input,
        expect: ExpectedFixture {
            promotion_state: "blocks_stable",
            expected_finding_kinds: vec!["finding_not_traced"],
        },
    }
}

fn print_json<T: Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
