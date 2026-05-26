//! Headless emitter for the stable docs-pack truth packet and its fixture
//! corpus.
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_pack_truth_packet -- packet
//! cargo run -q -p aureline-docs --bin aureline_docs_pack_truth_packet -- support-export
//! cargo run -q -p aureline-docs --bin aureline_docs_pack_truth_packet -- fixture baseline_stable
//! cargo run -q -p aureline-docs --bin aureline_docs_pack_truth_packet -- validate
//! ```

use aureline_docs::{
    seeded_stable_docs_pack_truth_packet_input, DocsPackFindingKind, DocsPackLocalAvailability,
    DocsPackPromotionState, DocsPackTruthPacket, DocsPackTruthPacketInput,
    StaleExampleFindingClass, DOCS_PACK_TRUTH_PACKET_SCHEMA_VERSION,
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
        Some("fixture") => emit_fixture(args.get(1).map(String::as_str))?,
        Some("validate") => validate_packet()?,
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn emit_packet() -> Result<(), Box<dyn std::error::Error>> {
    let packet = DocsPackTruthPacket::materialize(seeded_stable_docs_pack_truth_packet_input());
    print_json(&packet)
}

fn emit_support_export() -> Result<(), Box<dyn std::error::Error>> {
    let packet = DocsPackTruthPacket::materialize(seeded_stable_docs_pack_truth_packet_input());
    let export =
        packet.support_export("support-export:docs_pack_truth:001", "2026-05-26T12:00:10Z");
    print_json(&export)
}

fn emit_fixture(name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let name = name.ok_or("fixture name is required")?;
    let fixture = match name {
        "baseline_stable" => baseline_stable_fixture(),
        "offline_pack_loses_signer_identity_blocks_stable" => offline_loses_identity_fixture(),
        "nearby_version_dropped_collapses_stale_state_blocks_stable" => {
            nearby_version_collapsed_fixture()
        }
        "citation_set_bundles_raw_pack_blocks_stable" => citation_bundles_raw_fixture(),
        "stale_suppression_loses_attribution_blocks_stable" => {
            suppression_attribution_lost_fixture()
        }
        "consumer_projection_drops_render_mode_blocks_stable" => {
            projection_drops_render_mode_fixture()
        }
        "quarantined_finding_collapsed_blocks_stable" => quarantined_collapsed_fixture(),
        other => return Err(format!("unknown fixture: {other}").into()),
    };
    print_json(&fixture)
}

fn validate_packet() -> Result<(), Box<dyn std::error::Error>> {
    let packet = DocsPackTruthPacket::materialize(seeded_stable_docs_pack_truth_packet_input());
    if packet.promotion_state == DocsPackPromotionState::Stable
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
    input: DocsPackTruthPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Serialize)]
struct ExpectedFixture {
    promotion_state: &'static str,
    expected_finding_kinds: Vec<&'static str>,
}

const FIXTURE_RECORD_KIND: &str = "docs_pack_truth_stable_case";

fn baseline_stable_fixture() -> Fixture {
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_PACK_TRUTH_PACKET_SCHEMA_VERSION,
        case_name: "baseline_stable".to_owned(),
        scenario: "Baseline stable posture: the packet binds every required source class \
                   (project_docs, generated_reference, mirrored_official_docs, \
                   curated_knowledge_pack, support_runbook, extension_docs_pack), every required \
                   local-availability posture (available_local, mirror_offline_pinned, \
                   not_installed, unavailable_disclosed, quarantined), every required render mode \
                   (rendered, syntax_checked, executed_locally, executed_remotely, mirrored_only, \
                   browser_handoff_only, not_rendered), keeps nearby_version / stale_example / \
                   quarantined_pack distinct, exports citation sets without raw pack bodies, \
                   and the ten required consumer projections preserve the packet verbatim."
            .to_owned(),
        input: seeded_stable_docs_pack_truth_packet_input(),
        expect: ExpectedFixture {
            promotion_state: DocsPackPromotionState::Stable.as_str(),
            expected_finding_kinds: Vec::new(),
        },
    }
}

fn offline_loses_identity_fixture() -> Fixture {
    let mut input = seeded_stable_docs_pack_truth_packet_input();
    input.packet_id = "packet:m4:docs_pack_truth:offline_signer_lost".to_owned();
    for projection in input.consumer_projections.iter_mut() {
        projection.packet_id_ref = input.packet_id.clone();
    }
    for manifest in input.manifests.iter_mut() {
        if matches!(
            manifest.local_availability,
            DocsPackLocalAvailability::NotInstalled
                | DocsPackLocalAvailability::UnavailableDisclosed
        ) {
            manifest.signing.signing_authority_ref = String::new();
        }
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_PACK_TRUTH_PACKET_SCHEMA_VERSION,
        case_name: "offline_pack_loses_signer_identity_blocks_stable".to_owned(),
        scenario: "A pack with local content unavailable drops its signing-authority ref. \
                   The validator blocks promotion because the manifest is incomplete and \
                   signer / channel / mirror-source identity must stay attributable even when \
                   content is unavailable locally."
            .to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsPackPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![DocsPackFindingKind::PackManifestIncomplete.as_str()],
        },
    }
}

