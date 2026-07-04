//! Conformance dump for the M5 visual-designer component consumer packet.
//!
//! Prints the canonical support export (default), the Markdown summary
//! (`summary` argument), or the CSV matrix (`csv` argument) so the checked-in
//! artifacts stay byte-aligned with the in-crate builder.

use aureline_preview::seeded_m5_visual_designer_component_consumers_packet;

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());
    let packet = seeded_m5_visual_designer_component_consumers_packet();

    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "packet must validate: {violations:?}"
    );

    match which.as_str() {
        "summary" => print!("{}", packet.render_markdown_summary()),
        "csv" => print!("{}", packet.render_matrix_csv()),
        _ => println!("{}", packet.export_safe_json()),
    }
}
