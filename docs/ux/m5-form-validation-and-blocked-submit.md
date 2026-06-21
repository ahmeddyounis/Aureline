# M5 form validation, cross-field dependencies, and blocked-submit reasons

One canonical, export-safe model for the layer between the per-row primitive and
the whole-form claim: how a mutation-capable form rolls field-level validity up
into a **form-level validation summary** without replacing the field anchors, how
it **explains cross-field dependencies** when one choice narrows or invalidates
another, and how it emits a **machine-readable blocked-submit reason packet** that
desktop, CLI/headless, support-export, and docs/help surfaces can all reuse to
explain the same failure state.

Where [the field/control-row model](m5-field-control-rows.md) freezes a single
field's label, source, and validation anchor, and
[the structured-input / staged-review model](m5-structured-input-and-staged-review.md)
freezes the per-surface honesty claim of a whole form, this model freezes the
validation engine that connects them: a form cannot submit while a blocked
prerequisite or cross-field invalidation is hidden, its field-level and form-level
validation stay linked rather than duplicated or contradictory, and every blocked
submit ships a reason a headless run or a support bundle can reproduce.

- Schema: [`schemas/ux/m5-form-validation-and-blocked-submit.schema.json`](../../schemas/ux/m5-form-validation-and-blocked-submit.schema.json)
- Canonical support export: [`artifacts/ux/m5-form-validation-and-blocked-submit/support_export.json`](../../artifacts/ux/m5-form-validation-and-blocked-submit/support_export.json)
- Report: [`artifacts/ux/m5-form-validation-and-blocked-submit/report.md`](../../artifacts/ux/m5-form-validation-and-blocked-submit/report.md)
- Perturbation corpus: [`fixtures/ux/m5-form-validation-and-blocked-submit/`](../../fixtures/ux/m5-form-validation-and-blocked-submit/)
- Rust truth source: `crates/aureline-ui/src/m5_form_validation_and_blocked_submit`
- Validator: `tools/release/form_validation_and_blocked_submit.py`

## What a form carries

Each `FormValidationRecord` (`#/$defs/form`) is one mutation-capable form,
identified by `surface_id`, its `lane`, its `origin`, and its `claim_posture`,
plus the validation contract it must hold:

- **Field-level validation anchors** (`field_anchors[]`) — each field declares its
  `validation_state`, whether it is `state_labeled`, whether a blocking/warning
  state is `anchored_to_field` with `exact_rule_text_present`, and whether it is
  `rolled_up_into_summary`. A blocking or warning validation must stay anchored to
  the field with exact rule text; the form-level summary never replaces it.
- **Form-level validation summary** (`form_summary`) — `blocked_value_count`,
  `missing_prerequisite_count`, `derived_constraint_count`, and
  `submit_blocker_count` summarize the whole form, with explicit guarantees that
  the summary `summarizes_field_anchors`, is `consistent_with_fields`, never
  `replaces_field_anchors`, and discloses derived constraints.
- **Cross-field dependencies** (`dependencies[]`) — each declares its
  `dependency_kind` (`provider_account_mapping`, `environment_selection`,
  `package_source_registry_auth`, `import_export_mode`,
  `derived_field_constraint`), its `relation` (`narrows`, `invalidates`,
  `requires`, `mutually_exclusive`), its source/target fields, whether it
  `blocks_submit`, and whether it is `explained_before_submit`.
- **Blocked-submit reasons** (`blocked_submit_reasons[]`) — each carries a stable
  `machine_code`, a `blocker_class`, whether it `blocks_submit`, whether it is
  `explained_before_submit`, a `resolution_hint_present` flag, and the
  `reusable_by` consumer surfaces (`desktop`, `cli_headless`, `support_export`,
  `docs_help`) that can reproduce it.
- **Submit gate** (`submit_gate`) — `submit_allowed` must be closed while any
  blocker is active, every active blocker is `blockers_explained_before_submit`,
  and `commit_action_is_specific` names the scope/effect rather than a generic
  Continue.
- **Backing freshness and proof** — `declared_freshness_state` with
  `freshness_state_visible` and `superseded_state_marked`, plus a `verification`
  proof, so a stale, superseded, or unproven backing value narrows the form
  instead of reading as fresh.

`declared_blocked_fallback` declares the submit-control presentation a floored
form drops to, `lineage` (including `structured_input_ref`) attributes the form to
its structured-input surface, and `renderings[]` lists the consumer surfaces and
the claim each one shows.

## Effective claim

`FormValidationRecord::narrow` re-derives a `FormClaim` per form so a form can
never read wider than its evidence:

