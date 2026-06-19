//! Headless emitter for the docs-evidence-handoff packet and its fixture corpus.
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_evidence_handoff -- packet
//! cargo run -q -p aureline-docs --bin aureline_docs_evidence_handoff -- support-export
//! cargo run -q -p aureline-docs --bin aureline_docs_evidence_handoff -- summary
//! cargo run -q -p aureline-docs --bin aureline_docs_evidence_handoff -- fixture mirror_offline_narrows
//! cargo run -q -p aureline-docs --bin aureline_docs_evidence_handoff -- validate
//! ```

use aureline_docs::{
    seeded_stable_docs_evidence_handoff_input, DocsEvidenceHandoffPacket,
    DocsEvidenceHandoffPacketInput, EvidenceBinding, EvidenceFreshness, EvidenceKind,
    EvidenceLocality, EvidenceProvenance, EvidenceRedactionState, EvidenceScope,
    EvidenceVersionMatch, HandoffDegradation, HandoffDegradationClass, HandoffFindingSeverity,
    MirrorOfflinePosture,
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
        DocsEvidenceHandoffPacket::materialize(seeded_stable_docs_evidence_handoff_input());
    print_json(&packet)
}

fn emit_support_export() -> Result<(), Box<dyn std::error::Error>> {
    let packet =
        DocsEvidenceHandoffPacket::materialize(seeded_stable_docs_evidence_handoff_input());
    let export = packet.support_export(
        "support-export:docs_evidence_handoff:001",
        "2026-06-12T00:00:10Z",
    );
    print_json(&export)
}

fn emit_summary() {
    let packet =
        DocsEvidenceHandoffPacket::materialize(seeded_stable_docs_evidence_handoff_input());
    print!("{}", packet.render_markdown_summary());
}

fn emit_fixture(name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let name = name.ok_or("fixture name is required")?;
    let fixture = match name {
        "mirror_offline_narrows" => mirror_offline_fixture(),
        "local_only_marked_export_safe_blocks_stable" => local_only_export_safe_fixture(),
        "untraced_change_blocks_stable" => untraced_change_fixture(),
        other => return Err(format!("unknown fixture: {other}").into()),
    };
    print_json(&fixture)
}

fn validate_packet() {
    let packet =
        DocsEvidenceHandoffPacket::materialize(seeded_stable_docs_evidence_handoff_input());
    if packet.is_clean_stable() {
        println!("ok");
    } else {
        for finding in &packet.handoff_findings {
            eprintln!("{}: {}", finding.finding_kind.as_str(), finding.summary);
        }
        std::process::exit(3);
    }
}

#[derive(Serialize)]
struct DocsEvidenceHandoffFixture {
    record_kind: &'static str,
    schema_version: u32,
    case_name: &'static str,
    scenario: &'static str,
    input: DocsEvidenceHandoffPacketInput,
    expect: ExpectedFixture,
}

#[derive(Serialize)]
struct ExpectedFixture {
    promotion_state: &'static str,
    expected_finding_kinds: Vec<&'static str>,
}

fn fixture_input(packet_id: &str) -> DocsEvidenceHandoffPacketInput {
    let mut input = seeded_stable_docs_evidence_handoff_input();
    input.packet_id = packet_id.to_owned();
    for projection in input.consumer_projections.iter_mut() {
        projection.packet_id_ref = packet_id.to_owned();
    }
    input
}

fn rebuild_export(input: &mut DocsEvidenceHandoffPacketInput) {
    input.export.rows = input
        .entries
        .iter()
        .map(|entry| {
            let mut row = input
                .export
                .rows
                .iter()
                .find(|r| r.entry_id_ref == entry.entry_id)
                .cloned()
                .expect("export row exists for entry");
            row.evidence_kinds = entry.evidence_kinds().into_iter().collect();
            row.binding_count = entry.bindings.len() as u32;
            row.change_kind = entry.change.change_kind;
            row.doc_ref = entry.change.doc_ref.clone();
            row.entry_scope = entry.entry_scope;
            row.export_safe = entry.is_export_safe();
            row.reopenable = entry.reopen.is_reopenable();
            row.cited = entry.all_bindings_cited();
            row
        })
        .collect();
}

