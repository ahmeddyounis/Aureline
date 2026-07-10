//! Conformance dump for the M5 route-endpoint-row / component-service-tree-node controls.
//!
//! Prints the canonical support export, the machine-readable matrix CSV, the Markdown design
//! report, or one of the two checked-in scenario fixtures, so the checked artifacts and fixtures
//! can be regenerated deterministically from the canonical seed builders.
//!
//! ```text
//! cargo run -p aureline-templates --example dump_route_endpoint_tree_node_controls -- support-export
//! cargo run -p aureline-templates --example dump_route_endpoint_tree_node_controls -- csv
//! cargo run -p aureline-templates --example dump_route_endpoint_tree_node_controls -- report
//! cargo run -p aureline-templates --example dump_route_endpoint_tree_node_controls -- fixture-heuristic-generated-route
//! cargo run -p aureline-templates --example dump_route_endpoint_tree_node_controls -- fixture-inferred-node
//! cargo run -p aureline-templates --example dump_route_endpoint_tree_node_controls -- validate
//! ```

use aureline_templates::implement_route_endpoint_rows_and_component_service_tree_nodes_with_authored_versus_generated_state_proving_source_files_or_symbols_exact_versus_heuristic_labels_and_open_source_or_open_references_continuity::*;

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support-export".to_owned());
    match which.as_str() {
        "support-export" => {
            let packet = seeded_route_tree_controls();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "csv" => {
            let packet = seeded_route_tree_controls();
            assert_valid(&packet);
            print!("{}", packet.render_matrix_csv());
        }
        "report" => {
            let packet = seeded_route_tree_controls();
            assert_valid(&packet);
            print!("{}", packet.render_markdown_summary());
        }
        "fixture-heuristic-generated-route" => {
            let packet = seeded_route_tree_controls_heuristic_generated_route();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "fixture-inferred-node" => {
            let packet = seeded_route_tree_controls_inferred_node();
            assert_valid(&packet);
            println!("{}", packet.export_safe_json());
        }
        "validate" => {
            let packet = current_route_tree_controls_export()
                .expect("checked route tree controls export validates");
            println!(
                "checked route tree controls export valid: {} route rows, {} tree nodes",
                packet.route_rows.len(),
                packet.tree_nodes.len()
            );
        }
        other => {
            eprintln!("unknown dump selector: {other}");
            std::process::exit(2);
        }
    }
}

fn assert_valid(packet: &RouteEndpointTreeNodeControlsPacket) {
    assert!(
        packet.validate().is_empty(),
        "dump packet failed validation: {:?}",
        packet.validate()
    );
}