| Claim | Meaning |
| --- | --- |
| `form_certified` | Full field-linked, summary-honest, dependency-explained, blocked-submit-reusable validation contract. A form with an *explained, reusable, machine-readable* blocker and a closed gate is still certified — its blocked-submit truth is honest. |
| `form_narrowed` | A first-party form held below certified by a labelled, recoverable gap (a deferred non-blocking dependency, a missing resolution hint, pending async validation, stale/superseded backing, stale/missing proof); it stays usable and reopenable. |
| `form_review_overlay` | A read-only review of an imported/migrated/restored state; attributable but never a local submit. |
| `form_blocked` | The form-validation contract is broken; the form falls back to an explicit blocked state that names the reason instead of a clean-but-false submit. |
| `form_labs_not_claimed` | Labs/unadvertised; makes no public claim and is never widened. |

A higher-rank claim asserts more authority, so a narrowing or floor moves strictly
lower, and a rendering that shows wider than the effective claim is itself a floor
(`rendering_overclaims`).

### Floor reasons (drop to `form_blocked`)

These break the validation contract outright:
`submit_allowed_while_blocked_hidden`, `blocked_reason_unexplained`,
`cross_field_invalidation_hidden`, `field_form_validation_contradicts`,
`form_summary_replaces_field_anchors`, `derived_constraint_hidden`,
`blocked_reason_not_machine_readable`, `blocked_reason_not_reusable`,
`validation_anchor_missing`, `imported_submit_reads_as_applied`,
`rendering_overclaims`, and `validation_backing_missing`. A floored form keeps its
`declared_blocked_fallback` (`shows_reason_on_submit` or `disabled_with_hint`)
rather than a misleading clean submit.

### Narrowing reasons (hold at `form_narrowed`, stay usable)

`cross_field_dependency_deferred`, `resolution_hint_missing`,
`validation_state_unlabeled`, `async_validation_pending`, `freshness_unlabeled`,
`superseded_state_not_marked`, `form_stale`, `verification_proof_stale`,
`verification_proof_missing`, and `reopen_path_lost`. On an imported/restore review
overlay, any non-floor gap drops the form below the overlay because the overlay is
already the minimal honest claim.

## Guardrails enforced by the validator

`M5FormValidationSetPacket::validate` (Rust) and
`tools/release/form_validation_and_blocked_submit.py validate` (the CI gate) both
refuse a packet that:

- lets a form submit while a blocked prerequisite or cross-field invalidation is
  active or unexplained;
- lets the form-level summary contradict or replace the field-level anchors, or
  leaves a field invalid-blocking with no backing blocked-submit reason;
- hides a derived constraint, or defers a blocking validation to a banner instead
  of an exact, field-anchored rule;
- ships a blocked-submit reason with no stable machine code, or a blocking reason
  that the machine consumers (CLI/headless, support export) cannot reuse;
- lets an imported/restore review read as a local submit, or lets a rendering
  overclaim;
- floors a form to a silent submit control with no reason/hint;
- fails to represent every form lane, dependency kind, dependency relation,
  blocker class, blocked-submit consumer, or consumer render surface, or contains
  no form that demonstrates the auto-narrowing rule;
- leaks raw credential/secret material across the export boundary.

## Reusing blocked-submit reasons off the desktop

Each blocked-submit reason is a machine-readable packet: a stable `machine_code`,
a `blocker_class`, a `resolution_hint`, and the `reusable_by` set. A CLI/headless
run reports the same `machine_code` and resolution hint a desktop user sees; a
support export carries the same reason; docs/help can document the same code. The
validator requires the union of `reusable_by` across the set to cover all four
consumers, and every *blocking* reason to remain reusable by the machine consumers
— so a blocker can never exist that only the desktop can explain.

## Regenerating the artifacts

```bash
# Canonical support export + report (Rust seed is the source of truth)
cargo run -p aureline-ui --example dump_m5_form_validation_and_blocked_submit \
  > artifacts/ux/m5-form-validation-and-blocked-submit/support_export.json
cargo run -p aureline-ui --example dump_m5_form_validation_and_blocked_submit report \
  > artifacts/ux/m5-form-validation-and-blocked-submit/report.md

# Perturbation corpus
python3 tools/release/form_validation_and_blocked_submit.py emit-corpus

# Verify everything (schema, re-derivation, corpus)
python3 tools/release/form_validation_and_blocked_submit.py self-test
cargo test -p aureline-ui m5_form_validation
```

The Rust seed builder, the checked-in support export, and the Python re-derivation
are kept byte-aligned: a Rust test asserts the checked-in export equals the
in-crate builder, and the Python `self-test` re-derives every form and corpus case
so the artifacts can never imply a wider claim than the current evidence backs.
