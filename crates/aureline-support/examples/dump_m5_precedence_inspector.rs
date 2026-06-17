use aureline_support::current_m5_precedence_inspectors;

fn main() {
    let registry =
        current_m5_precedence_inspectors().expect("embedded precedence-inspector registry parses");
    let violations = registry.validate();
    assert!(violations.is_empty(), "registry is invalid: {violations:?}");
    println!(
        "{}",
        serde_json::to_string_pretty(&registry).expect("serialize precedence-inspector registry")
    );
}
