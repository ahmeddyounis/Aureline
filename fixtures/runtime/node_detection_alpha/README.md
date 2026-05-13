# Node detector alpha fixtures

These protected fixtures exercise the read-only Node/toolchain detector for
the TS/JS launch wedge. The detector reads manifests, version files, and
lockfiles only; it does not execute package-manager binaries or repo-owned
activation hooks.

| Fixture | Acceptance state |
|---|---|
| `pnpm_package_manager_wins` | `package.json#packageManager` selects pnpm, while an npm lockfile is preserved as a lower-precedence conflicting source. |
| `ambiguous_lockfiles` | `pnpm-lock.yaml` and `package-lock.json` appear without a package-manager pin, so package-manager selection remains ambiguous. |
| `fallback_npm` | No repo package-manager pin exists, so the detector exposes npm as a fallback before launch. |
| `ambiguous_node_runtime` | Same-precedence Node version files disagree, so Node runtime selection remains ambiguous. |

Verification:

```sh
cargo test -p aureline-runtime node
```
