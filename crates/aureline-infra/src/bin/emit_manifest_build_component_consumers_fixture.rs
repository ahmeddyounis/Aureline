//! Emits the canonical M05-817 manifest / build component consumer packet as
//! `support`, `csv`, or `summary`. The `support` output is the byte-for-byte
//! `include_str!` canonical checked in under
//! `artifacts/release/m5-manifest-build-component-consumer-proof/`.

fn main() {
    let mode = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());
    let packet = aureline_infra::seeded_m5_manifest_build_consumer_packet();
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
