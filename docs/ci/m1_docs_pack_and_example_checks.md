# Docs-pack and example freshness CI gate

This page is the reviewer-facing entry point for the **stale-example
detection pipeline** that keeps M1 docs-pack content, canonical example
payloads, and the controlled vocabularies they pin from drifting away
from the live M1 truth surfaces before a dogfood publication.

The gate exists so that:

- a docs/help/About/service-health page can never silently drop its
  source / version / freshness metadata rows;
- a canonical example payload can never silently stop validating
  against its pinned schema;
- a canonical example can never silently pin a vocabulary token that
  the upstream truth-source seed has retired or renamed;
- and every failure points the reviewer at the exact docs-pack row,
  example, schema, or vocabulary reference that drifted — without
  routing through a side spreadsheet.

The gate is fail-close on protected paths. The rollback lever in
`ci/docs/stale_example_gate.yml` allows an explicit advisory downgrade
while a stale-example incident is investigated; it never disables the
lane entirely.

## Canonical sources

- [`artifacts/ci/m1_stale_example_source_map.yaml`](../../artifacts/ci/m1_stale_example_source_map.yaml)
  — the maintained source map pinning the protected docs-pack rows,
  canonical example payloads, schema refs, vocabulary seeds, and named
  failure drill.
- [`tools/ci/check_stale_examples.py`](../../tools/ci/check_stale_examples.py)
  — validator entry point. Writes the durable JSON capture under
  `artifacts/milestones/m1/captures/`.
- [`ci/docs/stale_example_gate.yml`](../../ci/docs/stale_example_gate.yml)
  — CI gate manifest (owner, severity, protected paths, commands,
  rollback lever, refresh policy).
- [`tests/ci/m1_docs_pack_and_example_checks_lane/run_m1_docs_pack_and_example_checks_lane.py`](../../tests/ci/m1_docs_pack_and_example_checks_lane/run_m1_docs_pack_and_example_checks_lane.py)
  — unattended validation lane that exercises the gate and replays the
  named failure drill.

## Upstream truth sources the gate projects from

The gate does **not** mint a new docs/help vocabulary; it only checks
that the protected examples and docs pages keep agreeing with the
truth surfaces that already own those vocabularies:

- [`docs/help/docs_browser_contract.md`](../help/docs_browser_contract.md)
  — the docs/help browser skeleton contract whose `source_class`,
  `version_match_state`, and `freshness_class` rows ground the
  protected examples.
- [`artifacts/help/m1_truth_source_examples.yaml`](../../artifacts/help/m1_truth_source_examples.yaml)
  — the canonical badge-vocabulary seed for the docs/help source class,
  version-match state, and freshness class rows; vocabulary pins on
  the protected examples MUST appear in this seed's
  `vocabulary_tokens` lists.
- [`docs/help/truth_source_model.md`](../help/truth_source_model.md)
  — the reviewer-facing landing page for that seed, listed as
  `upstream_truth_source_ref` for every protected docs pack.

## What the gate checks per protected pack

For every entry in `protected_docs_packs[]` in the source map:

1. **Docs pack page exists.** `pack_ref` resolves to a real file on
   disk. Missing pages fail
   `stale_examples.docs_pack_missing`.
2. **Required metadata tokens are published.** The docs pack page
   contains every literal token listed in
   `required_metadata_tokens` (e.g. `source_class`,
   `version_match_state`, `freshness_class`). A dropped token fails
   `stale_examples.required_metadata_tokens_missing`.
3. **Every protected example payload exists, parses, and validates
   against its pinned schema.**
   - Missing payloads fail `stale_examples.payload_missing`.
   - Unparseable payloads fail `stale_examples.payload_parse_failed`.
   - Schema-invalid payloads fail
     `stale_examples.example_payload_schema_invalid`.
   - When the optional `jsonschema` package is not installed the
     finding `schema_validator_unavailable` is recorded in
     `diagnostics.schema_validator` so the proof capture stays honest
     about what was checked.
4. **Every vocabulary pin still resolves and still exists in the
   controlling seed row.**
   - A `payload_path` that does not resolve in the payload fails
     `stale_examples.vocabulary_pin_path_unresolved`.
   - A pinned token that is no longer present in the seed row's
     `vocabulary_tokens` list fails
     `stale_examples.vocabulary_pin_not_in_seed`. The finding records
     the stale token, the controlling seed row, and the seed file.

## Failure drill (reproducible)

The protected pack `docs_help_browser_skeleton` carries one named
failure drill, `stale_example_drill.vocabulary_pin_dropped_from_seed`.
Run it locally with:

```
python3 tools/ci/check_stale_examples.py \
    --repo-root . \
    --force-drill docs_help_browser_skeleton:stale_example_drill.vocabulary_pin_dropped_from_seed
```

The runner injects a token that does not exist in
`truth_source.docs_help_source_class.vocabulary_tokens` and the gate
MUST fail with `stale_examples.vocabulary_pin_not_in_seed`. The runner
exits `0` only when that exact check is observed; it exits `2` if the
drill silently passes (which would mean the gate has gone deaf to a
real stale-example pattern).

## Running locally

```
python3 tools/ci/check_stale_examples.py --repo-root .
```

The script writes a durable JSON capture to
`artifacts/milestones/m1/captures/docs_pack_and_example_checks_validation_capture.json`.
Exit codes:

- `0` — every protected pack passes (or `--force-drill` reproduced the
  declared `expected_check_id`).
- `1` — at least one protected pack failed; the human summary names
  the precise check id and remediation.
- `2` — a forced drill did **not** reproduce its declared
  `expected_check_id`.

## Rollback lever

If a stale-example incident needs to be investigated without blocking
unrelated docs publications, the gate can be downgraded to advisory
via the `rollback_lever` block in `ci/docs/stale_example_gate.yml`.
The lever:

- allows advisory downgrade (warnings instead of errors);
- does **not** allow destructive disable;
- requires that the capture history is retained so the incident
  timeline stays intact.

Restore fail-close by reverting the rollback lever entry in the
manifest as part of the same change that addresses the underlying
drift.

## Out of scope

This page does not own:

- a docs publishing platform, screenshot farm, or broad doc-site
  governance system;
- visual / pixel comparison of docs screenshots;
- contract-example schema drift outside the M1 docs/help-pack scope —
  see `docs/ci/schema_example_drift_gate.md` for the wider contract
  example pack.

The M1 gate is intentionally a skeleton: one maintained fixture pack
(the docs/help browser skeleton examples), one CI gate, one named
failure drill, and a stable proof-set entry. Later milestones expand
the protected packs and the controlled vocabularies the gate cross-
checks; the source map and validator are designed so that expansion is
additive.
