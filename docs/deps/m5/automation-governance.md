# AI, recipe, and CLI package-mutation governance

This document describes **package-mutation governance** — how an AI-generated
proposal, a recipe/automation step, and a CLI/headless invocation are bound to
the same reviewed mutation contract as a direct UI operation, so cross-surface
convenience never outruns lockfile-safe review. It is the user-facing companion
to the governed artifact at `artifacts/deps/m5/automation-governance.json`, the
schema at `schemas/deps/automation-governance.schema.json`, and the typed model
in the `aureline-deps` crate (`automation_governance`).

Once a package mutation can be *suggested* by AI, recipes, or CLI/headless
flows, the trust bar rises: those surfaces must not become a bypass lane around
review. A single **governed mutation proposal** is the one object every surface
produces. It reuses the
[reviewed mutation flows](./reviewed-mutation-flows.md) preview and the
[frozen package-state matrix](./freeze-the-m5-package-state-manifest-scope-registry-auth-and-lockfile-authority-matrix.md)
vocabulary, and it makes three governance facts explicit before anything
executes.

## A manifest/lockfile impact preview, reused — not re-derived

Each proposal carries a `reviewed_sheet` binding the canonical review sheet by
`sheet_ref` and reusing the same manifest scope, script/native-build label,
resolver identity and version, lockfile diff class, and registry/auth posture.
A governed proposal can never present a weaker preview than the direct review
sheet would.

## Selected validation tasks, before execution

Each proposal names the validation tasks chosen for it — `build`, `test`,
`lint`, `typecheck`, `security_audit`, `license_review`, and `lockfile_verify` —
and which are required before commit. A proposal that proceeds may not leave a
required validation task unselected, and must select at least one. Automation
never skips validation a direct operation would run.

## No unsafe fallback across claimed ecosystems

Each proposal records an ecosystem-capability posture: whether the claimed
ecosystem can actually deliver the promised review preview, deterministic
resolver, durable rollback, and validation execution. When any promised
capability is missing — or any intrinsic safety blocker holds (a policy-blocked
or unknown-hook script, unsatisfied auth, a divergent lockfile, or an
unconfirmed whole-workspace scope) — the **execution decision must narrow**:

| Execution decision | Meaning |
|--------------------|---------|
| `proceed_after_review` | All capabilities hold; the mutation may commit after review. |
| `narrow_to_inspect_only` | No mutation; review/preview only. |
| `narrow_to_export_only` | A redaction-safe export of the plan, not execution. |
| `handoff_to_browser` | Handed off to the provider's browser flow. |
| `handoff_to_cli` | Handed off to a CLI/headless flow. |
| `blocked_no_safe_path` | No safe execution path; the proposal is blocked. |

Only `proceed_after_review` permits a commit. An intrinsically unsafe proposal
can never carry it; the governance layer rejects any such "unsafe fallback".

## Cross-surface parity is the point

A `parity` attestation proves the governed proposal reuses the reviewed
contract, preserves the script/native-build disclosure, the registry/auth
posture, the validation selection, and the rollback packet, never becomes a
bypass lane, never turns package mutation into hidden scripting, and never
silently broadens scope. The **commit gate** refuses a `committed_reviewed`
result while any intrinsic safety blocker holds, while a required validation
task stays unselected, while parity is broken, or while the rollback handle is
not a durable recovery path.

The result class and rollback handle a proposal surfaces are **identical** across
the desktop, CLI, AI, and support/export surfaces — only the per-surface write
authority differs (desktop and CLI may execute an unblocked proposal; the AI
surface is inspect-only; support/export is redacted). A failed or partial
mutation leaves a durable rollback handle with revert / open-diff / export-patch
recovery, never a transient toast.

## What this is not

This lane adds no autonomous remediation and no background dependency updates.
Every governed proposal is preview-first and reviewed; automation is a proposer,
never an unattended executor.
