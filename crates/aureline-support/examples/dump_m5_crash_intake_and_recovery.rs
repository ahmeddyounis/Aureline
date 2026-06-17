use aureline_support::current_m5_crash_intake_and_recovery;

fn main() {
    let registry = current_m5_crash_intake_and_recovery()
        .expect("embedded crash-intake-and-recovery registry parses");
    let violations = registry.validate();
    assert!(violations.is_empty(), "registry is invalid: {violations:?}");
    println!(
        "{}",
        serde_json::to_string_pretty(&registry)
            .expect("serialize crash-intake-and-recovery registry")
    );
}
