use aureline_support::current_m5_support_bundle_consent;

fn main() {
    let registry = current_m5_support_bundle_consent()
        .expect("embedded support-bundle-consent registry parses");
    let violations = registry.validate();
    assert!(violations.is_empty(), "registry is invalid: {violations:?}");
    println!(
        "{}",
        serde_json::to_string_pretty(&registry).expect("serialize support-bundle-consent registry")
    );
}
