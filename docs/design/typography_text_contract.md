# Typography, text roles, and overflow/copy-honesty contract

This document freezes **typography selection**, **text roles**, and
**overflow / truncation / copy-honesty behavior** as governed system
assets. The goal is to prevent shell chrome, docs/help, editor-adjacent
UI, tables, badges, and teaching surfaces from inventing their own font
choices, scale math, wrapping heuristics, or "truncate and hope"
behaviors.

This contract is normative. Where it disagrees with the PRD, technical
architecture/design documents, UI/UX spec, design-system style guide, or
ADR decisions, those sources win and this contract plus the companion
artifacts, schema, and fixtures MUST be updated in the same change.

Companion artifacts:

- [`/artifacts/design/typography_scale.yaml`](../../artifacts/design/typography_scale.yaml)
  publishes the frozen `type.*` scale rows.
- [`/schemas/design/text_role.schema.json`](../../schemas/design/text_role.schema.json)
  defines the boundary shapes for typography scale, text roles, and text
  render-case fixtures.
- [`/fixtures/design/text_render_cases/`](../../fixtures/design/text_render_cases/)
  contains worked YAML cases for launch-critical surfaces.

## Composition, not redefinition

This contract composes with existing canonical sources and vocabularies
by reference rather than re-minting parallel token families:

- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` owns the
  recommended UI sans and monospace stacks plus the baseline scale rows.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` owns the token names and
  baseline size/line-height/weight values for `type.*`.
- [`/docs/design/design_token_component_state_vocabulary.md`](./design_token_component_state_vocabulary.md)
  owns the frozen token-family vocabulary and reserves the namespaces
  `type.role.*` (typography role), `type.*` (typography scale), and
  `text.rule.*` (text rules).
- [`/schemas/design/component_contract.schema.json`](../../schemas/design/component_contract.schema.json)
  owns the closed `overflow_behavior_class` vocabulary (`truncate_tail`,
  `wrap`, `multi_line_expand`, `preserve_raw`, `overflow_menu`,
  `hide_when_unavailable`) used by component contracts and referenced by
  this contract.
- [`/docs/adr/0002-renderer-text-stack-and-shaping-fallback.md`](../adr/0002-renderer-text-stack-and-shaping-fallback.md)
  owns the renderer font-discovery and fallback-chain contract.

## 1. Two-axis typography model

Every visible text style in the product MUST be describable as:

1. one **text role** (`text_role_class`) describing purpose and policy;
2. one **typography scale row** (`type.*`) describing size/metrics; and
3. one **typography role** (`type.role.*`) describing font-family stack.

Surfaces MUST NOT mint private one-off sizes (for example, "13.5 px") or
private font-family stacks in code or CSS; a surface that needs a new
scale row or typography role MUST update the style guide and the scale
artifact, then update fixtures.

### 1.1 Typography roles (`type.role.*`)

`type.role.*` selects a font-family stack and the minimum behavior
contract for that stack:

- `type.role.ui_sans` — primary UI text (shell, settings, menus,
  inspectors, docs UI chrome).
- `type.role.monospace` — code, logs, identifiers, terminal, stack
  traces, notebook code output.
- `type.role.prose` — long-form documentation and teaching copy when it
  differs from general UI chrome (still sans by default).

Canonical stacks (authoritative in the style guide):

- `type.role.ui_sans` — `Inter, ui-sans-serif, system-ui, -apple-system,
  "Segoe UI", Roboto, Ubuntu, "Noto Sans", sans-serif`
- `type.role.monospace` — `"JetBrains Mono", "IBM Plex Mono",
  "Cascadia Code", ui-monospace, "SFMono-Regular", Menlo, Consolas,
  monospace`
- `type.role.prose` — defaults to the same stack as `type.role.ui_sans`
  unless a future style-guide revision differentiates it explicitly.

If a surface exposes a user-selectable font family (for example, editor
or terminal font), the user selection is treated as **explicit family**
input to the renderer fallback chain; the role still governs policies
such as ligatures, ambiguous-glyph posture, and numeric shaping.

