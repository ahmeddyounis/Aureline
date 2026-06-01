# Stabilize the command descriptor, invocation-session/result packets, invocation preview, and cross-surface authority parity

This stable lane finalizes commands as public product objects. For one canonical
command family on a claimed stable row, the packet binds the descriptor's stable
fields, the invocation-session/result-packet contract, the palette diagnostics
contract, and the cross-surface authority parity into one export-safe artifact,
so UI, CLI, AI, support export, and documentation fixtures all read the *same*
command truth instead of cloning status text or inferring meaning from UI labels
and internal callbacks. The runtime owner is
`aureline_commands::stabilize_command_contract`.

A command whose meaning can only be read off a button tooltip is not a stable
command. The packet makes the descriptor fields, result codes, structured
disabled reasons, and surface parity inspectable and attributable from every
entry surface, and refuses any row where a surface widens authority, suppresses a
preview/approval, or claims the Stable lane while it is narrowed below it.

## Contract

The packet does **not** re-derive the descriptor, registry, invocation, result,
or authority models. The
`aureline_commands::descriptor::CommandDescriptorRecord`,
`aureline_commands::invocation::InvocationSessionPacketRecord`,
`aureline_commands::invocation::CommandResultPacketRecord`,
`aureline_commands::registry::CommandDescriptorPublicContractRecord`, and
`aureline_commands::authority::CommandAuthorityScenarioRecord` own those
contracts. This packet binds them by their canonical schema refs and adds the
finalized invariants the stable line needs:

- **One descriptor registry and one result schema** — `contract_refs` pins the
  single canonical descriptor schema, registry-entry schema, seeded-registry
  artifact, invocation-session schema, result-packet schema, public-contract
  projection schema, parity-expectation schema, and structured disabled-reason
  vocabulary every surface projects from. Any drift from this canonical set is a
  `contract_refs_not_canonical` violation, so consumers cannot quietly fork the
  registry or the result schema.
- **Stable descriptor fields** — the descriptor fields that make a command a
  public product object (command id, invocation schema, capability class,
  enablement rules, discoverability record, automation labels, result contract,
  lifecycle and origin metadata, alias/deprecation metadata, docs/help anchor,
  accessibility labels, and shortcut narration) must all be present, exported,
  and marked as stable interfaces. Stable command IDs may not be repurposed once
  they leave the experimental state, and docs/help and migration surfaces read
  these fields instead of maintaining a parallel hand-written command dictionary.
- **Invocation-session / result-packet contract** — `result_contract` enumerates
  the stable result-code vocabulary (`succeeded`, `succeeded_with_warnings`,
  `partially_applied`, `cancelled`, `blocked_by_policy`, `disabled_with_reason`,
  `preview_required`, `approval_required`, `failed`) and requires the result
  packet to preserve canonical command identity, alias resolution, the issuing
  surface, created-artifact refs, a notification/activity join, a reversible
  rollback handle for durable commands, checkpoint refs, evidence refs, and the
  strict no-bypass guards. Support, automation, and UI never infer an outcome
  from rendered text alone.
- **Palette diagnostics** — `palette_diagnostics` requires the palette row to
  show the source badge, keybinding, dominant side-effect cue, disabled-with-reason
  state, and preview/approval posture, and to expose the Copy command ID, Copy
  CLI equivalent, Add to recipe, and Why not automatable? actions where valid.
- **Structured disabled reasons** — `disabled_reason_cases` covers the
  disabled-by-policy, wrong-focus, missing-runtime, degraded-provider,
  preview-required, approval-required, and UI-only cases, each mapped to a
  canonical machine reason code from the frozen vocabulary with a structured
  explanation ref and repair-hook ref. Support bundles, CLI errors, palette
  diagnostics, and automation insertions all resolve the same reason rather than
  surface-local prose.
- **Cross-surface authority parity** — `surface_parity_rows` proves the same
  command requires the same preview, approval, rollback, and audit semantics from
  menu/button, keybinding, palette, CLI/headless, AI tool, voice, recipe, deep
  link, and browser companion. A Stable, reachable surface must share the
  descriptor, preview, approval, rollback, and audit models, resolve aliases to
  the canonical command id, disclose route truth, run the same policy checks, and
  keep its automation labels honest. A surface narrowed below Stable may not claim
  the Stable lane.
- **Evidence / rollback lineage** — `evidence_export` binds the in-product
  evidence id to the admin inspector and support export refs and carries the
  rollback lineage refs a revert reconstructs the command from.

The record is export-safe: refs, state tokens, coarse classes, counts, and
review labels only. Raw command arguments, raw prompts, endpoint URLs,
credentials, and signing-key material stay outside the support boundary, and the
validator rejects a packet that leaks them (`raw_material_in_export`).

## Frozen sources

- `docs/commands/command_descriptor_contract.md` — the command-descriptor
  contract this lane projects descriptor fields against.
- `docs/commands/invocation_result_and_parity_contract.md` — the invocation,
  result, and cross-surface parity contract.
- `docs/commands/disabled_reason_vocabulary.md` — the structured disabled-reason
  vocabulary the disabled-reason cases resolve against.
- `schemas/commands/stabilize_command_contract.schema.json` — the boundary schema
  for the stabilized command-contract record.

## Checked artifact

- `artifacts/commands/m4/stabilize_command_contract/support_export.json` — the
  canonical export consumed by UI, CLI, AI, support export, and documentation.
- `artifacts/commands/m4/stabilize_command_contract/summary.md` — the Markdown
  summary for support, docs, and review handoff.
- `fixtures/commands/m4/stabilize_command_contract/` — the clean stable fixture.

Verify the checked packet with:

```sh
cargo test -p aureline-commands stabilize_command_contract
```
