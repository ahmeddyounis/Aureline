//! Headless emitter for the docs-pack manager packet.

use aureline_docs::{
    seeded_stable_docs_pack_manager_input, DocsPackImportOrigin, DocsPackLifecycleFlow,
    DocsPackManagerAction, DocsPackManagerFindingKind, DocsPackManagerPacket,
    DocsPackManagerPacketInput, DocsPackManagerPromotionState, DOCS_PACK_MANAGER_SCHEMA_VERSION,
};
use serde::Serialize;

/// Stable export id pinned for the checked-in support export.
const SUPPORT_EXPORT_ID: &str = "support-export:docs_pack_manager:001";
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
    let packet = DocsPackManagerPacket::materialize(seeded_stable_docs_pack_manager_input());
    print_json(&packet)
}

fn emit_support_export() -> Result<(), Box<dyn std::error::Error>> {
    let packet = DocsPackManagerPacket::materialize(seeded_stable_docs_pack_manager_input());
    let export = packet.support_export(SUPPORT_EXPORT_ID, SUPPORT_EXPORTED_AT);
    print_json(&export)
}

fn emit_summary() -> Result<(), Box<dyn std::error::Error>> {
    let packet = DocsPackManagerPacket::materialize(seeded_stable_docs_pack_manager_input());
    print!("{}", packet.render_markdown_summary());
    Ok(())
}

fn emit_fixture(name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let name = name.ok_or("fixture name is required")?;
    let fixture = match name {
        "baseline_stable" => baseline_fixture(),
        "manager_row_hides_mirror_source_blocks_stable" => hides_mirror_source_fixture(),
        "unavailable_payload_hidden_blocks_stable" => unavailable_payload_hidden_fixture(),
        "mirror_offline_degraded_to_cache_blocks_stable" => mirror_offline_degraded_fixture(),
        "import_export_continuity_lost_blocks_stable" => continuity_lost_fixture(),
        "manager_action_reason_missing_blocks_stable" => action_reason_missing_fixture(),
        "lifecycle_flow_origin_mismatch_blocks_stable" => lifecycle_origin_mismatch_fixture(),
        "profile_projection_drops_truth_blocks_stable" => projection_drift_fixture(),
        other => return Err(format!("unknown fixture: {other}").into()),
    };
    print_json(&fixture)
}

fn validate_packet() -> Result<(), Box<dyn std::error::Error>> {
    let packet = DocsPackManagerPacket::materialize(seeded_stable_docs_pack_manager_input());
    if packet.promotion_state == DocsPackManagerPromotionState::Stable
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
    input: DocsPackManagerPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Serialize)]
struct ExpectedFixture {
    promotion_state: &'static str,
    expected_finding_kinds: Vec<&'static str>,
}

const FIXTURE_RECORD_KIND: &str = "docs_pack_manager_case";

fn baseline_fixture() -> Fixture {
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_PACK_MANAGER_SCHEMA_VERSION,
        case_name: "baseline_stable".to_owned(),
        scenario: "Baseline stable packet proves docs-pack manager rows over the canonical manifest carry signer, channel, mirror source, version range, refresh state, and pin/offline posture across local-only, mirrored, managed, and air-gapped flows, with pin/refresh/remove/mirror-source/offline actions and import/export continuity preserved on every claimed M5 manager profile.".to_owned(),
        input: seeded_stable_docs_pack_manager_input(),
        expect: ExpectedFixture {
            promotion_state: DocsPackManagerPromotionState::Stable.as_str(),
            expected_finding_kinds: Vec::new(),
        },
    }
}

fn hides_mirror_source_fixture() -> Fixture {
    let mut input = seeded_stable_docs_pack_manager_input();
    input.packet_id = "packet:docs_pack_manager:hides_mirror_source".to_owned();
    if let Some(row) = input
        .rows
        .iter_mut()
        .find(|row| row.row_id == "manager-row:std-mirror")
    {
        row.shows_mirror_source = false;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_PACK_MANAGER_SCHEMA_VERSION,
        case_name: "manager_row_hides_mirror_source_blocks_stable".to_owned(),
        scenario: "A mirrored docs-pack row stops showing its mirror source. The validator blocks promotion because every manager row must keep signer/channel/mirror source visible.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsPackManagerPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsPackManagerFindingKind::ManagerRowHidesManifestTruth.as_str(),
            ],
        },
    }
}

fn unavailable_payload_hidden_fixture() -> Fixture {
    let mut input = seeded_stable_docs_pack_manager_input();
    input.packet_id = "packet:docs_pack_manager:unavailable_hidden".to_owned();
    if let Some(row) = input
        .rows
        .iter_mut()
        .find(|row| row.row_id == "manager-row:extension-pack-unavailable")
    {
        row.unavailable_payload_disclosed = false;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_PACK_MANAGER_SCHEMA_VERSION,
        case_name: "unavailable_payload_hidden_blocks_stable".to_owned(),
        scenario: "A pack whose payload is unavailable locally stops disclosing the unavailable payload state. The validator blocks promotion because the manager must not hide unavailable payload or signature state.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsPackManagerPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsPackManagerFindingKind::UnavailablePayloadHidden.as_str(),
            ],
        },
    }
}

