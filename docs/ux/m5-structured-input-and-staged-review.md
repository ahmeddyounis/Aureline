# M5 structured input, parameter provenance, draft state, and staged review

One canonical, export-safe model for the structured-input contract shared by every
mutation-capable form, wizard, and review sheet in the product — provider
configuration forms, project-bootstrap wizards, request-workspace run dialogs,
package/install review sheets, admin policy-rollout sheets, and migration-center
restore reviews. Surfaces stop minting per-feature form semantics and instead bind
one record that makes draft-versus-applied state, value provenance, validation,
submit blockers, staged review, and recovery explicit before a user commits.

- Schema: [`schemas/ux/m5-structured-input-and-staged-review.schema.json`](../../schemas/ux/m5-structured-input-and-staged-review.schema.json)
- Canonical support export: [`artifacts/ux/m5-structured-input-and-staged-review/support_export.json`](../../artifacts/ux/m5-structured-input-and-staged-review/support_export.json)
- Report: [`artifacts/ux/m5-structured-input-and-staged-review/report.md`](../../artifacts/ux/m5-structured-input-and-staged-review/report.md)
- Perturbation corpus: [`fixtures/ux/m5-structured-input-and-staged-review/`](../../fixtures/ux/m5-structured-input-and-staged-review/)
- Rust truth source: `crates/aureline-ui/src/m5_structured_input_and_staged_review`
- Validator: `tools/release/structured_input_staged_review.py`

## What a surface record carries

Each `FormSurfaceRecord` (`#/$defs/surface`) is one claimed (or Labs) mutation-capable
surface, identified by `surface_id`, `surface_kind`, `lane`, `mutation_class`, and
`origin`, with five evidence blocks:

- **Field provenance** (`fields[]`). Every field declares a `source_class` — one of
  `default_value`, `detected_value`, `imported_value`, `policy_locked`,
  `user_override`, or `required_unset` — plus its `field_state`, its
  `validation_state`, whether a user override stays distinct from the value it
  replaced (`override_distinct_from_default`), and whether a policy lock is respected.
- **Form session** (`session`). The `draft_state`, whether draft is visibly distinct
  from applied, autosave/persistence, the `interruption_behavior`, and reconnect
  behavior.
- **Submit blockers** (`submit_blockers[]`). Each blocker declares its class, whether
  it blocks submit, and whether it is explained before submit.
- **Staged review** (`staged_review`). The commit sheet: a declared `target_scope`,
  disclosed omitted defaults, included/excluded/blocked `members[]`, declared
  `side_effects[]`, a rollback/export path, and whether the commit action names the
  scope and effect rather than being a generic Continue.
- **Draft recovery** (`draft_recovery`). Whether a recoverable draft survives
  interruption/restart/reconnect.

Two more blocks make the record self-checking: `integrity` (the headline invariant
booleans) and `verification` (proof currency against the packet's
`verification_freshness` window). `renderings[]` lists the consumer surfaces
(`form_view`, `wizard_step`, `review_sheet`, `diagnostics_panel`, `support_export`,
`ai_evidence`, `help_inline`) and the claim each one shows.

## Effective claim

`FormSurfaceRecord::narrow` re-derives a `SurfaceClaim` per surface so a surface can
never read wider than its evidence:

| Claim | Meaning |
| --- | --- |
| `surface_certified` | Full source-explicit, validation-honest, scope-disclosed, recoverable, rollback-visible contract. |
| `surface_narrowed` | A first-party surface held below certified by a labelled, recoverable gap (pending validation, stale source/proof); the draft stays recoverable and reopenable. |
| `surface_review_overlay` | A review of imported/migrated/restored state; attributable and reopenable but never a local apply. |
| `surface_unsafe` | The contract is broken; the surface falls back to an explicit blocked state with a reopen/keyboard recovery path instead of a clean-but-false submit. |
| `surface_labs_not_claimed` | Labs/unadvertised; makes no public claim and is never widened. |

A higher-rank claim asserts more authority, so a narrowing or floor moves strictly
lower, and a consumer surface that renders wider than the effective claim is itself a
floor (`surface_overclaims`).

### Floor reasons (drop to `surface_unsafe`)

These break the structured-input contract outright: `field_source_hidden`,
`draft_applied_ambiguous`, `policy_lock_overridden_silently`,
`submit_allowed_while_blocking_invalid`, `blocked_prereq_hidden`,
`target_scope_hidden`, `omitted_defaults_hidden`, `side_effect_undisclosed`,
`rollback_consequences_hidden`, `generic_continue_action`, `draft_recovery_lost`,
`imported_state_reads_as_applied`, `reopen_path_lost`, `surface_overclaims`, and
`form_backing_missing`. A floored surface keeps a reopen/keyboard fallback rather than
a misleading clean submit.

### Narrowing reasons (hold at `surface_narrowed`, stay recoverable)

`validation_state_unlabeled`, `cross_field_dependency_unexplained`,
`excluded_members_unlabeled`, `autosave_unavailable`, `restore_prompt_missing`,
`async_validation_pending`, `freshness_unlabeled`, `superseded_state_not_marked`,
`surface_stale`, `verification_proof_stale`, and `verification_proof_missing`. On an
imported/restore review overlay, any non-floor gap drops the surface below the review
overlay because the overlay is already the minimal honest claim.

## Guardrails enforced by the validator

`M5StructuredInputSetPacket::validate` (Rust) and
`tools/release/structured_input_staged_review.py validate` (the CI gate) both refuse a
packet that:

- submits from an ambiguous, source-hidden state, or lets a user override read as a
  detected/imported/default value;
- hides the target scope, omitted defaults, blocked prerequisites, or rollback
  consequences behind a generic Continue;
- silently overrides a policy lock, or submits over an invalid-blocking field;
- discards a recoverable draft on interruption, or loses the reopen path;
- lets an imported/restore review read as a local apply, or lets a rendering surface
  overclaim;
- fails to represent every surface kind, lane, mutation class, source-of-value class,
  or consumer surface, or contains no surface that demonstrates the auto-narrowing
  rule;
- leaks raw credential/secret material across the export boundary.

## Regenerating the artifacts

```bash
# Canonical support export + report (Rust seed is the source of truth)
cargo run -p aureline-ui --example dump_m5_structured_input_and_staged_review \
  > artifacts/ux/m5-structured-input-and-staged-review/support_export.json
cargo run -p aureline-ui --example dump_m5_structured_input_and_staged_review report \
  > artifacts/ux/m5-structured-input-and-staged-review/report.md

# Perturbation corpus
python3 tools/release/structured_input_staged_review.py emit-corpus

# Verify everything (schema, re-derivation, corpus)
python3 tools/release/structured_input_staged_review.py self-test
cargo test -p aureline-ui m5_structured_input
```

The Rust seed builder, the checked-in support export, and the Python re-derivation
are kept byte-aligned: a Rust test asserts the checked-in export equals the in-crate
builder, and the Python `self-test` re-derives every surface and corpus case so the
artifacts can never imply a wider claim than the current evidence backs.