fn mirror_offline_fixture() -> DocsEvidenceHandoffFixture {
    let mut input = fixture_input("packet:m5:docs_evidence_handoff:mirror_offline");
    // The docs mirror is offline; the handoff narrows but the entries stay visible.
    input.handoff_degradations.push(HandoffDegradation {
        degradation_class: HandoffDegradationClass::MirrorOfflineSnapshot,
        severity: HandoffFindingSeverity::Narrowing,
        summary:
            "the docs mirror is offline; the imported ops-pack binding is served from the last snapshot and the handoff is narrowed while every entry stays visible and reopenable"
                .to_owned(),
        entry_id_ref: Some("entry:help:offline_runbook_note".to_owned()),
        evidence_ref: Some("evidence:docs-evidence-handoff:mirror-offline".to_owned()),
    });
    DocsEvidenceHandoffFixture {
        record_kind: "docs_evidence_handoff_case",
        schema_version: 1,
        case_name: "mirror_offline_narrows",
        scenario: "The docs mirror is offline, so the imported ops-pack binding is served from the last snapshot. The narrowing degradation keeps every entry visible, reopenable, and source-linked, so the handoff narrows below Stable instead of hiding the evidence — the downgrade narrows the claim, it does not hide the causality.",
        input,
        expect: ExpectedFixture {
            promotion_state: "narrowed_below_stable",
            expected_finding_kinds: vec![],
        },
    }
}

fn local_only_export_safe_fixture() -> DocsEvidenceHandoffFixture {
    let mut input = fixture_input("packet:m5:docs_evidence_handoff:local_only_marked_export_safe");
    // The local-only help entry is marked export-safe while its bindings stay
    // local-only with local-only-redaction-required material.
    let entry = input
        .entries
        .iter_mut()
        .find(|e| e.entry_id == "entry:help:offline_runbook_note")
        .expect("local-only help entry present");
    entry.entry_scope = EvidenceScope::ExportSafeShared;
    // Mark the imported runbook binding export-safe even though it still requires
    // local-only redaction, while the maintainer note stays local-only.
    if let Some(binding) = entry
        .bindings
        .iter_mut()
        .find(|b| b.evidence_kind == EvidenceKind::SourceFile)
    {
        binding.scope = EvidenceScope::ExportSafeShared;
    }
    rebuild_export(&mut input);
    DocsEvidenceHandoffFixture {
        record_kind: "docs_evidence_handoff_case",
        schema_version: 1,
        case_name: "local_only_marked_export_safe_blocks_stable",
        scenario: "A local-only help entry whose bindings carry local-only-redaction-required material is marked export-safe. Scope may never be wider than its bindings, so the validator blocks promotion with entry_scope_wider_than_bindings and scope_redaction_inconsistent — local-only evidence is never silently widened to export.",
        input,
        expect: ExpectedFixture {
            promotion_state: "blocks_stable",
            expected_finding_kinds: vec![
                "entry_scope_wider_than_bindings",
                "scope_redaction_inconsistent",
            ],
        },
    }
}

fn untraced_change_fixture() -> DocsEvidenceHandoffFixture {
    let mut input = fixture_input("packet:m5:docs_evidence_handoff:untraced_change");
    // The changelog entry is reduced to a free-form human note with no concrete
    // typed evidence object.
    let entry = input
        .entries
        .iter_mut()
        .find(|e| e.entry_id == "entry:changelog:retry_backoff_release")
        .expect("changelog entry present");
    entry.bindings = vec![EvidenceBinding {
        binding_id: "binding:changelog:note_only".to_owned(),
        evidence_kind: EvidenceKind::HumanNote,
        target_ref: "note:maintainer:changelog-context".to_owned(),
        display_path: "Maintainer note → changelog".to_owned(),
        label: "a free-form changelog note with no concrete evidence".to_owned(),
        scope: EvidenceScope::ExportSafeShared,
        redaction_state: EvidenceRedactionState::MetadataSafe,
        provenance: EvidenceProvenance::FirstPartyVerified,
        freshness: EvidenceFreshness::WarmCached,
        version_match: EvidenceVersionMatch::ExactBuildMatch,
        locality: EvidenceLocality::Local,
        mirror_offline: MirrorOfflinePosture::OnlineLive,
        provenance_disclosure_note: "a free-form maintainer note".to_owned(),
        open_evidence_ref: "open-note:maintainer:changelog-context".to_owned(),
        detail: "the changelog entry is described only by a free-form note".to_owned(),
        cited: true,
        citation_ref: None,
    }];
    rebuild_export(&mut input);
    DocsEvidenceHandoffFixture {
        record_kind: "docs_evidence_handoff_case",
        schema_version: 1,
        case_name: "untraced_change_blocks_stable",
        scenario: "A changelog entry is described only by a free-form human note, with no file, symbol, contract, run, or release behind it. A docs change must be traceable to concrete typed evidence rather than narrative alone, so the validator blocks promotion with change_not_concretely_traced (and required_evidence_kind_missing once the release object and test run are gone).",
        input,
        expect: ExpectedFixture {
            promotion_state: "blocks_stable",
            expected_finding_kinds: vec!["change_not_concretely_traced"],
        },
    }
}

fn print_json<T: Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
