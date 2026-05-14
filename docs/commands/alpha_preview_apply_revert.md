# Alpha preview/apply/revert enforcement

This document defines the bounded alpha enforcement lane for destructive or
external-effect commands. It does not replace the command descriptor contract;
it consumes descriptor preview/approval fields and adds the release-facing audit
that says whether a command is reviewed, directly allowed, or explicitly outside
this lane.

Machine-readable proof:

- [`crates/aureline-shell/src/commands/review_enforcement/`](../../crates/aureline-shell/src/commands/review_enforcement/)
  materializes the enforcement snapshot and invocation decision.
- [`fixtures/commands/alpha_preview_apply_revert/`](../../fixtures/commands/alpha_preview_apply_revert/)
  carries the protected fixture manifest.

## Contract

Every command with destructive or external-effect posture must resolve to one
of these states:

| State | Meaning |
| --- | --- |
| `enforced` | Descriptor or fixture requires preview/approval before apply, and all claimed surfaces project that same posture. |
| `explicitly_out_of_scope` | The row is visible as outside this alpha lane, with a typed reason. |
| `direct_allowed` | The command is inert, read-only, or local-current-locus only. |
| `gap_missing_review` | A release-blocking gap: the command has destructive or external-effect posture without a reviewed path or explicit exclusion. |

An invocation session for an `enforced` command is denied when it tries
`apply_direct_trusted_path`, omits the preview record, or reaches apply while
approval is still pending.

## Explicit alpha exclusions

These rows remain visible in the enforcement snapshot:

| Command | Reason |
| --- | --- |
| `cmd:docs.open_in_browser` | Read-only docs handoff emits a browser handoff packet but has no mutating apply. |
| `cmd:terminal.toggle` | Opens the terminal surface only; shell input and paste have separate review lanes. |
| `cmd:task.rerun_last` | Seed descriptor only; the task rerun router belongs to the execution-context lane. |
| `cmd:test.rerun_last` | Seed descriptor only; the test rerun router belongs to the execution-context lane. |

## Covered alpha lanes

The snapshot covers the current registry rows plus fixture-backed rows for:

| Lane | Representative command | Review posture |
| --- | --- | --- |
| Workspace | `cmd:workspace.import_profile` | structured diff preview plus explicit confirmation |
| Git | `cmd:git.push_branch` | irreversible publish preview plus approval |
| Provider | `cmd:provider.publish_release_notes` | external mutation preview plus browser/approval evidence |
| Install | `cmd:package.install.apply` | install/update preview plus rollback checkpoint |
| AI mutation | `cmd:ai.apply_patch` | structured diff preview, approval ticket, evidence packet, checkpoint |

The first surfaced consumer is command deep-link review: deep links now carry the
same enforcement row as their diagnostics or invocation-preview sheet, so a
second entry point cannot silently skip the reviewed path.
