//! Headless emitter for the docs version-freshness findings packet.
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_version_freshness_findings -- packet
//! cargo run -q -p aureline-docs --bin aureline_docs_version_freshness_findings -- support-export
//! cargo run -q -p aureline-docs --bin aureline_docs_version_freshness_findings -- summary
//! cargo run -q -p aureline-docs --bin aureline_docs_version_freshness_findings -- fixture baseline_stable
//! cargo run -q -p aureline-docs --bin aureline_docs_version_freshness_findings -- validate
//! ```

use aureline_docs::{
    seeded_stable_docs_version_freshness_input, DocsVersionFreshnessConfidence,
    DocsVersionFreshnessFindingSeverity, DocsVersionFreshnessPacket,
    DocsVersionFreshnessPacketInput, DocsVersionFreshnessPromotionState, DocsVersionFreshnessState,
    DocsVersionFreshnessValidationKind, DOCS_VERSION_FRESHNESS_SCHEMA_VERSION,
};
use serde::Serialize;

/// Stable export id pinned for the checked-in support export.
const SUPPORT_EXPORT_ID: &str = "support-export:docs_version_freshness_findings:001";
/// Stable export timestamp pinned for the checked-in support export.
const SUPPORT_EXPORTED_AT: &str = "2026-06-26T00:00:00Z";

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.first().map(String::as_str) {
        Some("packet") | None => emit_packet()?,
        Some("support-export") => emit_support_export()?,
        Some("summary") => emit_summary()?,
        Some("fixture") => emit_fixture(args.get(1).map(String::as_str))?,
        Some("validate") => validate_packet()?,
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn emit_packet() -> Result<(), Box<dyn std::error::Error>> {
    let packet =
        DocsVersionFreshnessPacket::materialize(seeded_stable_docs_version_freshness_input());
    print_json(&packet)
}

fn emit_support_export() -> Result<(), Box<dyn std::error::Error>> {
    let packet =
        DocsVersionFreshnessPacket::materialize(seeded_stable_docs_version_freshness_input());
    let export = packet.support_export(SUPPORT_EXPORT_ID, SUPPORT_EXPORTED_AT);
    print_json(&export)
}

fn emit_summary() -> Result<(), Box<dyn std::error::Error>> {
    let packet =
        DocsVersionFreshnessPacket::materialize(seeded_stable_docs_version_freshness_input());
    print!("{}", packet.render_markdown_summary());
    Ok(())
}

fn emit_fixture(name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let name = name.ok_or("fixture name is required")?;
    let fixture = match name {
        "baseline_stable" => baseline_fixture(),
        "cached_shares_exact_confidence_blocks_stable" => cached_shares_exact_confidence_fixture(),
        "version_mismatch_hidden_blocks_stable" => version_mismatch_hidden_fixture(),
        "broken_link_finding_blocks_stable" => broken_link_finding_blocks_fixture(),
        "finding_actions_dropped_blocks_stable" => finding_actions_dropped_fixture(),
        "finding_orphan_blocks_stable" => finding_orphan_fixture(),
        "state_distinction_collapsed_blocks_stable" => state_distinction_collapsed_fixture(),
        "vocabulary_coverage_incomplete_blocks_stable" => vocabulary_coverage_incomplete_fixture(),
        "policy_blocked_reason_missing_blocks_stable" => policy_blocked_reason_missing_fixture(),
        other => return Err(format!("unknown fixture: {other}").into()),
    };
    print_json(&fixture)
}

fn validate_packet() -> Result<(), Box<dyn std::error::Error>> {
    let packet =
        DocsVersionFreshnessPacket::materialize(seeded_stable_docs_version_freshness_input());
    if packet.is_clean_stable() {
        println!("ok");
        Ok(())
    } else {
        for finding in &packet.validation_findings {
            eprintln!("{}: {}", finding.finding_kind.as_str(), finding.summary);
        }
        std::process::exit(3);
    }
}

fn print_json<T: Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

#[derive(Debug, Serialize)]
struct Fixture {
    record_kind: &'static str,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: DocsVersionFreshnessPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Serialize)]
struct ExpectedFixture {
    promotion_state: &'static str,
    expected_finding_kinds: Vec<&'static str>,
}

const FIXTURE_RECORD_KIND: &str = "docs_version_freshness_findings_case";

fn baseline_fixture() -> Fixture {
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_VERSION_FRESHNESS_SCHEMA_VERSION,
        case_name: "baseline_stable".to_owned(),
        scenario: "Baseline stable packet proves the controlled version/freshness vocabulary (exact, nearby, project_specific, mirrored, cached, stale, policy_blocked, browser_handoff_required) renders with distinct badges and distinct confidence treatments, carries stale-example and broken-link findings with suppress/compare/open-current-source actions, and is reused without drift by result rows, symbol-linked reference cards, docs pages, AI citation chips, onboarding/glossary surfaces, and support exports.".to_owned(),
        input: seeded_stable_docs_version_freshness_input(),
        expect: ExpectedFixture {
            promotion_state: DocsVersionFreshnessPromotionState::Stable.as_str(),
            expected_finding_kinds: Vec::new(),
        },
    }
}

fn cached_shares_exact_confidence_fixture() -> Fixture {
    let mut input = seeded_stable_docs_version_freshness_input();
    input.packet_id = "packet:docs_version_freshness_findings:cached_shares_exact".to_owned();
    repoint_projections(&mut input);
    if let Some(card) = input
        .cards
        .iter_mut()
        .find(|card| card.state == DocsVersionFreshnessState::Cached)
    {
        card.confidence = DocsVersionFreshnessConfidence::CurrentExact;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_VERSION_FRESHNESS_SCHEMA_VERSION,
        case_name: "cached_shares_exact_confidence_blocks_stable".to_owned(),
        scenario: "A cached card claims the exact-current confidence treatment. The validator blocks promotion because cached or nearby-version documentation must never render with the same confidence as exact current documentation.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsVersionFreshnessPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsVersionFreshnessValidationKind::CardConfidenceCollapsed.as_str(),
            ],
        },
    }
}

