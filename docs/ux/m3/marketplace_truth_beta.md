# Marketplace Truth Rows Beta

Marketplace discovery rows must expose trust, compatibility, lifecycle,
support, and mirror posture before install review opens. The implementation
lives in `crates/aureline-extensions/src/marketplace_truth/` with the shell
consumer in `crates/aureline-shell/src/extensions/marketplace/`.

## Source Of Truth

| Concern | Source |
|---|---|
| Publisher, registry source, moderation, revocation, and mirrorability | `CatalogDescriptorRecord` |
| Current support class and downgrade triggers | `artifacts/compat/m3/compatibility_report.json` |
| Marketplace row and support export shape | `schemas/extensions/marketplace_truth.schema.json` |
| Shell/headless projection fixtures | `fixtures/ux/m3/marketplace_truth/` |

Compatibility labels on the row are generated from the current compatibility
report row and catalog state. A catalog descriptor can carry an `exact` label,
but the marketplace row narrows it to `limited` when the current generated
report only claims experimental support.

## Controlled Badges

The row vocabulary is closed to:

| Badge | Meaning |
|---|---|
| `preview` | Row is staged, review-only, or pending moderation. |
| `beta` | Version or claim is beta-facing. |
| `stable` | Stable row backed by current support evidence. |
| `deprecated` | Present with a visible sunset path. |
| `limited` | Narrower guarantees apply. |
| `revoked` | Install and update are blocked. |
| `mirrored` | Row is mirrorable or served from an approved/private/offline source. |
| `retest_pending` | Current evidence must be refreshed before install/update. |

Support-class chips are separate from lifecycle badges. For example, a beta
extension row may render `beta`, `limited`, and `mirrored` while the support
chip says `experimental`.

## Validation Rules

- Every row must include at least one lifecycle badge, one support-class chip,
  and one trust/source chip.
- Every row must cite the generated compatibility report row used to compute
  the compatibility label.
- A `retest_pending` badge must render a `retest_pending` compatibility label.
- A `revoked` badge must block install and update.
- Support exports must quote the row badges, compatibility label, support
  chips, trust chips, and install-review ref without drift.

## Verification

```sh
cargo test -p aureline-extensions marketplace_truth
cargo test -p aureline-shell extensions::marketplace
cargo test -p aureline-shell --test marketplace_truth_beta_fixtures
cargo run -q -p aureline-shell --bin aureline_shell_marketplace_truth -- validate
```
