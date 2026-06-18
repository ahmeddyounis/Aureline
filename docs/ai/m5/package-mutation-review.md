# AI package-mutation review

This document describes the **AI package-mutation review** surface — the AI
composer's propose-only view of a governed package mutation. It is the
user-facing companion to the support export at
`artifacts/ai/m5/package_mutation_review/support_export.json`, the schema at
`schemas/ai/package-mutation-review.schema.json`, and the typed model in the
`aureline-ai` crate (`package_mutation_review`).

When AI suggests adding, upgrading, removing, or relocking a dependency, that
suggestion must not become a hidden scripting lane. The AI surface is a
**proposer, never an executor**. Every proposal:

- is **preview-first** and **routes through governed review** before any write;
- carries **no hidden scripting**;
- holds only `propose_only` or `inspect_only` write authority — it can never
  execute a mutation; and
- binds by reference to the governed cross-surface contract in `aureline-deps`
  (`automation_governance`), the frozen package-state matrix, and the
  reviewed-mutation contract.

## Mirrors the governed decision, never invents a weaker one

Each AI proposal mirrors the governed **safe-fallback decision** and **result
class** the cross-surface contract recorded. When the claimed ecosystem cannot
deliver the promised review preview, deterministic resolver, durable rollback,
or validation execution — or auth is unsatisfied or the registry is offline —
the AI proposal narrows to inspect-only, a redaction-safe export, or a
browser/CLI handoff, or it blocks. It never falls back to an unsafe in-product
install.

| Safe-fallback | AI result |
|---------------|-----------|
| `proceed_after_review` | `preview_pending` → `reviewed_ready` → `committed_reviewed` (or `rolled_back`) |
| `narrow_to_inspect_only` | `narrowed_inspect_only` |
| `narrow_to_export_only` | `handed_off` |
| `handoff_to_browser` / `handoff_to_cli` | `handed_off` |
| `blocked_no_safe_path` | `blocked_unsafe` |

## Requests the same validation a direct operation would run

Each proposal names the validation tasks it requests — `build`, `test`, `lint`,
`typecheck`, `security_audit`, `license_review`, `lockfile_verify`. The governed
contract enforces that a proposal cannot commit while a required validation task
is unselected; the AI surface requests that validation rather than skipping it.

## The parity proof

The `aureline-ai` tests load the real `aureline-deps` `automation_governance`
packet and assert that the AI surface references the exact governed packet, the
frozen matrix, and the reviewed-mutation contract, and that every AI proposal
whose governed reference resolves agrees with the governed truth on ecosystem,
fallback decision, result class, and rollback handle. That is the concrete proof
that the AI lane reuses the same reviewed mutation contract and is not a bypass
lane.
