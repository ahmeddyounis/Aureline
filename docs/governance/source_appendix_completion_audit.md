# Source appendix seed completion audit

This audit makes the seed artifacts promised by the source design appendices
under `.t2/docs/` mechanically traceable to concrete, checked-in repository
outputs.

Companion artifacts:

- `artifacts/governance/source_seed_completion_matrix.yaml`
  - machine-readable completion matrix (source appendix → seed family → artifact refs),
    including named gap rows and time-boxed waivers.
- `ci/check_source_seed_completion.py`
  - CI/local gate that fails when a required seed family is missing or the
    matrix is stale without a waiver.

## What this is (and is not)

This audit answers two questions:

1. For each required seed family promised by an appendix, what is the canonical
   repo-local artifact path that satisfies it?
2. If there is no concrete artifact yet, where is the explicit, named gap row
   (with owner, severity, carry-forward target, and blocker posture)?

This audit does **not** try to judge later-phase completeness or rewrite the
source documents. It only keeps the “seed promise → artifact home (or waiver)”
bridge explicit and reviewable.

## How the gate works

`ci/check_source_seed_completion.py` validates:

- the `source_documents[].sha256` snapshot in the matrix matches the on-disk
  `.t2/docs/*` files (or a time-boxed source-drift waiver is present); and
- every `seed_families[]` row marked `required: true` has at least one existing
  `artifact_refs[]` entry **or** carries an unexpired waiver.

## Updating the matrix

Update `artifacts/governance/source_seed_completion_matrix.yaml` when:

- a source appendix changes the seed promise set (update the doc digest snapshot);
- a seed family gains a new canonical artifact (add/replace `artifact_refs[]`);
- a seed family is deferred (add a named `gap` plus a time-boxed `waiver`).

## Running locally

```bash
python3 ci/check_source_seed_completion.py --repo-root .
```
