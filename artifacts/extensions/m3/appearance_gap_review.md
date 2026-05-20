# Extension appearance gap review

Generated from `fixtures/extensions/m3/appearance_inheritance/conformance_packet.json`.

## Summary

| Metric | Value |
| --- | ---: |
| Rows | 4 |
| Support rows | 4 |
| Conformant rows | 3 |
| Needs-review rows | 1 |
| Refused rows | 0 |
| Fully inherited rows | 1 |
| Reduced-support rows | 2 |
| Private-styling rows | 1 |
| Undisclosed rows | 0 |
| Overclaimed rows | 0 |
| Defects | 0 |

## Rows

| Surface | Overall support | Decision | Result-row badge |
| --- | --- | --- | --- |
| Markdown preview pane | `full_inheritance` | `conformant` | Inherits host appearance |
| Insights dashboard panel | `reduced_support` | `conformant` | Reduced appearance support |
| Theme settings surface | `reduced_support` | `needs_review` | Appearance support unverified |
| Custom toolbar surface (mirrored) | `unsupported_private_styling` | `conformant` | Private styling (no host inheritance) |

## Per-axis conformance

| Surface | theme | density | focus_ring | high_contrast | reduced_motion | host_token |
| --- | --- | --- | --- | --- | --- | --- |
| Markdown preview pane | full | full | full | full | full | full |
| Insights dashboard panel | full | reduced | full | reduced | full | full |
| Theme settings surface | full | full | full | reduced | full | full |
| Custom toolbar surface (mirrored) | full | unsupported | unsupported | unsupported | reduced | unsupported |

## Findings

- The Markdown preview pane declares `inherits` on every axis and the
  host probe proves it, so the row earns `full_inheritance` and is the
  only row whose surface caveats may imply full inheritance.
- The insights dashboard panel inherits theme, focus, motion, and tokens
  but declares reduced density and high-contrast support; both reduced
  axes are proven, so the row is conformant with disclosed caveats.
- The theme settings surface claims full inheritance, but high-contrast
  parity is `unproven`. The runtime downgrades the row to reduced support
  and routes it to `needs_review` rather than badging it compatible.
- The mirrored custom toolbar inherits theme color only and declares
  private styling for density, focus, contrast, and tokens. The
  disclosure is honest, so the row is conformant while clearly labeled as
  private styling on every surface.
- No row implies full inheritance without proof, no claim is contradicted
  by a host probe, and host-stable trust, severity, permission, and
  policy labels stay host-rendered on every row.

## Regenerate

```text
cargo run -q -p aureline-extensions --example dump_appearance_conformance_records -- packet
```
