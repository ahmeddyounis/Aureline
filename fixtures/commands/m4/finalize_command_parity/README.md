# Finalize command parity, palette diagnostics, modifier-action footers, and copy-CLI/recipe paths

This fixture set exercises the finalized command-parity record owned by
`aureline_commands::finalize_command_parity`. For one canonical command family
and one evidence id, the packet binds the one discoverability record, the
per-surface projection rows, the modifier-action footer contract, the palette
query-session privacy posture, and the disabled-with-reason chip parity so every
discoverability surface (palette, menus, tooltips, leader/help overlays,
keybinding help, onboarding tips, voice hints, deep links, docs/help pages)
projects the same command truth and no surface drifts, hides footer actions
behind a debug-only mode, dispatches from a copy action, or silently widens
palette history beyond the local device.

`finalize_command_parity_packet.json` covers the clean stable case:

- the canonical `contract_refs` set — the single descriptor schema,
  registry-entry schema, seeded-registry artifact, invocation-session schema,
  result-packet schema, public-contract projection schema, parity-expectation
  schema, and structured disabled-reason vocabulary every surface projects from;
- the one discoverability record (canonical command id, primary label ref, alias
  set, category refs, docs/help anchor, keyword refs) every surface reads from;
- the nine per-surface projection rows, each projecting from the canonical record
  with no drift in alias set, copy command ID, copy CLI form, add-to-recipe,
  modifier-action footer, disabled-reason chip, or examples, and resolving aliases
  to the canonical command id at the Stable lane;
- the modifier-action footer contract enumerating default run, split/open-alt,
  open alternate target, copy command ID, copy CLI form, add to recipe, and
  inspect "why not automatable?", with held-modifier intent surfaced, no
  debug-only requirement, copy/inspect actions that never dispatch, and
  placement/target deltas that never widen command authority;
- the local-first palette query-session privacy posture with a typed history
  policy, bounded retention, clear-or-disable controls, held-modifier intent, a
  local-private redaction posture, no raw-query export, and no cross-device
  widening without an explicit governing feature;
- the seven disabled-with-reason chips (disabled-by-policy, wrong-focus,
  missing-runtime, degraded-provider, preview-required, approval-required,
  UI-only), each mapped to a canonical machine reason code with a shared
  explanation ref and "why not automatable?" ref, resolving identically across
  surfaces; and
- the evidence export binding the in-product evidence id to the admin inspector
  and support export refs plus the rollback lineage refs a revert reconstructs
  the command from.

Verify the checked packet with:

```sh
cargo test -p aureline-commands finalize_command_parity --no-fail-fast
```

Regenerate the checked artifact, summary, and fixture after intentional changes
with:

```sh
cargo test -p aureline-commands finalize_command_parity::tests::emit_artifact -- --ignored
```
