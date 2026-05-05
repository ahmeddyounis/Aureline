# Locale and input-method readiness baseline

This document freezes the M0/M1 locale, layout, and input-method
baseline for Aureline's custom shell. It is the human-readable
companion to:

- [`/artifacts/i18n/test_mode_matrix.yaml`](../../artifacts/i18n/test_mode_matrix.yaml)
  — machine-readable readiness rows, phase posture, and design-rule ids.
- [`/fixtures/i18n/pseudoloc_rtl_ime_manifest.yaml`](../../fixtures/i18n/pseudoloc_rtl_ime_manifest.yaml)
  — harness-plan manifest naming test modes, seed strings, fixture
  surfaces, expected failure classes, and owners.
- [`/docs/i18n/locale_surface_matrix.md`](./locale_surface_matrix.md)
  — cross-surface matrix for what localizes vs. what stays machine-stable
  on UI/CLI/docs/extension surfaces.
- [`/artifacts/accessibility/platform_input_matrix.yaml`](../../artifacts/accessibility/platform_input_matrix.yaml)
  — platform-specific assistive-technology, locale, and input-path rows
  that later packets still cite for per-platform evidence.
- [`/docs/accessibility/a11y_ime_packet_template.md`](../accessibility/a11y_ime_packet_template.md)
  — review-packet template that consumes the shared locale/input rows.
- [`/docs/adr/0016-shell-windowing-input-accessibility-boundary.md`](../adr/0016-shell-windowing-input-accessibility-boundary.md)
  — canonical shell/input ownership boundary.

Normative sources projected here:

- `.t2/docs/Aureline_PRD.md`
  — Section 11.7 and the input-fidelity obligations for IME, bidi,
  grapheme correctness, font fallback, and RTL-ready shell design.
- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  — Section 23.3.1 and Section 27.23.
- `.t2/docs/Aureline_Technical_Design_Document.md`
  — Section 8.10, the accessibility/input fidelity summaries, and the
  localization verification lane.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md`
  — Sections 19.7, 19.10, 20.11, 23.41, and Appendix Y.4.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md`
  — the IME/degraded-state guidance and Section 28.10.

## Purpose

This baseline exists so shell, renderer, docs/help, onboarding, and
accessibility work all consume one locale/input readiness contract
before implementation broadens. It narrows M0/M1 scope in two ways:

- it names which locale and input-method rows are claimed, degraded,
  blocked, or unclaimed for shell work; and
- it freezes the design/layout rules every token, component, and
  packet-producing surface must follow under pseudoloc, RTL/bidi, CJK,
  IME, dead-key, AltGr, compose, and emoji stress.

The baseline does not claim translated builds, marketed locale support,
or a finished locale-pack publication pipeline. Those stay explicit in
the blocked or unclaimed rows until later packets land.

## Shared posture model

The machine-readable matrix carries two orthogonal fields:

| Field | Meaning |
|---|---|
| `support_class` | breadth of the defended surface set (`claimed`, `claimed_narrow`, `best_effort`, `unclaimed`) |
| `m0_state` / `m1_state` | minimum expected posture for that phase (`claimed`, `degraded`, `blocked`, `unclaimed`) |

Phase-state meanings:

| State | Meaning |
|---|---|
| `claimed` | the row is inside the defended shell scope and must be exercised by the named harness modes |
| `degraded` | the row is allowed only through a visible narrowing or fallback; marketing or release packets may not imply parity |
| `blocked` | a real requirement exists, but the capability is not yet defended; surfaces must say why instead of guessing or silently falling back |
| `unclaimed` | outside the defended M0/M1 shell scope; docs, support, and packets must not imply support |

## Minimum M0/M1 readiness rows

The authoritative row ids live in
[`/artifacts/i18n/test_mode_matrix.yaml`](../../artifacts/i18n/test_mode_matrix.yaml).
The minimum baseline is:

