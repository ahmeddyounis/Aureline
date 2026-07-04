//! Emits the canonical M5 manifest / build component matrix as `support`, `csv`,
//! or `summary`. The `support` output is the byte-for-byte `include_str!`
//! canonical checked in under
//! `artifacts/infra/m5-manifest-build-component-matrix/`.

fn main() {
    let mode = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());
    let packet = aureline_infra::seeded_manifest_build_component_matrix();
    match mode.as_str() {
        "support" => println!("{}", packet.export_safe_json()),
        "csv" => print!("{}", packet.render_matrix_csv()),
        "summary" => print!("{}", packet.render_markdown_summary()),
        other => {
            eprintln!("unknown mode: {other} (expected support|csv|summary)");
            std::process::exit(2);
        }
    }
}
