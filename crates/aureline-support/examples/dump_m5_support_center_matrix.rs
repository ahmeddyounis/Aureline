use aureline_support::current_m5_support_center_matrix;

fn main() {
    let matrix = current_m5_support_center_matrix().expect("embedded support-center matrix parses");
    let violations = matrix.validate();
    assert!(violations.is_empty(), "matrix is invalid: {violations:?}");
    println!(
        "{}",
        serde_json::to_string_pretty(&matrix).expect("serialize support-center matrix")
    );
}
