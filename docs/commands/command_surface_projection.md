# Canonical command surface-projection packet

This document defines the canonical *command surface-projection packet* used
to project **one** canonical command identity into shell and discoverability
surfaces without per-surface copies of names, aliases, shortcut display, or
disabled-reason explanations.

The machine-readable boundary is:
[`/schemas/commands/command_projection.schema.json`](../../schemas/commands/command_projection.schema.json).

Worked cases live in:
[`/fixtures/commands/command_surface_projection_cases/`](../../fixtures/commands/command_surface_projection_cases/).

## Problem this packet solves

The same command is rendered and explained across multiple entry and
discoverability surfaces:

- command palette rows
- application and context menus
- keybinding editor rows and keybinding help
- help search and docs/help pivots
- onboarding hints and contextual teaching handoffs
- migration bridge cards and imported-keymap shims
- “why unavailable” explainers when the command is disabled/hidden

If each surface independently re-derives command naming, aliasing, shortcut
display, enablement state, or “why unavailable” wording, the product drifts:
different strings for the same action, different alias stories, and different
disabled-reason explanations depending on where the user looks.

The projection packet is the *single join point* that carries:

- canonical command identity (`command_id`, `command_revision_ref`,
  `canonical_verb`)
- canonical label and accessibility label path (refs, not raw strings)
- docs/help anchor (ref, not a URL)
- lifecycle and capability cues (lifecycle/support/channel/freshness,
  capability scope, preview class, approval posture)
- alias history (canonical + deprecated/retired aliases with provenance)
- current shortcut display (per-platform assignment state + resolver layer)
- imported-keymap bridge anchors (opaque refs to keybinding bridge rows)
- typed enablement decision + typed disabled-reason references (not ad hoc copy)

Every surface that renders a command row MUST treat this packet as the
projection source-of-truth and MUST NOT mint parallel names, aliases, or
disabled reasons.

## Upstream canonical sources (inputs)

This packet is a projection of existing canonical records; it does not replace
them:

- `command_descriptor_record`
  ([`/schemas/commands/command_descriptor.schema.json`](../../schemas/commands/command_descriptor.schema.json))
  — canonical command identity, labels, docs/help anchor, capability scope,
  preview/approval posture, palette visibility, slot hints, lifecycle metadata.
- `command_registry_entry_record`
  ([`/schemas/commands/command_registry_entry.schema.json`](../../schemas/commands/command_registry_entry.schema.json))
  — alias lifecycle, discoverability refs, current shortcut refs, disabled-reason
  explanation refs, badges, and surface exposure refs.
- `keybinding_import_bridge_record`
  ([`/schemas/commands/keybinding_resolver.schema.json`](../../schemas/commands/keybinding_resolver.schema.json))
  — imported-keymap mapping truth (fidelity class, axes of behavior change).
- `command_diagnostic_row_record` / remediation projections
  ([`/schemas/commands/diagnostic_projection.schema.json`](../../schemas/commands/diagnostic_projection.schema.json))
  — typed disabled-reason and remediation projection rows consumed by surfaces.
- Teaching and learnability surfaces
  ([`/schemas/ux/teaching_surface.schema.json`](../../schemas/ux/teaching_surface.schema.json),
  [`/schemas/ux/guided_surface_state.schema.json`](../../schemas/ux/guided_surface_state.schema.json))
  — onboarding/migration/contextual surfaces that cite canonical command anchors.

## Projection targets (outputs)

The packet is designed so each surface can render and explain a command without
re-deriving identity, aliasing, shortcut display, or disabled reasons:

- **Menus.** Project `primary_label_ref`, `docs_help_anchor_ref`,
  preview/approval cues, and a typed enablement decision into menu rows and
  command bars. Menu paths are projected from `descriptor.ui_slot_hints` (not
  reauthored per menu surface).
- **Palette.** Project the same canonical fields into `palette_row` rows
  (palette rows may add ranking and contextual scope detail, but not identity).
- **Keybindings.** Project current shortcut display state and imported-keymap
  mapping anchors into keybinding editor/help rows. Surfaces MUST quote the
  resolver/bridge anchors rather than inventing “translated” vs “exact” labels.
- **Help search.** Project canonical identity, docs/help anchors, promoted alias
  ids, and shortcut display into help search results so `cmd:*` and legacy
  aliases resolve consistently.
- **Onboarding hints.** Project stable command anchors, docs/help pivots, and
  current shortcut display into onboarding/contextual hints; hints may hand off
  to a canonical invoking surface but must not mint a new command name.
- **Migration bridge.** Project alias history and keymap bridge anchors so the
  migration surface can explain shims without forking command identity.
- **Why unavailable.** Project typed disabled-reason codes + explanation/repair
  hook refs so every surface explains unavailability identically.

## Stability rules (docs, accessibility, headless/help)

Projection packets are treated as *stable documentation artifacts*:

- **Stable ids only.** Packets carry opaque ids and controlled vocabularies;
  raw URLs, raw hostnames, and raw user content are forbidden.
- **Single label source.** Surfaces MUST use `primary_label_ref` and
  `accessibility_label_path` from the canonical command descriptor; they MUST
  NOT author surface-specific synonyms for the same command.
- **Deterministic ordering.** Arrays in the packet are ordered deterministically
  (by surface class, then platform class, then stable id) so screenshot-based
  documentation and golden fixtures do not churn on reorder-only edits.
- **Headless/help parity.** CLI help and docs/help output MUST cite the same
  `canonical_verb`, docs anchor, alias records, and disabled-reason vocabulary
  as UI surfaces.
- **Accessibility narration parity.** Surfaces MUST not drop the accessibility
  label path when projecting a compact row; if compact UI hides visible labels,
  the accessible name and disabled-reason narration still resolve via the same
  canonical refs.

## Conformance checklist

A surface is non-conforming if it:

- invents a new command id, alias id, or disabled-reason code
- renders a command label or docs/help pivot not traceable to the descriptor
- shows a shortcut state that contradicts the resolver-projected display state
- explains an imported-keymap mapping without citing a bridge ref
- shows “disabled” without a typed `disabled_reason_code` and a repair hook ref

