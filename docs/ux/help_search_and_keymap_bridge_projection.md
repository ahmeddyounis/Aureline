# Help search, keymap bridge, and migration discoverability projection contract

This document freezes the surface-agnostic projection contract that
**help search**, **imported-keymap bridge surfaces**, **migration hints**,
**glossary cards**, and **contextual tips** use to talk about commands and
shortcuts without inventing surface-specific command names, aliases, or
shortcut strings.

The goal is a single discoverability truth: a user who arrives with an
imported keymap, incumbent habits, or non-default locale must see the same
canonical command identity and shortcut reality no matter where they look.

Machine-readable boundary:

- [`/schemas/ux/discoverability_projection.schema.json`](../../schemas/ux/discoverability_projection.schema.json)

Worked fixtures:

- [`/fixtures/ux/help_search_keymap_bridge_cases/`](../../fixtures/ux/help_search_keymap_bridge_cases/)

## Companion contracts this contract composes with

This contract does **not** mint a new command model. It consumes upstream
records by reference:

- [`/docs/commands/command_surface_projection.md`](../commands/command_surface_projection.md)
  and
  [`/schemas/commands/command_projection.schema.json`](../../schemas/commands/command_projection.schema.json)
  — canonical cross-surface command projection packet. Any surface that wants
  a command label, docs anchor, lifecycle cue, or current shortcut display
  resolves it from this packet (or the canonical records it projects).
- [`/docs/ux/keybinding_resolver_contract.md`](./keybinding_resolver_contract.md)
  and
  [`/schemas/commands/keybinding_resolver.schema.json`](../../schemas/commands/keybinding_resolver.schema.json)
  — the imported-keymap bridge truth model (`bridge_outcome_class`,
  `behavior_change_axes`, and sequence-shape vocabulary).
- [`/docs/ux/contextual_teaching_contract.md`](./contextual_teaching_contract.md)
  and
  [`/schemas/ux/teaching_surface.schema.json`](../../schemas/ux/teaching_surface.schema.json)
  — migration bridge cards and contextual tips that cite the imported mapping
  without smuggling ungoverned actions.
- [`/docs/ux/learnability_contract.md`](./learnability_contract.md)
  and
  [`/schemas/ux/guided_surface_state.schema.json`](../../schemas/ux/guided_surface_state.schema.json)
  — glossary cards and other learnability surfaces that must remain anchored
  to canonical command IDs and docs anchors.
- [`/docs/ux/localization_and_locale_pack_contract.md`](./localization_and_locale_pack_contract.md)
  and
  [`/schemas/ux/locale_fallback_state.schema.json`](../../schemas/ux/locale_fallback_state.schema.json)
  — source-language fallback disclosure and command-id preservation across
  locale fallback.
- [`/artifacts/ux/hint_source_ledger.yaml`](../../artifacts/ux/hint_source_ledger.yaml)
  — the optional allowlist ledger for which canonical refs onboarding and
  learnability surfaces may cite.

## Problem this contract solves

The same command identity is surfaced through multiple discoverability lanes:

- help search results (commands, docs/help pivots, shortcut lookup);
- migration and imported-keymap bridge hints (old-to-new mapping honesty);
- contextual tips and glossary cards (in-context learnability); and
- source-language fallback surfaces (continuity when localization is missing).

If each surface independently re-derives names, aliases, shortcut display, or
import fidelity explanations, the product drifts:

- a help result teaches a command label or shortcut that is not bound;
- a migration bridge claims parity (`Exact`) when gesture, sequence shape, or
  execution posture changed;
- a contextual tip links to a command docs anchor that does not match the
  command it references; or
- a locale fallback shows translated strings but loses canonical command ID
  continuity, making the projected shortcut untraceable.

This contract eliminates that drift by requiring every discoverability surface
to render command and shortcut truth from the same canonical sources.

## Discoverability projection packet

A **discoverability projection packet** is a transport-safe join that carries
only:

1. stable references to canonical command projection packets (so surfaces can
   resolve canonical command ID, docs/help anchor, lifecycle/capability cues,
   aliases, and current shortcut display); plus
2. optional imported-keymap bridge rows (so surfaces can quote the fidelity
   class and behavior-change axes without inventing labels); plus
3. optional locale fallback state (so source-language fallback is disclosed and
   command-id continuity is auditable).

Surfaces MUST treat this packet as their discoverability source-of-truth and
MUST NOT mint parallel command names, aliases, or shortcut strings.

## Bridge projection rules

Imported-keymap bridge surfaces and migration hints MUST quote the resolver’s
`bridge_outcome_class` and MUST remain honest about gaps and limitations.

The bridge outcome classes are frozen by the keybinding-resolver contract:

- `exact` — fully supported bridge; gesture/shape/command identity preserved.
- `translated` — supported but gesture/shape changed (e.g. leader sequence).
- `alias_only` — supported via an explicit canonical alias; command identity
  resolves, but the source command id is not treated as canonical.
- `partial` — some behavior or scope narrowed; parity is not promised.
- `shimmed` — a shim or dependency is required; the surface must not claim
  native parity.
- `unsupported` — no equivalent command; the surface must not imply the
  command exists or is invokable.

Rules:

1. A surface MUST NOT label a mapping as `exact` unless the resolver emitted
   `bridge_outcome_class = exact`.
2. A surface MUST NOT show a *target* command label, docs pivot, or shortcut
   unless it can resolve a canonical command projection for that target.
3. For `partial` and `shimmed`, the surface MUST disclose the limitation (via
   the bridge record’s axes and/or the cited migration/teaching surface record)
   and MUST NOT promise parity.
4. For `unsupported`, the surface MUST keep the source command and sequence
   visible, and route the user to canonical discovery (help search / command
   palette / docs) without fabricating an “equivalent” command name.
5. Multi-command bridges (one source command plausibly mapping to multiple
   target commands) MUST be presented as choices; a surface MUST NOT silently
   pick one and claim parity.

## Source-language fallback rules

When a discoverability surface renders a command label, shortcut narration, or
teaching text in a locale other than the requested locale (including
source-language fallback), it MUST:

1. emit a `locale_fallback_state_record` with `disclosed_to_reviewer = true`;
2. preserve the canonical `command_id` unchanged across fallback
   (`command_id_preservation_state = command_id_unchanged_across_fallback`);
3. keep at least one source-language escape hatch active; and
4. keep the docs/help pivot resolvable to a canonical anchor (no raw URLs).

## Conformance checklist

A surface conforms when a reviewer can verify:

1. Every projected command resolves to a canonical `command_id` and projection
   packet reference.
2. Every projected shortcut display is consistent with current shortcut truth
   and does not overclaim imported fidelity.
3. Every imported-keymap bridge explanation quotes the resolver’s typed outcome
   (`exact`, `translated`, `alias_only`, `partial`, `shimmed`, `unsupported`)
   and the declared behavior-change axes when non-exact.
4. No help-search or migration surface introduces a command label or shortcut
   that cannot resolve back to canonical command records.
5. Locale fallback, when present, is disclosed and preserves command identity.

