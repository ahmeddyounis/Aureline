//! Headless emitter for the browser/provider-console handoff-objects packet.
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_browser_provider_console_handoff_objects -- packet
//! cargo run -q -p aureline-docs --bin aureline_docs_browser_provider_console_handoff_objects -- support-export
//! cargo run -q -p aureline-docs --bin aureline_docs_browser_provider_console_handoff_objects -- summary
//! cargo run -q -p aureline-docs --bin aureline_docs_browser_provider_console_handoff_objects -- fixture baseline_stable
//! cargo run -q -p aureline-docs --bin aureline_docs_browser_provider_console_handoff_objects -- validate
//! ```

use aureline_docs::{
    seeded_stable_browser_handoff_input, BrowserHandoffConsumerSurface, BrowserHandoffPacket,
    BrowserHandoffPacketInput, BrowserHandoffPromotionState, BrowserHandoffValidationKind,
    DocsContractBrowserHandoffPrivacyConsequence, HandoffPolicyPosture, HandoffSourceSurface,
    BROWSER_HANDOFF_OBJECTS_SCHEMA_VERSION,
};
use serde::Serialize;

/// Stable export id pinned for the checked-in support export.
const SUPPORT_EXPORT_ID: &str = "support-export:browser_provider_console_handoff_objects:001";
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
    let packet = BrowserHandoffPacket::materialize(seeded_stable_browser_handoff_input());
    print_json(&packet)
}

fn emit_support_export() -> Result<(), Box<dyn std::error::Error>> {
    let packet = BrowserHandoffPacket::materialize(seeded_stable_browser_handoff_input());
    let export = packet.support_export(SUPPORT_EXPORT_ID, SUPPORT_EXPORTED_AT);
    print_json(&export)
}

fn emit_summary() -> Result<(), Box<dyn std::error::Error>> {
    let packet = BrowserHandoffPacket::materialize(seeded_stable_browser_handoff_input());
    print!("{}", packet.render_markdown_summary());
    Ok(())
}

fn emit_fixture(name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let name = name.ok_or("fixture name is required")?;
    let fixture = match name {
        "baseline_stable" => baseline_fixture(),
        "hidden_context_share_blocks_stable" => hidden_context_share_fixture(),
        "ordinary_navigation_shares_context_blocks_stable" => {
            ordinary_navigation_shares_context_fixture()
        }
        "raw_browser_open_bypass_blocks_stable" => raw_browser_open_bypass_fixture(),
        "return_anchor_missing_blocks_stable" => return_anchor_missing_fixture(),
        "privacy_consequence_inconsistent_blocks_stable" => {
            privacy_consequence_inconsistent_fixture()
        }
        "exit_coverage_missing_blocks_stable" => exit_coverage_missing_fixture(),
        "history_drops_handoff_blocks_stable" => history_drops_handoff_fixture(),
        "blocked_handoff_presented_available_blocks_stable" => {
            blocked_handoff_presented_available_fixture()
        }
        "blocked_handoff_narrows_below_stable" => blocked_handoff_narrows_fixture(),
        "shared_context_blocked_narrows_below_stable" => shared_context_blocked_narrows_fixture(),
        other => return Err(format!("unknown fixture: {other}").into()),
    };
    print_json(&fixture)
}

fn validate_packet() -> Result<(), Box<dyn std::error::Error>> {
    let packet = BrowserHandoffPacket::materialize(seeded_stable_browser_handoff_input());
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
    input: BrowserHandoffPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Serialize)]
struct ExpectedFixture {
    promotion_state: &'static str,
    expected_finding_kinds: Vec<&'static str>,
}

const FIXTURE_RECORD_KIND: &str = "browser_provider_console_handoff_objects_case";

fn baseline_fixture() -> Fixture {
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: BROWSER_HANDOFF_OBJECTS_SCHEMA_VERSION,
        case_name: "baseline_stable".to_owned(),
        scenario: "Baseline stable packet routes every docs/help/AI/provider-console exit through one handoff object. Each handoff names its destination class, the reason in-product viewing was insufficient, the privacy consequence, the trust/policy posture, and a return anchor; no handoff leaks raw code, README, ADR, or prompt context; and the support-export and docs-history surfaces reconstruct every handoff.".to_owned(),
        input: seeded_stable_browser_handoff_input(),
        expect: ExpectedFixture {
            promotion_state: BrowserHandoffPromotionState::Stable.as_str(),
            expected_finding_kinds: Vec::new(),
        },
    }
}

