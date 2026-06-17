//! Conformance dump for topology-aware lane review sheets.
//!
//! Prints the canonical export-safe [`GitTopologyReviewPacket`] as deterministic
//! JSON. The packet reviews the canonical [`aureline_git`] topology-action sheets
//! across the search, review, blame, and AI lanes, so every topology scope limit
//! surfaces as an explicit label instead of a generic empty or error state.
//!
//! The optional first argument narrows the packet to a single lane:
//!
//! * (no argument) — every lane over every reviewed sheet
//! * `search` / `review` / `blame` / `ai` — one lane only
//!
//! The canonical document is the source of the checked-in artifact.

use aureline_git::current_topology_action_review_packet;
use aureline_review::{
    GitTopologyReviewPacket, GitTopologyReviewSupportExport, TopologyReviewLane,
    TopologyReviewSheetRow, GIT_TOPOLOGY_REVIEW_PACKET_RECORD_KIND,
    GIT_TOPOLOGY_REVIEW_REQUIRED_RECONSTRUCTION_FIELDS, GIT_TOPOLOGY_REVIEW_SCHEMA_VERSION,
    GIT_TOPOLOGY_REVIEW_SUPPORT_EXPORT_RECORD_KIND,
};

const STAMP: &str = "2026-06-17T00:00:00Z";

fn build(packet_id: &str, lanes: &[TopologyReviewLane]) -> GitTopologyReviewPacket {
    let action_sheets = current_topology_action_review_packet()
        .expect("git action packet validates")
        .sheets;

    let mut rows = Vec::new();
    for lane in lanes {
        for sheet in &action_sheets {
            rows.push(TopologyReviewSheetRow::for_lane_and_sheet(
                *lane,
                sheet,
                format!("review-{}-{}", lane.as_str(), sheet.sheet_id),
            ));
        }
    }

    let support_export = GitTopologyReviewSupportExport {
        record_kind: GIT_TOPOLOGY_REVIEW_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
        export_id: "git-topology-review-export:0001".to_owned(),
        row_refs: rows.iter().map(|row| row.row_id.clone()).collect(),
        reconstruction_fields: GIT_TOPOLOGY_REVIEW_REQUIRED_RECONSTRUCTION_FIELDS
            .iter()
            .map(|field| (*field).to_owned())
            .collect(),
        raw_paths_redacted: true,
        raw_object_bytes_redacted: true,
    };

    GitTopologyReviewPacket {
        record_kind: GIT_TOPOLOGY_REVIEW_PACKET_RECORD_KIND.to_owned(),
        schema_version: GIT_TOPOLOGY_REVIEW_SCHEMA_VERSION,
        packet_id: packet_id.to_owned(),
        generated_at: STAMP.to_owned(),
        action_sheets,
        rows,
        support_export,
    }
}

fn main() {
    let variant = std::env::args().nth(1).unwrap_or_default();
    let packet = match variant.as_str() {
        "search" => build(
            "git-topology-review:search:0001",
            &[TopologyReviewLane::Search],
        ),
        "review" => build(
            "git-topology-review:review:0001",
            &[TopologyReviewLane::Review],
        ),
        "blame" => build(
            "git-topology-review:blame:0001",
            &[TopologyReviewLane::Blame],
        ),
        "ai" => build("git-topology-review:ai:0001", &[TopologyReviewLane::Ai]),
        _ => build("git-topology-review:0001", &TopologyReviewLane::ALL),
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
