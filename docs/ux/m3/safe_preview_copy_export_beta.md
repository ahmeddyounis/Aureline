# Safe Preview Copy and Export Beta

This contract closes the beta gap between safe-preview trust classes and the
representation that leaves, opens outside, or enters a support bundle. It
extends the existing suspicious-content and copy/export parity contracts to the
risky beta surfaces where a generic `Copy`, `Export`, `Open in browser`, or
`Attach to support` label can change security or support meaning.

Machine-readable companions:

- [`schemas/content/representation_export.schema.json`](../../../schemas/content/representation_export.schema.json)
- [`crates/aureline-content-safety/src/representation_labels/`](../../../crates/aureline-content-safety/src/representation_labels)
- [`fixtures/content/m3/representation_copy_export/`](../../../fixtures/content/m3/representation_copy_export)

## Covered Surfaces

The beta packet covers these surface kinds:

| Surface | Required truth |
|---|---|
| Docs/help pane | Trust class, raw/rendered label, browser handoff packet, sanitized export |
| Suspicious-content view | Warning refs, raw source, sanitized export, redacted support evidence |
| Generated artifact | Generated origin, citation anchors, rendered copy, sanitized/redacted export |
| Package/install review | Strict raw display, warning refs, blocked browser/deeper render when policy denies |
| Repair preview | Windowing/omission disclosure before copy, export, or support attachment |
| Support-bundle attachment | Sanitized or redacted attachment only; raw/rendered bodies do not attach by default |

## Representation Labels

Every protected action names the representation as one of:

- `raw`
- `rendered`
- `sanitized`
- `redacted`

Trust class remains separate from representation. A `SanitizedRich` surface may
copy `rendered`, export `sanitized`, and attach `redacted`; each action keeps the
source trust class visible in its structured packet.

## Action Families

Each covered surface maps the current trust class to these action families:

- `copy`
- `export`
- `browser_open`
- `deeper_render_override`
- `support_attachment`

Browser handoff is available only with policy and trust-class permission and a
handoff packet. Deeper render override is available only with policy and
trust-class permission and a review ref. Blocked rows are still represented so
support and automation can distinguish a missing affordance from a denied one.

## Green Gate

Validation is green only when:

- every covered surface is present exactly once;
- raw/rendered ambiguous content exposes both raw and rendered labels;
- suspicious content keeps warning refs and raw, sanitized, and redacted paths;
- oversized artifacts disclose windowing, omission, or metadata-only scope;
- generated output keeps generated origin and citation anchors on transferable actions;
- support attachments use only sanitized or redacted support-safe representations;
- redacted rows state that redaction was applied;
- sanitized rows disclose active-content removal; and
- copy/export/browser/support records preserve trust class, representation label, and source subject refs.

Run:

```sh
cargo test -p aureline-content-safety --test representation_labels_beta
cargo run -q -p aureline-content-safety --bin representation_labels_beta -- fixtures/content/m3/representation_copy_export/risky_beta_surfaces.json
```
