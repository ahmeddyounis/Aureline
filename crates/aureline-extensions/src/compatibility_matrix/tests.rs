//! Unit coverage for the checked extension bridge matrix.

use super::{
    current_extension_bridge_matrix, validate_extension_bridge_matrix, ExtensionBridgeStateClass,
    ExtensionCompatibilityLabel, ExtensionParityClaimClass,
};

#[test]
fn checked_bridge_matrix_validates_and_names_all_required_states() {
    let matrix = current_extension_bridge_matrix().expect("checked bridge matrix must parse");
    let findings = validate_extension_bridge_matrix(&matrix);
    assert!(
        findings.is_empty(),
        "checked bridge matrix produced findings: {findings:?}"
    );

    assert_eq!(matrix.matrix_id, "extension_bridge_matrix:m3.beta");
    for state in ExtensionBridgeStateClass::required_acceptance_states() {
        assert!(
            matrix.bridge_state_vocabulary.contains(&state),
            "bridge-state vocabulary missing {state:?}"
        );
    }

    let native = matrix
        .row_by_ref("extension_bridge_row:wasm_component_native_beta")
        .expect("native wasm row must exist");
    assert_eq!(
        native.bridge_window.bridge_state_class,
        ExtensionBridgeStateClass::Native
    );
    assert!(!native.runtime_window.window_id.trim().is_empty());
    assert!(!native.sdk_window.window_id.trim().is_empty());
    assert!(!native.manifest_window.window_id.trim().is_empty());
    assert!(!native.bridge_window.window_id.trim().is_empty());
}

#[test]
fn bridge_and_shimmed_rows_never_claim_exact_parity() {
    let matrix = current_extension_bridge_matrix().expect("checked bridge matrix must parse");

    for row in &matrix.rows {
        if row
            .bridge_window
            .bridge_state_class
            .requires_non_parity_disclosure()
        {
            assert_ne!(
                row.bridge_window.compatibility_label,
                ExtensionCompatibilityLabel::Exact,
                "{} must not render exact compatibility",
                row.row_id
            );
            assert_ne!(
                row.bridge_window.parity_claim_class,
                ExtensionParityClaimClass::NativeExactWhereDeclared,
                "{} must not claim native exact parity",
                row.row_id
            );
            assert!(
                !row.bridge_window.known_limits.is_empty(),
                "{} must carry known limits",
                row.row_id
            );
        }
    }
}

#[test]
fn matrix_names_marketplace_sdk_docs_and_release_packet_consumers() {
    let matrix = current_extension_bridge_matrix().expect("checked bridge matrix must parse");

    for expected in [
        "crates/aureline-shell/src/extensions/marketplace/mod.rs",
        "docs/extensions/m3/sdk_v1/README.md",
        "docs/extensions/m3/compatibility_matrix_beta.md",
        "artifacts/extensions/m3/publication_pipeline/publication_pipeline_record.json",
        "artifacts/release/m3/release_notes_draft.md",
    ] {
        assert!(
            matrix
                .consuming_surfaces
                .iter()
                .any(|surface| surface == expected),
            "{expected} must consume the bridge matrix"
        );
    }
}

#[test]
fn exact_label_on_bridge_row_is_refused() {
    let mut matrix = current_extension_bridge_matrix().expect("checked bridge matrix must parse");
    let bridge = matrix
        .rows
        .iter_mut()
        .find(|row| row.row_id == "extension_bridge_row:vscode_api_bridge_beta")
        .expect("bridge row must exist");
    bridge.bridge_window.compatibility_label = ExtensionCompatibilityLabel::Exact;

    let findings = validate_extension_bridge_matrix(&matrix);
    assert!(findings.iter().any(|finding| {
        finding.check_id == "extension_bridge_matrix.bridge_exact_label_refused"
            && finding.row_id.as_deref() == Some("extension_bridge_row:vscode_api_bridge_beta")
    }));
}
