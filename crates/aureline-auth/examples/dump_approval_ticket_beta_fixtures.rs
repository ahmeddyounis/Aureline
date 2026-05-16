//! Emits the seeded approval-ticket beta fixtures.
//!
//! The example is a one-shot helper that prints either the seeded page, one
//! of the per-record arrays, the support-export wrapper, or a named drill so
//! reviewer-facing fixtures can be regenerated:
//!
//! ```sh
//! cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- page
//! cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- sandbox-profile-rows
//! cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- capability-envelope-rows
//! cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- ticket-rows
//! cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- spend-attempt-events
//! cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- defects
//! cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- support-export
//! cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- drill-raw-authority-material
//! cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- drill-self-authorization-attempted
//! cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- drill-admitted-under-drift
//! cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- drill-denial-missing-audit-ref
//! cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- drill-ticket-lifetime-exceeds-sandbox-budget
//! cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- drill-envelope-capability-outside-sandbox
//! cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- drill-missing-requesting-surface-ref
//! ```

use aureline_auth::{
    audit_approval_ticket_beta_page, seeded_approval_ticket_beta_page, ApprovalTicketBetaPage,
    ApprovalTicketBetaSummary, ApprovalTicketBetaSupportExport, CapabilityClass,
    EvaluationOutcome, NativeReapprovalRoute, RequestOriginClass,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_approval_ticket_beta_page();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("sandbox-profile-rows") => print_json(&page.sandbox_profile_rows)?,
        Some("capability-envelope-rows") => print_json(&page.capability_envelope_rows)?,
        Some("ticket-rows") => print_json(&page.ticket_rows)?,
        Some("spend-attempt-events") => print_json(&page.spend_attempt_events)?,
        Some("defects") => print_json(&page.defects)?,
        Some("support-export") => {
            let export = ApprovalTicketBetaSupportExport::from_page(
                "approval-ticket-beta:support-export:001",
                "2026-05-16T05:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("drill-raw-authority-material") => {
            let mut page = page;
            page.ticket_rows[0]
                .guardrails
                .raw_authority_material_present = true;
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-self-authorization-attempted") => {
            let mut page = page;
            page.capability_envelope_rows[0]
                .guardrails
                .self_authorization_attempted = true;
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-admitted-under-drift") => {
            let mut page = page;
            if let Some(event) = page
                .spend_attempt_events
                .iter_mut()
                .find(|event| event.evaluation_outcome == EvaluationOutcome::DeniedTargetDrift)
            {
                event.evaluation_outcome = EvaluationOutcome::Admitted;
                event.evaluation_outcome_token = EvaluationOutcome::Admitted.as_str().to_owned();
                event.native_reapproval_route = NativeReapprovalRoute::NotRequired;
                event.native_reapproval_route_token =
                    NativeReapprovalRoute::NotRequired.as_str().to_owned();
            }
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-denial-missing-audit-ref") => {
            let mut page = page;
            if let Some(event) = page
                .spend_attempt_events
                .iter_mut()
                .find(|event| event.evaluation_outcome == EvaluationOutcome::DeniedExpired)
            {
                event.audit_event_refs.clear();
            }
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-ticket-lifetime-exceeds-sandbox-budget") => {
            let mut page = page;
            let ticket = &mut page.ticket_rows[0];
            ticket.expires_at = "2026-05-16T01:25:00Z".to_owned();
            ticket.lifetime_seconds = 1500;
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-envelope-capability-outside-sandbox") => {
            let mut page = page;
            let envelope = &mut page.capability_envelope_rows[0];
            envelope
                .allowed_capability_classes
                .push(CapabilityClass::MutateRemoteHelperTarget);
            envelope
                .allowed_capability_class_tokens
                .push(CapabilityClass::MutateRemoteHelperTarget.as_str().to_owned());
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-missing-requesting-surface-ref") => {
            let mut page = page;
            if let Some(ticket) = page
                .ticket_rows
                .iter_mut()
                .find(|ticket| ticket.request_origin_class == RequestOriginClass::AiToolPlan)
            {
                ticket.requesting_surface_ref = None;
            }
            print_json(&rebuild_with_defects(page))?;
        }
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }

    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(value)?;
    println!("{json}");
    Ok(())
}

fn rebuild_with_defects(mut page: ApprovalTicketBetaPage) -> ApprovalTicketBetaPage {
    page.defects = audit_approval_ticket_beta_page(
        &page.sandbox_profile_rows,
        &page.capability_envelope_rows,
        &page.ticket_rows,
        &page.spend_attempt_events,
    );
    page.summary = ApprovalTicketBetaSummary::from_records(
        &page.sandbox_profile_rows,
        &page.capability_envelope_rows,
        &page.ticket_rows,
        &page.spend_attempt_events,
        &page.defects,
    );
    page
}
