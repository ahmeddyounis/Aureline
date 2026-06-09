use super::*;

fn sample_python_kernelspec() -> Kernelspec {
    Kernelspec {
        record_kind: KERNELSPEC_RECORD_KIND.to_owned(),
        notebook_kernel_discovery_schema_version: NOTEBOOK_KERNEL_DISCOVERY_SCHEMA_VERSION,
        kernelspec_id: "kernelspec.python.312.local.01".to_owned(),
        display_name_label: "Python 3.12".to_owned(),
        language_label: "python".to_owned(),
        launch_command_template_ref: "launch.cmd.python.312.local.01".to_owned(),
        resource_dir_ref: Some("res.dir.python.312.local.01".to_owned()),
        metadata_ref: Some("meta.python.312.local.01".to_owned()),
        summary: "Local Python 3.12 kernelspec discovered from Jupyter data dir.".to_owned(),
    }
}

fn sample_r_kernelspec() -> Kernelspec {
    Kernelspec {
        record_kind: KERNELSPEC_RECORD_KIND.to_owned(),
        notebook_kernel_discovery_schema_version: NOTEBOOK_KERNEL_DISCOVERY_SCHEMA_VERSION,
        kernelspec_id: "kernelspec.r.430.conda.01".to_owned(),
        display_name_label: "R 4.3.0".to_owned(),
        language_label: "r".to_owned(),
        launch_command_template_ref: "launch.cmd.r.430.conda.01".to_owned(),
        resource_dir_ref: Some("res.dir.r.430.conda.01".to_owned()),
        metadata_ref: None,
        summary: "Conda R 4.3.0 kernelspec discovered from conda environment.".to_owned(),
    }
}

fn sample_julia_kernelspec() -> Kernelspec {
    Kernelspec {
        record_kind: KERNELSPEC_RECORD_KIND.to_owned(),
        notebook_kernel_discovery_schema_version: NOTEBOOK_KERNEL_DISCOVERY_SCHEMA_VERSION,
        kernelspec_id: "kernelspec.julia.110.remote.01".to_owned(),
        display_name_label: "Julia 1.10".to_owned(),
        language_label: "julia".to_owned(),
        launch_command_template_ref: "launch.cmd.julia.110.remote.01".to_owned(),
        resource_dir_ref: None,
        metadata_ref: Some("meta.julia.110.remote.01".to_owned()),
        summary: "Remote Julia 1.10 kernelspec from managed workspace registry.".to_owned(),
    }
}

fn sample_uv_resolution() -> InterpreterResolution {
    InterpreterResolution {
        record_kind: INTERPRETER_RESOLUTION_RECORD_KIND.to_owned(),
        notebook_kernel_discovery_schema_version: NOTEBOOK_KERNEL_DISCOVERY_SCHEMA_VERSION,
        resolution_id: "interp.uv.python.312.01".to_owned(),
        interpreter_path_token_ref: "vfs.path.token.uv.python.312.01".to_owned(),
        version_label: "3.12.4".to_owned(),
        language_label: "python".to_owned(),
        manager_class: InterpreterManagerClass::Uv,
        source_manifest_ref: Some("manifest.pyproject.01".to_owned()),
        summary: "Python 3.12.4 resolved via uv from pyproject.toml.".to_owned(),
    }
}

fn sample_conda_resolution() -> InterpreterResolution {
    InterpreterResolution {
        record_kind: INTERPRETER_RESOLUTION_RECORD_KIND.to_owned(),
        notebook_kernel_discovery_schema_version: NOTEBOOK_KERNEL_DISCOVERY_SCHEMA_VERSION,
        resolution_id: "interp.conda.r.430.01".to_owned(),
        interpreter_path_token_ref: "vfs.path.token.conda.r.430.01".to_owned(),
        version_label: "4.3.0".to_owned(),
        language_label: "r".to_owned(),
        manager_class: InterpreterManagerClass::Conda,
        source_manifest_ref: Some("manifest.environment.yml.01".to_owned()),
        summary: "R 4.3.0 resolved via conda from environment.yml.".to_owned(),
    }
}

