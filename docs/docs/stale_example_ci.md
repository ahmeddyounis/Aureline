# Stale-example scanner, snippet freshness ledger, and CI gate

This document describes how Aureline keeps docs, guided content, and
migration guidance from silently drifting away from reviewed product
truth.

The policy goal is simple: if an example/snippet is stale or cannot be
traced to source/version/compatibility anchors, promotion fails unless
there is an explicit, time-boxed exception.

## Components

- **Scanner tool**: `tools/docs/stale_example_scan/scan_docs_freshness.py`
- **Scan config**: `ci/check_docs_freshness.yml`
- **Freshness ledger**: `artifacts/docs/snippet_freshness_ledger.yaml`
- **CI entry point**: `ci/check_docs_freshness.sh`
- **GitHub Actions gate**: `.github/workflows/check_docs_freshness.yml`

## What the scanner enforces

For each observed snippet/example (scoped by the config):

1. **Stable snippet id exists** (example id, waypoint id, handoff id).
2. **Source/version anchors are present**, appropriate to the surface:
   - docs-pack examples: pack id + pack revision + semver + target build id;
   - guided steps: docs pack revision + version match state at mint;
   - provider/browser handoffs: pack revision + target build id + version match state;
   - migration examples: compatibility report ref + compatibility row refs.
3. **Ledger coverage exists**: tracked snippet kinds must have a ledger
   entry (`untracked_snippet` fails the gate).
4. **Drift is detected mechanically** by comparing observations to the
   ledger’s expected anchors.

The scanner’s failure classes are the contract-level buckets the gate
emits (version mismatch, missing anchor, drifted command id, etc.).

## Posture classes

The ledger assigns every tracked snippet one posture:

- `verified`: drift fails the gate unless explicitly excepted.
- `illustrative`: non-executable/illustrative guidance is allowed, but
  still must carry stable ids and traceable anchors.
- `retest_pending`: blocks promotion unless an exception window allows
  it; the exception makes the gap visible and time-bounded.

## Exception windows

Ledger entries may carry `exception_windows` to allow a specific failure
class until a deadline. Exceptions are scoped, explicit, and time-boxed:

- exception applies only to named `allowed_failure_classes`;
- exception expires at `until` (RFC 3339 timestamp).

Expired exceptions behave like no exception: CI fails.

## Running locally

```bash
./ci/check_docs_freshness.sh
```

To write the JSON report elsewhere:

```bash
./ci/check_docs_freshness.sh --out-dir target/docs-freshness
```

## Adding a new tracked snippet/example

1. Ensure the surface carries a stable id field (example id / step id /
   handoff id).
2. Ensure the surface carries required source/version/compat anchors.
3. Add a ledger entry in `artifacts/docs/snippet_freshness_ledger.yaml`
   with the expected anchors.
4. Run `./ci/check_docs_freshness.sh` and keep the gate green.

## Fixture cases (examples of pass/fail detection)

The repository also carries worked pass/fail cases under
`fixtures/docs/stale_example_scan_cases/`. They are not part of the CI
gate’s default scan scope, but they can be used to demonstrate how each
failure class is detected.