| Row | M0 posture | M1 posture | Minimum scope |
|---|---|---|---|
| `readiness.shell.source_language_and_pseudoloc_chrome` | `claimed` | `claimed` | source-language shell chrome, pseudoloc expansion, fallback disclosure, and controlled state labels on launch-critical shell/help/review surfaces |
| `readiness.shell.rtl_chrome_and_mixed_direction_technical_content` | `degraded` | `claimed` | directional shell chrome mirrors correctly and mixed-direction technical text remains copy-safe, but a fully localized RTL shell is not yet a defended M0 claim |
| `readiness.text.cjk_font_fallback_and_full_width_layout` | `claimed` | `claimed` | CJK punctuation, full-width glyphs, fallback chains, and dense-row layout survive editor/palette/terminal/settings stress |
| `readiness.input.ime_preedit_and_commit` | `claimed` | `claimed` | IME preedit and commit remain correct across editor, palette, settings, terminal, and trust/auth prompts |
| `readiness.input.dead_key_and_compose_sequences` | `claimed` | `claimed` | dead-key and compose paths remain text production rather than shortcut dispatch on named surfaces |
| `readiness.input.altgr_text` | `claimed` | `claimed` | Windows AltGr input remains text production on launch-critical text-entry surfaces |
| `readiness.input.emoji_commit_and_picker_visibility` | `claimed` | `claimed` | emoji entry survives insertion, fallback, and adjacent-text preservation on launch-critical text-entry surfaces |
| `readiness.locale.fallback_chain_and_locale_pack_contract` | `blocked` | `degraded` | inspectable fallback and source-language escape hatches are required before narrow locale experiments widen; signed locale-pack parity is still future work |
| `readiness.locale.translated_surface_and_locale_pack_marketing_parity` | `unclaimed` | `unclaimed` | marketed translated shell/docs/auth parity and community locale-pack promotion remain outside the defended M0/M1 shell claim |

## Design and layout baseline

The machine-readable rule ids live in the test-mode matrix. The rules
below are the human-readable baseline that token, component, and packet
work should reuse.

### Expansion and placeholder rules

- Launch-critical single-line actions, tabs, section headers, and state
  pills must tolerate at least 1.35x source-language width under
  pseudoloc before ellipsis or overlap. If they cannot, the control
  grows, wraps, or pushes secondary chrome out of line first.
- Multi-line banners, review summaries, permission sheets, and guided
  steps must tolerate at least 1.60x source-language length before the
  product considers truncation.
- Dynamic strings must use placeholder-driven message templates.
  Counts, file paths, command ids, hosts, tenants, flags, and policy
  owners stay as raw technical tokens even when prose reorders around
  them.
- Localization may reorder placeholders, but it may not paraphrase or
  silently normalize technical tokens whose literal spelling matters to
  debugging, security review, or support.

### Mirroring and bidirectional text rules

- Directional chrome mirrors: drawers, side sheets, breadcrumb
  separators, disclosure motion, split-view order, and navigation
  affordances should follow layout direction.
- Literal technical strings do not mirror: code spans, diffs, terminals,
  file paths, hostnames, flags, command ids, and escaped/raw text
  representations remain stable and copyable as authored.
- Focus order must still track the visual order after mirroring. A row
  that mirrors visually but leaves keyboard traversal in the old order
  is non-conforming.
- Mixed-direction technical content must preserve LTR islands inside RTL
  prose and keep raw, rendered, and escaped inspection paths available
  where directionality or invisible controls matter.

### Truncation and state-label rules

- Truncation is a last resort. Components expand, wrap, collapse empty
  siblings, or expose an inline full-text route before high-importance
  copy truncates.
- High-importance surfaces such as approvals, trust prompts, policy
  blocks, recovery banners, and sign-in fallbacks must keep a full-text
  route inside the same review flow.
- State labels keep scope, severity, and action meaning intact across
  localization. Terms equivalent to `Policy blocked`, `Read-only
  degraded`, and `Rollback available` must remain direct and centrally
  reviewed rather than softened into generic status words.
- Count and scope language must survive localization and truncation.
  `Loaded`, `matching`, `selected`, `blocked`, `visible`, and similar
  distinctions may not collapse into one ambiguous badge.

### Input-method and composition rules

- Candidate windows, inline composition, and the active caret stay
  visible during filtering, result churn, overlay transitions, and
  window-topology changes.
- A focus change, preview open, close action, or submit action may not
  silently commit or cancel composition. The product either keeps the
  composition target visible or blocks the action explicitly.
- Multi-cursor or column-selection flows may narrow to one visible
  composition target when coherent apply is impossible, but they may not
  corrupt text or hide the downgrade.
- Dead keys, compose sequences, AltGr, emoji pickers, and other
  platform text-production routes remain text input first. They must not
  be reclassified as command shortcuts because the surface assumed a US
  keyboard.

## Consumption rules

- Shell and renderer work should cite readiness row ids from
  [`/artifacts/i18n/test_mode_matrix.yaml`](../../artifacts/i18n/test_mode_matrix.yaml)
  rather than embedding one-off locale assumptions in implementation
  notes.
- Accessibility packets should continue to cite
  [`/artifacts/accessibility/platform_input_matrix.yaml`](../../artifacts/accessibility/platform_input_matrix.yaml)
  for platform rows, but use the i18n matrix and harness manifest for
  pseudoloc, RTL/layout-expansion, fallback-chain, and translated-copy
  scope.
- New surface families may extend the matrix only by adding rows or
  surfaces; they may not repurpose an existing readiness row to widen a
  claim silently.
