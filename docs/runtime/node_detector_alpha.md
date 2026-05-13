# Node detector alpha

This lane adds a read-only detector for TS/JS launch-wedge repositories.
The implementation lives in
[`crates/aureline-runtime/src/detectors/node`](../../crates/aureline-runtime/src/detectors/node)
and emits a `node_toolchain_detection_record` that can be embedded on the
canonical execution context before task, test, or debug launch.

## Contract

The detector reads repository declarations and caller-provided ambient
facts. It never runs repo-owned hooks, shell startup files, Node managers,
or package-manager binaries.

Runtime precedence:

1. action-local override
2. repo exact pins: `package.json#volta.node`, `.nvmrc`,
   `.node-version`, `.tool-versions`, `mise.toml`
3. `package.json#engines.node`
4. user/profile default
5. captured ambient `node` fact
6. detector fallback

Package-manager precedence:

1. action-local override
2. `package.json#packageManager` or `package.json#volta`
3. `.tool-versions` or `mise.toml`
4. lockfiles (`pnpm-lock.yaml`, `package-lock.json`,
   `npm-shrinkwrap.json`)
5. user/profile default
6. captured ambient or detector fallback

When same-precedence sources disagree, the report records an
`unresolved_ambiguities[]` row and leaves that subject in the
`ambiguous` state instead of choosing a silent winner. Lower-precedence
conflicts are preserved as provenance cards so the execution-context
inspector can explain why the winning source was used.

## Inspector surface

`ExecutionContext::with_node_toolchain_detection` attaches the detector
record to the canonical context. The shell execution-context inspector
adds a **Node detector** section when that report is present, showing:

- Node runtime state, selected value, winning source, and fallback path
- package-manager state, selected value, winning source, and fallback path
- unresolved ambiguity rows
- one provenance-card row per observed source

Fallback, missing, unsupported, and ambiguous states add visible honesty
markers before dispatch.

## Fixtures

Protected fixture repos live under
[`fixtures/runtime/node_detection_alpha`](../../fixtures/runtime/node_detection_alpha):

- `pnpm_package_manager_wins`
- `ambiguous_lockfiles`
- `fallback_npm`
- `ambiguous_node_runtime`

## How to verify

```sh
cargo test -p aureline-runtime node
cargo test -p aureline-shell node_detection
```
