# M5 parameter-source and precedence inspectors across forms

One canonical, export-safe model for the **parameter-source inspector** — the
surface a user (or a support agent) opens to answer *why a current form value is
present and which source actually wins* before they commit a change or assume a
setting took effect. One inspector model is reused across the provider
account-mapping, source-registration, request-environment, package-install,
settings-config, import-migration, and project-bootstrap forms, instead of each
domain inferring field origin from a single collapsed "current value".

Where [the field/control-row model](m5-field-control-rows.md) freezes a single
field's label, source *tag*, and validation anchor, and
[the staged-review model](m5-staged-review-sheets.md) freezes the commit sheet a
mutation stops at, this model freezes the **source-and-precedence truth** behind a
field's current value: which layer supplied it (default, detected, imported,
environment-resolved, user-override, policy-provided), what scope it carries
(personal/local vs workspace/shared vs policy-owned), why it won, and whether a
policy lock pins it — so imported values, policy locks, detected values, and user
overrides stay visually distinct instead of collapsing into one current field state.

- Schema: [`schemas/ux/m5-parameter-source-and-precedence.schema.json`](../../schemas/ux/m5-parameter-source-and-precedence.schema.json)
- Canonical support export: [`artifacts/ux/m5-parameter-source-and-precedence/support_export.json`](../../artifacts/ux/m5-parameter-source-and-precedence/support_export.json)
- Report: [`artifacts/ux/m5-parameter-source-and-precedence/report.md`](../../artifacts/ux/m5-parameter-source-and-precedence/report.md)
- Perturbation corpus: [`fixtures/ux/m5-parameter-source-and-precedence/`](../../fixtures/ux/m5-parameter-source-and-precedence/)
- Rust truth source: `crates/aureline-ui/src/m5_parameter_source_and_precedence`
- Validator: `tools/release/parameter_source_and_precedence.py`

## What a field carries

Each `ParameterFieldRecord` (`#/$defs/field`) is one field's source inspector,
identified by `field_id`, its `form`, its `lane`, its `origin`, and its
`claim_posture`, plus the source/precedence contract it must hold (`inspector`):

- **Source candidates** (`inspector.candidates`) — a `SourceCandidate` per
  `source_layer` (`default`, `detected`, `imported`, `environment_resolved`,
  `user_override`, `policy_provided`), each carrying its `value_scope`
  (`personal_local`, `workspace_shared`, `policy_owned`), whether it is `present`,
  and whether its source and scope stay individually labelled. `sources_distinct`
  records that the layers never collapse into one current field state.
- **Effective resolution** (`inspector.effective`) — the winning
  `effective_source_layer`, its `effective_value_scope`, whether the
  `effective_source_visible`/`effective_scope_visible`, and a
  `precedence_rank_declared`. The effective layer must be the **highest-precedence
  present candidate** (policy-provided > user-override > environment-resolved >
  imported > detected > default), and its declared rank must match the canonical
  rank.
- **Policy lock** (`inspector.policy_lock`) — whether the field is `policy_locked`,
  whether the `lock_surfaced`, and whether the form still allows an
  `override_allowed_despite_lock`. A locked field pins the effective value to the
  policy-provided value and never allows a silent user override.
- **Fallback disclosure** (`inspector.fallback`) — whether the effective value
  `is_fallback` (a built-in/auto source won because nothing higher was set), and
  whether the fallback reason is `fallback_reason_disclosed`/`fallback_reason_labeled`.
- **Precedence explanation** (`inspector.precedence_explained`) — whether the
  precedence ordering is surfaced rather than implied.

`declared_detection_state` with `detection_state_visible`/`superseded_state_marked`
and a `verification` proof age a stale, superseded, or unproven snapshot down instead
of letting it read as fresh; `integrity.submit_gated_on_source_clarity` records that a
mutation-capable field never submits from an ambiguous source-hidden state;
`declared_reopen_target` declares what an inspect/reopen returns the user to;
`lineage` (including `form_ref`, `provider_ref`, `source_artifact_ref`, `policy_ref`,
`environment_profile_ref`, and `reopen_backlink_ref`) attributes the field; and
`renderings[]` lists the consumer surfaces — the inspector panel, the field popover,
diagnostics, the support export, AI evidence, inline help/docs, and the CLI/headless
inspect path — and the claim each one shows.

## Effective claim

`ParameterFieldRecord::narrow` re-derives a `ParameterClaim` per field so an
inspector can never read wider than its evidence:

