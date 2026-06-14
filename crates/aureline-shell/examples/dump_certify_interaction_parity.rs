//! Conformance dump for the M5 keyboard-first modal-parity / clipboard-drop /
//! grouped-history / orientation-aid certification packet.
//!
//! Prints the canonical support export (default), the Markdown summary
//! (`summary` argument), or the protected narrow-drill fixture (`fixture`
//! argument), so the checked-in artifacts stay byte-aligned with the in-crate
//! builder.

use aureline_shell::certify_keyboard_first_modal_parity_clipboard_drop_safety_grouped_history_honesty_and_orie::*;

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());

    match which.as_str() {
        "summary" => {
            let packet = seeded_interaction_parity_certification_packet();
            assert!(packet.validate().is_empty(), "packet must validate");
            print!("{}", packet.render_markdown_summary());
        }
        "fixture" => {
            let packet = fixture_interaction_parity_certification_packet();
            assert!(packet.validate().is_empty(), "fixture must validate");
            println!("{}", packet.export_safe_json());
        }
        _ => {
            let packet = seeded_interaction_parity_certification_packet();
            assert!(packet.validate().is_empty(), "packet must validate");
            println!("{}", packet.export_safe_json());
        }
    }
}
