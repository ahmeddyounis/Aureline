//! Conformance dump for history-surgery decision rows.
//!
//! Prints the canonical export-safe [`GitHistoryReviewPacket`] as deterministic
//! JSON. The packet restates the canonical [`aureline_git`] history-surgery sheets
//! across the review, CLI, support-export, provider-overlay, and AI-context
//! surfaces, so every surface explains why a risky mutation was allowed, blocked,
//! or downgraded with the identical decision.
//!
//! The optional first argument narrows the packet to a single surface:
//!
//! * (no argument) — every surface over every reviewed sheet
//! * `review` / `cli` / `support` / `provider` / `ai` — one surface only
//!
//! The canonical document is the source of the checked-in artifact.

use aureline_git::current_history_surgery_review_sheets;
use aureline_review::{
    GitHistoryReviewPacket, GitHistoryReviewSupportExport, HistoryReviewSurface,
    HistorySurgeryDecisionRow, GIT_HISTORY_REVIEW_PACKET_RECORD_KIND,
    GIT_HISTORY_REVIEW_REQUIRED_RECONSTRUCTION_FIELDS, GIT_HISTORY_REVIEW_SCHEMA_VERSION,
    GIT_HISTORY_REVIEW_SUPPORT_EXPORT_RECORD_KIND,
};

const STAMP: &str = "2026-06-17T00:00:00Z";

fn build(packet_id: &str, surfaces: &[HistoryReviewSurface]) -> GitHistoryReviewPacket {
    let source = current_history_surgery_review_sheets().expect("git sheets packet validates");
    let sheets = source.sheets;

    let mut rows = Vec::new();
    for surface in surfaces {
        for sheet in &sheets {
            rows.push(HistorySurgeryDecisionRow::for_surface_and_sheet(
                *surface,
                sheet,
                format!("history-review-{}-{}", surface.as_str(), sheet.sheet_id),
            ));
        }
    }

    let support_export = GitHistoryReviewSupportExport {
        record_kind: GIT_HISTORY_REVIEW_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
        export_id: "git-history-review-export:0001".to_owned(),
        row_refs: rows.iter().map(|row| row.row_id.clone()).collect(),
        reconstruction_fields: GIT_HISTORY_REVIEW_REQUIRED_RECONSTRUCTION_FIELDS
            .iter()
            .map(|field| (*field).to_owned())
            .collect(),
        raw_paths_redacted: true,
        raw_patch_bodies_redacted: true,
        raw_provider_payloads_redacted: true,
    };

    GitHistoryReviewPacket {
        record_kind: GIT_HISTORY_REVIEW_PACKET_RECORD_KIND.to_owned(),
        schema_version: GIT_HISTORY_REVIEW_SCHEMA_VERSION,
        packet_id: packet_id.to_owned(),
        generated_at: STAMP.to_owned(),
        repo_ref: source.repo_ref,
        sheets,
        rows,
        support_export,
    }
}

fn main() {
    let variant = std::env::args().nth(1).unwrap_or_default();
    let packet = match variant.as_str() {
        "review" => build(
            "git-history-review:review:0001",
            &[HistoryReviewSurface::Review],
        ),
        "cli" => build(
            "git-history-review:cli:0001",
            &[HistoryReviewSurface::CliHeadless],
        ),
        "support" => build(
            "git-history-review:support:0001",
            &[HistoryReviewSurface::SupportExport],
        ),
        "provider" => build(
            "git-history-review:provider:0001",
            &[HistoryReviewSurface::ProviderOverlay],
        ),
        "ai" => build(
            "git-history-review:ai:0001",
            &[HistoryReviewSurface::AiContext],
        ),
        _ => build("git-history-review:0001", &HistoryReviewSurface::ALL),
    };
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "review packet invalid: {violations:?}"
    );
    if std::env::args().any(|arg| arg == "--markdown") {
        print!("{}", packet.render_markdown_summary());
    } else {
        println!("{}", packet.export_safe_json());
    }
}
