//! Emits the seeded runtime-authority-issuer conformance fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-policy --example dump_runtime_authority_issuer_fixtures -- page
//! cargo run -q -p aureline-policy --example dump_runtime_authority_issuer_fixtures -- issuers
//! cargo run -q -p aureline-policy --example dump_runtime_authority_issuer_fixtures -- requesting-surfaces
//! cargo run -q -p aureline-policy --example dump_runtime_authority_issuer_fixtures -- remembered-rules
//! cargo run -q -p aureline-policy --example dump_runtime_authority_issuer_fixtures -- requests
//! cargo run -q -p aureline-policy --example dump_runtime_authority_issuer_fixtures -- decisions
//! cargo run -q -p aureline-policy --example dump_runtime_authority_issuer_fixtures -- defects
//! cargo run -q -p aureline-policy --example dump_runtime_authority_issuer_fixtures -- summary
//! cargo run -q -p aureline-policy --example dump_runtime_authority_issuer_fixtures -- lineage-packet
//! cargo run -q -p aureline-policy --example dump_runtime_authority_issuer_fixtures -- drill-admitted-self-authorization
//! cargo run -q -p aureline-policy --example dump_runtime_authority_issuer_fixtures -- drill-admitted-ambient-privilege
//! cargo run -q -p aureline-policy --example dump_runtime_authority_issuer_fixtures -- drill-admitted-broadened-remembered-rule
//! cargo run -q -p aureline-policy --example dump_runtime_authority_issuer_fixtures -- drill-remembered-rule-broadened-scope
//! cargo run -q -p aureline-policy --example dump_runtime_authority_issuer_fixtures -- drill-remembered-rule-lifetime-exceeds-budget
//! cargo run -q -p aureline-policy --example dump_runtime_authority_issuer_fixtures -- drill-remembered-rule-forbidden-class
//! cargo run -q -p aureline-policy --example dump_runtime_authority_issuer_fixtures -- drill-shell-root-authority-overreach
//! cargo run -q -p aureline-policy --example dump_runtime_authority_issuer_fixtures -- drill-refused-without-reason
//! cargo run -q -p aureline-policy --example dump_runtime_authority_issuer_fixtures -- drill-local-only-admitted-to-provider
//! cargo run -q -p aureline-policy --example dump_runtime_authority_issuer_fixtures -- drill-admitted-beyond-rule-expiry
//! cargo run -q -p aureline-policy --example dump_runtime_authority_issuer_fixtures -- drill-admitted-without-root-proof
//! cargo run -q -p aureline-policy --example dump_runtime_authority_issuer_fixtures -- drill-refused-without-recovery-guidance
//! ```