fn nearby_version_collapsed_fixture() -> Fixture {
    let mut input = seeded_stable_docs_pack_truth_packet_input();
    input.packet_id = "packet:m4:docs_pack_truth:nearby_version_collapsed".to_owned();
    for projection in input.consumer_projections.iter_mut() {
        projection.packet_id_ref = input.packet_id.clone();
    }
    for finding in input.stale_example_findings.iter_mut() {
        if finding.finding_class == StaleExampleFindingClass::NearbyVersion {
            finding.nearby_version_ref = None;
        }
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_PACK_TRUTH_PACKET_SCHEMA_VERSION,
        case_name: "nearby_version_dropped_collapses_stale_state_blocks_stable".to_owned(),
        scenario: "A nearby-version finding drops its nearby_version_ref and collapses into a \
                   generic stale warning, erasing the distinction between nearby_version, \
                   stale_example, and quarantined_pack. The validator blocks promotion with \
                   stale_state_collapsed."
            .to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsPackPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![DocsPackFindingKind::StaleStateCollapsed.as_str()],
        },
    }
}

fn citation_bundles_raw_fixture() -> Fixture {
    let mut input = seeded_stable_docs_pack_truth_packet_input();
    input.packet_id = "packet:m4:docs_pack_truth:citation_bundles_raw".to_owned();
    for projection in input.consumer_projections.iter_mut() {
        projection.packet_id_ref = input.packet_id.clone();
    }
    if let Some(set) = input.citation_sets.first_mut() {
        set.raw_pack_bodies_excluded = false;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_PACK_TRUTH_PACKET_SCHEMA_VERSION,
        case_name: "citation_set_bundles_raw_pack_blocks_stable".to_owned(),
        scenario: "A citation-set export bundles raw pack bodies by default, breaking the \
                   reference-only contract that lets AI evidence, onboarding/help, and \
                   support-export lanes consume citation sets without shipping whole docs packs. \
                   The validator blocks promotion with citation_set_bundles_raw_pack."
            .to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsPackPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![DocsPackFindingKind::CitationSetBundlesRawPack.as_str()],
        },
    }
}

fn suppression_attribution_lost_fixture() -> Fixture {
    let mut input = seeded_stable_docs_pack_truth_packet_input();
    input.packet_id = "packet:m4:docs_pack_truth:suppression_attribution_lost".to_owned();
    for projection in input.consumer_projections.iter_mut() {
        projection.packet_id_ref = input.packet_id.clone();
    }
    for finding in input.stale_example_findings.iter_mut() {
        if let Some(suppression) = finding.suppression.as_mut() {
            suppression.actor_ref = String::new();
        }
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_PACK_TRUTH_PACKET_SCHEMA_VERSION,
        case_name: "stale_suppression_loses_attribution_blocks_stable".to_owned(),
        scenario: "A stale-example suppression drops its actor ref, removing the audit trail \
                   that must survive export, mirror, and release-packet reuse. The validator \
                   blocks promotion with suppression_attribution_lost."
            .to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsPackPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![DocsPackFindingKind::SuppressionAttributionLost.as_str()],
        },
    }
}

fn projection_drops_render_mode_fixture() -> Fixture {
    let mut input = seeded_stable_docs_pack_truth_packet_input();
    input.packet_id = "packet:m4:docs_pack_truth:projection_drops_render_mode".to_owned();
    for projection in input.consumer_projections.iter_mut() {
        projection.packet_id_ref = input.packet_id.clone();
    }
    if let Some(projection) = input.consumer_projections.first_mut() {
        projection.preserves_render_mode = false;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_PACK_TRUTH_PACKET_SCHEMA_VERSION,
        case_name: "consumer_projection_drops_render_mode_blocks_stable".to_owned(),
        scenario: "A consumer projection sets preserves_render_mode = false, collapsing the \
                   rendered / syntax_checked / executed / mirrored / browser-handoff / \
                   not_rendered taxonomy into one generic success badge. The validator blocks \
                   promotion with render_mode_vocabulary_dropped and consumer_projection_drift."
            .to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsPackPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsPackFindingKind::RenderModeVocabularyDropped.as_str(),
                DocsPackFindingKind::ConsumerProjectionDrift.as_str(),
            ],
        },
    }
}

fn quarantined_collapsed_fixture() -> Fixture {
    let mut input = seeded_stable_docs_pack_truth_packet_input();
    input.packet_id = "packet:m4:docs_pack_truth:quarantined_collapsed".to_owned();
    for projection in input.consumer_projections.iter_mut() {
        projection.packet_id_ref = input.packet_id.clone();
    }
    for finding in input.stale_example_findings.iter_mut() {
        if finding.finding_class == StaleExampleFindingClass::QuarantinedPack {
            finding.pack_id_ref = "pack:project-docs:aureline-workspace".to_owned();
        }
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_PACK_TRUTH_PACKET_SCHEMA_VERSION,
        case_name: "quarantined_finding_collapsed_blocks_stable".to_owned(),
        scenario: "A quarantined-pack finding now references a publishable pack, collapsing the \
                   stale_example / quarantined_pack distinction. The validator blocks promotion \
                   with stale_state_collapsed."
            .to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsPackPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![DocsPackFindingKind::StaleStateCollapsed.as_str()],
        },
    }
}
