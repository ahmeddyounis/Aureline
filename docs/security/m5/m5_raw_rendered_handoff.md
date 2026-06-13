# M5 Raw-versus-Rendered Representation & Handoff

This document is the contract for raw-versus-rendered representation honesty and
handoff preservation across the new M5 review, docs, AI, and structured viewer
surfaces. The prior lanes in `aureline-content-safety` own byte-level
suspicious-content findings and project one threat-class vocabulary across the
new M5 panes. This lane covers the orthogonal honesty gap they leave open: a
viewer that *renders* content produces a display form that differs *materially*
from the canonical raw bytes. Without explicit labels and copy/export semantics,
that rendered text or summarized output can masquerade as the raw source during
copy, export, or support handoff.

- Record kind: `m5_raw_rendered_handoff_packet`
- Schema: [`schemas/security/m5-raw-rendered-handoff.schema.json`](../../../schemas/security/m5-raw-rendered-handoff.schema.json)
- Canonical support export: [`artifacts/security/m5/m5_raw_rendered_handoff/support_export.json`](../../../artifacts/security/m5/m5_raw_rendered_handoff/support_export.json)
- Summary artifact: [`artifacts/security/m5/m5_raw_rendered_handoff.md`](../../../artifacts/security/m5/m5_raw_rendered_handoff.md)
- Fixtures: [`fixtures/security/m5/m5_raw_rendered_handoff/`](../../../fixtures/security/m5/m5_raw_rendered_handoff/)
- Producer: `aureline_content_safety::project_m5_raw_rendered_handoff` /
  `frozen_m5_raw_rendered_handoff_packet`
- Headless tool: `m5_raw_rendered_handoff` (`--markdown`, `--clean`, `--validate <packet.json>`)

## Covered surfaces

The lane projects raw-versus-rendered honesty across exactly these new M5
surfaces, each with the render transform that makes its rendered form diverge
from the raw bytes:

| Surface | Display mode | Render transform | Divergence |
| --- | --- | --- | --- |
| `docs_rendered_panel` | ordinary browsing | `markdown_html_render` | rendered reflows layout |
| `notebook_rendered_output` | ordinary browsing | `notebook_output_render` | rendered applies styling |
| `ai_summary_evidence` | ordinary browsing | `ai_summarization` | rendered summarizes content |
| `review_structured_diff` | ordinary browsing | `diff_normalization` | rendered normalizes for comparison |
| `structured_artifact_viewer` | ordinary browsing | `structured_pretty_print` | rendered reflows layout |
| `marketplace_install_review` | strong-decision strict identity | `manifest_render` | rendered reflows layout |
| `policy_review_overlay` | strong-decision strict identity | `policy_render` | rendered reflows layout |

A surface whose transform is `no_transform` is byte-identical (`byte_identical`)
and does not need the Raw-versus-Rendered split; it collapses to a single
canonical-bytes label and a single raw copy action.

## Representation labels and copy/export semantics

Every surface whose rendered form materially diverges from the raw bytes exposes
two labels — `raw` (canonical bytes) and `rendered` (the viewer's display form) —
and three copy/export actions:

| Action | Representation | Preserves canonical bytes | Implies byte-identical raw |
| --- | --- | --- | --- |
| `copy_raw` | raw | yes | yes (it *is* the raw bytes) |
| `copy_rendered` | rendered | no | no |
| `export_sanitized_snapshot` | sanitized | no | no |

Only `copy_raw` yields canonical bytes; neither rendered copy nor export ever
claims to be byte-identical raw, so rendered output cannot masquerade as the
source on copy, export, or handoff.

## Handoff preservation

The `handoff_preservation` block preserves the representation warning across
three carriers so a downstream reader can tell what the original surface warned
about:

| Carrier | Captured representation | Preserved labels |
| --- | --- | --- |
| `support_export` | raw and rendered labeled | raw, rendered |
| `screenshot_caption` | rendered only with disclaimer | rendered |
| `handoff_packet` | raw and rendered labeled | raw, rendered |

A screenshot can only show the rendered view, so it preserves a rendered label
plus a disclaimer that the canonical raw bytes differ and remain available from
the source surface; the export and handoff carriers preserve both labels and the
per-surface render transforms. Each carrier carries an escaped exemplar only —
re-running the detector over it must come back clean.

## Invariants

`M5RawRenderedHandoffPacket::validate` enforces the lane invariants and fails
closed when any is broken:

- `surface_missing` — every new M5 surface must be present exactly once.
- `diverging_surface_missing_raw_rendered_labels` — a surface whose rendered form
  materially diverges must expose both a canonical-raw label and a rendered
  label.
- `diverging_surface_missing_copy_export_actions` — a diverging surface must
  offer labeled raw copy, rendered copy, and an export-safe action.
- `rendered_copy_implies_raw` — no rendered or export action may claim to be
  byte-identical raw content.
- `strong_decision_display_too_weak` — install/update review and policy review
  must render in `strong_decision_strict_identity`, not ordinary browsing.
- `normalization_applied` — the projection never normalizes or strips bytes.
- `diverging_count_mismatch` — the declared diverging-surface count must match the
  projections.
- `handoff_carrier_missing` / `handoff_drops_divergence_warning` /
  `handoff_leaks_raw_bytes` — the handoff block must preserve the divergence
  warning across all three carriers and carry only escaped exemplars.

## Scope

This lane delivers only the raw-versus-rendered representation labels,
copy/export semantics, and handoff preservation needed for the claimed M5
viewer and trust surfaces. It does not implement a general browser engine or
media suite, and it does not restate the per-surface viewer implementations that
consume the packet.