### 1.2 Typography scale (`type.*`)

The `type.*` scale rows are fixed-size rows defined in the style guide
and published to tooling in the scale artifact. Each scale row includes:

- `font_size_px`
- `line_height_px`
- `font_weight`
- `intended_use` (reviewer-facing summary; not a replacement for the
  role selection above)

Surfaces MUST NOT synthesize intermediate scale values at runtime. Zoom,
platform scaling, and larger-text preferences act as multiplicative
transforms on the rendered output, not as a reason to create new scale
rows.

## 2. Text roles (purpose + policy)

Text roles exist to prevent per-surface improvisation about *what kind*
of text something is (identity label vs supporting explanation vs raw
identifier) and therefore what overflow, ligature, bidi, and copy policy
applies.

The closed text-role set is:

- `display` — rare hero/teaching headers inside the product.
- `title` — panel, dialog, and section headers.
- `body` — default UI body copy and longer explanatory paragraphs.
- `supporting` — secondary/supporting text (hints, explanations, helper
  labels) that must remain readable but can yield space first.
- `caption` — micro labels and compact metadata where scan speed
  matters.
- `code` — code-like spans in UI, docs, and teaching surfaces.
- `terminal` — terminal rows, prompts, and terminal metadata.
- `dense_metric` — data-dense numeric surfaces (tables, counters,
  timings, quotas, diagnostics counts).

### 2.1 Default role-to-token mapping

Text roles constrain which `type.role.*` and `type.*` rows are valid.
Surfaces MAY choose a smaller subset, but MUST NOT step outside the
allowed mapping without updating this contract.

| `text_role_class` | Default `type.role.*` | Allowed `type.*` scale rows |
|---|---|---|
| `display` | `type.role.ui_sans` | `type.display` |
| `title` | `type.role.ui_sans` | `type.title.1`, `type.title.2`, `type.title.3` |
| `body` | `type.role.ui_sans` or `type.role.prose` | `type.body.lg`, `type.body.md`, `type.body.sm` |
| `supporting` | `type.role.ui_sans` | `type.body.md`, `type.body.sm`, `type.label.md` |
| `caption` | `type.role.ui_sans` | `type.label.md`, `type.label.sm` |
| `code` | `type.role.monospace` | `type.code.md`, `type.code.sm`, `type.body.sm` |
| `terminal` | `type.role.monospace` | `type.code.md`, `type.code.sm` |
| `dense_metric` | `type.role.monospace` | `type.label.md`, `type.label.sm`, `type.code.sm` |

Rules (frozen):

1. A slot that carries a raw identifier (paths, command ids, policy
   names, capability ids, hostnames) MUST use `code`, `terminal`, or
   `dense_metric` role and MUST declare `overflow_behavior_class =
   preserve_raw` or a detail-view expansion route. Identity MUST NOT be
   rewritten to "fit".
2. A slot that carries prose or explanation uses `body` or `supporting`
   and MUST prefer wrap/reflow before truncation at larger text sizes.
3. `dense_metric` MUST enable tabular numerals (or equivalent shaping)
   so aligned columns remain aligned under font fallback and zoom.

## 3. Font fallback, missing glyphs, ligatures, and mixed scripts

The renderer fallback chain is defined by ADR 0002 and applies to every
surface, including UI chrome, docs, code, terminal, notebooks, and
tables.

### 3.0 Fallback chain (summary)

Per shaping run, glyph resolution follows this deterministic chain:

1. **Explicit family** declared by the caller (theme, user preference,
   or surface contract).
2. **Script-aware preference group** (for example, Han/Kana, Arabic,
   Emoji) selected by `crates/aureline-text` segmentation.
3. **OS system-UI family** for the active locale.
4. **Last-resort bundled subset** to prevent `.notdef` on supported
   hosts.

### 3.1 Fallback transparency and missing glyph posture

If a glyph resolves through fallback stage ≥ 2 (script preference group
or later), the renderer MUST be able to report that resolution stage for
support, accessibility, and diagnostics purposes.

On supported hosts, visible `.notdef` boxes are non-conforming. If a
required glyph is missing even after the last-resort bundled subset, the
surface MUST:

