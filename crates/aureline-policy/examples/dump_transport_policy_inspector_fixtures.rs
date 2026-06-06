//! Emits the seeded transport policy inspector fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-policy --example dump_transport_policy_inspector_fixtures -- page
//! cargo run -q -p aureline-policy --example dump_transport_policy_inspector_fixtures -- policies
//! cargo run -q -p aureline-policy --example dump_transport_policy_inspector_fixtures -- events
//! cargo run -q -p aureline-policy --example dump_transport_policy_inspector_fixtures -- trust-layers
//! cargo run -q -p aureline-policy --example dump_transport_policy_inspector_fixtures -- summary
//! cargo run -q -p aureline-policy --example dump_transport_policy_inspector_fixtures -- support-export
//! cargo run -q -p aureline-policy --example dump_transport_policy_inspector_fixtures -- drill-raw-secret-withdrawn
//! ```

use aureline_policy::{
    seeded_transport_policy_inspector_page, EndpointClass, RouteSourceClass,
    TransportPolicyInspectorPage, TransportPolicyInspectorSupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_transport_policy_inspector_page();
    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("policies") => print_json(&page.policies)?,
        Some("events") => print_json(&page.network_events)?,
        Some("trust-layers") => print_json(&page.trust_store_layers)?,
        Some("defects") => print_json(&page.defects)?,
        Some("summary") => print_json(&page.summary)?,
        Some("support-export") => {
            let export = TransportPolicyInspectorSupportExport::from_page(
                "policy:transport-policy-inspector:support-export:fixture-001",
                "2026-06-01T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("drill-raw-secret-withdrawn") => {
            let mut page = page;
            page.policies[0].raw_secret_material_excluded = false;
            let drill = TransportPolicyInspectorPage::new(
                "policy:transport-policy-inspector:drill:raw-secret",
                "Drill - raw secret material exposed",
                "2026-06-01T00:00:00Z",
                page.route_source_precedence,
                page.policies,
                page.network_events,
                page.trust_store_layers,
            );
            print_json(&drill)?;
        }
        Some("drill-missing-endpoint-preview") => {
            let mut page = page;
            page.policies
                .retain(|policy| policy.endpoint_class != EndpointClass::Ai);
            let drill = TransportPolicyInspectorPage::new(
                "policy:transport-policy-inspector:drill:missing-endpoint",
                "Drill - missing endpoint coverage",
                "2026-06-01T00:00:00Z",
                page.route_source_precedence,
                page.policies,
                page.network_events,
                page.trust_store_layers,
            );
            print_json(&drill)?;
        }
        Some("drill-precedence-preview") => {
            let mut precedence = RouteSourceClass::PRECEDENCE.to_vec();
            precedence.swap(0, 1);
            let drill = TransportPolicyInspectorPage::new(
                "policy:transport-policy-inspector:drill:precedence",
                "Drill - route source precedence drift",
                "2026-06-01T00:00:00Z",
                precedence,
                page.policies,
                page.network_events,
                page.trust_store_layers,
            );
            print_json(&drill)?;
        }
        Some("drill-no-recovery-beta") => {
            let mut page = page;
            if let Some(event) = page
                .network_events
                .iter_mut()
                .find(|event| event.endpoint_class == EndpointClass::Ai)
            {
                event.recovery_action_hint_token.clear();
            }
            let drill = TransportPolicyInspectorPage::new(
                "policy:transport-policy-inspector:drill:no-recovery",
                "Drill - missing recovery action",
                "2026-06-01T00:00:00Z",
                page.route_source_precedence,
                page.policies,
                page.network_events,
                page.trust_store_layers,
            );
            print_json(&drill)?;
        }
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
