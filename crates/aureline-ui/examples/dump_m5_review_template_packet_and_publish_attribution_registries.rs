//! Headless emitter for the M5 line-review_template_packet and line-downgrade-packet registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/review/m5-review-template-packet-and-publish-attribution-registries-proof/`, its matrix CSV,
//! the Markdown summary, and the narrowed fixtures under
//! `fixtures/review/m5-review-template-packet-and-publish-attribution-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_review_template_packet_and_publish_attribution_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_review_template_packet_and_publish_attribution_registries -- report
//! cargo run -p aureline-ui --example dump_m5_review_template_packet_and_publish_attribution_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_review_template_packet_and_publish_attribution_registries -- review-template-packet-table
//! cargo run -p aureline-ui --example dump_m5_review_template_packet_and_publish_attribution_registries -- fixture-review-template-packet-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_review_template_packet_and_publish_attribution_registries -- fixture-template-publish-attribution-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_review_template_packet_and_publish_attribution_registries -- validate
//! ```

use aureline_ui::m5_review_template_packet_and_publish_attribution_registries::{
    seeded_m5_review_template_packet_and_publish_attribution_registries,
    seeded_m5_review_template_packet_and_publish_attribution_registries_review_template_packet_beta_narrowed,
    seeded_m5_review_template_packet_and_publish_attribution_registries_template_publish_attribution_preview_narrowed,
    M5ReviewTemplatePacketAndPublishAttributionRegistriesPacket,
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
            let packet = seeded_m5_review_template_packet_and_publish_attribution_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_review_template_packet_and_publish_attribution_registries()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_review_template_packet_and_publish_attribution_registries()
                    .render_matrix_csv()
            );
        }
        Some("review-template-packet-table") => {
            print!(
                "{}",
                seeded_m5_review_template_packet_and_publish_attribution_registries()
                    .render_review_template_packet_table()
            );
        }
        Some("fixture-review-template-packet-beta-narrowed") => {
            let packet =
                seeded_m5_review_template_packet_and_publish_attribution_registries_review_template_packet_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-template-publish-attribution-preview-narrowed") => {
            let packet =
                seeded_m5_review_template_packet_and_publish_attribution_registries_template_publish_attribution_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_review_template_packet_and_publish_attribution_registries(),
                seeded_m5_review_template_packet_and_publish_attribution_registries_review_template_packet_beta_narrowed(),
                seeded_m5_review_template_packet_and_publish_attribution_registries_template_publish_attribution_preview_narrowed(),
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
    packet: &M5ReviewTemplatePacketAndPublishAttributionRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
