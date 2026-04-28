# Install-matrix seed cases

These fixtures exercise
[`docs/release/packaging_installation_matrix.md`](../../../docs/release/packaging_installation_matrix.md)
and its machine-readable companions:

- [`artifacts/release/install_artifact_families.yaml`](../../../artifacts/release/install_artifact_families.yaml)
- [`artifacts/release/channel_identity_and_state_roots.yaml`](../../../artifacts/release/channel_identity_and_state_roots.yaml)

Each file is an `install_matrix_case_record`. The records are seed
cases rather than a separate schema family; they cite artifact-family
rows, channel-identity rows, state-root refs, coexistence rules, and
rollback requirements already frozen in the companion artifacts.

Every case:

- names one stable `case_id`;
- names the package family or helper runtime under test;
- cites the channel identity rows and state-root refs the case relies on;
- declares the expected result (`promote`, `hold`, `block`, or
  `repair_required`);
- lists forbidden collisions that the installer, updater, support, or
  diagnostics surface must catch; and
- includes reviewer-facing summaries for release, support, and
  enterprise operators.

## Case list

- `stable_preview_desktop_coexistence.yaml` covers Stable + Preview
  desktop packages on one host.
- `portable_zip_no_host_mutation.yaml` covers portable archive behavior
  with no machine-global mutation.
- `managed_install_with_portable_neighbor.yaml` covers a managed
  install next to a portable install.
- `macos_bundle_identity_and_scheme.yaml` covers macOS bundle identity,
  notarization posture, and scheme separation.
- `linux_package_manager_channel_identity.yaml` covers DEB/RPM package
  manager ownership, XDG state separation, and mirror posture.
- `remote_helper_runtime_rollback.yaml` covers remote agent/helper
  tarballs and image-layer rollback.
- `state_schema_downgrade_requires_journal.yaml` covers blocked
  downgrade until backup, migration journal, or repair evidence exists.
