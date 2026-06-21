# M5 field and control row primitives

One canonical, export-safe model for the per-row primitive every mutation-capable
form, wizard, and review sheet in the product is built from. Where
[the structured-input / staged-review model](m5-structured-input-and-staged-review.md)
freezes the *per-surface* honesty claim of a whole form, this model freezes the
*per-row* building block: each field or control row makes its own label, source,
validation, and lifecycle implications explicit so a user can tell exactly what a
value is, where it came from, and why it is currently blocked or constrained
without leaving the form. The shared primitive is proven first across the five
highest-risk M5 lanes — provider/account mapping, source registration,
request-environment, package/install, and migration/import.

- Schema: [`schemas/ux/m5-field-control-rows.schema.json`](../../schemas/ux/m5-field-control-rows.schema.json)
- Canonical support export: [`artifacts/ux/m5-field-control-rows/support_export.json`](../../artifacts/ux/m5-field-control-rows/support_export.json)
- Report: [`artifacts/ux/m5-field-control-rows/report.md`](../../artifacts/ux/m5-field-control-rows/report.md)
- Perturbation corpus: [`fixtures/ux/m5-field-control-rows/`](../../fixtures/ux/m5-field-control-rows/)
- Rust truth source: `crates/aureline-ui/src/m5_field_control_rows`
- Validator: `tools/release/field_control_rows.py`

## What a row carries

Each `FieldControlRow` (`#/$defs/row`) is one field or control row, identified by
`row_id`, its `consumer_lane`, the `consumer_surface_ref` that hosts it, its
`origin`, and its `claim_posture`, plus the primitive contract it must hold:

- **Permanent label and requirement** — `label_mode` is `permanent` (not a
  placeholder that vanishes on focus), and `requirement`
  (`required`/`optional`/`conditional`/`system_managed`) is explicitly
  `requirement_marked` on the row.
- **Source-of-value tag** — `source_class`
  (`default_value`/`detected_value`/`imported_value`/`policy_locked`/
  `user_override`/`required_unset`) is shown via `source_tag_visible`, a user
  override stays `override_distinct_from_origin`, and a policy lock is
  `policy_lock_respected`.
- **Exact validation anchor** (`validation`) — the validation `state`, whether it
  is `state_labeled`, whether the message is `anchored_to_field` with
  `exact_rule_text_present`, and whether it lives `summary_banner_only`. A blocking
  or warning validation must be anchored directly to the field with exact rule
  text, never deferred to a form-level banner alone.
- **Lifecycle implication** (`lifecycle`) — a `restart_required`,
  `reconnect_required`, `trust_required`, or `policy_blocked` implication is
  `surfaced_on_row` on the affected control rather than only in a generic banner.
- **Backing freshness and proof** — `declared_freshness_state` with
  `freshness_state_visible` and `superseded_state_marked`, plus a `verification`
  proof, so a stale, superseded, or unproven backing value narrows the row instead
  of reading as fresh.

`blocked_fallback` declares the presentation a floored row drops to,
`provenance_ref` attributes an imported/restore overlay row, and `renderings[]`
lists the consumer surfaces (`form_view`, `wizard_step`, `review_sheet`,
`diagnostics_panel`, `support_export`, `ai_evidence`, `help_inline`) and the claim
each one shows.

## Effective claim

`FieldControlRow::narrow` re-derives a `RowClaim` per row so a row can never read
wider than its evidence:

| Claim | Meaning |
| --- | --- |
| `row_certified` | Full permanent-label, requirement-clear, source-tagged, validation-anchored, lifecycle-explicit row. |
| `row_narrowed` | A first-party row held below certified by a labelled, recoverable gap (pending async validation, stale/superseded backing, unmarked requirement, stale/missing proof); it stays usable and attributable. |
| `row_review_overlay` | A read-only review of an imported/migrated/restored value; attributable but never an editable local value. |
| `row_blocked` | The row primitive contract is broken; the row falls back to an explicit blocked state that shows the reason on the row instead of a clean-but-false control. |
| `row_labs_not_claimed` | Labs/unadvertised; makes no public claim and is never widened. |

A higher-rank claim asserts more authority, so a narrowing or floor moves strictly
lower, and a rendering that shows wider than the effective claim is itself a floor
(`row_overclaims`).

### Floor reasons (drop to `row_blocked`)

These break the row contract outright: `label_not_permanent`, `source_tag_hidden`,
`policy_lock_overridden`, `validation_anchor_missing`,
`lifecycle_implication_hidden`, `imported_value_reads_as_editable`,
`row_overclaims`, and `row_backing_missing`. A floored row keeps its
`blocked_fallback` (`shows_reason_on_row` or `disabled_with_hint`) rather than a
misleading clean control.

### Narrowing reasons (hold at `row_narrowed`, stay usable)

`requirement_unmarked`, `validation_state_unlabeled`, `async_validation_pending`,
`freshness_unlabeled`, `superseded_state_not_marked`, `row_stale`,
`verification_proof_stale`, and `verification_proof_missing`. On an
imported/restore review overlay, any non-floor gap drops the row below the overlay
because the overlay is already the minimal honest claim.

## Guardrails enforced by the validator

`M5FieldControlRowSetPacket::validate` (Rust) and
`tools/release/field_control_rows.py validate` (the CI gate) both refuse a packet
that:

- hides a field's permanent label or source-of-value tag, or lets a user override
  read as the value it replaced;
- silently overrides a policy lock;
- defers a blocking validation to a summary banner instead of an exact,
  field-anchored rule;
- buries a restart/reconnect/trust/policy implication in a generic banner instead
  of surfacing it on the control;
- lets an imported/restore value read as an editable local value, or lets a
  rendering overclaim;
- floors a row to a silent state with no on-row reason/hint;
- fails to represent every consumer lane, source-of-value class, lifecycle
  implication, requirement class, or consumer render surface, or contains no row
  that demonstrates the auto-narrowing rule;
- leaks raw credential/secret material across the export boundary.

## Regenerating the artifacts

```bash
# Canonical support export + report (Rust seed is the source of truth)
cargo run -p aureline-ui --example dump_m5_field_control_rows \
  > artifacts/ux/m5-field-control-rows/support_export.json
cargo run -p aureline-ui --example dump_m5_field_control_rows report \
  > artifacts/ux/m5-field-control-rows/report.md

# Perturbation corpus
python3 tools/release/field_control_rows.py emit-corpus

# Verify everything (schema, re-derivation, corpus)
python3 tools/release/field_control_rows.py self-test
cargo test -p aureline-ui m5_field_control_rows
```

The Rust seed builder, the checked-in support export, and the Python re-derivation
are kept byte-aligned: a Rust test asserts the checked-in export equals the
in-crate builder, and the Python `self-test` re-derives every row and corpus case
so the artifacts can never imply a wider claim than the current evidence backs.