- preserve the original scalar values for copy/export/search; and
- render a visible replacement with a typed disclosure (for example,
  "missing glyph") rather than silently dropping or normalizing text.

### 3.2 Code-font fallback and ambiguous glyph posture

For code-like roles (`code`, `terminal`, `dense_metric`):

- the default posture is **legibility over decoration**;
- ligatures are **off by default** unless a user explicitly enables
  them; and
- raw identifiers MUST remain inspectable even when font fallback
  changes glyph shapes.

Ambiguous-glyph posture is a security and review boundary:

- In security-sensitive surfaces (diffs, policy, trust prompts, install
  flows, capability/authority surfaces), discretionary typographic
  features that can change perceived tokenization MUST be suppressed for
  raw identifiers (no substitution ligatures, no contextual alternates
  that change codepoint boundaries).
- Mixed-script identifiers and suspicious bidi/invisible controls are
  detected and surfaced by the source-rendering security suite (PRD
  security requirements). Typography MUST NOT hide those signals (for
  example by collapsing invisible marks into whitespace without a cue).

### 3.3 Mixed-script runs and bidi isolation posture

Mixed-script text is normal (paths, commands, code, localized prose),
but mixed-script *identifiers* and bidi control characters can be a
review/security boundary.

Rules (frozen):

1. Code-like spans embedded in prose MUST be bidi-isolated so punctuation
   and separators (slashes, flags, parentheses, ellipsis) are not
   reordered or mirrored as part of the surrounding paragraph.
2. Under RTL locales, surrounding prose follows the active direction,
   while code/terminal/dense-metric spans remain LTR unless the span
   itself is truly RTL text.
3. A surface MUST NOT "fix up" mixed-direction strings by inserting or
   deleting characters. When additional marks are needed for visual
   stability, they apply at render time only and MUST NOT change
   copy/export bytes.

### 3.4 Zoom and text-scale interaction

Zoom, platform scaling, and larger-text preferences compose with this
contract:

- at larger text sizes, `body` and `supporting` copy MUST wrap/reflow
  before truncating meaning-bearing text; and
- editor/terminal/code roles MUST preserve cursor/selection geometry and
  line-height floors defined by the accessibility zoom contract.

## 4. Overflow, truncation, clamping, and copy honesty

Overflow behavior is part of truthfulness, not only layout.

Definitions:

- **Wrap/reflow**: text flows to additional lines within the same
  container.
- **Clamp**: text is limited to N lines; remainder is hidden with an
  ellipsis or continuation affordance.
- **Truncate**: text is shortened to fit a single line (typically with
  ellipsis).

Rules (frozen):

1. Silent truncation is non-conforming. If the user can see only a
   shortened value, the full value MUST remain recoverable via at least
   one of:
   - copy-full-value action,
   - reveal-on-focus (keyboard),
   - accessible name (AT),
   - expansion route (multi-line expand or detail view).
2. Primary identity labels truncate last. Secondary metadata (badges,
   shortcut hints, timestamps, secondary scope) MUST yield space before
   the primary label truncates.
3. Truncation MUST be stable under bidi and locale direction changes:
   code-like spans are bidi-isolated; surrounding prose follows the
   active locale direction.
4. Copy and export MUST operate on the **full underlying text**, not
   the visually truncated string. If the surface supports selection-based
   copy that only captures visible text, it MUST also expose an explicit
   copy-full route for truncated/clamped fields.

## 5. Worked render cases (fixtures)

The fixtures in `fixtures/design/text_render_cases/` provide worked
examples for:

- tabs (single-line title/filename behavior),
- status items and badges (caption/dense-metric treatment),
- settings rows (label/value/source overflow and copy),
- tree/list/table rows (primary label vs metadata collapse order),
- docs citations (prose + code spans under bidi),
- terminal metadata (prompt + path + exit status),
- notebook outputs (code-heavy output preview vs open-detail behavior).

Surfaces that diverge from these cases MUST add a new fixture and update
this contract, rather than implementing a local one-off rule.
