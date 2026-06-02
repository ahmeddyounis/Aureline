//! Emits the seeded enterprise docs, matrices, and known-limits fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-policy --example dump_enterprise_docs_matrices_known_limits_fixtures -- page
//! cargo run -q -p aureline-policy --example dump_enterprise_docs_matrices_known_limits_fixtures -- rows
//! cargo run -q -p aureline-policy --example dump_enterprise_docs_matrices_known_limits_fixtures -- defects
//! cargo run -q -p aureline-policy --example dump_enterprise_docs_matrices_known_limits_fixtures -- summary
//! cargo run -q -p aureline-policy --example dump_enterprise_docs_matrices_known_limits_fixtures -- support-export
//! cargo run -q -p aureline-policy --example dump_enterprise_docs_matrices_known_limits_fixtures -- drill-local-core-blocked-withdrawn
//! cargo run -q -p aureline-policy --example dump_enterprise_docs_matrices_known_limits_fixtures -- drill-aspirational-proof-withdrawn
//! cargo run -q -p aureline-policy --example dump_enterprise_docs_matrices_known_limits_fixtures -- drill-missing-profiles-preview
//! cargo run -q -p aureline-policy --example dump_enterprise_docs_matrices_known_limits_fixtures -- drill-stale-docs-beta
//! cargo run -q -p aureline-policy --example dump_enterprise_docs_matrices_known_limits_fixtures -- drill-partial-matrix-beta
//! cargo run -q -p aureline-policy --example dump_enterprise_docs_matrices_known_limits_fixtures -- drill-partially-disclosed-known-limits-beta
//! ```

use aureline_policy::{
    seeded_enterprise_docs_matrices_known_limits_page,
    EnterpriseDocsMatricesKnownLimitsPage, EnterpriseDocsMatricesKnownLimitsSupportExport,
};
use aureline_policy::publish_enterprise_self_hosted_and_air_gapped_docs_matrices_and_known_limits::{
    DocsCompletenessClass, KnownLimitCompletenessClass, LocalCoreContinuityPostureClass,
    MatrixCompletenessClass, ProofCurrencyClass,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_enterprise_docs_matrices_known_limits_page();
    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("rows") => print_json(&page.rows)?,
        Some("defects") => print_json(&page.defects)?,
        Some("summary") => print_json(&page.summary)?,
        Some("support-export") => {
            let export = EnterpriseDocsMatricesKnownLimitsSupportExport::from_page(
                "policy:enterprise-docs-matrices-known-limits:support-export:fixture-001",
                "2026-06-01T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("drill-local-core-blocked-withdrawn") => {
            let mut rows = page.rows;
            for row in rows.iter_mut() {
                if row.enterprise_profile
                    == aureline_policy::publish_enterprise_self_hosted_and_air_gapped_docs_matrices_and_known_limits::EnterpriseProfileClass::SelfHosted
                {
                    row.local_core_posture = LocalCoreContinuityPostureClass::BlockedByDefault;
                    row.local_core_posture_token =
                        LocalCoreContinuityPostureClass::BlockedByDefault.as_str().to_owned();
                }
            }
            let drill_page = EnterpriseDocsMatricesKnownLimitsPage::new(
                "policy:enterprise_docs_matrices_known_limits:drill:local-core-blocked",
                "Drill — local core blocked by default (withdrawn)",
                "2026-06-01T00:00:00Z",
                rows,
            );
            print_json(&drill_page)?;
        }
        Some("drill-aspirational-proof-withdrawn") => {
            let mut rows = page.rows;
            for row in rows.iter_mut() {
                if row.enterprise_profile
                    == aureline_policy::publish_enterprise_self_hosted_and_air_gapped_docs_matrices_and_known_limits::EnterpriseProfileClass::AirGapped
                {
                    row.proof_currency.proof_currency = ProofCurrencyClass::Aspirational;
                    row.proof_currency.proof_currency_token =
                        ProofCurrencyClass::Aspirational.as_str().to_owned();
                }
            }
            let drill_page = EnterpriseDocsMatricesKnownLimitsPage::new(
                "policy:enterprise_docs_matrices_known_limits:drill:aspirational-proof",
                "Drill — aspirational proof on sovereign profile (withdrawn)",
                "2026-06-01T00:00:00Z",
                rows,
            );
            print_json(&drill_page)?;
        }
        Some("drill-missing-profiles-preview") => {
            let rows = vec![page
                .rows
                .into_iter()
                .find(|r| {
                    r.enterprise_profile
                        == aureline_policy::publish_enterprise_self_hosted_and_air_gapped_docs_matrices_and_known_limits::EnterpriseProfileClass::IndividualLocal
                })
                .unwrap()];
            let drill_page = EnterpriseDocsMatricesKnownLimitsPage::new(
                "policy:enterprise_docs_matrices_known_limits:drill:missing-profiles",
                "Drill — missing enterprise profiles (preview)",
                "2026-06-01T00:00:00Z",
                rows,
            );
            print_json(&drill_page)?;
        }
        Some("drill-stale-docs-beta") => {
            let mut rows = page.rows;
            for row in rows.iter_mut() {
                if row.enterprise_profile
                    == aureline_policy::publish_enterprise_self_hosted_and_air_gapped_docs_matrices_and_known_limits::EnterpriseProfileClass::ManagedCloud
                {
                    row.docs.docs_state = DocsCompletenessClass::Stale;
                    row.docs.docs_state_token =
                        DocsCompletenessClass::Stale.as_str().to_owned();
                }
            }
            let drill_page = EnterpriseDocsMatricesKnownLimitsPage::new(
                "policy:enterprise_docs_matrices_known_limits:drill:stale-docs",
                "Drill — stale docs on managed cloud (beta)",
                "2026-06-01T00:00:00Z",
                rows,
            );
            print_json(&drill_page)?;
        }
        Some("drill-partial-matrix-beta") => {
            let mut rows = page.rows;
            for row in rows.iter_mut() {
                if row.enterprise_profile
                    == aureline_policy::publish_enterprise_self_hosted_and_air_gapped_docs_matrices_and_known_limits::EnterpriseProfileClass::EnterpriseOnline
                {
                    row.matrix.matrix_state = MatrixCompletenessClass::Partial;
                    row.matrix.matrix_state_token =
                        MatrixCompletenessClass::Partial.as_str().to_owned();
                }
            }
            let drill_page = EnterpriseDocsMatricesKnownLimitsPage::new(
                "policy:enterprise_docs_matrices_known_limits:drill:partial-matrix",
                "Drill — partial matrix on enterprise online (beta)",
                "2026-06-01T00:00:00Z",
                rows,
            );
            print_json(&drill_page)?;
        }
        Some("drill-partially-disclosed-known-limits-beta") => {
            let mut rows = page.rows;
            for row in rows.iter_mut() {
                if row.enterprise_profile
                    == aureline_policy::publish_enterprise_self_hosted_and_air_gapped_docs_matrices_and_known_limits::EnterpriseProfileClass::SelfHosted
                {
                    row.known_limits.known_limits_state = KnownLimitCompletenessClass::PartiallyDisclosed;
                    row.known_limits.known_limits_state_token =
                        KnownLimitCompletenessClass::PartiallyDisclosed.as_str().to_owned();
                }
            }
            let drill_page = EnterpriseDocsMatricesKnownLimitsPage::new(
                "policy:enterprise_docs_matrices_known_limits:drill:partially-disclosed-known-limits",
                "Drill — partially disclosed known limits on self-hosted (beta)",
                "2026-06-01T00:00:00Z",
                rows,
            );
            print_json(&drill_page)?;
        }
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
