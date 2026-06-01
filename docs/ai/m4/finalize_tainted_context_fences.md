# Finalize tainted-context fences, content-boundary handling, and imported-data downgrade rules

This stable lane finalizes the tainted-context model so that users, admins,
support, and release packets can all explain — for the *same* evidence id — how
suspicious or externally sourced context was fenced, how untrusted data content
was kept out of the instruction lane, and how a real imported artifact was mapped
and authority-downgraded before it could touch the workspace. The runtime owner
is `aureline_ai::finalize_tainted_context_fences`.

A taint label is not enough. For every material AI or high-risk command run on a
claimed stable row, the packet binds the finalized fence, the content boundary,
the imported-data downgrade, and the cross-surface command parity into one
export-safe artifact, so no apply-capable path can widen authority behind the
user's back.

## Contract

The packet does **not** re-derive context-assembly, evidence, run-history, or
scoped-apply truth. The beta `aureline_ai::tainted_context::TaintedContextBetaPacket`
proves the live narrowing run, the `aureline_ai::evidence::TaintedContextFence`
rows prove a given mutation carried its fence, and the
`aureline_ai::harden_ai_scoped_apply::AiScopedApplyHardeningPacket` proves the
apply lifecycle and cross-wedge command parity. This packet re-exports those
source/taint/origin and command-surface vocabularies verbatim and adds the
finalized invariants the stable line needs:

- **Tainted fences** — every suspicious or externally sourced context row carries
  a fence that strips instruction authority, blocks hidden provider/tool/workspace
  writes, names its strategy and usage constraints, stays auditable, and forbids
  raw bodies on the boundary.
- **Content-boundary handling** — assembled context sits in exactly one lane:
  `trusted_instruction_surface` (the only lane allowed to carry instruction
  authority), `untrusted_data_content`, `quarantined_data_content`, or
  `unknown_boundary_fail_closed`. Every data lane has executable authority
  stripped, is joined to a fence, and the unknown lane fails closed to a summary
  ref or is dropped from context entirely.
- **Imported-data downgrade rules** — each imported artifact is mapped from a
  real artifact (never a synthesized placeholder) to one of the
  `exact` / `translated` / `partial` / `shimmed` / `unsupported` outcome labels.
  The authority downgrade must agree with the outcome: an exact mapping never
  narrows, a translated artifact is reviewed before apply, a partial or shimmed
  artifact narrows below full and keeps mapping diagnostics, and an unsupported
  artifact is blocked. A rollback checkpoint is always preserved, and a narrowed
  import names the approval fence that re-gated it.
- **Command-surface parity** — the AI assistant, palette, menu, keybinding,
  CLI/headless, deep-link, and automation routes share one command descriptor,
  preview, approval, result, and rollback model, enforce the same content
  boundary, honor the same import downgrade, disclose route truth, and run the
  same policy checks. A surface that cannot qualify narrows below Stable rather
  than inheriting an adjacent green row.
- **Exportable evidence lineage** — the in-product evidence id is the join key the
  admin inspector and support export reconstruct the same run from, and the
  rollback lineage refs let a revert reconstruct the run's checkpoints.

## Required behavior

`FinalizedTaintedContextPacket::validate` rejects a packet when:

- a tainted fence does not strip instruction authority, does not block hidden
  writes, is not auditable, or is missing its strategy or usage constraints;
- the content boundaries do not cover both the trusted instruction surface and the
  untrusted data lane, a data lane carries instruction authority or keeps
  executable authority, a data lane is not joined to a fence, or the unknown lane
  does not fail closed;
- the import rows do not cover all five outcome labels, a row was not generated
  from a real artifact, the authority downgrade disagrees with the mapping
  outcome, a lossy mapping is missing diagnostics, a rollback checkpoint is
  missing, or a narrowed import is missing its approval fence;
- the command surfaces do not cover all seven wedges, a claimed-stable surface
  does not preserve full parity, or a surface that cannot qualify still claims
  Stable; or
- an evidence/export ref or the shared evidence id is missing, the rollback
  lineage is empty, or any field carries raw boundary material.

## Boundary

The packet is export-safe. It carries refs, state tokens, coarse classes, counts,
and review labels only. Raw imported bodies, raw prompt bodies, source file
bodies, provider payloads, endpoint URLs, credentials, raw token counts, exact
prices, and billing-account ids stay outside the support boundary.

## Truth source

The checked artifact at
`artifacts/ai/m4/finalize_tainted_context_fences/support_export.json` is canonical
for this lane. Dashboards, docs, Help/About surfaces, and support exports should
ingest it instead of cloning status text. The boundary schema is
`schemas/ai/finalize_tainted_context.schema.json`; the protected fixtures are in
`fixtures/ai/m4/finalize_tainted_context_fences/`. The frozen contracts this lane
projects against are `docs/ai/prompt_injection_and_taint_contract.md` and
`docs/ai/context_assembly_contract.md`.

Verify the checked packet with:

```sh
cargo test -p aureline-ai finalize_tainted_context_fences
```
