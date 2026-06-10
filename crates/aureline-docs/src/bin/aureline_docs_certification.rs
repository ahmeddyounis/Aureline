//! Headless emitter for the M5 docs/code-understanding certification packet.
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_certification -- packet
//! cargo run -q -p aureline-docs --bin aureline_docs_certification -- summary
//! cargo run -q -p aureline-docs --bin aureline_docs_certification -- fixture recall_freshness_expired_narrows
//! cargo run -q -p aureline-docs --bin aureline_docs_certification -- validate
//! cargo run -q -p aureline-docs --bin aureline_docs_certification -- blockers
//! ```

use aureline_docs::{
    seeded_stable_certification_input, CertificationDowngradeTrigger, CertificationPacket,
    CertificationQualificationClass, CertificationVerdict, CertifiedSurfaceLane,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.first().map(String::as_str) {
        Some("packet") | Some("support-export") | None => emit_packet(),
        Some("summary") => emit_summary(),
        Some("fixture") => emit_fixture(args.get(1).map(String::as_str))?,
        Some("validate") => validate_packet(),
        Some("narrowed") => emit_narrowed(),
        Some("blockers") => emit_blockers(),
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn seeded_packet() -> CertificationPacket {
    CertificationPacket::new(seeded_stable_certification_input())
}

fn emit_packet() {
    println!("{}", seeded_packet().export_safe_json());
}

fn emit_summary() {
    print!("{}", seeded_packet().render_markdown_summary());
}

fn validate_packet() {
    let violations = seeded_packet().validate();
    if violations.is_empty() {
        println!("ok");
    } else {
        for violation in &violations {
            eprintln!("{}", violation.as_str());
        }
        std::process::exit(3);
    }
}

fn emit_narrowed() {
    for lane in seeded_packet().narrowed_surfaces() {
        println!("{}", lane.as_str());
    }
}

fn emit_blockers() {
    for lane in seeded_packet().promotion_blockers() {
        println!("{}", lane.as_str());
    }
}

fn emit_fixture(name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let name = name.ok_or("fixture name is required")?;
    let packet = match name {
        "recall_freshness_expired_narrows" => recall_narrowed_fixture(),
        "browser_scope_expansion_blocked" => browser_blocked_fixture(),
        other => return Err(format!("unknown fixture: {other}").into()),
    };
    let violations = packet.validate();
    if !violations.is_empty() {
        return Err(format!(
            "fixture {name} does not validate: {}",
            violations
                .iter()
                .map(|v| v.as_str())
                .collect::<Vec<_>>()
                .join(",")
        )
        .into());
    }
    println!("{}", packet.export_safe_json());
    Ok(())
}

/// Recall lanes narrow to Beta because the pinned mirror's freshness expired.
fn recall_narrowed_fixture() -> CertificationPacket {
    let mut input = seeded_stable_certification_input();
    input.packet_id = "m5-docs-certification:recall-narrowed:0002".to_owned();
    input.certification_label =
        "M5 Docs and Code-Understanding Certification — Recall Narrowed".to_owned();
    for row in input.surface_rows.iter_mut() {
        if matches!(
            row.lane,
            CertifiedSurfaceLane::DocsPackRecall | CertifiedSurfaceLane::SemanticRecall
        ) {
            row.qualification = CertificationQualificationClass::Beta;
            row.verdict = CertificationVerdict::NarrowedToQualified;
            if !row
                .downgrade_triggers
                .contains(&CertificationDowngradeTrigger::FreshnessExpired)
            {
                row.downgrade_triggers
                    .push(CertificationDowngradeTrigger::FreshnessExpired);
            }
            row.scope_summary = format!(
                "{} — narrowed to Beta: the pinned, signed mirror freshness window expired, so recall falls back to last-known-good with explicit freshness labels",
                row.scope_summary
            );
        }
    }
    CertificationPacket::new(input)
}

/// The scoped browser surface is blocked after an unqualified scope expansion.
fn browser_blocked_fixture() -> CertificationPacket {
    let mut input = seeded_stable_certification_input();
    input.packet_id = "m5-docs-certification:browser-blocked:0003".to_owned();
    input.certification_label =
        "M5 Docs and Code-Understanding Certification — Browser Held".to_owned();
    for row in input.surface_rows.iter_mut() {
        if row.lane == CertifiedSurfaceLane::ScopedBrowserSurface {
            row.qualification = CertificationQualificationClass::Held;
            row.verdict = CertificationVerdict::BlockedUnderqualified;
            row.scope_summary =
                "Held and blocked from promotion: a scope expansion beyond the qualified docs/review boundary was detected; no browser handoff is offered while held".to_owned();
            if !row
                .downgrade_triggers
                .contains(&CertificationDowngradeTrigger::ScopeExpansionUnqualified)
            {
                row.downgrade_triggers
                    .push(CertificationDowngradeTrigger::ScopeExpansionUnqualified);
            }
        }
    }
    CertificationPacket::new(input)
}