| Claim | Meaning |
| --- | --- |
| `parameter_certified` | Full source-explicit, precedence-correct, scope-explicit, lock-honoured parameter-source contract. |
| `parameter_narrowed` | A first-party field held below certified by a labelled, recoverable gap (an unlabelled non-winning candidate, a generic fallback reason, an unsurfaced precedence explanation or detection state, a superseded/stale detection snapshot, a stale/missing proof); the source stays inspectable. |
| `parameter_review_overlay` | A read-only review of imported/migrated values; attributable and inspectable but never a user-set value. |
| `parameter_unsafe` | The source/precedence contract is broken; the field falls back to an explicit blocked-submit state with an inspect/keyboard recovery path instead of a clean-but-false value. |
| `parameter_labs_not_claimed` | Labs/unadvertised; makes no public claim and is never widened. |

A higher-rank claim asserts more authority, so a narrowing or floor moves strictly
lower, and a rendering that shows wider than the effective claim is itself a floor
(`inspector_overclaims`).

### Floor reasons (drop to `parameter_unsafe`)

These break the source/precedence contract outright: `effective_source_hidden`,
`sources_collapsed`, `precedence_inconsistent`, `policy_lock_hidden`,
`policy_lock_not_enforced`, `imported_value_reads_as_user_set`,
`fallback_reason_hidden`, `value_scope_hidden`, `ambiguous_submit_allowed`,
`inspect_path_lost`, `inspector_overclaims`, and `provenance_backing_missing`. A
floored field keeps an inspect/keyboard recovery fallback (`declared_reopen_target`
of `inspector_only`/`none_keyboard_fallback` or a `reopen_backlink_ref`) rather than a
misleading clean submit.

### Narrowing reasons (hold at `parameter_narrowed`, stay usable)

`source_labels_unlabeled`, `scope_labels_unlabeled`, `fallback_reason_unlabeled`,
`precedence_explanation_unlabeled`, `detection_state_unlabeled`,
`detection_superseded_unmarked`, `detection_stale`, `verification_proof_stale`, and
`verification_proof_missing`. On an imported/migration review overlay, any non-floor
gap drops the field below the overlay because the overlay is already the minimal
honest claim.

## Guardrails enforced by the validator

`M5ParameterSourceSetPacket::validate` (Rust) and
`tools/release/parameter_source_and_precedence.py validate` (the CI gate) both refuse
a packet that:

- hides a field's effective source, collapses its distinct source layers into one
  current field state, or declares an effective layer that is not the
  highest-precedence present candidate;
- hides or fails to enforce a policy lock, lets an imported/migration review read as a
  user-set value, hides a fallback reason or the value scope, or allows a submit from
  an ambiguous source-hidden state;
- loses the inspect-to-source path, lets a rendering overclaim, or floors a field that
  loses its inspect/keyboard recovery fallback;
- fails to represent every form, lane, source layer, value scope, or consumer surface,
  or contains no field that demonstrates the auto-narrowing rule;
- leaks raw credential/secret material across the export boundary.

## Source and precedence are always named, and an import review is never a user value

The model keeps a value's *origin* and its *precedence* explicit and independent.
`source_layer` records where each candidate came from — and the effective value must
be the highest-precedence present candidate, with its declared rank matching the
canonical order. `policy_locked` records that a policy-owned value pins the field and
forbids a silent override (the override candidate stays visible but never wins).
`is_fallback` records that a built-in/auto value applies only because nothing higher
was set — and the reason must be disclosed. An imported/migrated review stays an
`imported_review` overlay: it is attributable and inspectable but never reads as a
user-set value, and any non-floor gap on it drops it below the overlay rather than
holding it. The same precedence data is what the support export, the CLI/headless
inspect path, and the docs/help references consume — they re-render it rather than
re-describing it.

## Regenerating the artifacts

```bash
# Canonical support export + report (Rust seed is the source of truth)
cargo run -p aureline-ui --example dump_m5_parameter_source_and_precedence \
  > artifacts/ux/m5-parameter-source-and-precedence/support_export.json
cargo run -p aureline-ui --example dump_m5_parameter_source_and_precedence report \
  > artifacts/ux/m5-parameter-source-and-precedence/report.md

# Perturbation corpus
python3 tools/release/parameter_source_and_precedence.py emit-corpus

# Verify everything (schema, re-derivation, corpus)
python3 tools/release/parameter_source_and_precedence.py self-test
cargo test -p aureline-ui m5_parameter_source
```

The Rust seed builder, the checked-in support export, and the Python re-derivation
are kept byte-aligned: a Rust test asserts the checked-in export equals the in-crate
builder, and the Python `self-test` re-derives every field and corpus case so the
artifacts can never imply a wider claim than the current evidence backs.
