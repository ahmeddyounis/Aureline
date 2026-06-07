//! Headless inspector for the service-health aggregator and contract-state
//! cards. This is the CLI / headless inspect lane the
//! `service_health_contract_beta` review reads: every card, summary,
//! and rollup is identical to what the shell paints in About,
//! service-health, diagnostics, and the support export.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_service_health_inspect -- aggregator
//! cargo run -q -p aureline-shell --bin aureline_shell_service_health_inspect -- cards
//! cargo run -q -p aureline-shell --bin aureline_shell_service_health_inspect -- card <card_id>
//! cargo run -q -p aureline-shell --bin aureline_shell_service_health_inspect -- summary
//! cargo run -q -p aureline-shell --bin aureline_shell_service_health_inspect -- plaintext
//! cargo run -q -p aureline-shell --bin aureline_shell_service_health_inspect -- shared-feed
//! cargo run -q -p aureline-shell --bin aureline_shell_service_health_inspect -- shared-support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_service_health_inspect -- vocabulary
//! ```

use aureline_shell::service_health::seed::seeded_aggregator;
use aureline_shell::service_health::{
    AffectedWorkflowClass, BoundaryClass, LastCheckedAgeClass, LocalContinuityClass,
    ServiceContractStateClass, ServiceFamilyClass,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let aggregator = seeded_aggregator();

    match args.first().map(String::as_str) {
        Some("aggregator") | None => print_json(&aggregator)?,
        Some("cards") => print_json(&aggregator.cards)?,
        Some("card") => {
            let id = args
                .get(1)
                .ok_or("card <card_id> requires an id argument")?;
            let card = aggregator
                .cards
                .iter()
                .find(|c| c.card_id == *id)
                .ok_or_else(|| format!("no card with id {id}"))?;
            print_json(card)?;
        }
        Some("summary") => print_json(&aggregator.summary)?,
        Some("plaintext") => print!("{}", aggregator.render_plaintext()),
        Some("shared-feed") => print_json(&aggregator.shared_service_health_feed())?,
        Some("shared-support-export") => {
            print_json(&aggregator.shared_service_health_feed().support_export_projection())?
        }
        Some("vocabulary") => print_vocabulary(),
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

fn print_vocabulary() {
    println!("contract_state:");
    for state in [
        ServiceContractStateClass::Ready,
        ServiceContractStateClass::Degraded,
        ServiceContractStateClass::LocalOnly,
        ServiceContractStateClass::Stale,
        ServiceContractStateClass::ContractMismatch,
        ServiceContractStateClass::PolicyBlocked,
        ServiceContractStateClass::Unavailable,
    ] {
        println!("  {} -> {}", state.as_str(), state.label());
    }

    println!("boundary_class:");
    for b in [
        BoundaryClass::LocalOnly,
        BoundaryClass::LocalWithRemoteOptional,
        BoundaryClass::LocalWithRemoteRequired,
        BoundaryClass::Hosted,
        BoundaryClass::VendorProvider,
    ] {
        println!("  {} -> {}", b.as_str(), b.label());
    }

    println!("local_continuity:");
    for c in [
        LocalContinuityClass::LocalSafe,
        LocalContinuityClass::LocalSafeReadOnly,
        LocalContinuityClass::LocalReviewOnly,
        LocalContinuityClass::LocalUnsafe,
    ] {
        println!("  {} -> {}", c.as_str(), c.label());
    }

    println!("last_checked_age:");
    for a in [
        LastCheckedAgeClass::Fresh,
        LastCheckedAgeClass::Recent,
        LastCheckedAgeClass::Stale,
        LastCheckedAgeClass::VeryStale,
        LastCheckedAgeClass::NeverChecked,
    ] {
        println!("  {} -> {}", a.as_str(), a.label());
    }

    println!("service_family:");
    for f in [
        ServiceFamilyClass::LanguageServices,
        ServiceFamilyClass::AiAssist,
        ServiceFamilyClass::Sync,
        ServiceFamilyClass::LicenseEntitlement,
        ServiceFamilyClass::Telemetry,
        ServiceFamilyClass::Marketplace,
        ServiceFamilyClass::RemoteRuntime,
        ServiceFamilyClass::ReleaseChannel,
        ServiceFamilyClass::DocsKnowledge,
        ServiceFamilyClass::StatusFeed,
    ] {
        println!("  {} -> {}", f.as_str(), f.label());
    }

    println!("affected_workflow (selection):");
    for w in [
        AffectedWorkflowClass::Edit,
        AffectedWorkflowClass::Save,
        AffectedWorkflowClass::Search,
        AffectedWorkflowClass::Build,
        AffectedWorkflowClass::WorkspaceSync,
        AffectedWorkflowClass::AiCompletion,
        AffectedWorkflowClass::DocsBrowseRemote,
    ] {
        println!("  {} -> {}", w.as_str(), w.label());
    }
}
