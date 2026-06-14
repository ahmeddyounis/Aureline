//! Conformance dump for the M5 orientation-aid packet.
//!
//! Prints the canonical support export (default), the Markdown summary
//! (`summary` argument), or the protected fixture variant (`fixture` argument),
//! so the checked-in artifacts stay byte-aligned with the in-crate builder.

use aureline_shell::implement_multi_cursor_fold_state_breadcrumb_minimap_overview_ruler_and_degraded_orientati::*;

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());

    match which.as_str() {
        "summary" => {
            let packet = seeded_orientation_aid_packet();
            assert!(packet.validate().is_empty(), "packet must validate");
            print!("{}", packet.render_markdown_summary());
        }
        "fixture" => {
            let packet = fixture_orientation_aid_packet();
            assert!(packet.validate().is_empty(), "fixture must validate");
            println!("{}", packet.export_safe_json());
        }
        _ => {
            let packet = seeded_orientation_aid_packet();
            assert!(packet.validate().is_empty(), "packet must validate");
            println!("{}", packet.export_safe_json());
        }
    }
}