fn version_mismatch_hidden_fixture() -> Fixture {
    let mut input = seeded_stable_docs_version_freshness_input();
    input.packet_id = "packet:docs_version_freshness_findings:version_mismatch_hidden".to_owned();
    repoint_projections(&mut input);
    if let Some(card) = input
        .cards
        .iter_mut()
        .find(|card| card.state == DocsVersionFreshnessState::Nearby)
    {
        card.version_disclosure = None;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_VERSION_FRESHNESS_SCHEMA_VERSION,
        case_name: "version_mismatch_hidden_blocks_stable".to_owned(),
        scenario: "A nearby-version card hides the active and viewed versions. The validator blocks promotion because a version-mismatch surface must show both the active code/package version and the viewed docs version.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsVersionFreshnessPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsVersionFreshnessValidationKind::VersionDisclosureMissing.as_str(),
            ],
        },
    }
}

fn broken_link_finding_blocks_fixture() -> Fixture {
    let mut input = seeded_stable_docs_version_freshness_input();
    input.packet_id = "packet:docs_version_freshness_findings:broken_link_blocks".to_owned();
    repoint_projections(&mut input);
    if let Some(finding) = input.findings.first_mut() {
        finding.severity = DocsVersionFreshnessFindingSeverity::Blocking;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_VERSION_FRESHNESS_SCHEMA_VERSION,
        case_name: "broken_link_finding_blocks_stable".to_owned(),
        scenario: "A stale-example finding is raised to blocking severity. The packet blocks promotion, proving stale-example and broken-link findings are actionable review items that can gate the stable claim while keeping stable object identity.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsVersionFreshnessPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: Vec::new(),
        },
    }
}

fn finding_actions_dropped_fixture() -> Fixture {
    let mut input = seeded_stable_docs_version_freshness_input();
    input.packet_id = "packet:docs_version_freshness_findings:finding_actions_dropped".to_owned();
    repoint_projections(&mut input);
    if let Some(finding) = input.findings.first_mut() {
        finding.actions.compare_ref = String::new();
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_VERSION_FRESHNESS_SCHEMA_VERSION,
        case_name: "finding_actions_dropped_blocks_stable".to_owned(),
        scenario: "A finding drops its compare action. The validator blocks promotion because every finding must preserve its suppress/compare/open-current-source actions.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsVersionFreshnessPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsVersionFreshnessValidationKind::FindingActionsMissing.as_str(),
            ],
        },
    }
}

