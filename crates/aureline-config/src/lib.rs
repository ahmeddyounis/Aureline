//! Contracts for configuration, manifest, environment-file, and resolved-state
//! surfaces.
//!
//! The crate keeps the stable structured-editor qualification record separate
//! from settings-specific precedence code so first-party editors, CLI/headless
//! inspection, Help, and support export can consume one source/effective/live
//! vocabulary for files that are not ordinary settings.

pub mod structured_config_artifact_modes_and_layers;
pub mod structured_config_manifest_environment_editor_qualification;
pub mod structured_config_policy_bundle_and_entitlement_matrix;
