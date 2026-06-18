# Extension appearance-inheritance audit

Every extension-backed or embedded surface declares whether it inherits Aureline's theme, focus, contrast, density, and reduced-motion semantics. The badge below is rendered in extension details, embedded panes, diagnostics, and support/export packets.

## Summary

| Metric | Count |
| ------ | ----- |
| Descriptors | 5 |
| Inherits appearance | 1 |
| Partly inherits | 3 |
| Does not inherit | 1 |
| Undisclosed | 0 |
| Host-parity claims granted | 1 |
| Partial parity claims | 1 |
| Parity claims denied | 0 |
| Defects | 0 |

## Descriptors

| Surface | Package | Badge | Parity claim |
| ------- | ------- | ----- | ------------ |
| Markdown preview (Preview pane) | dev.aureline.samples/markdown-lens | Inherits Aureline appearance | claims_host_parity |
| Insights dashboard (Embedded webview) | com.acme.insights/analytics | Partly inherits appearance | no_parity_claim |
| Legacy console panel (Provider panel) | io.devtools.legacy/console | Does not inherit appearance | no_parity_claim |
| API reference (Docs/help pane) | dev.aureline.samples/api-docs | Partly inherits appearance | partial_claim_with_gaps |
| Insights diagnostics (Diagnostics pane) | com.acme.insights/analytics | Partly inherits appearance | no_parity_claim |

## Per-axis posture

| Surface | Theme | Focus | High contrast | Density | Reduced motion |
| ------- | ----- | ----- | ------------- | ------- | -------------- |
| Markdown preview | inherits | inherits | inherits | inherits | inherits |
| Insights dashboard | inherits | does_not_inherit | partial | inherits | partial |
| Legacy console panel | does_not_inherit | does_not_inherit | does_not_inherit | does_not_inherit | does_not_inherit |
| API reference | inherits | inherits | inherits | partial | inherits |
| Insights diagnostics | inherits | inherits | inherits | partial | inherits |

## Findings

No defects: no surface overclaims parity, no axis is undisclosed, and the host renders every appearance badge.

Regenerate: `cargo run -q -p aureline-extensions --example dump_extension_appearance_descriptor_records -- markdown > artifacts/ux/m5/extension-appearance-audit/extension_appearance_audit.md`
