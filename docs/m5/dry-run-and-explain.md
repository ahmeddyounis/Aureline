# Dry-run and explain

This document describes the **reusable dry-run/explain preview object** and the
first M5 automation families that consume it. The
[recipe-builder / parameter-review / dry-run contract](recipe-builder-and-macro-contract.md)
already froze *what* a dry-run/explain preview is (`dry_run_explain_packet_record`)
and the aggregate-outcome and safety-label vocabularies every surface reads. This
contract closes the runtime gap it left open: the concrete, mutable preview that
explains each step's side effects before a claimed automation runs.

The central rule: automation stays preview-first where the underlying commands
support it. Predicted writes, process launches, network or remote mutations,
trust/policy blockers, artifact destinations, and idempotence hints stay explicit
before run and survive into exported evidence. A mutating automation is never
presented as safe merely because its preview is compact — the outcome and safety
labels are **derived** from the actions, not asserted.

## Companion artifacts

- [`/schemas/automation/dry-run-explain.schema.json`](../../schemas/automation/dry-run-explain.schema.json)
  — boundary schema for the `m5_dry_run_explain_first_consumers_packet`,
  `dry_run_explain_export_record`, run-history-row, support-export, and
  CLI/headless shapes.
- [`/schemas/automation/recipe-builder.schema.json`](../../schemas/automation/recipe-builder.schema.json)
  — the frozen `dry_run_explain_packet_record` the preview projects onto.
- [`/artifacts/m5/automation/dry-run-explain/`](../../artifacts/m5/automation/dry-run-explain/)
  — the checked-in first-consumers packet, support export, CLI/headless view, and
  compact projection.
- [`/fixtures/automation/m5/side-effect-preview/`](../../fixtures/automation/m5/side-effect-preview/)
  — worked-example preview export, blocked preview, survival demonstration, and
  the mutation cases that prove the fail-closed gate.
- [`/tools/ci/m5/dry_run_explain_check.py`](../../tools/ci/m5/dry_run_explain_check.py)
  — the fail-closed CI gate over the artifacts and fixtures.

The Rust types in `crates/aureline-runtime/src/dry_run_explain` are the schema of
record; the headless inspector
`crates/aureline-runtime/examples/dump_m5_dry_run_explain.rs` regenerates every
artifact and fixture from the seed so they are bit-for-bit derivable.

## The dry-run/explain preview object

A `DryRunExplainPreview` is the live preview state for one recipe's side effects.
It owns an ordered list of previewed actions; it derives the aggregate outcome and
the safety-label union; and it projects the frozen `dry_run_explain_packet_record`
on demand. It asserts no safety — every projection reads back through the actions.

### Side-effect classes

Each previewed action declares a **side-effect class**: `read_only_inspection`
(mutates nothing), `predicted_write`, `process_launch`, `network_call`, or
`remote_mutation`. Every class but `read_only_inspection` projects the matching
frozen safety label (`writes_files`, `runs_process`, `network_call`,
`remote_mutation`), so a mutating action cannot drop its label and read as safe.

### Predicted writes

A `predicted_write` action declares each write it would make: a `write_kind`
(`create_file`, `modify_file`, `delete_file`, `append_file`, `buffer_edit`, or
`stage_vcs`), an opaque workspace-relative `target_ref` (never a raw absolute
path), whether the write is reversible, and a reviewable summary. A predicted
write that declares no write is non-conforming.

### Process, network, and remote actions

A `process_launch`, `network_call`, or `remote_mutation` action is always labeled
as such; it carries its capability declarations and the artifact destinations its
output would reach. This is how a launch or an outbound or remote effect stays
visible before apply rather than hiding behind an inert-looking step.

### Trust and policy blockers

Each action discloses the **trust/policy blockers** in its way: a `blocker_class`
(`trust_gate`, `policy_gate`, `capability_gate`, `approval_required_gate`, or
`missing_credential_gate`), whether it is currently `blocking`, and an opaque
policy or ticket reference. A blocking gate of any class but
`approval_required_gate` denies apply outright; a blocking approval gate gates
apply behind an approval ticket.

### Artifact destinations

Each action names the **artifact destinations** its output would land in:
`workspace_file`, `device_local_path`, `remote_target`, `network_endpoint`,
`external_registry`, or `support_bundle`. The destination is an opaque reference,
never a raw path, URL, or host.