fn hidden_context_share_fixture() -> Fixture {
    let mut input = relabel("hidden_context_share");
    if let Some(handoff) = input
        .handoffs
        .iter_mut()
        .find(|handoff| handoff.source_surface == HandoffSourceSurface::DocsBrowser)
    {
        handoff.shared_context.shares_raw_code_selection = true;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: BROWSER_HANDOFF_OBJECTS_SCHEMA_VERSION,
        case_name: "hidden_context_share_blocks_stable".to_owned(),
        scenario: "A docs-browser handoff would carry a raw code selection across the boundary. The validator blocks promotion because a handoff never silently exfiltrates raw code, README, ADR, or prompt context.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: BrowserHandoffPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                BrowserHandoffValidationKind::HiddenContextShareDetected.as_str(),
            ],
        },
    }
}

fn ordinary_navigation_shares_context_fixture() -> Fixture {
    let mut input = relabel("ordinary_navigation_shares_context");
    if let Some(handoff) = input
        .handoffs
        .iter_mut()
        .find(|handoff| handoff.source_surface == HandoffSourceSurface::DocsBrowser)
    {
        handoff.shared_context.shares_user_query_terms = true;
        handoff.privacy_consequence =
            DocsContractBrowserHandoffPrivacyConsequence::QueryTermsDisclosed;
        handoff.user_initiated = true;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: BROWSER_HANDOFF_OBJECTS_SCHEMA_VERSION,
        case_name: "ordinary_navigation_shares_context_blocks_stable".to_owned(),
        scenario: "A docs-browser handoff that is part of ordinary navigation shares the user's query terms. The validator blocks promotion because ordinary docs navigation must not share workspace or query context; query-term sharing is allowed only on an explicitly user-initiated handoff.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: BrowserHandoffPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                BrowserHandoffValidationKind::OrdinaryNavigationSharesContext.as_str(),
            ],
        },
    }
}

fn raw_browser_open_bypass_fixture() -> Fixture {
    let mut input = relabel("raw_browser_open_bypass");
    if let Some(handoff) = input
        .handoffs
        .iter_mut()
        .find(|handoff| handoff.source_surface == HandoffSourceSurface::ProviderConsolePivot)
    {
        handoff.routed_through_handoff_review = false;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: BROWSER_HANDOFF_OBJECTS_SCHEMA_VERSION,
        case_name: "raw_browser_open_bypass_blocks_stable".to_owned(),
        scenario: "A provider-console pivot opens without going through explicit handoff review. The validator blocks promotion because raw browser opens, provider-console pivots, and docs fallbacks may not bypass handoff review.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: BrowserHandoffPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                BrowserHandoffValidationKind::RawBrowserOpenBypass.as_str(),
            ],
        },
    }
}

fn return_anchor_missing_fixture() -> Fixture {
    let mut input = relabel("return_anchor_missing");
    if let Some(handoff) = input.handoffs.first_mut() {
        handoff.return_anchor.anchor_ref.clear();
        handoff.return_anchor.label.clear();
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: BROWSER_HANDOFF_OBJECTS_SCHEMA_VERSION,
        case_name: "return_anchor_missing_blocks_stable".to_owned(),
        scenario: "A handoff drops its return anchor. The validator blocks promotion because every handoff must keep a return anchor so the reader can get back to the governed surface.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: BrowserHandoffPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![BrowserHandoffValidationKind::ReturnAnchorMissing.as_str()],
        },
    }
}

fn privacy_consequence_inconsistent_fixture() -> Fixture {
    let mut input = relabel("privacy_consequence_inconsistent");
    if let Some(handoff) = input
        .handoffs
        .iter_mut()
        .find(|handoff| handoff.source_surface == HandoffSourceSurface::DocsBrowser)
    {
        // Claims no context shared, but the shared context still carries the
        // resolved destination ref.
        handoff.privacy_consequence = DocsContractBrowserHandoffPrivacyConsequence::NoContextShared;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: BROWSER_HANDOFF_OBJECTS_SCHEMA_VERSION,
        case_name: "privacy_consequence_inconsistent_blocks_stable".to_owned(),
        scenario: "A handoff declares `no_context_shared` while its shared-context object still carries the resolved destination ref. The validator blocks promotion because the declared privacy consequence must match what actually crosses the boundary.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: BrowserHandoffPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                BrowserHandoffValidationKind::PrivacyConsequenceInconsistent.as_str(),
            ],
        },
    }
}

fn exit_coverage_missing_fixture() -> Fixture {
    let mut input = relabel("exit_coverage_missing");
    input
        .handoffs
        .retain(|handoff| handoff.source_surface != HandoffSourceSurface::HelpAbout);
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: BROWSER_HANDOFF_OBJECTS_SCHEMA_VERSION,
        case_name: "exit_coverage_missing_blocks_stable".to_owned(),
        scenario: "The help/about exit loses its handoff object. The validator blocks promotion because every docs/help/AI/provider-console exit must route through a handoff object.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: BrowserHandoffPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![BrowserHandoffValidationKind::ExitCoverageMissing.as_str()],
        },
    }
}