fn sample_unknown_resolution() -> InterpreterResolution {
    InterpreterResolution {
        record_kind: INTERPRETER_RESOLUTION_RECORD_KIND.to_owned(),
        notebook_kernel_discovery_schema_version: NOTEBOOK_KERNEL_DISCOVERY_SCHEMA_VERSION,
        resolution_id: "interp.unknown.01".to_owned(),
        interpreter_path_token_ref: "vfs.path.token.unknown.01".to_owned(),
        version_label: "".to_owned(),
        language_label: "python".to_owned(),
        manager_class: InterpreterManagerClass::Unknown,
        source_manifest_ref: None,
        summary: "Unresolved interpreter; manager unknown.".to_owned(),
    }
}

fn sample_fresh_fingerprint() -> EnvironmentFingerprint {
    EnvironmentFingerprint {
        record_kind: ENVIRONMENT_FINGERPRINT_RECORD_KIND.to_owned(),
        notebook_kernel_discovery_schema_version: NOTEBOOK_KERNEL_DISCOVERY_SCHEMA_VERSION,
        fingerprint_id: "env.fp.python.312.fresh.01".to_owned(),
        environment_identity_label: "python3.12 + pandas2.3 + torch2.8".to_owned(),
        interpreter_resolution_ref: "interp.uv.python.312.01".to_owned(),
        package_summary_label: Some("pandas 2.3, torch 2.8, numpy 2.0".to_owned()),
        toolchain_summary_label: Some("uv 0.4, rust toolchain nightly".to_owned()),
        target_origin_label: "local_host".to_owned(),
        policy_epoch_ref: Some("policy.epoch.01".to_owned()),
        freshness_class: EnvironmentFingerprintFreshnessClass::Fresh,
        last_known_good_at: Some("2026-06-09T10:00:00Z".to_owned()),
        summary: "Fresh local environment fingerprint for Python 3.12.".to_owned(),
    }
}

fn sample_stale_fingerprint() -> EnvironmentFingerprint {
    EnvironmentFingerprint {
        record_kind: ENVIRONMENT_FINGERPRINT_RECORD_KIND.to_owned(),
        notebook_kernel_discovery_schema_version: NOTEBOOK_KERNEL_DISCOVERY_SCHEMA_VERSION,
        fingerprint_id: "env.fp.r.430.stale.01".to_owned(),
        environment_identity_label: "r4.3.0 + tidyverse 2.0".to_owned(),
        interpreter_resolution_ref: "interp.conda.r.430.01".to_owned(),
        package_summary_label: Some("tidyverse 2.0, dplyr 1.1".to_owned()),
        toolchain_summary_label: None,
        target_origin_label: "local_host".to_owned(),
        policy_epoch_ref: None,
        freshness_class: EnvironmentFingerprintFreshnessClass::Stale,
        last_known_good_at: Some("2026-05-01T10:00:00Z".to_owned()),
        summary: "Stale local environment fingerprint for R 4.3.0.".to_owned(),
    }
}

fn sample_policy_blocked_fingerprint() -> EnvironmentFingerprint {
    EnvironmentFingerprint {
        record_kind: ENVIRONMENT_FINGERPRINT_RECORD_KIND.to_owned(),
        notebook_kernel_discovery_schema_version: NOTEBOOK_KERNEL_DISCOVERY_SCHEMA_VERSION,
        fingerprint_id: "env.fp.remote.policy.01".to_owned(),
        environment_identity_label: "python3.11 + unmanaged-packages".to_owned(),
        interpreter_resolution_ref: "interp.remote.01".to_owned(),
        package_summary_label: None,
        toolchain_summary_label: None,
        target_origin_label: "managed_workspace:gpu-pool".to_owned(),
        policy_epoch_ref: Some("policy.epoch.blocked.01".to_owned()),
        freshness_class: EnvironmentFingerprintFreshnessClass::PolicyBlocked,
        last_known_good_at: None,
        summary: "Policy-blocked remote environment fingerprint.".to_owned(),
    }
}

fn sample_local_discovery_entry() -> KernelDiscoveryEntry {
    KernelDiscoveryEntry {
        record_kind: KERNEL_DISCOVERY_ENTRY_RECORD_KIND.to_owned(),
        notebook_kernel_discovery_schema_version: NOTEBOOK_KERNEL_DISCOVERY_SCHEMA_VERSION,
        entry_id: "discovery.entry.python.312.local.01".to_owned(),
        kernelspec: sample_python_kernelspec(),
        discovery_source_class: KernelspecDiscoverySourceClass::JupyterDataDir,
        interpreter_resolution_ref: "interp.uv.python.312.01".to_owned(),
        environment_fingerprint_ref: "env.fp.python.312.fresh.01".to_owned(),
        compatibility_class: KernelDiscoveryCompatibilityClass::Compatible,
        availability_class: KernelDiscoveryAvailabilityClass::Available,
        target_origin_label: "local_host".to_owned(),
        summary: "Compatible local Python 3.12 kernel discovered from Jupyter data dir.".to_owned(),
    }
}

