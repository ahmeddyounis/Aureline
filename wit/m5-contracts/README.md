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

## Versioned packages

Each capability world is a versioned WIT package (`aureline:<slug>@<semver>`).
The canonical 0.1.0 worlds live under [`wit/aureline/`](../aureline/). This
directory carries published successor versions:

- [`editor-read-0.2.0.wit`](editor-read-0.2.0.wit) — `aureline:editor-read@0.2.0`,
  the additive-minor successor to `aureline:editor-read@0.1.0`. It preserves every
  0.1.0 item byte-compatible and only adds `visible-range`, `word-at`, and the
  `visibility-range` record. A 0.1.0 guest runs unchanged on a 0.2.0 host; a 0.1.0
  host narrows a 0.2.0 guest to the 0.1.0 surface rather than denying it.

The machine-readable publication packet at
[`artifacts/contracts/m5-wit-contract-publication.json`](../../artifacts/contracts/m5-wit-contract-publication.json)
records every published package with its lifecycle label (`stable` / `beta` /
`experimental` / `deprecated` / `retired`), compatibility note, and
predecessor/successor links; the host/guest negotiation fixtures under
[`fixtures/contracts/m5-wit-negotiation/`](../../fixtures/contracts/m5-wit-negotiation/)
prove supported, downgraded, deprecated, and unsupported-skew behaviour; and the
capability-diff report at
[`artifacts/contracts/m5-wit-capability-diff.md`](../../artifacts/contracts/m5-wit-capability-diff.md)
shows what changed between versions. The regenerator and validator are
`tools/regenerate_m5_wit_contract_publication.py` and
`tools/validate_m5_wit_contract_publication.py`.

The compatibility-surface row for this contract is
`extensions.wit_host_worlds_and_bindings`
(`artifacts/governance/compatibility_surfaces.yaml`) and the qualification row is
`compat_row:extension_host.sdk_wit_permission_window`
(`artifacts/compat/qualification_matrix_seed.yaml`).
