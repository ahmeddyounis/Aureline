# Reviewed mutation flows: install, update, remove, and regenerate

This document describes the **reviewed mutation flows** — the preview-first
review object the four package-mutation flows share. It is the user-facing
companion to the governed artifact at
`artifacts/deps/m5/reviewed-mutation-flows.json`, the schema at
`schemas/deps/reviewed-mutation-flows.schema.json`, and the typed model in the
`aureline-deps` crate (`reviewed_mutation_flows`).

A package mutation is not a text edit. Installing, updating, removing, or
regenerating dependencies can run install scripts, trigger a native build, churn
a lockfile across the whole workspace, and reach a registry that needs
credentials. The **mutation review sheet** makes all of that explicit *before*
commit, and it is the **one object** rendered by the desktop review surface, the
CLI/headless dry run, AI and recipe proposals, and support/export packets — so
the same review and the same rollback guarantee hold everywhere.

Where the
[package-state matrix](./freeze-the-m5-package-state-manifest-scope-registry-auth-and-lockfile-authority-matrix.md)
*freezes the vocabulary*, the
[manifest-scope review](./manifest-scope-review.md) answers *which manifest is
changed*, and the
[grouped-update review](./grouped-update-and-rollback-review.md) reviews a
*group* of version bumps, this object reviews **one mutation flow end to end**:
what it runs, what it resolves with, how far the lockfile churns, and how to undo
it.

## Four flows, never collapsed into one "change"

Each sheet names its flow — `install`, `update`, `remove`, or `regenerate` — so
adding a dependency, bumping one, dropping one, and re-resolving the lockfile are
distinct reviews. The three re-resolving flows (install, update, regenerate)
must disclose the resolver version they used; a regenerate that produces a broad
churn must disclose that churn before any write.

## Script and native-build risk is always labeled

Every sheet carries one script/native-build label, and the five states stay
distinct so a mutation can never masquerade as a harmless text edit:

- **no scripts** — no install scripts or native build are expected;
- **known install scripts** — known package lifecycle scripts will run; the
  operator must acknowledge them;
- **native build required** — a compiler/toolchain build is required, with the
  required toolchains disclosed;
- **unknown hook risk** — script or hook behavior cannot be determined; **commit
  is blocked** until reviewed;
- **policy blocked** — policy blocks the behavior; **commit is blocked** and
  policy can never report that it allows it.

Any risky label must carry a disclosure note. An unknown or policy-blocked label
holds the commit gate closed.

## Resolver identity and lockfile blast radius are explicit

Each sheet keeps the requested constraint and the resolved identity in separate
fields, and names the **resolver** that produced the resolution — its class
(first-party, ecosystem-native, mirror-backed, offline-cache, or unknown) and its
**version**. The **lockfile blast radius** carries a diff class
(`no_lockfile_change`, `entries_added`, `entries_removed`, `entries_repinned`,
`lockfile_created`, or `full_regeneration`), the counts of entries added,
removed, and re-pinned, and whether the lockfile stays authoritative for **exact
restore** — true exactly when the lockfile authority is exact-pinned or
policy-frozen. A whole-workspace scope must be confirmed explicitly before it can
commit.

## Every mutation leaves a durable rollback receipt

Each sheet links to a durable [`RollbackReceipt`]. A receipt preserves the
affected manifests, the lockfile identity **before and after**, the resulting
state, and a rollback class with the **revert / open-diff / export-patch**
recovery actions. A failed or partial mutation leaves a receipt in
`partial_recovery_pending` or `reverted` state — a durable record, never a
transient toast. A receipt must always be durable and offer all three recovery
actions, and its rollback class must leave a real recovery path.

## The commit gate

A sheet's commit gate is **closed** while any of these hold:

- the disposition is `blocked_until_resolved`;
- the script/native-build label is `unknown_hook_risk` or `policy_blocked`;
- registry trust is unsatisfied (`auth_required_unsatisfied`);
- the lockfile is divergent;
- a whole-workspace scope was not confirmed; or
- the rollback checkpoint is missing, non-durable, incomplete, or
  non-recoverable.

A sheet may carry a `committed_after_review` disposition only when the gate is
open, and its disposition must agree with its checkpoint state — a committed
sheet points at a committed receipt, a rolled-back sheet at a reverted one. An AI
or recipe proposal passes the same gate as a manual one; automation cannot bypass
review.

## The same object feeds every surface

Each sheet projects into:

- `surface_projection(sheet_id, surface)` — the sheet rendered for a marketed
  surface with the write authority that surface may carry, pinned from the frozen
  matrix, so only a mutating surface (desktop, CLI) can commit an unblocked
  sheet, the review workspace stages, AI inspects, and support/export is
  redacted;
- `export_projection()` — the redaction-safe rows reused by support/export
  packets and release evidence, with whether any sheet blocks commit, whether
  every sheet discloses all required disclosures, and whether every label binds
  to the matrix.

Every sheet binds to the frozen matrix through `references_matrix_id`, and every
label it surfaces resolves to a frozen state row, so product, CLI, and
support/export paths express the same governed vocabulary mechanically.

[`RollbackReceipt`]: ./reviewed-mutation-flows.md
