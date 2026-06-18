# Parameter review and secret references

This document describes the **typed parameter-review object** and the first M5
automation families that consume it. The
[recipe-builder / parameter-review / dry-run contract](recipe-builder-and-macro-contract.md)
already froze *what* a pre-apply parameter-review sheet is
(`parameter_review_sheet_record`) and the reused argument-inspection, verdict,
and redaction vocabularies every surface reads. This contract closes the runtime
gap it left open: the concrete, mutable sheet that reviews each runtime input as
a **typed, provenance-bearing field** before a recipe or macro-backed flow runs
or is shared.

The central rule: runtime input stays reviewable. Every field is typed, the
source layer a value came from is explicit, default-versus-override state is
visible, the save-to-scope choice is explicit and portable, and a secret-bearing
value becomes a **reference** — never a raw literal that could leak into a recipe
file or a default run-history record.

## Companion artifacts

- [`/schemas/automation/parameter-review.schema.json`](../../schemas/automation/parameter-review.schema.json)
  — boundary schema for the `m5_parameter_review_first_consumers_packet`,
  `parameter_review_export_record`, support-export, and CLI/headless shapes.
- [`/schemas/automation/recipe-builder.schema.json`](../../schemas/automation/recipe-builder.schema.json)
  — the frozen `parameter_review_sheet_record` the sheet projects onto.
- [`/artifacts/m5/automation/parameter-review/`](../../artifacts/m5/automation/parameter-review/)
  — the checked-in first-consumers packet, support export, CLI/headless view, and
  compact projection.
- [`/fixtures/automation/m5/parameter-review/`](../../fixtures/automation/m5/parameter-review/)
  — worked-example sheet export, secret-reference sheet, rerun demonstration, and
  the mutation cases that prove the fail-closed gate.
- [`/tools/ci/m5/parameter_review_check.py`](../../tools/ci/m5/parameter_review_check.py)
  — the fail-closed CI gate over the artifacts and fixtures.

The Rust types in `crates/aureline-runtime/src/parameter_review` are the schema
of record; the headless inspector
`crates/aureline-runtime/examples/dump_m5_parameter_review.rs` regenerates every
artifact and fixture from the seed so they are bit-for-bit derivable.

## The parameter-review object

A `ParameterReviewBuilder` is the live review state for one recipe's inputs. It
owns an ordered list of reviewed parameters; it derives each parameter's verdict
and the sheet's unresolved-required count; and it projects the frozen
`parameter_review_sheet_record` on demand. It holds no raw secret and no untyped
control — every projection reads back through the reviewed parameters.

### Typed fields

Each reviewed parameter declares a **field type**: `text`, `integer`, `boolean`,
`enumeration`, `path_reference`, `url_reference`, `secret_reference`,
`duration_ms`, or `environment_profile_ref`. A generic untyped control is not
admissible. Typed validation (`integer_range`, `enum_membership`,
`workspace_relative_path`, `url_scheme`, `secret_broker_handle_present`, …) drives
a `blocked` verdict when a constraint fails.

### Source layer (provenance)

Every parameter carries a **source layer** — where the value came from:
`descriptor_default`, `workspace_saved`, `user_saved`, `recipe_supplied`,
`selection_backed`, `focused_context_backed`, `ai_proposed`, `policy_pinned`, or
`secret_broker`. Each layer maps to a frozen `argument_inspection_kind`, so
provenance reuse is mechanically checkable. The tenth value,
`unspecified_generic_control`, is the inadmissible state the gate refuses: an
ambiguous value hiding in a generic form control with no declared origin.

### Default or override state

The `value_state` keeps default-versus-override visible per parameter:
`default_value` (the unchanged default from the source layer), `overridden` (the
user changed it for this run), `awaiting_input` (a required value not yet
provided), or `policy_pinned` (fixed by admin policy). Overriding a parameter
changes only this state — the type, source layer, and save scope are preserved.

### Secret references, never raw values

A secret-bearing field holds a `SecretReference`: an opaque broker handle plus
the redaction class that governs it. The raw secret never appears as a literal,
so it cannot land in a recipe file or a default run-history record. A secret
field reads as `sensitive_held_for_review` until it is reviewed; a non-secret
field that smuggles a broker handle, or a secret field that claims a resolved
value with no handle, is non-conforming.

