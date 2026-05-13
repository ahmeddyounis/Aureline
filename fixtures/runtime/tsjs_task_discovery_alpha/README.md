# TS/JS Task Discovery Fixtures

These fixtures exercise package-script discovery for TS/JS launch-wedge
workspaces. The runtime tests read these manifests directly, attach the Node
detector output to the canonical execution context, and project package-script
run contracts through the shared task-event stream.

| Fixture | Acceptance state |
|---|---|
| `ready_pnpm` | `package.json#packageManager` and `volta.node` resolve a ready pnpm runner; build/test scripts launch through direct package-manager argv. |
| `missing_node_runtime` | Scripts are present, but no Node or package-manager runtime can be resolved from workspace or ambient facts. |
| `unsupported_yarn` | Scripts remain discoverable, but the package manager is outside the bounded npm/pnpm launch contract. |

Verification:

```sh
cargo test -p aureline-runtime package_scripts
```