fn sample_conda_discovery_entry() -> KernelDiscoveryEntry {
    KernelDiscoveryEntry {
        record_kind: KERNEL_DISCOVERY_ENTRY_RECORD_KIND.to_owned(),
        notebook_kernel_discovery_schema_version: NOTEBOOK_KERNEL_DISCOVERY_SCHEMA_VERSION,
        entry_id: "discovery.entry.r.430.conda.01".to_owned(),
        kernelspec: sample_r_kernelspec(),
        discovery_source_class: KernelspecDiscoverySourceClass::CondaEnv,
        interpreter_resolution_ref: "interp.conda.r.430.01".to_owned(),
        environment_fingerprint_ref: "env.fp.r.430.stale.01".to_owned(),
        compatibility_class: KernelDiscoveryCompatibilityClass::Compatible,
        availability_class: KernelDiscoveryAvailabilityClass::Available,
        target_origin_label: "local_host".to_owned(),
        summary: "Compatible conda R 4.3.0 kernel discovered from conda environment.".to_owned(),
    }
}

fn sample_remote_discovery_entry() -> KernelDiscoveryEntry {
    KernelDiscoveryEntry {
        record_kind: KERNEL_DISCOVERY_ENTRY_RECORD_KIND.to_owned(),
        notebook_kernel_discovery_schema_version: NOTEBOOK_KERNEL_DISCOVERY_SCHEMA_VERSION,
        entry_id: "discovery.entry.julia.110.remote.01".to_owned(),
        kernelspec: sample_julia_kernelspec(),
        discovery_source_class: KernelspecDiscoverySourceClass::ManagedWorkspace,
        interpreter_resolution_ref: "interp.managed.julia.110.01".to_owned(),
        environment_fingerprint_ref: "env.fp.julia.110.remote.01".to_owned(),
        compatibility_class: KernelDiscoveryCompatibilityClass::Compatible,
        availability_class: KernelDiscoveryAvailabilityClass::Busy,
        target_origin_label: "managed_workspace:compute".to_owned(),
        summary: "Compatible managed-workspace Julia 1.10 kernel; currently busy.".to_owned(),
    }
}

fn sample_policy_blocked_entry() -> KernelDiscoveryEntry {
    KernelDiscoveryEntry {
        record_kind: KERNEL_DISCOVERY_ENTRY_RECORD_KIND.to_owned(),
        notebook_kernel_discovery_schema_version: NOTEBOOK_KERNEL_DISCOVERY_SCHEMA_VERSION,
        entry_id: "discovery.entry.python.311.policy.01".to_owned(),
        kernelspec: Kernelspec {
            record_kind: KERNELSPEC_RECORD_KIND.to_owned(),
            notebook_kernel_discovery_schema_version: NOTEBOOK_KERNEL_DISCOVERY_SCHEMA_VERSION,
            kernelspec_id: "kernelspec.python.311.remote.01".to_owned(),
            display_name_label: "Python 3.11 (remote)".to_owned(),
            language_label: "python".to_owned(),
            launch_command_template_ref: "launch.cmd.python.311.remote.01".to_owned(),
            resource_dir_ref: None,
            metadata_ref: None,
            summary: "Remote Python 3.11 kernelspec.".to_owned(),
        },
        discovery_source_class: KernelspecDiscoverySourceClass::RemoteRegistry,
        interpreter_resolution_ref: "interp.remote.01".to_owned(),
        environment_fingerprint_ref: "env.fp.remote.policy.01".to_owned(),
        compatibility_class: KernelDiscoveryCompatibilityClass::PolicyNarrowed,
        availability_class: KernelDiscoveryAvailabilityClass::PolicyBlocked,
        target_origin_label: "managed_workspace:gpu-pool".to_owned(),
        summary: "Policy-blocked remote Python 3.11 kernel from remote registry.".to_owned(),
    }
}