### Idempotence hints

Each action carries an **idempotence hint** — `idempotent`, `idempotent_with_key`,
`not_idempotent`, or `unknown_idempotence` — so a reviewer can tell whether
re-running is safe before it repeats.

### Aggregate outcome (derived)

The aggregate `dry_run_outcome_class` is derived from the actions and posture,
reusing the frozen vocabulary:

- a blocking denial gate (trust, policy, capability, or missing credential) drives
  `would_be_denied_at_gate`;
- a `no_safe_preview` posture drives `no_safe_preview`;
- a required approval (posture or a blocking approval gate) drives
  `would_apply_under_approval`; and
- otherwise the recipe `would_apply`.

The `aggregate_safety_labels` are the union of the recipe-wide portability labels
and each action's derived label, emitted in canonical order.

### Export, run history, and support

`DryRunExplainPreview::export` nests the whole preview verbatim alongside the
derived frozen-packet projection, an attributable `dry_run_preview_run_history_row`,
and an order-stable digest, so import reconstructs the identical preview and the
preview result does not disappear after the dialog closes. The run-history row
carries the outcome, label union, side-effect counts, and digest into run history;
the support export carries one such row per entrypoint. The survival demonstration
fixture proves the outcome and digest come through export, run history, and a
re-import unchanged.

## First consumers

The first-consumers packet binds the six M5 automation families that now support a
preview, each to a seeded preview:

| Entrypoint | Side effects previewed | Outcome |
|---|---|---|
| `notebook` | Read-only cell run, workspace export write | Would apply |
| `task_test_debug` | Test process launch, coverage report write | Would apply |
| `request_api` | Outbound request, saved-response write | Would apply under approval |
| `package` | Lockfile write, external-registry publish | Would apply under approval |
| `incident` | Local bundle write, remote runbook action | Would be denied at gate |
| `ai_assistant` | Proposed-edit write, VCS stage | Would apply under approval |

Each binding carries the preview's projected `dry_run_explain_packet_record`, the
live previewed actions, the run-history row, and the side-effect counts — proving
the surface reuses the canonical preview rather than a feature-local note.

## Freeze invariants

The packet pins these invariants as schema-level constants. A false value is
non-conforming.

1. `every_entrypoint_binds_a_preview`
2. `predicted_writes_are_explicit_before_apply`
3. `process_network_remote_actions_are_labeled`
4. `trust_and_policy_blockers_are_visible`
5. `artifact_destinations_are_named`
6. `idempotence_hints_are_present`
7. `outcome_and_labels_reuse_the_frozen_vocabulary`
8. `preview_survives_export_history_and_support`

## How the freeze is enforced

[`/tools/ci/m5/dry_run_explain_check.py`](../../tools/ci/m5/dry_run_explain_check.py)
is the fail-closed gate. It blocks stable when an entrypoint is dropped, a preview
is empty, a predicted write is not declared, a mutating action is mislabeled
read-only, the frozen outcome or label projection disagrees with the live actions,
or an invariant is violated. The mutation fixtures under
[`/fixtures/automation/m5/side-effect-preview/`](../../fixtures/automation/m5/side-effect-preview/)
each reproduce one blocking state, and the typed Rust consumer mints the identical
packet so `cargo test -p aureline-runtime --test m5_dry_run_explain` enforces the
same invariants.

## Source anchors

- [`.t2/docs/Aureline_PRD.md`](../../.t2/docs/Aureline_PRD.md) — power-user
  automation requirements, dry-run/explain posture, CLI/headless rules.
- [`.t2/docs/Aureline_Technical_Architecture_Document.md`](../../.t2/docs/Aureline_Technical_Architecture_Document.md)
  — safe-automation matrix and side-effect classification.
- [`.t2/docs/Aureline_Technical_Design_Document.md`](../../.t2/docs/Aureline_Technical_Design_Document.md)
  — command invocation / session / result contracts and the safe-automation object.
- [`.t2/docs/Aureline_UI_UX_Spec_Document.md`](../../.t2/docs/Aureline_UI_UX_Spec_Document.md)
  — dry-run/explain, run history, and side-effect preview UX.
- [`.t2/docs/Aureline_UX_Design_System_Style_Guide.md`](../../.t2/docs/Aureline_UX_Design_System_Style_Guide.md)
  — dry-run/explain sheet, blockers, and idempotence rules.