fn history_drops_handoff_fixture() -> Fixture {
    let mut input = relabel("history_drops_handoff");
    if let Some(projection) = input
        .consumer_projections
        .iter_mut()
        .find(|projection| projection.surface == BrowserHandoffConsumerSurface::DocsHistory)
    {
        projection
            .handoff_id_refs
            .retain(|handoff_ref| handoff_ref != "handoff:docs_browser:tokio-spawn-anchor");
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: BROWSER_HANDOFF_OBJECTS_SCHEMA_VERSION,
        case_name: "history_drops_handoff_blocks_stable".to_owned(),
        scenario: "The reopened docs-history projection stops reconstructing one handoff. The validator blocks promotion because support-export and history surfaces must reconstruct every handoff rather than flattening one into ordinary navigation.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: BrowserHandoffPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                BrowserHandoffValidationKind::HistoryReconstructionDropsHandoff.as_str(),
            ],
        },
    }
}

fn blocked_handoff_presented_available_fixture() -> Fixture {
    let mut input = relabel("blocked_handoff_presented_available");
    if let Some(handoff) = input
        .handoffs
        .iter_mut()
        .find(|handoff| handoff.source_surface == HandoffSourceSurface::ProviderConsolePivot)
    {
        handoff.policy_posture = HandoffPolicyPosture::BlockedByPolicy;
        handoff.offered_as_actionable = true;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: BROWSER_HANDOFF_OBJECTS_SCHEMA_VERSION,
        case_name: "blocked_handoff_presented_available_blocks_stable".to_owned(),
        scenario: "A provider-console pivot is blocked by policy yet still offered as an actionable open. The validator blocks promotion because a policy-blocked or unavailable destination may not be presented as available.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: BrowserHandoffPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                BrowserHandoffValidationKind::BlockedHandoffPresentedAvailable.as_str(),
            ],
        },
    }
}

fn blocked_handoff_narrows_fixture() -> Fixture {
    let mut input = relabel("blocked_handoff_narrows");
    if let Some(handoff) = input
        .handoffs
        .iter_mut()
        .find(|handoff| handoff.source_surface == HandoffSourceSurface::ProviderConsolePivot)
    {
        handoff.policy_posture = HandoffPolicyPosture::BlockedByPolicy;
        handoff.offered_as_actionable = false;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: BROWSER_HANDOFF_OBJECTS_SCHEMA_VERSION,
        case_name: "blocked_handoff_narrows_below_stable".to_owned(),
        scenario: "A provider-console pivot is blocked by policy and honestly disclosed as blocked, not offered as actionable. The packet narrows below stable rather than blocking, because the handoff stays valid and attributable but cannot claim an available action.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: BrowserHandoffPromotionState::NarrowedBelowStable.as_str(),
            expected_finding_kinds: vec![
                BrowserHandoffValidationKind::HandoffUnavailableNarrowed.as_str(),
            ],
        },
    }
}

fn shared_context_blocked_narrows_fixture() -> Fixture {
    let mut input = relabel("shared_context_blocked_narrows");
    if let Some(handoff) = input
        .handoffs
        .iter_mut()
        .find(|handoff| handoff.source_surface == HandoffSourceSurface::AiAnswer)
    {
        // The product blocked the context share that would have crossed; nothing
        // crosses now, and the handoff discloses the blocked share.
        handoff.privacy_consequence =
            DocsContractBrowserHandoffPrivacyConsequence::SharedContextBlocked;
        handoff.shared_context = aureline_docs::SharedContext::NOTHING;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: BROWSER_HANDOFF_OBJECTS_SCHEMA_VERSION,
        case_name: "shared_context_blocked_narrows_below_stable".to_owned(),
        scenario: "An AI-answer handoff honestly blocks a context share that would have exceeded its qualified scope. The packet narrows below stable rather than blocking, because the share was prevented and disclosed.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: BrowserHandoffPromotionState::NarrowedBelowStable.as_str(),
            expected_finding_kinds: vec![
                BrowserHandoffValidationKind::SharedContextBlockedNarrowed.as_str(),
            ],
        },
    }
}

/// Reseeds the packet id with a per-fixture suffix and realigns each projection's
/// `packet_id_ref` so a failing fixture isolates the single invariant it
/// exercises.
fn relabel(suffix: &str) -> BrowserHandoffPacketInput {
    let mut input = seeded_stable_browser_handoff_input();
    let packet_id = format!("packet:browser_provider_console_handoff_objects:{suffix}");
    for projection in input.consumer_projections.iter_mut() {
        projection.packet_id_ref = packet_id.clone();
    }
    input.packet_id = packet_id;
    input
}
