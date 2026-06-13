//! Emit the canonical M5 provider-backed team-workflow certification packet and
//! Markdown summary.

use std::fs;
use std::io::{self, Write};

fn main() {
    let packet = aureline_review::certify_from_current_team_workflow_exports(
        "m5-team-workflow-certification:certified:0001".to_owned(),
        "M5 Provider-Backed Team-Workflow Certification".to_owned(),
        "2026-06-12T00:00:00Z".to_owned(),
        aureline_review::M5TeamWorkflowCertificationProofFreshness {
            proof_freshness_slo_hours: 168,
            last_proof_refresh: "2026-06-12T00:00:00Z".to_owned(),
            auto_narrow_on_provider_authority_stale: true,
            auto_narrow_on_publish_later_stale: true,
            auto_narrow_on_reconciliation_stale: true,
        },
    );
    let json = serde_json::to_string_pretty(&packet).expect("serialize packet");
    let summary = packet.render_markdown_summary();

    fs::write(
        "/Users/ahmedyounis/Documents/Projects/Aureline/artifacts/review/m5/certify-work-item-provider-linked-mutation-acting-identity-and-publish-later-or-reconciliation-depth-on-all-claimed-m5-team-workflow-rows/support_export.json",
        format!("{json}\n"),
    )
    .expect("write support export");
    fs::write(
        "/Users/ahmedyounis/Documents/Projects/Aureline/artifacts/review/m5/certify-work-item-provider-linked-mutation-acting-identity-and-publish-later-or-reconciliation-depth-on-all-claimed-m5-team-workflow-rows.md",
        summary,
    )
    .expect("write markdown summary");

    io::stdout()
        .write_all(json.as_bytes())
        .expect("write stdout");
    io::stdout().write_all(b"\n").expect("write newline");
}
