# M5 Safe-Preview Limited Mode & Expensive-Render Guards

This document is the contract for how the new M5 large or generated artifact
families open. The frozen content-integrity matrix
(`freeze_the_m5_suspicious_content_safe_preview_and_representation_copy_export_matrix`)
locks the *static* qualification each M5 artifact/viewer family may claim,
the trust-class ladder lane (`m5_trust_class_ladder`) resolves the *runtime*
trust class of active content, and the raw-versus-rendered handoff lane
(`m5_raw_rendered_handoff`) keeps raw/rendered copy and export honest. This lane
covers the orthogonal *cost* gap they leave open: a log, lockfile, snapshot,
bundle, or evidence packet must not jump straight into an expensive or unsafe
render path just because the surface can technically try.

- Record kind: `m5_safe_preview_limited_mode_packet`
- Schema: [`schemas/security/m5-safe-preview-limited-mode.schema.json`](../../../schemas/security/m5-safe-preview-limited-mode.schema.json)
- Canonical support export: [`artifacts/security/m5/m5_safe_preview_limited_mode/support_export.json`](../../../artifacts/security/m5/m5_safe_preview_limited_mode/support_export.json)
- Summary artifact: [`artifacts/security/m5/m5_safe_preview_limited_mode.md`](../../../artifacts/security/m5/m5_safe_preview_limited_mode.md)
- Fixtures: [`fixtures/security/m5/m5_safe_preview_limited_mode/`](../../../fixtures/security/m5/m5_safe_preview_limited_mode/)
- Producer: `aureline_content_safety::project_m5_safe_preview_limited_mode` /
  `frozen_m5_safe_preview_limited_mode_packet`
- Headless tool: `m5_safe_preview_limited_mode` (`--markdown`, `--clean`, `--validate <packet.json>`)

## Covered artifact families

| Family | Derives from |
| --- | --- |
| `build_log` | the originating run |
| `dependency_lockfile` | the source manifest |
| `test_snapshot` | the originating test |
| `distribution_bundle` | the build inputs |
| `evidence_packet` | the underlying records |
| `generated_artifact` | the generator source |

## Open mode

Each artifact resolves to an initial open mode before any explicit opt-in:

| Mode | Meaning |
| --- | --- |
| `safe_preview_limited` | A bounded preview shown first: limited bytes/lines, banners, and explicit open-raw / open-source / expand actions. |
| `full_render_inline` | The full render shown immediately; chosen only for small, non-generated artifacts whose full render is cheap and inert. |

An artifact opens in `safe_preview_limited` when it is **oversized** (above the
`byte_budget` or `line_budget`), **generated**, or its full render would be
**expensive or unsafe**. Otherwise it opens `full_render_inline`. The default
view is always cheap — the expensive or unsafe render is never what loads first.

## Banners

Limited-mode viewers show typed banners above the bounded preview so the trust
and representation cues are always visible:

| Banner | When |
| --- | --- |
| `oversized` | The artifact exceeds the preview byte/line budget. |
| `generated_artifact` | The artifact is generated/derived; the message names its canonical source. |
| `limited_preview` | The preview is bounded; the full artifact is not all shown. |
| `expensive_render_guarded` | A fuller, expensive render is available but gated behind opt-in. |
| `active_content_guarded` | Active content is present and renders only behind explicit opt-in. |

## Actions

Every artifact offers, at minimum, an open-raw and an open-canonical-source
action. Limited-mode artifacts add an expand-full-render action:

| Action | Posture |
| --- | --- |
| `open_raw` | `available_immediately` (cheap) — raw bytes/text stay reachable everywhere. |
| `open_canonical_source` | `available_immediately` (cheap) — opens the source or generator the artifact derived from. |
| `expand_full_render` | `requires_explicit_opt_in` when the full render is expensive or unsafe; `available_immediately` only when the full render is cheap. |

Render costs order `cheap` < `expensive` < `unsafe`. Any action whose render
cost is `expensive` or `unsafe` is gated behind an explicit opt-in; it never
fires silently.

## Invariants

The producer guarantees, and `validate` enforces, that:

- Oversized or generated artifacts open in `safe_preview_limited` first.
- No expensive or unsafe render path is available without an explicit opt-in;
  the default view of every artifact is cheap.
- Open-raw stays reachable immediately and cheaply on every artifact; suspicious
  bytes are surfaced, never normalized away.
- The canonical-source or generator relationship is preserved: each artifact
  carries a non-empty `canonical_source_ref` and an `open_canonical_source`
  action, so the derived artifact never pretends to be the only truth.
- Every limited-mode artifact carries at least one banner as a visible cue.
- Every artifact with a guarded render path offers a gated expand action.

The packet is metadata only: no raw artifact bytes, raw rendered trees, raw
provider payloads, or credentials cross the export boundary.

## Consumers

The headless `m5_safe_preview_limited_mode` tool is the first CLI/headless
consumer; it emits the canonical support export, the Markdown summary, the clean
fixture, and validates any packet. Support, diagnostics, and release tooling read
the machine-readable packet and banner/action affordances directly rather than
cloning prose.