### Save-to-scope is explicit

Each parameter declares a **chosen save scope** and the **allowed set** it may be
saved to: `run_only` (not remembered), `workspace`, `user`, or
`organization_policy` (read-only). A chosen scope outside the allowed set is
non-conforming, so a reviewer always knows where a remembered value will persist.
For a secret-bearing value only the broker reference is ever remembered — never
the secret.

### Review verdicts

The verdict is derived, never asserted, and reuses the frozen verdict
vocabulary:

- a failed constraint drives `blocked`;
- a policy-pinned value drives `policy_pinned`;
- a required value awaiting input drives `needs_input`;
- a secret reference held behind a broker handle drives
  `sensitive_held_for_review`; and
- otherwise the value is `resolved`.

### Export, import, and rerun

`ParameterReviewBuilder::export` nests the whole builder verbatim alongside the
derived frozen-sheet projection and an order-stable digest, so import
reconstructs the identical sheet. Provenance and redaction posture survive
export, import, and rerun: every parameter keeps its source layer and redaction
class, and every secret-bearing field keeps its reference. The rerun
demonstration fixture proves a re-projected sheet carries the same source layers
and redaction classes with no raw secret re-materialized.

## First consumers

The first-consumers packet binds the six M5 automation families that now gather
runtime input, each to a seeded sheet:

| Entrypoint | Inputs reviewed |
|---|---|
| `notebook` | Kernel profile, export format, output directory |
| `task_test_debug` | Test selector, parallelism, coverage threshold |
| `request_api` | Environment profile, URL, bearer-token reference, body variable |
| `package` | Audit scope, policy-pinned update channel, registry-token reference |
| `incident` | Incident reference, redaction profile, bundle destination |
| `ai_assistant` | Proposal id, apply mode, signing-key reference |

Each binding carries the sheet's projected `parameter_review_sheet_record`, the
live reviewed parameters, the unresolved count, and the secret-reference count —
proving the surface reuses the canonical sheet rather than a private form.

## Freeze invariants

The packet pins these invariants as schema-level constants. A false value is
non-conforming.

1. `every_parameter_is_typed`
2. `source_layer_is_explicit_for_every_parameter`
3. `default_or_override_state_is_visible`
4. `secret_values_are_references_not_raw`
5. `save_to_scope_is_explicit_and_allowed`
6. `verdicts_reuse_the_frozen_vocabulary`
7. `provenance_and_redaction_survive_export_import`

## How the freeze is enforced

[`/tools/ci/m5/parameter_review_check.py`](../../tools/ci/m5/parameter_review_check.py)
is the fail-closed gate. It blocks stable when an entrypoint is dropped, a sheet
is empty, a parameter loses its source layer, a secret value is not held as a
reference, a save scope is outside its allowed set, the frozen projection
disagrees with the live parameters, or an invariant is violated. The mutation
fixtures under
[`/fixtures/automation/m5/parameter-review/`](../../fixtures/automation/m5/parameter-review/)
each reproduce one blocking state, and the typed Rust consumer mints the
identical packet so `cargo test -p aureline-runtime --test m5_parameter_review`
enforces the same invariants.

## Source anchors

- [`.t2/docs/Aureline_PRD.md`](../../.t2/docs/Aureline_PRD.md) — power-user
  automation requirements, input-safety posture, CLI/headless rules.
- [`.t2/docs/Aureline_Technical_Architecture_Document.md`](../../.t2/docs/Aureline_Technical_Architecture_Document.md)
  — safe-automation matrix, secret-broker and credential-handle architecture.
- [`.t2/docs/Aureline_Technical_Design_Document.md`](../../.t2/docs/Aureline_Technical_Design_Document.md)
  — command invocation / session / result contracts and parameter-review objects.
- [`.t2/docs/Aureline_UI_UX_Spec_Document.md`](../../.t2/docs/Aureline_UI_UX_Spec_Document.md)
  — parameter review, secret-reference handling, and save-to-scope UX.
- [`.t2/docs/Aureline_UX_Design_System_Style_Guide.md`](../../.t2/docs/Aureline_UX_Design_System_Style_Guide.md)
  — parameter-review sheet, typed fields, and source-layer rules.
