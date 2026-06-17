//! Conformance dump for stash/recovery decision rows.
//!
//! Prints the canonical export-safe [`GitStashRecoveryReviewPacket`] as
//! deterministic JSON. The packet restates the canonical [`aureline_git`]
//! stash/recovery sheets across the review, CLI, support-export, provider-overlay,
//! and AI-context surfaces, so every surface explains why a stash or recovery verb
//! was allowed, blocked, or downgraded with the identical decision.
//!
//! The optional first argument narrows the packet to a single surface:
//!
//! * (no argument) — every surface over every reviewed sheet
//! * `review` / `cli` / `support` / `provider` / `ai` — one surface only
//!
//! The canonical document is the source of the checked-in artifact.

use aureline_git::current_stash_recovery_sheets;
use aureline_review::{
    GitStashRecoveryReviewPacket, GitStashRecoveryReviewSupportExport, StashRecoveryDecisionRow,
    StashRecoveryReviewSurface, GIT_STASH_RECOVERY_REVIEW_PACKET_RECORD_KIND,
    GIT_STASH_RECOVERY_REVIEW_REQUIRED_RECONSTRUCTION_FIELDS,
    GIT_STASH_RECOVERY_REVIEW_SCHEMA_VERSION, GIT_STASH_RECOVERY_REVIEW_SUPPORT_EXPORT_RECORD_KIND,
};

const STAMP: &str = "2026-06-17T00:00:00Z";

fn build(packet_id: &str, surfaces: &[StashRecoveryReviewSurface]) -> GitStashRecoveryReviewPacket {
    let source = current_stash_recovery_sheets().expect("git stash recovery sheets validate");
    let sheets = source.sheets;

    let mut rows = Vec::new();
    for surface in surfaces {
        for sheet in &sheets {
            rows.push(StashRecoveryDecisionRow::for_surface_and_sheet(
                *surface,
                sheet,
                format!(
                    "stash-recovery-review-{}-{}",
                    surface.as_str(),
                    sheet.sheet_id
                ),
            ));
        }
    }

    let support_export = GitStashRecoveryReviewSupportExport {
        record_kind: GIT_STASH_RECOVERY_REVIEW_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
        export_id: "git-stash-recovery-review-export:0001".to_owned(),
        row_refs: rows.iter().map(|row| row.row_id.clone()).collect(),
        reconstruction_fields: GIT_STASH_RECOVERY_REVIEW_REQUIRED_RECONSTRUCTION_FIELDS
            .iter()
            .map(|field| (*field).to_owned())
            .collect(),
        raw_paths_redacted: true,
        raw_patch_bodies_redacted: true,
        raw_provider_payloads_redacted: true,
    };

    GitStashRecoveryReviewPacket {
        record_kind: GIT_STASH_RECOVERY_REVIEW_PACKET_RECORD_KIND.to_owned(),
        schema_version: GIT_STASH_RECOVERY_REVIEW_SCHEMA_VERSION,
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
            "git-stash-recovery-review:review:0001",
            &[StashRecoveryReviewSurface::Review],
        ),
        "cli" => build(
            "git-stash-recovery-review:cli:0001",
            &[StashRecoveryReviewSurface::CliHeadless],
        ),
        "support" => build(
            "git-stash-recovery-review:support:0001",
            &[StashRecoveryReviewSurface::SupportExport],
        ),
        "provider" => build(
            "git-stash-recovery-review:provider:0001",
            &[StashRecoveryReviewSurface::ProviderOverlay],
        ),
        "ai" => build(
            "git-stash-recovery-review:ai:0001",
            &[StashRecoveryReviewSurface::AiContext],
        ),
        _ => build(
            "git-stash-recovery-review:0001",
            &StashRecoveryReviewSurface::ALL,
        ),
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
