//! Conformance dump for the M5 clipboard-contract packet.
//!
//! Prints the canonical support export (default), the Markdown summary
//! (`summary` argument), or the protected fixture variant (`fixture` argument),
//! so the checked-in artifacts stay byte-aligned with the in-crate builder.

use aureline_shell::implement_clipboard_contracts_with_plain_text_default_copy_with_context_variants_sensitive::*;

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());

    match which.as_str() {
        "summary" => {
            let packet = seeded_clipboard_contract_packet();
            assert!(packet.validate().is_empty(), "packet must validate");
            print!("{}", packet.render_markdown_summary());
        }
        "fixture" => {
            let packet = fixture_clipboard_contract_packet();
            assert!(packet.validate().is_empty(), "fixture must validate");
            println!("{}", packet.export_safe_json());
        }
        _ => {
            let packet = seeded_clipboard_contract_packet();
            assert!(packet.validate().is_empty(), "packet must validate");
            println!("{}", packet.export_safe_json());
        }
    }
}
