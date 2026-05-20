# Extension inheritance-gap packet

Generated from `fixtures/ux/m3/appearance_evidence/appearance_evidence_packet.json`. Do not edit by hand; refresh the upstream truth and re-run the generator.

Each row names an exact contributed surface, its unsupported appearance states, the downgrade note shown to users, and the remediation link. Host-stable trust, severity, permission, and policy labels stay host-rendered on every row.

## Rows

| Extension | Surface | Support | Decision | Persists after install | Remediation |
| --- | --- | --- | --- | --- | --- |
| dev.aureline.samples/markdown-lens | Markdown preview pane | `full_inheritance` | `conformant` | yes | `docs/extensions/m3/appearance_conformance_beta.md` |
| com.acme.dashboards | Insights dashboard panel | `reduced_support` | `conformant` | yes | `docs/extensions/m3/appearance_conformance_beta.md` |
| io.contrib.theme-extras | Theme settings surface | `reduced_support` | `needs_review` | yes | `docs/extensions/m3/appearance_conformance_beta.md#declaration-joined-with-host-proof` |
| net.legacy.toolbar | Custom toolbar surface | `unsupported_private_styling` | `conformant` | yes | `docs/extensions/m3/appearance_conformance_beta.md` |

### Markdown preview pane — Markdown Lens

- Surface id: `surface:markdown-lens:preview-pane`
- Publisher: Aureline Samples
- Lifecycle: `stable`
- Overall support: `full_inheritance`
- Decision: `conformant` (full_inheritance_proven)
- Downgrade note: Inherits host appearance across theme, density, focus, contrast, motion, and tokens.
- Host-stable labels: trust=Trusted publisher; severity=No active warnings; permission=Read-only workspace docs; policy=Allowed by org policy
- Remediation: `docs/extensions/m3/appearance_conformance_beta.md`
- Upstream row: `fixtures/extensions/m3/appearance_inheritance/conformance_packet.json#extension-appearance-conformance:dev.aureline.samples/markdown-lens:preview-pane`

### Insights dashboard panel — Acme Insights

- Surface id: `surface:acme-dashboards:insights-panel`
- Publisher: Acme Cloud, Inc.
- Lifecycle: `beta`
- Overall support: `reduced_support`
- Decision: `conformant` (reduced_support_disclosed)
- Downgrade note: Inherits most host appearance; declares reduced density and high-contrast support in charts.
- Host-stable labels: trust=Verified publisher; severity=No active warnings; permission=Workspace read + network status; policy=Allowed by org policy
- Gap axes:
  - `density` -> `reduced_support`: Density inherits host tokens partially.
  - `high_contrast` -> `reduced_support`: High contrast inherits host tokens partially.
- Known unsupported states:
  - `density` / `compact_density`: Compact density keeps a fixed chart row height.
  - `high_contrast` / `forced_colors_dark`: Chart series colors are fixed under forced-colors dark.
- Remediation: `docs/extensions/m3/appearance_conformance_beta.md`
- Upstream row: `fixtures/extensions/m3/appearance_inheritance/conformance_packet.json#extension-appearance-conformance:com.acme.dashboards:insights-panel`

### Theme settings surface — Theme Extras

- Surface id: `surface:theme-extras:settings-surface`
- Publisher: Community Contributor
- Lifecycle: `beta`
- Overall support: `reduced_support`
- Decision: `needs_review` (needs_verification_before_badge)
- Downgrade note: Claims full inheritance; high-contrast parity not yet proven, so the badge stays unverified.
- Host-stable labels: trust=Community publisher; severity=Beta surface; permission=Settings read + write; policy=Allowed by org policy
- Gap axes:
  - `high_contrast` -> `reduced_support` (needs verification): High contrast inheritance is claimed but not yet proven by a host probe.
- Remediation: `docs/extensions/m3/appearance_conformance_beta.md#declaration-joined-with-host-proof`
- Upstream row: `fixtures/extensions/m3/appearance_inheritance/conformance_packet.json#extension-appearance-conformance:io.contrib.theme-extras:settings-surface`

### Custom toolbar surface — Legacy Toolbar

- Surface id: `surface:legacy-toolbar:custom-toolbar`
- Publisher: Legacy Tools (mirrored)
- Lifecycle: `limited`
- Overall support: `unsupported_private_styling`
- Decision: `conformant` (unsupported_private_styling_disclosed)
- Downgrade note: Inherits theme color only; density, focus, contrast, and tokens use private styling.
- Host-stable labels: trust=Mirrored — publisher continuity limited; severity=Reduced appearance support; permission=Workspace read; policy=Allowed for mirrored catalog
- Gap axes:
  - `density` -> `unsupported_private_styling`: Density uses private styling, not host tokens.
  - `focus_ring` -> `unsupported_private_styling`: Focus ring uses private styling, not host tokens.
  - `high_contrast` -> `unsupported_private_styling`: High contrast uses private styling, not host tokens.
  - `reduced_motion` -> `reduced_support`: Reduced motion inherits host tokens partially.
  - `host_token` -> `unsupported_private_styling`: Host tokens uses private styling, not host tokens.
- Known unsupported states:
  - `focus_ring` / `keyboard_focus_ring`: Toolbar buttons draw a custom focus outline, not the host ring.
  - `high_contrast` / `forced_colors`: Toolbar icons keep fixed colors under forced-colors modes.
  - `host_token` / `spacing_tokens`: Toolbar spacing uses a private scale instead of host tokens.
- Remediation: `docs/extensions/m3/appearance_conformance_beta.md`
- Upstream row: `fixtures/extensions/m3/appearance_inheritance/conformance_packet.json#extension-appearance-conformance:net.legacy.toolbar:custom-toolbar`

## Verify

```sh
python3 ci/check_m3_appearance_evidence.py --repo-root . --check
```
