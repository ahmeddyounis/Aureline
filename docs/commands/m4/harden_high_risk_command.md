# Harden high-risk command preview, approval lineage, and rollback-handle issuance

This stable lane hardens the *write-capable safety* half of a stable command.
Where the command-contract stabilization packet froze the descriptor fields, the
invocation/result contract, and cross-surface authority parity, and the
command-parity finalization packet finalized the discoverability half, this packet
binds the high-risk preview contract, the approval lineage, the rollback-handle
issuance contract, and the cross-surface authority parity for a high-risk command
family into one export-safe artifact. UI, CLI, AI, support export, and
documentation fixtures read the *same* preview/approval/rollback truth instead of
inferring safety posture from rendered text per surface. The runtime owner is
`aureline_commands::harden_high_risk_command`.

A high-risk command that can apply without a preview, without an attributable
approval, or without an issued rollback handle on one surface but not another is
not a stable command. The packet makes the preview requirements, approval
lineage, rollback handle, and route disclosure inspectable and attributable from
every entry surface, and refuses any row where a write-capable apply skips the
preview, the approval is self-granted or unaudited, the rollback handle is never
issued, a surface widens authority, or a surface claims the Stable lane while
narrowed below it.

## Contract

The packet does **not** re-derive the descriptor, registry, invocation, result,
authority, or preview-gate models. It reuses the canonical contract refs, the
command-surface vocabulary, the surface-qualification posture, and the
evidence-export shape from `aureline_commands::stabilize_command_contract`, and
binds the frozen contracts by their refs, adding the hardened invariants the
write-capable safety lane needs:

- **Declared risk classes** — `risk_classes` records the coarse high-risk effect
  classes (destructive filesystem, bulk multi-file edit, irreversible VCS,
  external network effect, credential/secret access, spend-incurring, policy-
  sensitive automation) that drive preview depth, approval requirements, and
  rollback posture. A high-risk row declares at least one.
- **High-risk preview** — `preview_contract` requires a non-bypassable preview on
  apply-capable paths that enumerates the effect summary, scope/targets, diff or
  impact, route-and-spend disclosure, and destructive confirmation, shows every
  disclosure cue, carries an `apply_guard_ref`, and keeps the no-blind-apply
  guard so an apply can never proceed without an acknowledged preview.
- **Approval lineage** — `approval_lineage` requires the requested, reviewed,
  granted, and recorded-in-audit steps, each carrying an authority class, a
  decision ref, and a basis snapshot ref, bound to one policy epoch, with the
  grant recorded in audit and the no-self-approval, no-authority-widening, and
  enforced-expiry guards, so support and audit can reconstruct who authorized what
  against which basis.
- **Rollback-handle issuance** — `rollback_handle` requires an issued, replayable
  rollback handle bound to checkpoint refs and the in-product evidence id, with a
  typed revert posture that can issue a handle (fully reversible, checkpoint
  revert, or compensating action) and the no-durable-apply-without-handle guard.
  `no_rollback_available` is never a valid posture for a claimed-stable durable
  apply.
- **Cross-surface parity** — `surface_parity_rows` proves that menu/button,
  keybinding, palette, CLI/headless, AI tool, voice, recipe, deep link, and
  browser companion each enforce the same preview, approval, and rollback, disclose
  the same provider/route, run the same policy checks, and never widen authority. A
  surface narrowed below Stable may not claim the Stable lane; a Stable, reachable
  surface may not drift.
- **Evidence / rollback lineage** — `evidence_export` binds the in-product
  evidence id to the admin inspector and support export refs and carries the
  rollback lineage refs a revert reconstructs the command from.

The record is export-safe: refs, state tokens, coarse classes, counts, and review
labels only. Raw command arguments, raw prompts, raw diff bodies, endpoint URLs,
credentials, and signing-key material stay outside the support boundary, and the
validator rejects a packet that leaks them (`raw_material_in_export`).

## Frozen sources

- `docs/commands/command_descriptor_contract.md` — the canonical command object.
- `docs/commands/invocation_result_and_parity_contract.md` — the invocation,
  result, and cross-surface parity contract.
- `schemas/commands/harden_high_risk_command.schema.json` — the boundary schema
  for the hardened high-risk command record.

## Checked artifact

- `artifacts/commands/m4/harden_high_risk_command/support_export.json` — the
  canonical export consumed by UI, CLI, AI, support export, and documentation.
- `artifacts/commands/m4/harden_high_risk_command/summary.md` — the Markdown
  summary for support, docs, and review handoff.
- `fixtures/commands/m4/harden_high_risk_command/` — the clean stable fixture.

Verify the checked packet with:

```sh
cargo test -p aureline-commands harden_high_risk_command
```
