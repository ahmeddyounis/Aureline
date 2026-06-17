# Package operation history and export-safe receipts

This document describes **package operation history** — the durable, export-safe
receipt every completed (or attempted) package mutation leaves behind. It is the
user-facing companion to the governed artifact at
`artifacts/deps/m5/operation-history.json`, the schema at
`schemas/deps/operation-history.schema.json`, and the typed model in the
`aureline-deps` crate (`operation_history`).

Where the
[reviewed mutation flows](./reviewed-mutation-flows.md) review a package mutation
*before* commit, operation history owns what survives *after* it. One
`OperationHistoryEntry` is the **single object** rendered by the desktop history
surface, the CLI/headless history listing, AI and recipe follow-ups, and
support/export packets — so users and support can answer *what changed, which
chain it affected, and how to revert it* without reverse-engineering
ecosystem-specific logs.

## A receipt, never a transient toast

Each entry preserves the things a package mutation must not lose:

- the **manifest scope** it targeted and the ecosystem it ran in;
- the **origin** — desktop, CLI/headless, AI, or recipe — and a precise
  **result class**: `applied`, `no_change_needed`, `partially_applied`,
  `rolled_back`, `failed_no_change`, `blocked_by_policy`, or `blocked_by_auth`.
  A failure never collapses into a generic "install failed", and an auth or
  policy block is recorded distinctly from a not-found;
- the **manifest and lockfile identity before and after**, as redacted digests
  and ids rather than full manifest bodies;
- the **resolver state** that produced the resolution;
- the **direct-versus-transitive impact chain**; and
- the **validation outcome** and a **rollback handle** with the
  revert / open-diff / export-patch recovery actions, the durable checkpoint they
  reach, and **evidence refs** back to the validation log, lockfile diff, and
  checkpoint.

## The direct-versus-transitive impact chain

The impact chain is the heart of a receipt. Every package the operation touched
is a link with its **relation** (`direct`, `transitive`, `workspace_local`, or
`path_or_vcs`), its **change** (`added`, `removed`, `upgraded`, `downgraded`,
`repinned`, or `unchanged`), its depth, and — for a transitive package — the
**parent links** that pulled it in. So a receipt reads as a graph: adding `serde`
shows `serde_derive` entering transitively beneath it; removing a crate shows its
now-orphaned transitive dependencies leaving with it.

A receipt that wrote changes (or wrote then reverted) must surface a chain with
at least one changed link and at least one direct link; a no-write or blocked
receipt claims no change. The chain stays visible on **every** surface —
including the redacted support export — so support never has to reconstruct it.

## Result class drives the rest of the receipt

The result class is checked against the rest of the receipt so a receipt can
never be vague changelog prose:

- an `applied` or `partially_applied` operation moved its manifest/lockfile
  identity and carries a **durable, revertible** rollback handle;
- a `rolled_back` operation records the attempted chain but its net identity
  returns to where it started, and its checkpoint and recovery actions remain
  inspectable;
- a `no_change_needed`, `failed_no_change`, `blocked_by_policy`, or
  `blocked_by_auth` operation leaves the identity untouched and carries no
  rollback;
- an `applied` operation cannot carry a failed validation, and a `rolled_back`
  operation reverted because validation failed;
- a produced-impact operation records a validation outcome rather than leaving it
  `not_run`.

## Redaction-default retention

History is bounded-local and redaction-default. Each receipt carries a retention
posture bound to the frozen
[`operation_history`](./freeze-the-m5-package-state-manifest-scope-registry-auth-and-lockfile-authority-matrix.md)
retention rule: it retains **neither raw credentials nor full manifest bodies**,
storing redacted digests and ids instead. No field may leak a raw URL, token, or
secret body, and the AI/recipe origins pass through the same receipts as a manual
operation — automation gets no quieter record.

## The same receipt feeds every surface

Each entry projects into:

- `surface_projection(operation_id, surface)` — the receipt rendered for a
  marketed surface with the write authority that surface may carry, pinned from
  the frozen matrix, so only a mutating surface (desktop, CLI) can revert a
  recoverable operation, AI inspects, and support/export is redacted; the impact
  chain stays visible everywhere;
- `export_projection()` — the redaction-safe rows reused by support/export
  packets and release evidence, with whether every receipt binds to the matrix,
  keeps its chain visible, and is redaction-safe.

Every receipt binds to the frozen matrix through `references_matrix_id`, and
every package-state label it surfaces resolves to a frozen state row, so product,
CLI, and support/export paths express the same governed vocabulary mechanically.
