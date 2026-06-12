//! Emit the canonical M5 work-item mutation review packet and Markdown summary.

use std::fs;
use std::io::{self, Write};

fn main() {
    let packet = aureline_review::canonical_work_item_mutation_review_packet();
    let json = serde_json::to_string_pretty(&packet).expect("serialize packet");
    let summary = packet.render_markdown_summary();

    fs::write(
        "/Users/ahmedyounis/Documents/Projects/Aureline/artifacts/review/m5/ship_work_item_detail_headers_status_transition_sheets_comment_publish_review_and_offline_handoff_packets_with_side_effect_previews/support_export.json",
        format!("{json}\n"),
    )
    .expect("write support export");
    fs::write(
        "/Users/ahmedyounis/Documents/Projects/Aureline/artifacts/review/m5/ship_work_item_detail_headers_status_transition_sheets_comment_publish_review_and_offline_handoff_packets_with_side_effect_previews.md",
        summary,
    )
    .expect("write markdown summary");

    io::stdout()
        .write_all(json.as_bytes())
        .expect("write stdout");
    io::stdout().write_all(b"\n").expect("write newline");
}
