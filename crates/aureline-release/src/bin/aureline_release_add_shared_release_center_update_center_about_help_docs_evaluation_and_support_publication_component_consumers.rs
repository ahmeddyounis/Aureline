//! Headless emitter for the M5 publication-component-consumer parity lane.
//!
//! The bin is the only mint-from-truth path for the support export checked in
//! under `artifacts/release/m5-publication-component-consumer-proof/`, its matrix
//! CSV, the Markdown report
//! `artifacts/components/m5-publication-component-consumer.md`, and the narrowed
//! fixtures under `fixtures/ui/m5-publication-component-consumers/`. Every claimed
//! M5 publication-component consumer (the release center, the update center,
//! About/help, the docs portal, the enterprise-evaluation packet, and the support
//! export) adopts the same canonical release/publication components so provenance,
//! freshness, qualification, and client-scope descriptors stay aligned, and so a
//! narrowed rendering is understood from a self-contained reduced-scope banner
//! rather than a generic note.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_add_shared_release_center_update -- support-export
//! cargo run -q -p aureline-release --bin aureline_release_add_shared_release_center_update -- report
//! cargo run -q -p aureline-release --bin aureline_release_add_shared_release_center_update -- csv
//! cargo run -q -p aureline-release --bin aureline_release_add_shared_release_center_update -- fixture-about-help-handoff-narrowed
//! cargo run -q -p aureline-release --bin aureline_release_add_shared_release_center_update -- fixture-docs-mirror-offline-narrowed
//! cargo run -q -p aureline-release --bin aureline_release_add_shared_release_center_update -- validate
//! ```

use aureline_release::add_shared_release_center_update_center_about_help_docs_evaluation_and_support_publication_component_consumers::{
    seeded_m5_publication_component_consumer_about_help_handoff_narrowed,
    seeded_m5_publication_component_consumer_docs_mirror_offline_narrowed,
    seeded_m5_publication_component_consumer_packet,
    M5PublicationComponentConsumerPacket,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("support-export") | None => {
            let packet = seeded_m5_publication_component_consumer_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_publication_component_consumer_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_publication_component_consumer_packet().render_matrix_csv()
            );
        }
        Some("fixture-about-help-handoff-narrowed") => {
            let packet = seeded_m5_publication_component_consumer_about_help_handoff_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-docs-mirror-offline-narrowed") => {
            let packet = seeded_m5_publication_component_consumer_docs_mirror_offline_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_publication_component_consumer_packet(),
                seeded_m5_publication_component_consumer_about_help_handoff_narrowed(),
                seeded_m5_publication_component_consumer_docs_mirror_offline_narrowed(),
            ] {
                assert_valid(&packet)?;
            }
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn assert_valid(
    packet: &M5PublicationComponentConsumerPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("packet failed validation: {}", tokens.join(",")).into())
    }
}