fn mirror_offline_degraded_fixture() -> Fixture {
    let mut input = seeded_stable_docs_pack_manager_input();
    input.packet_id = "packet:docs_pack_manager:offline_degraded".to_owned();
    if let Some(row) = input
        .rows
        .iter_mut()
        .find(|row| row.row_id == "manager-row:support-runbook")
    {
        row.degraded_to_opaque_cache = true;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_PACK_MANAGER_SCHEMA_VERSION,
        case_name: "mirror_offline_degraded_to_cache_blocks_stable".to_owned(),
        scenario: "An air-gapped pack collapses into an opaque cache badge. The validator blocks promotion because mirror and offline flows must stay first-class and never degrade into opaque cache or browser-only fallback wording.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsPackManagerPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![DocsPackManagerFindingKind::MirrorOfflineDegraded.as_str()],
        },
    }
}

fn continuity_lost_fixture() -> Fixture {
    let mut input = seeded_stable_docs_pack_manager_input();
    input.packet_id = "packet:docs_pack_manager:continuity_lost".to_owned();
    if let Some(row) = input.rows.first_mut() {
        row.import_export_continuity.preserves_identity_on_export = false;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_PACK_MANAGER_SCHEMA_VERSION,
        case_name: "import_export_continuity_lost_blocks_stable".to_owned(),
        scenario: "A docs pack stops preserving its identity on export. The validator blocks promotion because import/export must retain docs-pack identity and lifecycle state rather than flattening into generic cache metadata.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsPackManagerPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsPackManagerFindingKind::ImportExportContinuityLost.as_str(),
            ],
        },
    }
}

fn action_reason_missing_fixture() -> Fixture {
    let mut input = seeded_stable_docs_pack_manager_input();
    input.packet_id = "packet:docs_pack_manager:action_reason_missing".to_owned();
    if let Some(row) = input
        .rows
        .iter_mut()
        .find(|row| row.row_id == "manager-row:support-runbook")
    {
        if let Some(action) = row
            .actions
            .iter_mut()
            .find(|state| state.action == DocsPackManagerAction::ChangeMirrorSource)
        {
            action.disabled_reason = None;
        }
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_PACK_MANAGER_SCHEMA_VERSION,
        case_name: "manager_action_reason_missing_blocks_stable".to_owned(),
        scenario: "A disabled manager action drops its disclosed reason. The validator blocks promotion because a disabled or not-applicable action must always name why it is unavailable.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsPackManagerPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsPackManagerFindingKind::ManagerActionReasonMissing.as_str(),
            ],
        },
    }
}

fn lifecycle_origin_mismatch_fixture() -> Fixture {
    let mut input = seeded_stable_docs_pack_manager_input();
    input.packet_id = "packet:docs_pack_manager:lifecycle_origin_mismatch".to_owned();
    if let Some(row) = input
        .rows
        .iter_mut()
        .find(|row| row.lifecycle_flow == DocsPackLifecycleFlow::AirGapped)
    {
        row.import_export_continuity.import_origin = DocsPackImportOrigin::FreshlyInstalled;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_PACK_MANAGER_SCHEMA_VERSION,
        case_name: "lifecycle_flow_origin_mismatch_blocks_stable".to_owned(),
        scenario: "An air-gapped pack claims a fresh-install import origin. The validator blocks promotion because a pack's lifecycle flow and its import provenance must stay consistent.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsPackManagerPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![
                DocsPackManagerFindingKind::LifecycleFlowOriginMismatch.as_str(),
            ],
        },
    }
}

fn projection_drift_fixture() -> Fixture {
    let mut input = seeded_stable_docs_pack_manager_input();
    input.packet_id = "packet:docs_pack_manager:projection_drift".to_owned();
    if let Some(projection) = input.profile_projections.first_mut() {
        projection.preserves_import_export_continuity = false;
    }
    Fixture {
        record_kind: FIXTURE_RECORD_KIND,
        schema_version: DOCS_PACK_MANAGER_SCHEMA_VERSION,
        case_name: "profile_projection_drops_truth_blocks_stable".to_owned(),
        scenario: "A manager-profile projection stops preserving import/export continuity. The validator blocks promotion because every claimed profile must reuse the manager packet without dropping truth.".to_owned(),
        input,
        expect: ExpectedFixture {
            promotion_state: DocsPackManagerPromotionState::BlocksStable.as_str(),
            expected_finding_kinds: vec![DocsPackManagerFindingKind::ProfileProjectionDrift.as_str()],
        },
    }
}
