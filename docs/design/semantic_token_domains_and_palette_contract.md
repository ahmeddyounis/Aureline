# Semantic token-domain ledger and palette contract

This document freezes **which semantic token domain owns each kind of visual
meaning** so themes, diffs, diagnostics, status badges, evidence cards, and
charts share one stable meaning system.

The goal is to make visual semantics **mechanical rather than interpretive**:
given any colour used for *status*, *severity*, *syntax*, *diff*, *chart*, or
*trust/freshness/lifecycle*, a reviewer can identify the owning domain and the
override rules.

This contract is normative. Where it disagrees with the PRD, technical
architecture/design documents, UI/UX spec, or the UX design-system style guide,
those sources win and this contract plus its companion artifacts MUST be
updated in the same change.

## Companion artifacts

- [`/artifacts/design/semantic_token_domains.yaml`](../../artifacts/design/semantic_token_domains.yaml)
  publishes the machine-readable token-domain ledger and cross-surface mapping
  rules.
- [`/schemas/design/palette_mapping_row.schema.json`](../../schemas/design/palette_mapping_row.schema.json)
  defines the boundary shape for palette-mapping rows and examples.
- [`/fixtures/design/palette_examples/`](../../fixtures/design/palette_examples/)
  contains machine-readable examples for syntax, diff, chart, and status/trust
  surfaces.

## Composition, not duplication

This contract composes with existing canonical sources:

- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` owns the concrete palette,
  semantic theme tokens, and the baseline recommendations for syntax, diff, and
  chart colour roles.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` owns controlled lifecycle and
  freshness vocabulary plus safe-preview trust classes.
- [`/docs/design/design_token_component_state_vocabulary.md`](./design_token_component_state_vocabulary.md)
  freezes token families and the namespaces each family owns.
- [`/docs/design/component_state_taxonomy.md`](./component_state_taxonomy.md)
  freezes degraded/stale/restricted/locked semantics and the required
  non-colour cues.
- [`/docs/ux/decoration_precedence_contract.md`](../ux/decoration_precedence_contract.md)
  freezes the precedence rules that prevent syntax colour from overpowering
  diagnostics, diff, review, and trust cues.
- [`/artifacts/ux/color_safe_diagnostic_palette.yaml`](../../artifacts/ux/color_safe_diagnostic_palette.yaml)
  and
  [`/artifacts/ux/status_icon_legend.yaml`](../../artifacts/ux/status_icon_legend.yaml)
  publish protected-cue rows and “no hue-only meaning” requirements across
  dark/light/high-contrast/forced-colors and low-saturation review.

## 1. Definitions (frozen)

- **Token family**: one of the closed families frozen in
  `docs/design/design_token_component_state_vocabulary.md` (example:
  `color_syntax`, `color_diff`, `semantic_status`).
- **Token domain**: a closed semantic “meaning owner” inside one family. Domains
  exist so a surface can answer “who owns this colour meaning?” without reading
  component-local prose.
- **Palette mapping row**: a machine-readable row that names a token, its
  semantic domain, its default values per theme class, where it may propagate,
  and which override scopes are allowed.

## 2. Domain ledger (summary)

Each domain below owns a distinct kind of meaning. A surface MUST NOT mint
private domains (for example “diff-v2-green” or “status-orange-alt”).

