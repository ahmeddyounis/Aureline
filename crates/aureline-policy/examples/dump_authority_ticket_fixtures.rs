//! Emits the seeded authority-ticket fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-policy --example dump_authority_ticket_fixtures -- page
//! cargo run -q -p aureline-policy --example dump_authority_ticket_fixtures -- support-export
//! cargo run -q -p aureline-policy --example dump_authority_ticket_fixtures -- drill-raw-secret-projection
//! cargo run -q -p aureline-policy --example dump_authority_ticket_fixtures -- drill-unsigned-root-change
//! cargo run -q -p aureline-policy --example dump_authority_ticket_fixtures -- drill-admitted-without-ticket
//! ```

use aureline_policy::{
    audit_authority_ticket_page, seeded_authority_ticket_page, AuthorityEvaluationOutcome,
    AuthoritySourceProofClass, AuthorityTicketPage, AuthorityTicketSummary,
    AuthorityTicketSupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_authority_ticket_page();
    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("tickets") => print_json(&page.tickets)?,
        Some("credential-projections") => print_json(&page.credential_projections)?,
        Some("root-authority-changes") => print_json(&page.root_authority_changes)?,
        Some("spend-attempts") => print_json(&page.spend_attempts)?,
        Some("defects") => print_json(&page.defects)?,
        Some("summary") => print_json(&page.summary)?,
        Some("support-export") => {
            let export = AuthorityTicketSupportExport::from_page(
                "authority-ticket:support-export:fixture-001",
                "2026-05-18T10:30:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("drill-raw-secret-projection") => {
            let mut page = page;
            page.credential_projections[0]
                .guardrails
                .plaintext_secret_present = true;
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-unsigned-root-change") => {
            let mut page = page;
            let proof = &mut page.root_authority_changes[0].source_proof;
            proof.proof_class = AuthoritySourceProofClass::MissingOrUnverified;
            proof.proof_class_token = AuthoritySourceProofClass::MissingOrUnverified
                .as_str()
                .to_owned();
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-admitted-without-ticket") => {
            let mut page = page;
            if let Some(spend) = page.spend_attempts.iter_mut().find(|spend| {
                spend.evaluation_outcome == AuthorityEvaluationOutcome::DeniedMissingTicket
            }) {
                spend.evaluation_outcome = AuthorityEvaluationOutcome::Admitted;
                spend.evaluation_outcome_token =
                    AuthorityEvaluationOutcome::Admitted.as_str().to_owned();
            }
            print_json(&rebuild_with_defects(page))?;
        }
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

fn rebuild_with_defects(mut page: AuthorityTicketPage) -> AuthorityTicketPage {
    page.defects = audit_authority_ticket_page(
        &page.tickets,
        &page.credential_projections,
        &page.root_authority_changes,
        &page.spend_attempts,
    );
    page.summary = AuthorityTicketSummary::from_records(
        &page.tickets,
        &page.credential_projections,
        &page.root_authority_changes,
        &page.spend_attempts,
        &page.defects,
    );
    page
}
