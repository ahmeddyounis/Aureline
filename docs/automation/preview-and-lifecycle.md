# Safe Automation Preview and Lifecycle Contract

This document defines the stable vocabulary for command automation labels and
saved automation manifests. It qualifies command surfaces that expose
`Macro-safe`, `Recipe-safe`, `Headless-safe`, `UI-only`, `Approval required`,
`Writes files`, `Runs process`, `Network call`, and `Remote mutation` without
graduating a broad recipe runner.

The machine-readable boundary is
[`/schemas/automation/automation-manifest.schema.json`](../../schemas/automation/automation-manifest.schema.json).
The checked support export is
[`/artifacts/automation/m4/safe_automation_qualification/support_export.json`](../../artifacts/automation/m4/safe_automation_qualification/support_export.json),
and the release-facing matrix is
[`/artifacts/automation/m4/safe-automation-matrix.md`](../../artifacts/automation/m4/safe-automation-matrix.md).

## Controlled Labels

| Label | Stable meaning |
|---|---|
| `Macro-safe` | The command can be captured and replayed locally against explicit editor or review state only. |
| `Recipe-safe` | The command can be inserted as a typed declarative recipe step that preserves command id, revision, arguments, capability, trust, preview, idempotency, provenance, and lifecycle fields. |
| `Headless-safe` | The command has a supported CLI or headless contract with schema-governed output. |
| `UI-only` | The command needs an interactive surface and must not be promised to recipes or headless callers. |
| `Approval required` | The command can run only after current trust, policy, or permission review. |
| `Writes files` | The dominant side effect includes local file or buffer mutation. |
| `Runs process` | The dominant side effect includes process or terminal launch. |
| `Network call` | The dominant side effect includes outbound network access. |
| `Remote mutation` | The dominant side effect can change a remote target. |

Surfaces must render these labels from the descriptor or automation manifest. A
surface must not infer automation safety from UI placement, implementation
callbacks, or command names.

## Manifest Fields

Every saved automation object carries:

- `storage_form`
- `required_capabilities`
- `trust_requirement`
- `preview_policy`
- `idempotency_hint`
- `provenance`
- `lifecycle_label`
- `artifact_authority`

Recorded macros are profile-local and scoped to editor or review replay. They
must create or cite an edit-history checkpoint before replay, must preserve a
deterministic replay boundary, and must reject network, process, secret, remote,
AI, extension, and admin-policy capabilities.

Workspace recipes, extension recipes, admin or curated recipe packs, and
ephemeral generated recipes are intentionally narrower than Stable until their
own proof is green. They may expose label truth, drafts, manifests, and preview
contracts, but an unqualified runner, generated-plan save, or workspace pack
must carry `stable_labels_only_narrowed_runner`, `preview_only`, `labs_only`,
`dependency_gated`, or `denied_for_stable`.

## Surface Rules

`Add to recipe` inserts a typed recipe draft. It does not execute the command,
does not copy ambient authority, and is enabled only when the command carries
`Recipe-safe`.

`Inspect descriptor` is always an inspection surface. It reads the same
descriptor, manifest fields, preview policy, lifecycle label, and why-unavailable
reason that palette, menus, help, diagnostics, CLI, and support export read.

`Replay as macro` is enabled only for `Macro-safe` commands. It remains local to
the editor or review surface and requires the recorded-macro checkpoint and
deterministic replay boundary.

All three surfaces must reject saved artifacts that capture undeclared network
or process access, raw secret values, prompt bodies, clipboard content, or
hidden authority from the invoking surface.

## Export And Import

Automation exports distinguish local-only, signed, policy-provided, and
support-projection artifacts. Support projections are non-executable and
redacted. Import revalidates trust and policy, preserves manifest identity refs
and deterministic replay boundaries, and prevents signed or policy-provided
artifacts from silently downgrading to local-only authority.
