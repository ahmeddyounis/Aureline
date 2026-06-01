# Harden AI scoped-apply, preview/approval/revert, and multi-file patch honesty

This stable lane hardens the AI scoped-apply path into one route-explicit,
policy-governed, export-safe artifact. The runtime owner is
`aureline_ai::harden_ai_scoped_apply`.

An AI apply is not stable unless every write-capable entry point goes through the
same previewed, approved, revertible, scope-bounded, honestly-disclosed path —
and unless that path looks identical from the UI, the CLI, a deep link, an
automation recipe, and the AI assistant. The scoped-apply hardening packet binds
those invariants into one attributable artifact.

## Contract

The packet does **not** re-derive mutation, run-history, patch-review, or replay
truth. The `aureline_ai::evidence::AiMutationEvidencePacket` mutation wedge, the
`aureline_ai::run_history` lane, the `aureline_ai::finalize_ai_evidence_packets`
finalization lane, and the frozen multi-file patch-review-summary boundary
(`schemas/ai/patch_review_summary.schema.json`) remain canonical for their own
slices. The packet references those lineages by id and adds:

- **Preview → approval → apply → revert lifecycle** — a mandatory preview record,
  an explicit approval when required, a rollback checkpoint captured before any
  live mutation, the mutation journal and apply audit produced by an apply, and
  an available revert handle afterward. A direct trusted-path apply is always
  denied, and a rejected or blocked apply must never have mutated the live tree.
- **Scoped-apply honesty** — a declared scope (label, class, requested-scope ref,
  and path-class count) that the produced patch must stay inside. A file that
  reaches the live tree outside the declared scope is a release-blocking gap.
- **Multi-file patch honesty** — a content-addressed patch digest plus per-file
  rows (change kind, hunk count, in-scope, disclosed-in-preview, approved, and
  reached-live-tree). The declared and disclosed file counts must match the rows,
  so a patch can never carry a file the operator was not shown, and only
  disclosed, approved, in-scope hunks may reach the tree.
- **Cross-wedge command parity** — palette, menu, keybinding, CLI/headless, deep
  link, automation recipe, and AI assistant each project the same command
  descriptor, preview, approval, result, and rollback model, disclose route
  truth, and run the same policy checks. A surface that does not preserve the
  full shared model, or that claims Stable without qualifying for it, is
  automatically narrowed below Stable rather than inheriting an adjacent green
  row.
- **Route/spend authority truth** — provider/model identity, route and spend
  receipt refs, disclosed egress, a tainted-context fence when tainted context
  participates, and an explicit "authority did not widen without disclosure"
  assertion.
- **Exportable evidence/rollback lineage** — the bound AI evidence packet, the
  patch-review summary, and a rollback handle that describes recovery without
  granting ambient authority, plus the JSON/Markdown export refs.

## Required behavior

`AiScopedApplyHardeningPacket::validate` rejects a packet when:

- a write-capable path is reachable without a shown preview, or an applied state
  lacks granted/required approval;
- a direct trusted-path apply was attempted;
- an applied state is missing its checkpoint, mutation journal, apply audit, or
  an available revert handle, or a rejected/blocked state still mutated the tree;
- the scope contract is incomplete, the apply is not bounded to its declared
  scope, or a file reaches the live tree outside the declared scope;
- the declared/disclosed file counts do not match the rows, a modify row carries
  no hunks, a rename row omits its source, a file reaches the tree without being
  disclosed in the preview, or a file reaches the tree without approval;
- a launch wedge is not covered, a surface row drops any part of the shared
  command/preview/approval/result/rollback model or its route/policy disclosure,
  or a surface claims Stable without qualifying for it;
- route/spend authority is incomplete, egress is not disclosed, tainted context
  participates without a fence, or authority widened without disclosure;
- an evidence/rollback export ref is missing; or
- any field carries raw boundary material.

## Boundary

The packet is export-safe. It carries refs, state tokens, coarse classes,
counts, digests, and review labels only. Raw patch bodies, raw diff text, raw
prompt bodies, source file bodies, provider payloads, endpoint URLs, credentials,
raw token counts, exact prices, and billing-account ids stay outside the support
boundary.

## Truth source

The checked artifact at
`artifacts/ai/m4/harden_ai_scoped_apply/support_export.json` is canonical for
this lane. Dashboards, docs, Help/About surfaces, and support exports should
ingest it instead of cloning status text. The boundary schema is
`schemas/ai/ai_scoped_apply_hardening.schema.json`; the protected fixture is
`fixtures/ai/m4/harden_ai_scoped_apply/`. The frozen contracts this lane projects
against are `docs/commands/alpha_preview_apply_revert.md` and
`docs/commands/invocation_result_and_parity_contract.md`.

Verify the checked packet with:

```sh
cargo test -p aureline-ai harden_ai_scoped_apply
```
