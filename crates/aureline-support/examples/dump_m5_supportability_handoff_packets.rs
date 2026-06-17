use aureline_support::current_m5_supportability_handoff_packets;

fn main() {
    let registry = current_m5_supportability_handoff_packets()
        .expect("embedded supportability-handoff-packets registry parses");
    let violations = registry.validate();
    assert!(violations.is_empty(), "registry is invalid: {violations:?}");
    println!(
        "{}",
        serde_json::to_string_pretty(&registry)
            .expect("serialize supportability-handoff-packets registry")
    );
}
