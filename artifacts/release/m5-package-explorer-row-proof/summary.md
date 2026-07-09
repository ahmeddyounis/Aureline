# Package Explorer Rows

- Packet: `package-explorer-row:stable:0001`
- Surface: `Package explorer rows`
- Rows: 7 (3 directly actionable, 1 transitive)
- Proof freshness SLO: 168 hours (last refresh: 2026-07-08T00:00:00Z)

## Rows

- **left-pad** (npm) `installed`: direct in runtime_dependency — action `manage_installed` [public_registry]
- **chalk** (npm) `available`: direct in runtime_dependency — action `install_available` [enterprise_mirror]
- **lodash** (npm) `outdated`: direct in runtime_dependency — action `update_available` [public_registry]
- **ms** (npm) `installed`: transitive in runtime_dependency — action `transitive_read_only` [public_registry]
- **vendored-icons** (npm) `imported`: direct in runtime_dependency — action `imported_read_only` [path_or_vendored]
- **openssl-sys** (npm) `policy_pinned`: direct in runtime_dependency — action `policy_pinned_blocked` [public_registry]
- **react** (npm) `remove_blocked`: direct in runtime_dependency — action `remove_blocked` [public_registry]
