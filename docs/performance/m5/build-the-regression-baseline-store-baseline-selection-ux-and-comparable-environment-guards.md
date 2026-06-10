# Build the Regression Baseline Store, Baseline Selection UX, and Comparable-Environment Guards

This document is the reviewer-facing landing page for the M5 regression-baseline
store, baseline-selection UX, and comparable-environment guards lane.

## Scope

This lane governs how profiler and trace surfaces:

- store baseline profiles and traces with explicit build identity, environment
  fingerprint, capture mode, storage location, and freshness so users always
  know what they are comparing against;
- present baseline selection UX that shows comparison basis, environment match
  state, and honest mismatch or stale warnings before executing any comparison;
- enforce comparable-environment guards that check build identity, architecture,
  OS version, runtime version, capture mode compatibility, mapping quality
  compatibility, and freshness policy so comparison claims narrow automatically
  when environments are not truly comparable.

## Canonical Artifacts

- **Implementation:** `crates/aureline-profiler/src/build_the_regression_baseline_store_baseline_selection_ux_and_comparable_environment_guards/`
- **Packet:** `artifacts/perf/m5/build-the-regression-baseline-store-baseline-selection-ux-and-comparable-environment-guards.json`
- **Schema:** `schemas/perf/build-the-regression-baseline-store-baseline-selection-ux-and-comparable-environment-guards.schema.json`
- **Fixtures:** `fixtures/performance/m5/build-the-regression-baseline-store-baseline-selection-ux-and-comparable-environment-guards/`

## Surfaces

| Surface | Claim | Rationale |
|---|---|---|
| Baseline store | Stable | Shows baseline identity, build identity, environment fingerprint, capture mode, storage location, freshness state, comparison basis, environment match, mismatch warnings, guard criteria, and degraded-state labels. |
| Baseline selection UX | Stable | Shows comparison basis, environment match state, and honest mismatch or stale warnings before any comparison is executed. |
| Comparison report | Stable | Leads with baseline identity, environment comparability, and guard criteria before showing delta metrics. |
| Environment guard inspector | Stable | Shows each comparability criterion, whether it passed or failed, and narrows the comparison claim automatically on mismatch. |
| Export review | Preview | Redaction-safe export flows for regression baseline evidence are still under qualification. |
| Support export | Preview | Support-bundle redaction for regression baseline payloads is still under qualification. |

## Baseline-Store Rows

Baseline-store rows carry:

- `baseline_id` — stable identifier;
- `exact_build_identity_ref` — build identity at capture time;
- `environment_fingerprint_ref` — normalized environment used for comparability;
- `capture_mode_ref` — capture mode descriptor ref;
- `storage_location_ref` — where the baseline is stored;
- `freshness` — `current`, `stale`, `expired`, `missing`, `imported`, or `unverified`.

Every baseline-store row MUST show its freshness state and build identity.

## Baseline-Selection UX Rows

Selection UX rows carry:

- `selection_kind` — `picker`, `list`, `recent`, or `pinned`;
- `comparison_basis_label` — human-readable basis shown to the user;
- `baseline_ref` — selected baseline;
- `current_environment_fingerprint_ref` — current environment for comparison;
- `environment_match_state` — `comparable`, `partial`, `mismatch`, `unknown`, or `stale`.

When the environment match state is `partial`, `mismatch`, or `stale`, the
selection UX MUST show a mismatch or stale warning.

## Comparable-Environment Guard Rows

Guard rows define the criteria used to decide whether two environments are
comparable:

- `exact_build_identity_required` — exact build must match;
- `architecture_required` — architecture must match;
- `os_version_required` — OS and version must match;
- `runtime_version_required` — runtime version must match;
- `capture_mode_compatible_required` — capture modes must be compatible;
- `mapping_quality_compatible_required` — mapping quality must be compatible;
- `freshness_policy` — time window for baseline freshness.

Every guard row MUST show its criteria breakdown and MUST narrow the comparison
claim on mismatch.

## Downgrade and Rollback

- Any surface that claims `stable` with an incomplete guard set is narrowed
  automatically by the validator.
- Baseline-store rows MUST show freshness state and build identity; missing
  truth labels trigger a validation violation.
- Baseline-selection UX rows MUST show a warning when the environment match
  state warns; missing warnings trigger a validation violation.
- Comparable-environment guard rows MUST show criteria breakdown and narrow on
  mismatch; missing behavior triggers a validation violation.
- Cross-reference failures (unknown baseline, fingerprint, or guard refs)
  trigger validation violations.

## Invariants

- Raw payload bytes, raw command lines, secrets, and ambient credentials do not
  cross this boundary.
- Every baseline points to exactly one environment fingerprint.
- Every selection UX points to exactly one baseline and one current environment
  fingerprint.
- Comparison claims narrow automatically when guard criteria fail.
