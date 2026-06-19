# M5 WIT contract index

This directory is the M5 publication index for the component-model **WIT world
package** contract form. It does not redefine the worlds — the canonical WIT
worlds and host capability bindings live in [`wit/aureline/`](../aureline/):

- [`aureline.wit`](../aureline/aureline.wit) — top-level world.
- [`editor-read.wit`](../aureline/editor-read.wit)
- [`workspace-read.wit`](../aureline/workspace-read.wit)
- [`diff-apply-preview.wit`](../aureline/diff-apply-preview.wit)
- [`terminal-observe.wit`](../aureline/terminal-observe.wit)
- [`network-egress.wit`](../aureline/network-egress.wit)

The `extension_host_wit_world` row of the M5 public-contract matrix
(`artifacts/contracts/m5-stability-lifecycle-map.json`,
`artifacts/contracts/m5-public-contract-matrix.md`) records the publication
requirements this contract form must satisfy before it can carry a Stable
contract claim: the WIT world package, a Markdown summary, an example world, and
a validator suite. The matrix is the source of truth; this index only points at
the published worlds so SDK/docs and support-export surfaces consume one map.

The compatibility-surface row for this contract is
`extensions.wit_host_worlds_and_bindings`
(`artifacts/governance/compatibility_surfaces.yaml`) and the qualification row is
`compat_row:extension_host.sdk_wit_permission_window`
(`artifacts/compat/qualification_matrix_seed.yaml`).
