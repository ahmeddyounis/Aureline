# Install / update / About truth fixtures

Worked `install_truth_record` fixtures for the install / update / About
truth packet. Each fixture binds one install-profile card from
[`/artifacts/release/install_topology_matrix.yaml`](../../../artifacts/release/install_topology_matrix.yaml)
and one channel-identity row from
[`/artifacts/release/channel_identity_and_state_roots.yaml`](../../../artifacts/release/channel_identity_and_state_roots.yaml)
to the surface projections frozen by
[`/artifacts/release/install_update_about_truth_packet.md`](../../../artifacts/release/install_update_about_truth_packet.md).

If a fixture and the truth packet disagree, the truth packet wins and
the fixture updates in the same change. If a fixture's referenced card,
state-root, channel-identity, update-marker, file-association, or
protocol-handler row disagrees with this fixture, the upstream artifact
wins and the fixture updates in the same change.

## Files

| Fixture | Surface |
|---|---|
| `desktop_stable_about_truth.yaml` | Per-user installed Stable on Windows; demonstrates baseline About / update / diagnostics projection. |
| `desktop_preview_side_by_side_about_truth.yaml` | Side-by-side Preview on macOS coexisting with Stable; demonstrates the channel-ownership audit pairing for `stable_and_preview`. |
| `desktop_lts_admin_about_truth.yaml` | Per-machine long-support install on Windows; demonstrates admin-only handler default and admin-pinned rollback target. |
| `managed_with_portable_neighbor_about_truth.yaml` | Managed-deployed Stable on Linux with a portable Stable neighbor; demonstrates the `managed_and_portable` audit row and the portable limitation refs. |
| `portable_zip_about_truth.yaml` | Portable Stable on Linux; demonstrates the full portable-mode limitation citation and the `installed_and_portable` audit row. |
| `external_package_manager_about_truth.yaml` | macOS install owned by an external package manager; demonstrates `updater_owner_class == external_package_manager` suppressing the in-product update affordance. |
| `offline_bundle_about_truth.yaml` | Offline bundle on the air-gap target row; demonstrates the offline-bundle mirror metadata diagnostics path and rollback evidence. |
