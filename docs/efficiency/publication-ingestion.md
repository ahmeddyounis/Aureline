# Efficiency publication ingestion

Every stable-facing description of low-power or thermal behavior must derive from
the same canonical efficiency-state object. Documentation, in-product help, the
About surface, service-health, the policy-or-admin surface, and support exports
do not write their own low-power prose; they **ingest** an efficiency-state claim
entry and render its fields. This page describes the ingestion contract and the
register that enforces it.

## Why ingestion instead of cloned prose

A low-power claim is only trustworthy while the efficiency-state evidence behind
it holds. The
[M5 efficiency-state governance matrix](../../artifacts/efficiency/m5-efficiency-governance.json)
recomputes, for every M5 surface row, a certification state and a **narrowed
effective posture**: when a row's efficiency-state evidence, hidden-work
suppression, protected-path preservation, override policy-awareness, recovery
staging, or consumer propagation cannot back its claim, the row narrows or
quarantines automatically.

If a docs page, help topic, About box, service-health row, admin field, or
support export carried its own copy of the wording, that copy would keep
asserting the old posture after the matrix narrowed it. Hand-written low-power
copy in a stable-facing surface is therefore not allowed once a canonical claim
entry exists. Each surface binds to the entry and renders the entry's current
values, so a narrowing in the matrix downgrades every surface in the same train.

## The canonical objects

| Object | Location | Identifier |
| --- | --- | --- |
| Efficiency-state claim entry | [`m5-efficiency-governance.json`](../../artifacts/efficiency/m5-efficiency-governance.json) (a matrix `row`) | `row_id` (the binding's `consumes_entry_id`) |
| Closed efficiency vocabulary | [`m5-efficiency-governance.json`](../../artifacts/efficiency/m5-efficiency-governance.json) `closed_vocabularies` | token |
| Ingestion register | [`publication-ingestion-register.json`](../../artifacts/efficiency/publication-ingestion-register.json) | `register_id` |
| Admin / support field reference | [`admin-surface-fields.md`](../../artifacts/efficiency/admin-surface-fields.md) | field name |

Each claim entry carries the `efficiency_state`, the `source_of_change`, the
`posture`, the `published_claim_ceiling`, the narrowed `effective_posture`, the
`certification_state`, the `override_posture`, the `recovery_state`, and the
`fired_narrowing_reasons` the surface renders. The register derives a
`claim_support` level from the certification state — `supported`, `narrowed`, or
`unsupported` — so the same word appears on every surface.

## The ingestion register

[`publication-ingestion-register.json`](../../artifacts/efficiency/publication-ingestion-register.json)
binds each consuming surface to the entry it renders. It is regenerated from the
governance matrix by
[`tools/regenerate_efficiency_publication_ingestion.py`](../../tools/regenerate_efficiency_publication_ingestion.py).
Each binding records:

- `surface` — `docs`, `help`, `about`, `service_health`, `admin`, or
  `support_export`.
- `surface_locator` — where the surface renders the entry.
- `consumes_entry_id` — the governance-matrix row id.
- `renders_verbatim_from_entry` — asserts the surface renders the entry's values,
  not hand-written prose.
- `rendered_projection` — the `efficiency_state`, `source_of_change`, `posture`,
  `published_claim_ceiling`, `effective_posture`, `certification_state`,
  `claim_support`, `override_posture`, `recovery_state`, and
  `fired_narrowing_reasons` the surface shows. Every value must equal the entry's.
- `disclosed_fields` — the export-safe field names the surface discloses.

## What the gate enforces

`ci/check_efficiency_publication_ingestion.py` (workflow
`check_efficiency_publication_ingestion`) fails closed when:

1. **A binding renders a posture that differs from the entry.** The
   `rendered_projection` must equal the matrix row's efficiency state,
   source-of-change, posture, published ceiling, narrowed effective posture,
   certification state, claim-support level, override posture, recovery state, and
   fired narrowing reasons. This is what makes a narrowed or unsupported claim —
   and its override rules — propagate to every surface at once, and what stops a
   surface from ever publishing a stronger claim than the entry supports.
2. **A binding points at an unknown entry**, i.e. a row absent from the matrix.
3. **The About surface advertises an unsupported claim.** About is authorized only
   for entries whose effective posture is claim-bearing; a quarantined entry is
   never advertised there, only documented on docs, help, service-health, admin,
   and support exports.
4. **A binding discloses a field that is not export-safe**, or a field on the
   forbidden raw-telemetry denylist. See
   [`admin-surface-fields.md`](../../artifacts/efficiency/admin-surface-fields.md)
   for the export-safe field set and the redaction rules.
5. **A surface that must render an entry is missing.** Docs, help, service-health,
   admin, and support exports must render every entry; the About surface must
   render every claim-bearing entry.
6. **The register's narrowed/quarantined projection disagrees with the matrix.**

The checked-in fixtures in
[`fixtures/efficiency/docs-ingestion/`](../../fixtures/efficiency/docs-ingestion/)
and the validator's negative drills prove each rejection path fires.

## How a narrowed or unsupported claim propagates

Because every surface renders the entry's `effective_posture`,
`certification_state`, and `claim_support`, a narrowed or unsupported claim shows
the same way everywhere. The single quarantined entry —
`eff.companion_adjacent.badge` — renders as `unsupported` with its fired
narrowing reasons on docs, help, service-health, admin, and support exports, is
withheld from About, and never re-inflates to a claim-bearing posture. The
register's `propagation_projection` lists exactly the narrowed and quarantined
entries, recomputed from the matrix, so a surface can never keep advertising a
ceiling the matrix has withdrawn.

## Adding or changing a surface

1. Add or update the claim entry in the governance matrix first; never start from
   the surface copy. Regenerate the matrix with
   `python3 tools/regenerate_m5_efficiency_governance.py`.
2. Regenerate the ingestion register with
   `python3 tools/regenerate_efficiency_publication_ingestion.py`, which copies
   each entry's projection into a binding and discloses only export-safe fields.
3. Run `python3 ci/check_efficiency_publication_ingestion.py --repo-root .` until
   it passes.

Never leave hand-written low-power copy in a stable-facing surface once its
canonical claim entry exists.
