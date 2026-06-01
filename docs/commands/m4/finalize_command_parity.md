# Finalize command parity, palette diagnostics, modifier-action footers, and copy-CLI/recipe paths

This stable lane finalizes the *discoverability* half of a stable command. Where
the command-contract stabilization packet froze the descriptor fields, the
invocation/result contract, and cross-surface authority parity, this packet binds
the one discoverability record, the per-surface projection rows, the
modifier-action footer contract, the palette query-session privacy posture, and
the disabled-with-reason chip parity into one export-safe artifact. UI, CLI, AI,
support export, and documentation fixtures read the *same* discoverability truth
instead of cloning labels, aliases, examples, or disabled-reason prose per
surface. The runtime owner is `aureline_commands::finalize_command_parity`.

A command whose name, alias, footer actions, or disabled reason mean different
things in the palette than in a menu, a leader overlay, a voice hint, a deep
link, or the docs is not a stable command. The packet makes the discoverability
record, footer actions, query-session privacy posture, and disabled reasons
inspectable and attributable from every entry surface, and refuses any row where
a surface drifts from the canonical record, claims the Stable lane while narrowed
below it, hides footer actions behind a debug-only mode, dispatches from a copy
action, or silently widens palette history beyond the local device.

## Contract

The packet does **not** re-derive the descriptor, registry, palette-row,
query-session, or discoverability models. It reuses the canonical contract refs,
disabled-reason case vocabulary, surface-qualification posture, and
evidence-export shape from `aureline_commands::stabilize_command_contract`, and
binds the frozen contracts by their refs, adding the finalized invariants the
discoverability lane needs:

- **One discoverability record** — `discoverability_record` pins the canonical
  command id, primary label ref, alias set, category refs, docs/help anchor, and
  keyword refs. Every projection surface reads from this record; none mints a
  local label, alias, example, or category.
- **Per-surface projection parity** — `projection_rows` proves that the command
  palette, menus, tooltips, leader/help overlays, keybinding help, onboarding
  tips, voice hints, deep links, and docs/help pages each project from the
  canonical record with no drift in alias set, copy command ID, copy CLI form,
  add-to-recipe, modifier-action footer, disabled-reason chip, or examples, and
  that each alias resolves to the canonical command id. A surface narrowed below
  Stable may not claim the Stable lane; a Stable surface may not drift.
- **Modifier-action footers** — `footer_contract` requires the default run,
  split/open-alt, open-alternate-target, copy command ID, copy CLI form, add to
  recipe, and inspect "why not automatable?" actions, with held-modifier intent
  surfaced, no debug-only requirement, copy/inspect actions that never dispatch,
  and placement/target deltas that never widen command authority.
- **Query-session privacy** — `query_session_privacy` requires the palette query
  session to be local-first with a typed history policy, bounded retention,
  clear-or-disable controls, held-modifier intent, and a redaction posture. Raw
  query text is never exported, and query history is never silently widened into
  cross-device or cross-tenant memory: any widening beyond the device requires an
  explicit governing feature ref.
- **Disabled-with-reason chips** — `disabled_reason_chips` covers the
  disabled-by-policy, wrong-focus, missing-runtime, degraded-provider,
  preview-required, approval-required, and UI-only cases, each mapped to a
  canonical machine reason code with a shared explanation ref and a "why not
  automatable?" ref, and resolved identically across palette, menus, keybindings,
  voice, help, onboarding, and deep links rather than surface-local prose.
- **Evidence / rollback lineage** — `evidence_export` binds the in-product
  evidence id to the admin inspector and support export refs and carries the
  rollback lineage refs a revert reconstructs the command from.

The record is export-safe: refs, state tokens, coarse classes, counts, and
review labels only. Raw query text, raw command arguments, raw prompts, endpoint
URLs, credentials, and signing-key material stay outside the support boundary,
and the validator rejects a packet that leaks them (`raw_material_in_export`).

## Frozen sources

- `docs/commands/palette_row_and_modifier_contract.md` — the combined palette
  row, modifier-action, automation-cue, and degraded-state contract.
- `docs/commands/palette_query_session_contract.md` — the query-session, history,
  and alternate-invocation contract.
- `docs/commands/sequence_and_modal_discoverability_contract.md` — the
  discoverability projection contract across palette, menus, leader overlays,
  modal sequences, and docs/help.
- `docs/commands/command_descriptor_contract.md` — the canonical command object.
- `schemas/commands/finalize_command_parity.schema.json` — the boundary schema
  for the finalized command-parity record.

## Checked artifact

- `artifacts/commands/m4/finalize_command_parity/support_export.json` — the
  canonical export consumed by UI, CLI, AI, support export, and documentation.
- `artifacts/commands/m4/finalize_command_parity/summary.md` — the Markdown
  summary for support, docs, and review handoff.
- `fixtures/commands/m4/finalize_command_parity/` — the clean stable fixture.

Verify the checked packet with:

```sh
cargo test -p aureline-commands finalize_command_parity
```
