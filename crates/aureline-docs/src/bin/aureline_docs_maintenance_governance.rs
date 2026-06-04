//! Headless emitter for the stable docs-maintenance governance packet.

use aureline_docs::{
    seeded_docs_maintenance_and_stale_example_governance_input,
    DocsMaintenanceGovernanceFindingKind, DocsMaintenanceGovernancePacket,
    DocsMaintenanceGovernancePacketInput, DocsMaintenanceGovernancePromotionState,
    DOCS_MAINTENANCE_GOVERNANCE_SCHEMA_VERSION,
};
use serde::Serialize;

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
        Some("fixture") => emit_fixture(args.get(1).map(String::as_str))?,
        Some("validate") => validate_packet()?,
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn emit_packet() -> Result<(), Box<dyn std::error::Error>> {
    let packet = DocsMaintenanceGovernancePacket::materialize(
        seeded_docs_maintenance_and_stale_example_governance_input(),
    );
    print_json(&packet)
}

fn emit_support_export() -> Result<(), Box<dyn std::error::Error>> {
    let packet = DocsMaintenanceGovernancePacket::materialize(
        seeded_docs_maintenance_and_stale_example_governance_input(),
    );
    print_json(&packet.support_export(
        "support-export:docs-maintenance-governance:stable-2026.06",
        "2026-06-04T17:10:00Z",
    ))
}

fn emit_fixture(name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let name = name.ok_or("fixture name is required")?;
    let fixture = match name {
        "baseline_stable" => baseline_stable_fixture(),
        "rendered_preview_claims_canonical_blocks_stable" => {
            rendered_preview_claims_canonical_fixture()
        }
        "suppression_loses_actor_blocks_review" => suppression_loses_actor_fixture(),
        "projection_drops_publish_boundary_blocks_stable" => {
            projection_drops_publish_boundary_fixture()
        }
        other => return Err(format!("unknown fixture: {other}").into()),
    };
    print_json(&fixture)
}

fn validate_packet() -> Result<(), Box<dyn std::error::Error>> {
    let packet = DocsMaintenanceGovernancePacket::materialize(
        seeded_docs_maintenance_and_stale_example_governance_input(),
    );
    if packet.promotion_state == DocsMaintenanceGovernancePromotionState::Stable
        && packet.validation_findings.is_empty()
    {
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
    input: DocsMaintenanceGovernancePacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Serialize)]
struct ExpectedFixture {
    promotion_state: &'static str,
    expected_finding_kinds: Vec<&'static str>,
}

const FIXTURE_RECORD_KIND: &str = "docs_maintenance_governance_case";

fn baseline_stable_fixture() -> Fixture {
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_MAINTENANCE_GOVERNANCE_SCHEMA_VERSION,
        case_name: "baseline_stable".to_owned(),
        scenario: "Baseline stable packet covers docs-render configs, evidence-backed suggestions, validation results, stale-example findings with governed suppression, and maintenance packets for README, changelog, onboarding, module docs, and support articles. Help/About, onboarding, migration, release notes, docs feedback export, support/community handoff, docs pack, and CLI projections preserve the same vocabulary.".to_owned(),
        input: seeded_docs_maintenance_and_stale_example_governance_input(),
        expect: ExpectedFixture {
            promotion_state: DocsMaintenanceGovernancePromotionState::Stable.as_str(),
            expected_finding_kinds: Vec::new(),
        },
    }
}

fn rendered_preview_claims_canonical_fixture() -> Fixture {
    let mut input = seeded_docs_maintenance_and_stale_example_governance_input();
    input.packet_id = "packet:docs-maintenance-governance:rendered-canonical".to_owned();
    for projection in &mut input.consumer_projections {
        projection.packet_id_ref = input.packet_id.clone();
    }
    if let Some(config) = input
        .render_configs
        .iter_mut()
        .find(|config| config.mode.renders_preview())
    {
        config.rendered_preview_not_canonical = false;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_MAINTENANCE_GOVERNANCE_SCHEMA_VERSION,
        case_name: "rendered_preview_claims_canonical_blocks_stable".to_owned(),
        scenario: "A rendered preview drops the non-canonical disclosure. The validator blocks stable promotion because rendered preview cannot masquerade as canonical source or release proof.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsMaintenanceGovernancePromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsMaintenanceGovernanceFindingKind::RenderedPreviewCanonicalized.as_str(),
            ],
        },
    }
}

fn suppression_loses_actor_fixture() -> Fixture {
    let mut input = seeded_docs_maintenance_and_stale_example_governance_input();
    input.packet_id = "packet:docs-maintenance-governance:suppression-loses-actor".to_owned();
    for projection in &mut input.consumer_projections {
        projection.packet_id_ref = input.packet_id.clone();
    }
    if let Some(finding) = input
        .stale_example_findings
        .iter_mut()
        .find(|finding| finding.suppression.is_some())
    {
        if let Some(suppression) = &mut finding.suppression {
            suppression.actor_ref.clear();
        }
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_MAINTENANCE_GOVERNANCE_SCHEMA_VERSION,
        case_name: "suppression_loses_actor_blocks_review".to_owned(),
        scenario: "A stale-example suppression loses its actor ref. The validator keeps the packet out of stable review because suppressions must carry actor, reason, expiry, and evidence refs.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsMaintenanceGovernancePromotionState::NeedsReview.as_str(),
            expected_finding_kinds: vec![
                DocsMaintenanceGovernanceFindingKind::StaleFindingGovernanceIncomplete.as_str(),
            ],
        },
    }
}

fn projection_drops_publish_boundary_fixture() -> Fixture {
    let mut input = seeded_docs_maintenance_and_stale_example_governance_input();
    input.packet_id = "packet:docs-maintenance-governance:projection-drops-boundary".to_owned();
    for projection in &mut input.consumer_projections {
        projection.packet_id_ref = input.packet_id.clone();
    }
    if let Some(projection) = input.consumer_projections.first_mut() {
        projection.preserves_publish_boundary_and_handoff = false;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_MAINTENANCE_GOVERNANCE_SCHEMA_VERSION,
        case_name: "projection_drops_publish_boundary_blocks_stable".to_owned(),
        scenario: "A consumer projection keeps packet refs but drops publish-boundary and browser-handoff state. The validator blocks stable promotion so Help/About, onboarding, release notes, exports, and support cannot collapse boundary truth into generic docs success.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsMaintenanceGovernancePromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsMaintenanceGovernanceFindingKind::ConsumerProjectionDroppedVocabulary.as_str(),
            ],
        },
    }
}
