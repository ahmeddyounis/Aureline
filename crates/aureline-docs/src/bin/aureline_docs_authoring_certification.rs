//! Headless emitter for the M5 docs-authoring certification report.
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_authoring_certification -- packet
//! cargo run -q -p aureline-docs --bin aureline_docs_authoring_certification -- summary
//! cargo run -q -p aureline-docs --bin aureline_docs_authoring_certification -- waiver-log
//! cargo run -q -p aureline-docs --bin aureline_docs_authoring_certification -- fixture mirror_offline_narrows_recall
//! cargo run -q -p aureline-docs --bin aureline_docs_authoring_certification -- validate
//! cargo run -q -p aureline-docs --bin aureline_docs_authoring_certification -- narrowed
//! cargo run -q -p aureline-docs --bin aureline_docs_authoring_certification -- blockers
//! ```

use aureline_docs::{
    certify_profile_row, full_surface_coverage, seeded_stable_docs_authoring_cert_input,
    CertDowngradeTrigger, CertQualificationClass, DocsAuthoringCertReport, DocsAuthoringProfile,
    ProfileRowInput,
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
        Some("waiver-log") => emit_waiver_log(),
        Some("fixture") => emit_fixture(args.get(1).map(String::as_str))?,
        Some("validate") => validate_report(),
        Some("narrowed") => emit_narrowed(),
        Some("blockers") => emit_blockers(),
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn seeded_report() -> DocsAuthoringCertReport {
    DocsAuthoringCertReport::new(seeded_stable_docs_authoring_cert_input())
}

fn emit_packet() {
    println!("{}", seeded_report().export_safe_json());
}

fn emit_summary() {
    print!("{}", seeded_report().render_markdown_summary());
}

fn emit_waiver_log() {
    let log = seeded_report().waiver_and_downgrade_log();
    println!(
        "{}",
        serde_json::to_string_pretty(&log).expect("waiver log serializes")
    );
}

fn validate_report() {
    let violations = seeded_report().validate();
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
    for profile in seeded_report().narrowed_profiles() {
        println!("{}", profile.as_str());
    }
}

fn emit_blockers() {
    for profile in seeded_report().promotion_blockers() {
        println!("{}", profile.as_str());
    }
}

fn emit_fixture(name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let name = name.ok_or("fixture name is required")?;
    let report = match name {
        "mirror_offline_narrows_recall" => mirror_offline_fixture(),
        "unsafe_preview_blocks_handoff" => unsafe_preview_fixture(),
        other => return Err(format!("unknown fixture: {other}").into()),
    };
    let violations = report.validate();
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
    println!("{}", report.export_safe_json());
    Ok(())
}

/// The mirrored and pinned-pack profiles narrow to Beta after the signed mirror
/// goes offline and the source-version truth can no longer be confirmed live.
fn mirror_offline_fixture() -> DocsAuthoringCertReport {
    let mut input = seeded_stable_docs_authoring_cert_input();
    input.report_id = "m5-docs-authoring-certification:mirror-offline:0002".to_owned();
    input.certification_label = "M5 Docs Authoring Certification — Mirror Offline".to_owned();
    for row in input.profile_rows.iter_mut() {
        if matches!(
            row.profile,
            DocsAuthoringProfile::Mirrored | DocsAuthoringProfile::PinnedPack
        ) {
            *row = certify_profile_row(ProfileRowInput {
                profile: row.profile,
                claimed_qualification: CertQualificationClass::Stable,
                scope_summary: format!(
                    "{} — narrowed to Beta: the pinned, signed mirror went offline, so source/version truth falls back to last-known-good with explicit freshness labels",
                    row.scope_summary
                ),
                surface_coverage: full_surface_coverage(),
                source_version_freshness_truth: false,
                safe_rendered_preview_boundaries: true,
                export_support_parity: true,
                proof_age_hours: 12,
                freshness_window_hours: 168,
                evidence_packet_refs: row.evidence_packet_refs.clone(),
                downgrade_triggers: row.downgrade_triggers.clone(),
                class_cap_trigger: None,
                class_cap_rationale: None,
            });
        }
    }
    DocsAuthoringCertReport::new(input)
}

/// The browser-handoff profile is blocked after its rendered preview loses its
/// safe capability boundaries; the profile stays visible but cannot promote.
fn unsafe_preview_fixture() -> DocsAuthoringCertReport {
    let mut input = seeded_stable_docs_authoring_cert_input();
    input.report_id = "m5-docs-authoring-certification:unsafe-preview:0003".to_owned();
    input.certification_label = "M5 Docs Authoring Certification — Preview Held".to_owned();
    for row in input.profile_rows.iter_mut() {
        if row.profile == DocsAuthoringProfile::BrowserHandoff {
            *row = certify_profile_row(ProfileRowInput {
                profile: row.profile,
                claimed_qualification: CertQualificationClass::Beta,
                scope_summary:
                    "Browser-handoff companion docs editing held: the rendered preview lost its safe capability boundaries, so promotion is blocked until preview is re-sanitized"
                        .to_owned(),
                surface_coverage: full_surface_coverage(),
                source_version_freshness_truth: true,
                safe_rendered_preview_boundaries: false,
                export_support_parity: true,
                proof_age_hours: 12,
                freshness_window_hours: 168,
                evidence_packet_refs: row.evidence_packet_refs.clone(),
                downgrade_triggers: vec![
                    CertDowngradeTrigger::UnsafePreviewBlocked,
                    CertDowngradeTrigger::TrustNarrowing,
                ],
                class_cap_trigger: row.class_cap_trigger,
                class_cap_rationale: row.class_cap_rationale.clone(),
            });
        }
    }
    DocsAuthoringCertReport::new(input)
}