use aureline_policy::{
    audit_runtime_authority_issuer_page, seeded_runtime_authority_issuer_page,
    AuthorityIssuerClass, AuthorityTicketClass, IssuerBoundaryDecisionClass,
    IssuerBoundaryRejectionReason, RuntimeAuthorityIssuerPage, RuntimeAuthorityIssuerSummary,
    RuntimeAuthorityLineagePacket,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_runtime_authority_issuer_page();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("issuers") => print_json(&page.issuers)?,
        Some("requesting-surfaces") => print_json(&page.requesting_surfaces)?,
        Some("remembered-rules") => print_json(&page.remembered_rules)?,
        Some("requests") => print_json(&page.requests)?,
        Some("decisions") => print_json(&page.decisions)?,
        Some("defects") => print_json(&page.defects)?,
        Some("summary") => print_json(&page.summary)?,
        Some("lineage-packet") => {
            let packet = RuntimeAuthorityLineagePacket::from_page(
                "runtime-authority-lineage-packet:m3:fixture-001",
                "Runtime authority lineage packet (M3 conformance fixture)",
                "2026-05-18T10:30:00Z",
                &page,
            );
            print_json(&packet)?;
        }
        Some("drill-admitted-self-authorization") => {
            let mut page = page;
            let decision = page
                .decisions
                .iter_mut()
                .find(|decision| {
                    decision
                        .rejection_reasons
                        .contains(&IssuerBoundaryRejectionReason::SelfAuthorizationByNonIssuer)
                })
                .expect("self-authorization refused decision");
            decision.decision_class = IssuerBoundaryDecisionClass::Granted;
            decision.decision_class_token =
                IssuerBoundaryDecisionClass::Granted.as_str().to_owned();
            decision.rejection_reasons.clear();
            decision.rejection_reason_tokens.clear();
            decision.minted_authority_ticket_ref =
                Some("authority-ticket:credential-projection:invalid:0099".to_owned());
            decision.reprompt_required = false;
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-admitted-ambient-privilege") => {
            let mut page = page;
            let decision = page
                .decisions
                .iter_mut()
                .find(|decision| {
                    decision
                        .rejection_reasons
                        .contains(&IssuerBoundaryRejectionReason::AmbientPrivilegeInferred)
                })
                .expect("ambient-privilege refused decision");
            decision.decision_class = IssuerBoundaryDecisionClass::Granted;
            decision.decision_class_token =
                IssuerBoundaryDecisionClass::Granted.as_str().to_owned();
            decision.rejection_reasons.clear();
            decision.rejection_reason_tokens.clear();
            decision.minted_authority_ticket_ref =
                Some("authority-ticket:external-provider:invalid:0098".to_owned());
            decision.reprompt_required = false;
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-admitted-broadened-remembered-rule") => {
            let mut page = page;
            let decision = page
                .decisions
                .iter_mut()
                .find(|decision| decision.request_id == "request:recipe:broaden-remembered:0005")
                .expect("recipe broaden decision");
            decision.decision_class = IssuerBoundaryDecisionClass::RememberedDecisionNarrowed;
            decision.decision_class_token = IssuerBoundaryDecisionClass::RememberedDecisionNarrowed
                .as_str()
                .to_owned();
            decision.rejection_reasons.clear();
            decision.rejection_reason_tokens.clear();
            decision.renewed_from_rule_id = Some("remembered-rule:local-format:0001".to_owned());
            decision.minted_authority_ticket_ref =
                Some("authority-ticket:local-mutation:invalid-broadening:0097".to_owned());
            decision.reprompt_required = false;
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-remembered-rule-broadened-scope") => {
            let mut page = page;
            page.remembered_rules[0].scope_ref.clear();
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-remembered-rule-lifetime-exceeds-budget") => {
            let mut page = page;
            // local_mutation budget is 900s; push the renewable lifetime past it.
            page.remembered_rules[0].renewable_ticket_lifetime_seconds = 7_200;
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-remembered-rule-forbidden-class") => {
            let mut page = page;
            page.remembered_rules[0].ticket_class = AuthorityTicketClass::CredentialProjection;
            page.remembered_rules[0].ticket_class_token =
                AuthorityTicketClass::CredentialProjection
                    .as_str()
                    .to_owned();
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-shell-root-authority-overreach") => {
            let mut page = page;
            let shell = page
                .issuers
                .iter_mut()
                .find(|issuer| issuer.issuer_class == AuthorityIssuerClass::Shell)
                .expect("shell issuer");
            shell.may_mint_root_authority_changes = true;
            shell
                .mintable_ticket_classes
                .push(AuthorityTicketClass::PolicyTrustAdminChange);
            shell.mintable_ticket_class_tokens.push(
                AuthorityTicketClass::PolicyTrustAdminChange
                    .as_str()
                    .to_owned(),
            );
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-refused-without-reason") => {
            let mut page = page;
            let decision = page
                .decisions
                .iter_mut()
                .find(|decision| decision.decision_class == IssuerBoundaryDecisionClass::Refused)
                .expect("refused decision");
            decision.rejection_reasons.clear();
            decision.rejection_reason_tokens.clear();
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-local-only-admitted-to-provider") => {
            let mut page = page;
            let decision = page
                .decisions
                .iter_mut()
                .find(|decision| {
                    decision
                        .rejection_reasons
                        .contains(&IssuerBoundaryRejectionReason::AuthoritySourceUnreachableTarget)
                })
                .expect("local-only refusal");
            decision.decision_class = IssuerBoundaryDecisionClass::Granted;
            decision.decision_class_token =
                IssuerBoundaryDecisionClass::Granted.as_str().to_owned();
            decision.rejection_reasons.clear();
            decision.rejection_reason_tokens.clear();
            decision.minted_authority_ticket_ref =
                Some("authority-ticket:external-provider:invalid:0096".to_owned());
            decision.reprompt_required = false;
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-admitted-beyond-rule-expiry") => {
            let mut page = page;
            // Push the rule expiry before the renewal decision timestamp.
            page.remembered_rules[0].rule_expires_at = "2026-05-18T09:59:00Z".to_owned();
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-admitted-without-root-proof") => {
            let mut page = page;
            let request = page
                .requests
                .iter_mut()
                .find(|request| request.request_id == "request:admin-console:root-rotation:0007")
                .expect("root-rotation request");
            request.root_authority_proof_present = false;
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-refused-without-recovery-guidance") => {
            let mut page = page;
            let decision = page
                .decisions
                .iter_mut()
                .find(|decision| decision.decision_class == IssuerBoundaryDecisionClass::Refused)
                .expect("refused decision");
            decision.local_editing_preserved = false;
            decision.reprompt_required = false;
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

fn rebuild_with_defects(mut page: RuntimeAuthorityIssuerPage) -> RuntimeAuthorityIssuerPage {
    page.defects = audit_runtime_authority_issuer_page(
        &page.issuers,
        &page.requesting_surfaces,
        &page.remembered_rules,
        &page.requests,
        &page.decisions,
    );
    page.summary = RuntimeAuthorityIssuerSummary::from_records(
        &page.issuers,
        &page.requesting_surfaces,
        &page.remembered_rules,
        &page.requests,
        &page.decisions,
        &page.defects,
    );
    page
}
