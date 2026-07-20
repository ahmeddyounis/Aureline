<!--
SPDX-FileCopyrightText: 2026 Aureline contributors
SPDX-License-Identifier: Apache-2.0
-->

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
  `.t2/docs/*` files (or a complete, approved, time-boxed source-drift waiver
  is present); and
- every `seed_families[]` row marked `required: true` has at least one existing
  `artifact_refs[]` entry **or** carries a complete, approved, unexpired
  waiver.

Every waiver, including `source_drift_waiver`, fails closed unless it carries:

- a stable `waiver_id`;
- exact `scope` and `justification`;
- explicit `risk`, accountable `owner`, and `mitigation`;
- `opened_on` plus an `approval` object containing non-empty `approved_by` and
  a recognized waiver-authority `forum` value;
- a strict `expires_on` date; and
- a concrete `exit_plan`.

Dates use `YYYY-MM-DD`. Opening may not follow expiry or be future-dated.
Expired, malformed, or incomplete waivers do not satisfy a missing artifact or
permit source-digest drift.

## Source-document provisioning policy

The authoritative `.t2/docs` source pack is intentionally excluded from Git,
so a canonical clean checkout does not contain it. The shared
`ci/contract_validation.sh` lane therefore invokes this one checker with
`--source-doc-policy if-present`:

- when none of the declared source documents are provisioned, the checker
  emits `source_document.not_provisioned` as a visible warning and continues;
- a partially provisioned source pack fails; and
- when the complete source pack is present, every digest is verified exactly
  as in required mode.

Use `--source-doc-policy required` for authoritative local source verification.
That remains the direct checker's default and fails if any declared source
document is absent.

## Updating the matrix

Update `artifacts/governance/source_seed_completion_matrix.yaml` when:

- a source appendix changes the seed promise set (update the doc digest snapshot);
- a seed family gains a new canonical artifact (add/replace `artifact_refs[]`);
- a seed family is deferred (add a named `gap` plus a complete, approved,
  time-boxed `waiver` with the fields above).

## Running locally

```bash
python3 ci/check_source_seed_completion.py \
  --repo-root . \
  --source-doc-policy required
```

Run the fail-closed waiver and source-provisioning regression drills with:

```bash
python3 ci/check_source_seed_completion.py --self-test
```
