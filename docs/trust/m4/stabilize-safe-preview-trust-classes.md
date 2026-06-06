# Stable safe-preview trust classes

This document freezes the stable safe-preview contract consumed by editor,
docs/help preview, notebook rich output, preview/runtime, embedded
marketplace/account webviews, browser-runtime viewers, support/export, and
trust-sensitive review surfaces.

Machine-readable sources:

- `schemas/trust/safe-preview-trust-class.schema.json`
- `fixtures/trust/m4/stabilize-safe-preview-trust-classes/canonical_packet.json`
- `crates/aureline-content-safety/src/stable_safe_preview_trust.rs`

The shared contract ref is
`content-safety:stable_safe_preview_trust:v1`.

## Trust-Class Ladder

The stable ladder is closed:

| Trust class | Allowed behavior | Default transfer posture | Downgrade posture |
|---|---|---|---|
| `RawText` | Exact bytes/text with reveal and warning overlays only | `copy_raw`, `copy_escaped`, metadata-only when bytes cannot leave | Static snapshot or metadata-only |
| `SanitizedRich` | Sanitized markup, no active script or ambient privilege | `copy_rendered`, `copy_raw` when source is available, sanitized snapshot export | Static snapshot or metadata-only |
| `TrustedLocalActive` | Active local behavior inside a declared local capability sandbox | Rendered copy for the live view, sanitized snapshot for export, raw path when source exists | Sanitized, static snapshot, or metadata-only |
| `IsolatedRemoteActive` | Active remote or embedded behavior inside a declared isolated boundary | Rendered copy only while origin is verified; sanitized or metadata-only export | Static snapshot, metadata-only, or blocked |

Sanitized rendering is not trust elevation. A richer renderer must satisfy the
destination class requirements and may not auto-upgrade because focus changes,
content finishes loading, or a widget becomes available.

## Required Consumer Matrix

Every stable row in the packet must expose:

- trust class;
- visible representation label;
- owner identity;
- origin or host boundary;
- raw-view path when the source representation exists and raw/rendered meaning
  can differ;
- capability and permission summary for active classes;
- downgrade trigger handling;
- screenshot, support-bundle, exported-evidence, and diagnostics lineage.

The stable matrix covers:

- editor;
- docs/help preview;
- notebook rich output;
- preview/runtime;
- marketplace/account webview;
- browser-runtime viewer;
- support/export;
- install, attach, approval, publish, and delete review.

The packet also carries a below-Stable embedded-widget drill. It proves that a
surface with missing origin or capability truth is narrowed rather than borrowing
Stable from an adjacent embedded row.

## Copy And Export

Copy/export actions name the transferred representation separately from trust
class. `Copy raw` and `Copy rendered` are both required when the distinction is
meaningful and the source representation is safely available. Remote and embedded
surfaces that cannot expose source bytes must keep origin and permission truth
and fall back to sanitized or metadata-only export instead of pretending raw
source exists.

Support/export boundaries never carry active content as active. The admissible
support/export results are sanitized snapshots or metadata-only envelopes with
trust-class, origin, permission, and downgrade lineage.

## Downgrade Truth

Active content must visibly narrow on:

- trust loss;
- policy deny;
- disconnect;
- origin loss;
- unsupported host;
- blocked script, widget, or active capability;
- support/export boundary.

The stable states after downgrade are sanitized, static snapshot, metadata-only,
or blocked. Silent failure and hidden best-effort rendering are non-conforming.

## Verification

Run:

```sh
cargo test -p aureline-content-safety --test stable_safe_preview_trust
cargo run -q -p aureline-content-safety --bin stable_safe_preview_trust -- validate
```

Regenerate the canonical packet after an intentional model change:

```sh
cargo run -q -p aureline-content-safety --bin stable_safe_preview_trust -- packet > fixtures/trust/m4/stabilize-safe-preview-trust-classes/canonical_packet.json
```
