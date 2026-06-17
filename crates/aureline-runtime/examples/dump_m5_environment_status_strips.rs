use aureline_runtime::current_m5_environment_status_strips;

fn main() {
    let registry = current_m5_environment_status_strips()
        .expect("embedded environment-status-strip registry parses");
    let violations = registry.validate();
    assert!(violations.is_empty(), "registry is invalid: {violations:?}");
    println!(
        "{}",
        serde_json::to_string_pretty(&registry)
            .expect("serialize environment-status-strip registry")
    );
}
