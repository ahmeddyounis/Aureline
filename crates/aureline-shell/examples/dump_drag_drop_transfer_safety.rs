//! Conformance dump for the M5 drag/drop transfer-safety packet.
//!
//! Prints the canonical support export (default), the Markdown summary
//! (`summary` argument), or the protected fixture variant (`fixture` argument),
//! so the checked-in artifacts stay byte-aligned with the in-crate builder.

use aureline_shell::add_drag_and_drop_verb_disclosure_insertion_indicators_cross_window_detach_and_long_transf::*;

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());

    match which.as_str() {
        "summary" => {
            let packet = seeded_transfer_safety_packet();
            assert!(packet.validate().is_empty(), "packet must validate");
            print!("{}", packet.render_markdown_summary());
        }
        "fixture" => {
            let packet = fixture_transfer_safety_packet();
            assert!(packet.validate().is_empty(), "fixture must validate");
            println!("{}", packet.export_safe_json());
        }
        _ => {
            let packet = seeded_transfer_safety_packet();
            assert!(packet.validate().is_empty(), "packet must validate");
            println!("{}", packet.export_safe_json());
        }
    }
}