fn finding_orphan_fixture() -> Fixture {
    let mut input = seeded_stable_docs_version_freshness_input();
    input.packet_id = "packet:docs_version_freshness_findings:finding_orphan".to_owned();
    repoint_projections(&mut input);
    if let Some(finding) = input.findings.first_mut() {
        finding.card_id_ref = "card:does-not-exist".to_owned();
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_VERSION_FRESHNESS_SCHEMA_VERSION,
        case_name: "finding_orphan_blocks_stable".to_owned(),
        scenario: "A finding references a card absent from the packet. The validator blocks promotion because every finding must attach to a real card with stable object identity.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsVersionFreshnessPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![DocsVersionFreshnessValidationKind::FindingOrphan.as_str()],
        },
    }
}

fn state_distinction_collapsed_fixture() -> Fixture {
    let mut input = seeded_stable_docs_version_freshness_input();
    input.packet_id =
        "packet:docs_version_freshness_findings:state_distinction_collapsed".to_owned();
    repoint_projections(&mut input);
    if let Some(projection) = input.consumer_projections.first_mut() {
        projection.preserves_state_distinctions = false;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_VERSION_FRESHNESS_SCHEMA_VERSION,
        case_name: "state_distinction_collapsed_blocks_stable".to_owned(),
        scenario: "A consumer surface collapses the distinct state badges into one generic info badge. The validator blocks promotion because browser_handoff_required, cached, mirrored, and project_specific must never collapse into one badge.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsVersionFreshnessPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsVersionFreshnessValidationKind::StateDistinctionCollapsed.as_str(),
            ],
        },
    }
}

fn vocabulary_coverage_incomplete_fixture() -> Fixture {
    let mut input = seeded_stable_docs_version_freshness_input();
    input.packet_id = "packet:docs_version_freshness_findings:vocabulary_incomplete".to_owned();
    repoint_projections(&mut input);
    input
        .cards
        .retain(|card| card.state != DocsVersionFreshnessState::BrowserHandoffRequired);
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_VERSION_FRESHNESS_SCHEMA_VERSION,
        case_name: "vocabulary_coverage_incomplete_blocks_stable".to_owned(),
        scenario: "The cards drop the browser_handoff_required state. The validator blocks promotion because the controlled vocabulary must stay whole and every state must be a real, reachable badge.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsVersionFreshnessPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsVersionFreshnessValidationKind::VocabularyCoverageMissing.as_str(),
            ],
        },
    }
}

fn policy_blocked_reason_missing_fixture() -> Fixture {
    let mut input = seeded_stable_docs_version_freshness_input();
    input.packet_id = "packet:docs_version_freshness_findings:policy_reason_missing".to_owned();
    repoint_projections(&mut input);
    if let Some(card) = input
        .cards
        .iter_mut()
        .find(|card| card.state == DocsVersionFreshnessState::PolicyBlocked)
    {
        card.state_reason = None;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_VERSION_FRESHNESS_SCHEMA_VERSION,
        case_name: "policy_blocked_reason_missing_blocks_stable".to_owned(),
        scenario: "A policy-blocked card drops its reason. The validator blocks promotion because a policy-blocked or browser-handoff-required state must name why the answer is not rendered inline.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsVersionFreshnessPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsVersionFreshnessValidationKind::StateReasonMissing.as_str(),
            ],
        },
    }
}

/// Realigns each projection's `packet_id_ref` with the fixture's packet id so a
/// failing fixture isolates the single invariant it exercises.
fn repoint_projections(input: &mut DocsVersionFreshnessPacketInput) {
    let packet_id = input.packet_id.clone();
    for projection in input.consumer_projections.iter_mut() {
        projection.packet_id_ref = packet_id.clone();
    }
}
