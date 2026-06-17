use aureline_support::current_m5_support_center_layout;

fn main() {
    let layout = current_m5_support_center_layout().expect("embedded support-center layout parses");
    let violations = layout.validate();
    assert!(violations.is_empty(), "layout is invalid: {violations:?}");
    println!(
        "{}",
        serde_json::to_string_pretty(&layout).expect("serialize support-center layout")
    );
}
