# Truth-vocabulary parity corpus

Fixtures for the cross-surface truth-vocabulary parity gate
(`ci/check_truth_vocabulary_parity.py`, run via
`scripts/ci/run_truth_vocabulary_parity.sh`).

The gate enforces one controlled vocabulary for the trust-bearing and
release-bearing state words every user-visible channel quotes, against the
governed registry at `artifacts/governance/product_truth_vocabulary.yaml`.

## Files

- `surface_corpus.json` — the **conforming** cross-surface state map. Each
  subject is one protected state instance; every surface usage must resolve to
  the subject's `expected_canonical` term using a canonical word or an allowed
  alias. This corpus is expected to pass clean.

- `conformance_corpus.json` — the **failure drills**. Each drill injects a
  known drift (cross-surface conflict, forbidden synonym, unknown term,
  deprecated alias within window, expired alias, advisory-surface warning) and
  declares the finding the gate must produce. The validator re-runs its own
  classification per drill and asserts the expected finding fires, so the gate
  cannot rot into a no-op. These drills do not fail the overall run; only a
  drill that fails to fire does.

## Subject shape

```json
{
  "subject_id": "unique id",
  "class_id": "lifecycle",
  "axis_id": "lifecycle_state",
  "expected_canonical": "preview",
  "usages": [
    {"surface_class": "help_about", "term": "Preview", "source_ref": "path"}
  ]
}
```

`class_id`/`axis_id` and `surface_class` must exist in the registry. `term` is
the literal state word the surface uses; the validator resolves it to a
canonical term, an allowed/deprecated/forbidden alias, or unknown.

## Refresh

These fixtures are hand-authored. After changing the registry or a fixture,
regenerate the parity report:

```bash
scripts/ci/run_truth_vocabulary_parity.sh --write
```
