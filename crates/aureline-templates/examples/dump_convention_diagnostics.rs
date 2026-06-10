//! Conformance dump for the convention-diagnostic packet.
//!
//! Prints one of the canonical export, the two checked-in narrowed fixtures, or
//! the Markdown summary, so the artifact and fixtures can be regenerated
//! deterministically from the canonical builder.
//!
//! ```text
//! cargo run -p aureline-templates --example dump_convention_diagnostics -- canonical
//! cargo run -p aureline-templates --example dump_convention_diagnostics -- proving_file_unavailable
//! cargo run -p aureline-templates --example dump_convention_diagnostics -- confidence_unverified
//! cargo run -p aureline-templates --example dump_convention_diagnostics -- markdown
//! ```

use aureline_templates::add_convention_diagnostics_confidence_labels_suppressibility_and_proving_file_disclosure::*;

const PACKET_ID: &str = "convention-diagnostic:stable:0001";
const PACKET_LABEL: &str =
    "Convention Diagnostics with Confidence Labels, Suppressibility, and Proving-File Disclosure";
const MINTED_AT: &str = "2026-06-08T00:00:00Z";
const HIGH_NAMING: &str = "convention-diagnostic-row:naming.model.high:2026.06";
const EXACT_FILE_LOCATION: &str =
    "convention-diagnostic-row:file_location.controllers.exact:2026.06";

fn canonical() -> ConventionDiagnosticPacket {
    canonical_convention_diagnostics(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        MINTED_AT.to_owned(),
        ConventionDiagnosticProofFreshness {
            proof_freshness_slo_hours: 168,
            last_proof_refresh: MINTED_AT.to_owned(),
            auto_narrow_on_stale: true,
        },
    )
}

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "canonical".to_owned());
    let mut packet = canonical();
    match which.as_str() {
        "canonical" => {}
        "markdown" => {
            print!("{}", packet.render_markdown_summary());
            return;
        }
        "proving_file_unavailable" => {
            // A high-confidence naming diagnostic whose proving file can no longer
            // be disclosed is narrowed to unknown confidence and blocked rather than
            // presented as grounded truth.
            packet.apply_downgrade_automation(&[ConventionDiagnosticRowObservation {
                row_id: HIGH_NAMING.to_owned(),
                proving_file_available: false,
                confidence_verified: true,
                analysis_fresh: true,
                suppression_active: false,
                proof_fresh: true,
                upstream_narrowed: false,
            }]);
        }
        "confidence_unverified" => {
            // An exact file-location diagnostic whose confidence cannot be verified
            // is narrowed to unknown confidence and withheld behind its banner.
            packet.apply_downgrade_automation(&[ConventionDiagnosticRowObservation {
                row_id: EXACT_FILE_LOCATION.to_owned(),
                proving_file_available: true,
                confidence_verified: false,
                analysis_fresh: true,
                suppression_active: false,
                proof_fresh: true,
                upstream_narrowed: false,
            }]);
        }
        other => {
            eprintln!("unknown dump selector: {other}");
            std::process::exit(2);
        }
    }
    assert!(
        packet.validate().is_empty(),
        "dump packet failed validation: {:?}",
        packet.validate()
    );
    println!("{}", packet.export_safe_json());
}