| Domain | Owns meaning for | Token family | Namespace | Allowed overrides (high level) |
|---|---|---|---|---|
| Theme semantic | background/text/border/icon/focus/accent | `color_semantic_theme` | `al.color.*` | theme packages + user overlays |
| State hues | base hue anchors for state-like roles | `color_semantic_theme` | `al.color.state.*` | theme packages + user overlays |
| Semantic status | success/warning/danger/info/insight ink + fills/borders | `semantic_status` | `status.*` | theme packages + user overlays |
| Product/trust visual state | restricted/policy-locked, remote/collab, AI state, debugging active, safe-preview trust cues | `trust_visual_state` | `trust.*` | theme packages + user overlays (must preserve meaning) |
| Syntax | comment/keyword/type/function/string/etc | `color_syntax` | `al.color.syntax.*` | theme packages + imported/language syntax themes |
| Diff | add/remove/modify/move/comment-anchor roles | `color_diff` | `al.color.diff.*` | theme packages |
| Chart | baseline/current/improved/regressed/attention/AI series roles | `color_chart` | `al.color.chart.*` | theme packages |
| Lifecycle & freshness labels | Ready/Warming/Partial/Stale/etc and their required cues | composed | (maps to `status.*` + `trust.*`) | no free-form overrides; map via ledger |

Rules (frozen):

1. **Product semantics are not language semantics.** Syntax themes may override
   only the `Syntax` domain; they MUST NOT override `Diff`, `Chart`, `Semantic
   status`, or `Product/trust visual state`.
2. **No hue-only critical meaning.** Any critical meaning (trust loss, policy
   lock, severity, diff change, blocked/degraded state) MUST have a non-colour
   cue (icon, label chip, border, underline pattern, lane position, etc.).
3. **Precedence preserves truth.** When domains compete, the precedence rules
   in `docs/ux/decoration_precedence_contract.md` win over “pretty” syntax.

## 3. Mapping and override rules (frozen)

### 3.1 Theme classes and parity

- Dark reference and light parity MUST both provide values for every token in
  product-semantic domains (status, diff, chart, trust).
- High-contrast and forced-colors modes may flatten decorative fills, but must
  preserve **text/icon/border hierarchy** and the required non-colour cues.

### 3.2 Reduced-motion and low-contrast fallbacks

Motion may clarify freshness or completion, but it MUST NOT be the only carrier
of state. If a cue relies on motion (spinner, pulse, progress shimmer), the
same meaning must remain present via:

- a controlled label (for example `Warming`, `Reconnecting`, `Degraded`), and
- a non-colour cue (icon, border, shape, or lane position).

### 3.3 Colour-blind-safe and low-saturation alternatives

Domains that rely on hue families (status, diff, chart) MUST remain decipherable
under:

- grayscale,
- low-saturation palettes, and
- forced-colors/high-contrast strategies.

This is achieved by pairing hue with at least one additional channel:
stroke/underline patterns, glyphs, label chips, lane position, or explicit text
labels (see `artifacts/ux/color_safe_diagnostic_palette.yaml`).

## 4. Freshness, degraded-mode, trust class, and lifecycle cues

Surfaces MUST use the controlled vocabulary from
`.t2/docs/Aureline_UI_UX_Spec_Document.md` for lifecycle and freshness labels:

`Ready`, `Warming`, `Partial`, `Stale`, `Rebuilding`, `Restricted`,
`Policy blocked`, `Reconnecting`, `Degraded`, `Read-only degraded`,
`Unavailable`, `Rollback available`.

Rules (frozen):

1. **A stale or partial claim is not a success.** It uses degraded-mode patterns
   (“what still works / what is reduced / how to recover / whether certainty is
   affected”) plus a freshness/age label when relevant.
2. **Restricted vs policy blocked remain separate axes.** They map to distinct
   `trust.*` tokens and must not collapse into one generic “locked” colour.
3. **Safe-preview trust classes remain visible.** When content is `RawText`,
   `SanitizedRich`, `TrustedLocalActive`, or `IsolatedRemoteActive`, the active
   trust class must be visible in-place and preserved in exports/screenshots.

## 5. Machine-readable examples

Worked examples are published under `fixtures/design/palette_examples/` so a
reviewer and tooling can trace:

- syntax roles → editor and review surfaces,
- diff roles → diff views and editor gutters,
- chart roles → evidence/benchmark charts, and
- semantic status + trust cues → badges, status items, and safe-preview surfaces.

