//! Headless emitter for the docs-search symbol-link packet and its fixture
//! corpus.
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_search_link -- packet
//! cargo run -q -p aureline-docs --bin aureline_docs_search_link -- support-export
//! cargo run -q -p aureline-docs --bin aureline_docs_search_link -- summary
//! cargo run -q -p aureline-docs --bin aureline_docs_search_link -- fixture symbol_link_unresolved_narrowed
//! cargo run -q -p aureline-docs --bin aureline_docs_search_link -- validate
//! ```

use aureline_docs::{
    seeded_stable_docs_search_link_input, DocsSearchLinkDisclosureClass, DocsSearchLinkFindingKind,
    DocsSearchLinkFindingSeverity, DocsSearchLinkPacket, DocsSearchLinkPacketInput,
    DocsSearchLinkProjectVendorCue, DocsSearchLinkPromotionState, DocsSearchLinkRepairHook,
    DocsSearchLinkRepairHookKind, DocsSearchLinkResolutionClass, DocsSearchLinkReuseState,
    DocsSearchLinkSourceClass, DOCS_SEARCH_LINK_SCHEMA_VERSION,
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
    let packet = DocsSearchLinkPacket::materialize(seeded_stable_docs_search_link_input());
    print_json(&packet)
}

fn emit_support_export() -> Result<(), Box<dyn std::error::Error>> {
    let packet = DocsSearchLinkPacket::materialize(seeded_stable_docs_search_link_input());
    let export = packet.support_export(
        "support-export:docs_search_link:001",
        "2026-06-10T00:00:10Z",
    );
    print_json(&export)
}

fn emit_summary() {
    let packet = DocsSearchLinkPacket::materialize(seeded_stable_docs_search_link_input());
    print!("{}", packet.render_markdown_summary());
}

fn emit_fixture(name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let name = name.ok_or("fixture name is required")?;
    let fixture = match name {
        "symbol_link_unresolved_narrowed" => symbol_link_unresolved_narrowed_fixture(),
        "deep_link_drops_anchor_blocks_stable" => deep_link_drops_anchor_fixture(),
        "vendor_overlay_uncited_blocks_stable" => vendor_overlay_uncited_fixture(),
        other => return Err(format!("unknown fixture: {other}").into()),
    };
    print_json(&fixture)
}

fn validate_packet() {
    let packet = DocsSearchLinkPacket::materialize(seeded_stable_docs_search_link_input());
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
    input: DocsSearchLinkPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Serialize)]
struct ExpectedFixture {
    promotion_state: &'static str,
    expected_finding_kinds: Vec<&'static str>,
}

const FIXTURE_RECORD_KIND: &str = "docs_search_link_case";

fn retarget(input: &mut DocsSearchLinkPacketInput, packet_id: &str) {
    input.packet_id = packet_id.to_owned();
    for projection in input.consumer_projections.iter_mut() {
        projection.packet_id_ref = packet_id.to_owned();
    }
}

fn symbol_link_unresolved_narrowed_fixture() -> Fixture {
    let mut input = seeded_stable_docs_search_link_input();
    retarget(
        &mut input,
        "packet:m5:docs_search_link:symbol_link_unresolved_narrowed",
    );
    // A symbol page has no coverage yet: the card resolves to an honest
    // no-claim state that routes to support, and the disclosure narrows the
    // packet rather than hiding the row.
    if let Some(card) = input.symbol_cards.get_mut(1) {
        card.resolution_class = DocsSearchLinkResolutionClass::NoClaimYetSupportRouted;
        card.resolution_fallback_chain = vec![
            DocsSearchLinkResolutionClass::ExactSymbolMatch,
            DocsSearchLinkResolutionClass::PackageLevelGuideFallback,
            DocsSearchLinkResolutionClass::NoClaimYetSupportRouted,
        ];
        card.reuse_state = DocsSearchLinkReuseState::RefusedUncited;
        card.citation_anchor_refs.clear();
        card.repair_hook = Some(DocsSearchLinkRepairHook {
            hook_kind: DocsSearchLinkRepairHookKind::ContactSupport,
            hook_id: "hook:support:request-docs-coverage".to_owned(),
            display_label: "Request docs coverage".to_owned(),
        });
    }
    if let Some(disclosure) = input.resolution_disclosures.get_mut(0) {
        disclosure.disclosure_class = DocsSearchLinkDisclosureClass::NoClaimSupportRouted;
        disclosure.severity = DocsSearchLinkFindingSeverity::Narrowing;
        disclosure.summary =
            "no docs coverage exists yet for this symbol; the request is routed to support"
                .to_owned();
        disclosure.repair_hook = Some(DocsSearchLinkRepairHook {
            hook_kind: DocsSearchLinkRepairHookKind::ContactSupport,
            hook_id: "hook:support:request-docs-coverage".to_owned(),
            display_label: "Request docs coverage".to_owned(),
        });
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_SEARCH_LINK_SCHEMA_VERSION,
        case_name: "symbol_link_unresolved_narrowed".to_owned(),
        scenario: "A symbol has no docs coverage yet. The card resolves to an honest \
                   no_claim_yet_support_routed state with a repair hook, and its narrowing \
                   disclosure narrows the packet to narrowed_below_stable instead of hiding the \
                   row. The downgrade narrows the claim, it does not hide the lane."
            .to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsSearchLinkPromotionState::NarrowedBelowStable.as_str(),
            expected_finding_kinds: Vec::new(),
        },
    }
}

fn deep_link_drops_anchor_fixture() -> Fixture {
    let mut input = seeded_stable_docs_search_link_input();
    retarget(
        &mut input,
        "packet:m5:docs_search_link:deep_link_drops_anchor",
    );
    // A deep link drops its code anchor on export, breaking the
    // code-anchor-preserving guarantee.
    if let Some(link) = input.code_anchor_deep_links.get_mut(0) {
        link.preserves_anchor_across_export = false;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_SEARCH_LINK_SCHEMA_VERSION,
        case_name: "deep_link_drops_anchor_blocks_stable".to_owned(),
        scenario: "A code-anchor-preserving deep link reports that its anchor does not survive \
                   export, breaking the deep-link guarantee. The validator blocks promotion with \
                   deep_link_anchor_not_preserved."
            .to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsSearchLinkPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsSearchLinkFindingKind::DeepLinkAnchorNotPreserved.as_str()
            ],
        },
    }
}

fn vendor_overlay_uncited_fixture() -> Fixture {
    let mut input = seeded_stable_docs_search_link_input();
    retarget(
        &mut input,
        "packet:m5:docs_search_link:vendor_overlay_uncited",
    );
    // A vendor overlay row is presented as project-authoritative and drops its
    // browser handoff and citation, making the vendor docs look more
    // authoritative than proven.
    if let Some(card) = input.symbol_cards.get_mut(0) {
        card.source_class = DocsSearchLinkSourceClass::VendorProviderDocs;
        card.project_vs_vendor_cue =
            DocsSearchLinkProjectVendorCue::VendorProviderOverlayInspectOnly;
        card.browser_handoff_reason = None;
        card.destination_descriptor_ref = None;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_SEARCH_LINK_SCHEMA_VERSION,
        case_name: "vendor_overlay_uncited_blocks_stable".to_owned(),
        scenario: "A vendor_provider_docs overlay card is surfaced inspect-only but drops its \
                   browser handoff and destination descriptor, so the vendor overlay cannot be \
                   attributed. The validator blocks promotion with vendor_overlay_missing_handoff."
            .to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsSearchLinkPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsSearchLinkFindingKind::VendorOverlayMissingHandoff.as_str()
            ],
        },
    }
}
