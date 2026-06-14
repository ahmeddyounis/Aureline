//! Conformance dump for the M5 grouped-history continuity packet.
//!
//! Prints the canonical support export (default), the Markdown summary
//! (`summary` argument), or the protected fixture variant (`fixture` argument),
//! so the checked-in artifacts stay byte-aligned with the in-crate builder.

use aureline_shell::ship_named_undo_groups_exact_versus_compensating_recovery_labels_back_forward_history_cont::*;

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());

    match which.as_str() {
        "summary" => {
            let packet = seeded_history_continuity_packet();
            assert!(packet.validate().is_empty(), "packet must validate");
            print!("{}", packet.render_markdown_summary());
        }
        "fixture" => {
            let packet = fixture_history_continuity_packet();
            assert!(packet.validate().is_empty(), "fixture must validate");
            println!("{}", packet.export_safe_json());
        }
        _ => {
            let packet = seeded_history_continuity_packet();
            assert!(packet.validate().is_empty(), "packet must validate");
            println!("{}", packet.export_safe_json());
        }
    }
}
