# M5 mutation-capable form family certification

One canonical, export-safe **promotion model** that certifies each claimed M5
mutation-capable form family against the shared structured-input component lanes and
**auto-narrows the family's qualification claim** when its structured-input, provenance,
draft-recovery, or staged-review proof is stale, partial, missing, or failing. No claimed
M5 mutation-capable form family can stay fully certified while the evidence behind it has
gone cold.

Where [the field/control-row model](m5-field-control-rows.md),
[the form-validation model](m5-form-validation-and-blocked-submit.md),
[the parameter-source model](m5-parameter-source-and-precedence.md),
[the draft-state model](m5-draft-state-and-autosave.md),
[the staged-review model](m5-staged-review-sheets.md),
[the structured-input model](m5-structured-input-and-staged-review.md), and
[the accessibility-and-continuity model](m5-accessibility-and-continuity.md) each freeze one
*component* of the shared structured-input contract, this model freezes the **certification
verdict** for a whole claimed form family: it rolls those components into one
auto-narrowing claim that the About, help, service-health, compatibility, release, and
support surfaces consume instead of re-deciding which families are certified.

- Schema: [`schemas/ux/m5-form-family-certification.schema.json`](../../schemas/ux/m5-form-family-certification.schema.json)
- Canonical support export: [`artifacts/ux/m5-form-family-certification/support_export.json`](../../artifacts/ux/m5-form-family-certification/support_export.json)
- Report: [`artifacts/ux/m5-form-family-certification/report.md`](../../artifacts/ux/m5-form-family-certification/report.md)
- Perturbation corpus: [`fixtures/ux/m5-form-family-certification/`](../../fixtures/ux/m5-form-family-certification/)
- Rust truth source: `crates/aureline-ui/src/m5_form_family_certification`
- Validator: `tools/release/form_family_certification.py`

## What a family carries

Each `FamilyRecord` (`#/$defs/family`) certifies one claimed mutation-capable form family,
identified by `family_id` and its `family` token — one of `provider_connect`,
`admin_source_management`, `request_workspace`, `package_install_review`,
`settings_config_editor`, `import_migration_center`, or `generated_project_bootstrap`. It
binds:

- **The claimed tier** (`claimed_tier`) — the qualification tier the family publicly claims
  (`stable` for a certified family), the target the evidence either upholds or narrows.
- **Lineage** (`lineage`) — the `evidence_run_ref` the certification rides on and an
  actionable `rerun_ref` (the refresh path a narrowed family must keep).
- **Evidence** (`evidence`) — one `EvidenceCell` per required `(dimension, lane)` proof
  pair (see below), each carrying a freshness/pass `state`, a `proof_ref` to the upstream
  lane's support export, a `captured_at`, and a non-generic `proof_label`.
- **Renderings** (`renderings`) — how each `ConsumerSurface` (`about`, `help_inline`,
  `service_health`, `compatibility`, `release_packet`, `support_export`,
  `docs_public_truth`) renders the verdict it consumes, each pointing back at the family via
  `source_family_ref`. A rendering that shows a wider tier than the evidence supports is an
  overclaim.

## The required proof pairs

Every claimed family must certify all five proof dimensions, each mapped onto the upstream
lane(s) that prove it:

| Dimension (`dimension`) | Proof lane(s) (`source_lane`) |
| --- | --- |
| `field_form_validation` | `field_control_rows`, `form_validation_and_blocked_submit` |
| `parameter_provenance` | `parameter_source_and_precedence` |
| `draft_versus_applied` | `draft_state_and_autosave` |
| `interruption_recovery` | `accessibility_and_continuity` |
| `staged_review_before_commit` | `staged_review_sheets`, `structured_input_and_staged_review` |

That makes seven required `(dimension, lane)` evidence cells per family. A missing cell is
treated as unproven, not as passing.

## How the verdict is re-derived

`FamilyRecord::narrow` floors the claimed tier by the weakest evidence the family rests on,
so the effective tier never reads wider than its proof:

| Evidence `state` | Tier floor |
| --- | --- |
| `current`, `not_applicable` | none |
| `stale`, `partial` | `beta` |
| `missing` | `preview` |
| `failing` | `withdrawn` |

In addition: a required proof pair with no cell floors to `preview`; an elapsed
certification-freshness window (`certification_freshness`) ages every certified family to
`beta`; a consumer surface that does not reuse the verdict floors to `beta`; and a rendering
that overclaims the intrinsic tier withdraws the family. The re-derived
`CertificationVerdict` is `certified` when the effective tier matches the claim, `narrowed`
when it is lower, and `withdrawn` at the bottom. Each narrow carries ordered
`NarrowingReason`s and the `stale_or_missing_dimensions` that drove it, plus a non-generic
narrow label — a downgrade is always actionable.

## What `validate` guarantees

`M5FormFamilyCertificationSetPacket::validate` (and its Python twin
`tools/release/form_family_certification.py validate`) independently re-derive every verdict
and fail when the artifact would imply a wider claim than the evidence backs:

- header / identity / redaction / freshness are present and well-formed;
- every form family, proof dimension, proof lane, and consumer surface is represented;
- every required proof pair is present and evidence-coherent (a ref and capture exist iff
  the proof ran), and no cell carries a generic label;
- no `current` cell claims fresher than its `captured_at` allows;
- a narrowed family keeps an actionable rerun path and a non-generic narrow label;
- no rendering overclaims the effective tier;
- at least one family demonstrates the auto-narrowing rule; and
- no raw credential / body material crosses the export.

## Consuming surfaces

About, inline help, service health, compatibility, the release evidence packet, the support
export, and the public docs truth ingest the certified verdicts from this packet rather than
restating form-quality claims. Because each family renders to all seven surfaces, the
qualification state a user sees on About is the same state the release packet promotes on and
support exports for diagnosis — and when the underlying proof goes stale, all of them narrow
together.

## Boundary safety

No credential bodies, secret values, raw provider payloads, absolute paths, or URLs ever
cross this boundary; the packet carries only typed class tokens, booleans, opaque ids, and
redaction-aware reviewable labels. Adding an enum value is additive-minor and requires a
`schema_version` bump; repurposing an existing value is breaking and requires a new decision
row.