#[test]
fn python_kernelspec_validates_clean() {
    let ks = sample_python_kernelspec();
    assert!(
        ks.validate().is_empty(),
        "python kernelspec should be clean: {:?}",
        ks.validate()
    );
}

#[test]
fn r_kernelspec_validates_clean() {
    let ks = sample_r_kernelspec();
    assert!(
        ks.validate().is_empty(),
        "r kernelspec should be clean: {:?}",
        ks.validate()
    );
}

#[test]
fn julia_kernelspec_validates_clean() {
    let ks = sample_julia_kernelspec();
    assert!(
        ks.validate().is_empty(),
        "julia kernelspec should be clean: {:?}",
        ks.validate()
    );
}

#[test]
fn kernelspec_rejects_empty_display_name() {
    let mut ks = sample_python_kernelspec();
    ks.display_name_label = "".to_owned();
    let findings = ks.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "kernelspec.display_name_label"));
}

#[test]
fn kernelspec_rejects_empty_language() {
    let mut ks = sample_python_kernelspec();
    ks.language_label = "".to_owned();
    let findings = ks.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "kernelspec.language_label"));
}

#[test]
fn uv_resolution_validates_clean() {
    let ir = sample_uv_resolution();
    assert!(
        ir.validate().is_empty(),
        "uv resolution should be clean: {:?}",
        ir.validate()
    );
}

#[test]
fn conda_resolution_validates_clean() {
    let ir = sample_conda_resolution();
    assert!(
        ir.validate().is_empty(),
        "conda resolution should be clean: {:?}",
        ir.validate()
    );
}

#[test]
fn unknown_resolution_validates_clean() {
    let ir = sample_unknown_resolution();
    assert!(
        ir.validate().is_empty(),
        "unknown resolution should be clean: {:?}",
        ir.validate()
    );
}

#[test]
fn resolution_rejects_empty_path_token() {
    let mut ir = sample_uv_resolution();
    ir.interpreter_path_token_ref = "".to_owned();
    let findings = ir.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "interpreter_resolution.interpreter_path_token_ref"));
}

#[test]
fn fresh_fingerprint_validates_clean() {
    let ef = sample_fresh_fingerprint();
    assert!(
        ef.validate().is_empty(),
        "fresh fingerprint should be clean: {:?}",
        ef.validate()
    );
}

#[test]
fn stale_fingerprint_validates_clean() {
    let ef = sample_stale_fingerprint();
    assert!(
        ef.validate().is_empty(),
        "stale fingerprint should be clean: {:?}",
        ef.validate()
    );
}

#[test]
fn policy_blocked_fingerprint_validates_clean() {
    let ef = sample_policy_blocked_fingerprint();
    assert!(
        ef.validate().is_empty(),
        "policy_blocked fingerprint should be clean: {:?}",
        ef.validate()
    );
}

#[test]
fn fresh_fingerprint_requires_last_known_good() {
    let mut ef = sample_fresh_fingerprint();
    ef.last_known_good_at = None;
    let findings = ef.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "environment_fingerprint.fresh_requires_last_known_good"));
}

#[test]
fn policy_blocked_fingerprint_requires_epoch() {
    let mut ef = sample_policy_blocked_fingerprint();
    ef.policy_epoch_ref = None;
    let findings = ef.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "environment_fingerprint.policy_blocked_requires_epoch"));
}

#[test]
fn fingerprint_rejects_empty_identity() {
    let mut ef = sample_fresh_fingerprint();
    ef.environment_identity_label = "".to_owned();
    let findings = ef.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "environment_fingerprint.environment_identity_label"));
}

#[test]
fn local_discovery_entry_validates_clean() {
    let entry = sample_local_discovery_entry();
    assert!(
        entry.validate().is_empty(),
        "local discovery entry should be clean: {:?}",
        entry.validate()
    );
}

#[test]
fn conda_discovery_entry_validates_clean() {
    let entry = sample_conda_discovery_entry();
    assert!(
        entry.validate().is_empty(),
        "conda discovery entry should be clean: {:?}",
        entry.validate()
    );
}

#[test]
fn remote_discovery_entry_validates_clean() {
    let entry = sample_remote_discovery_entry();
    assert!(
        entry.validate().is_empty(),
        "remote discovery entry should be clean: {:?}",
        entry.validate()
    );
}

