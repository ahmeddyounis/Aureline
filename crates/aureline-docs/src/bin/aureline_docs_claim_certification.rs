//! Headless emitter for the M5 documentation-claim certification packet.
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_claim_certification -- packet
//! cargo run -q -p aureline-docs --bin aureline_docs_claim_certification -- summary
//! cargo run -q -p aureline-docs --bin aureline_docs_claim_certification -- fixture source_class_evidence_stale_retest_pending
//! cargo run -q -p aureline-docs --bin aureline_docs_claim_certification -- validate
//! cargo run -q -p aureline-docs --bin aureline_docs_claim_certification -- retest-pending
//! cargo run -q -p aureline-docs --bin aureline_docs_claim_certification -- blockers
//! ```

use aureline_docs::{
    seeded_stable_docs_claim_certification_input, DocsClaimCertificationPacket,
    DocsClaimDowngradeTrigger, DocsClaimQualificationClass, DocsClaimVerdict, DocsEvidenceClass,
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
        Some("retest-pending") => emit_retest_pending(),
        Some("blockers") => emit_blockers(),
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn seeded_packet() -> DocsClaimCertificationPacket {
    DocsClaimCertificationPacket::new(seeded_stable_docs_claim_certification_input())
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
    for profile in seeded_packet().narrowed_profiles() {
        println!("{}", profile.as_str());
    }
}

fn emit_retest_pending() {
    for profile in seeded_packet().retest_pending_profiles() {
        println!("{}", profile.as_str());
    }
}

fn emit_blockers() {
    for profile in seeded_packet().publication_blockers() {
        println!("{}", profile.as_str());
    }
}

fn emit_fixture(name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let name = name.ok_or("fixture name is required")?;
    let packet = match name {
        "source_class_evidence_stale_retest_pending" => source_class_retest_fixture(),
        "browser_handoff_evidence_stale_blocks_publication" => browser_handoff_blocked_fixture(),
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

/// Every profile narrows to Preview and is marked retest-pending because the
/// docs source-class evidence exceeded its freshness SLO.
fn source_class_retest_fixture() -> DocsClaimCertificationPacket {
    let mut packet = seeded_packet().narrowed_for_stale_evidence(&[DocsEvidenceClass::SourceClass]);
    packet.packet_id = "m5-docs-claim-certification:source-class-retest:0002".to_owned();
    packet.certification_label =
        "M5 Documentation-Claim Certification — Source-Class Retest Pending".to_owned();
    packet
}

/// The browser-handoff-bearing profiles are blocked from publication after their
/// handoff evidence went stale; the profiles stay present, labeled, not hidden.
fn browser_handoff_blocked_fixture() -> DocsClaimCertificationPacket {
    let mut input = seeded_stable_docs_claim_certification_input();
    input.packet_id = "m5-docs-claim-certification:browser-handoff-blocked:0003".to_owned();
    input.certification_label =
        "M5 Documentation-Claim Certification — Browser Handoff Blocked".to_owned();
    for row in input.profile_rows.iter_mut() {
        if row
            .evidence_classes
            .contains(&DocsEvidenceClass::BrowserHandoff)
        {
            row.qualification = DocsClaimQualificationClass::Held;
            row.verdict = DocsClaimVerdict::BlockedUnderqualified;
            if !row
                .downgrade_triggers
                .contains(&DocsClaimDowngradeTrigger::BrowserHandoffEvidenceStale)
            {
                row.downgrade_triggers
                    .push(DocsClaimDowngradeTrigger::BrowserHandoffEvidenceStale);
            }
            row.scope_summary = format!(
                "{} — held and blocked from publication: the browser-handoff evidence went stale, so no handoff is offered until it is re-proven",
                row.scope_summary
            );
        }
    }
    DocsClaimCertificationPacket::new(input)
}