#[test]
fn policy_blocked_discovery_entry_validates_clean() {
    let entry = sample_policy_blocked_entry();
    assert!(
        entry.validate().is_empty(),
        "policy blocked discovery entry should be clean: {:?}",
        entry.validate()
    );
}

#[test]
fn entry_rejects_policy_blocked_with_compatible() {
    let mut entry = sample_policy_blocked_entry();
    entry.compatibility_class = KernelDiscoveryCompatibilityClass::Compatible;
    let findings = entry.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "kernel_discovery_entry.policy_blocked_compatible"));
}

#[test]
fn entry_rejects_remote_source_with_local_origin() {
    let mut entry = sample_remote_discovery_entry();
    entry.discovery_source_class = KernelspecDiscoverySourceClass::RemoteRegistry;
    entry.target_origin_label = "local_host".to_owned();
    let findings = entry.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "kernel_discovery_entry.remote_source_local_origin"));
}

#[test]
fn packet_validates_clean() {
    let packet = KernelDiscoveryPacket {
        schema_version: NOTEBOOK_KERNEL_DISCOVERY_SCHEMA_VERSION,
        record_kind: KERNEL_DISCOVERY_PACKET_RECORD_KIND.to_owned(),
        packet_id: "nb.kernel_discovery.packet.m5.01".to_owned(),
        as_of: "2026-06-09T00:00:00Z".to_owned(),
        kernelspec_discovery_source_classes: KernelspecDiscoverySourceClass::ALL.to_vec(),
        interpreter_manager_classes: InterpreterManagerClass::ALL.to_vec(),
        environment_fingerprint_freshness_classes: EnvironmentFingerprintFreshnessClass::ALL
            .to_vec(),
        kernel_discovery_compatibility_classes: KernelDiscoveryCompatibilityClass::ALL.to_vec(),
        kernel_discovery_availability_classes: KernelDiscoveryAvailabilityClass::ALL.to_vec(),
        example_kernelspecs: vec![
            sample_python_kernelspec(),
            sample_r_kernelspec(),
            sample_julia_kernelspec(),
        ],
        example_interpreter_resolutions: vec![
            sample_uv_resolution(),
            sample_conda_resolution(),
            sample_unknown_resolution(),
        ],
        example_environment_fingerprints: vec![
            sample_fresh_fingerprint(),
            sample_stale_fingerprint(),
            sample_policy_blocked_fingerprint(),
        ],
        example_kernel_discovery_entries: vec![
            sample_local_discovery_entry(),
            sample_conda_discovery_entry(),
            sample_remote_discovery_entry(),
            sample_policy_blocked_entry(),
        ],
        summary:
            "Kernel discovery, kernelspec, interpreter resolution, and environment fingerprint inspectors packet v1."
                .to_owned(),
    };
    assert!(
        packet.validate().is_empty(),
        "packet should be clean: {:?}",
        packet.validate()
    );
}

#[test]
fn embedded_packet_parses() {
    let packet = current_kernel_discovery_packet().expect("embedded packet must parse");
    assert_eq!(
        packet.schema_version,
        NOTEBOOK_KERNEL_DISCOVERY_SCHEMA_VERSION
    );
    assert_eq!(packet.record_kind, KERNEL_DISCOVERY_PACKET_RECORD_KIND);
}

#[test]
fn closed_vocabularies_expose_stable_tokens() {
    assert_eq!(
        KernelspecDiscoverySourceClass::JupyterDataDir.as_str(),
        "jupyter_data_dir"
    );
    assert!(KernelspecDiscoverySourceClass::ManagedWorkspace.is_remote_source());
    assert!(!KernelspecDiscoverySourceClass::VirtualEnv.is_remote_source());

    assert_eq!(InterpreterManagerClass::Uv.as_str(), "uv");
    assert_eq!(InterpreterManagerClass::Unknown.as_str(), "unknown");

    assert_eq!(
        EnvironmentFingerprintFreshnessClass::PolicyBlocked.as_str(),
        "policy_blocked"
    );

    assert_eq!(
        KernelDiscoveryCompatibilityClass::IncompatibleLanguage.as_str(),
        "incompatible_language"
    );

    assert_eq!(
        KernelDiscoveryAvailabilityClass::PolicyBlocked.as_str(),
        "policy_blocked"
    );
}
