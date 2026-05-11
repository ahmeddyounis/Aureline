# M1 extension manifest-baseline validation lane

Unattended proof lane that validates
[`fixtures/extensions/m1_extension_manifest_baseline_rows/m1_rows.yaml`](../../../fixtures/extensions/m1_extension_manifest_baseline_rows/m1_rows.yaml)
against
[`schemas/extensions/m1_extension_manifest.schema.json`](../../../schemas/extensions/m1_extension_manifest.schema.json)
and the
[`crates/aureline-extensions/src/manifest_baseline/mod.rs`](../../../crates/aureline-extensions/src/manifest_baseline/mod.rs)
contract.

The lane is deliberately runnable on CI / nightly without a graphical
display: it only consumes the pure data joined out of the canonical
sources.

## What the lane proves

For every row in the matrix the runner asserts:

- **`row_id` is namespaced** under `extension_manifest:` and is unique.
- **Discriminators are honored** — the manifest baseline carries
  `extension_manifest_baseline_record`, the effective-permission
  baseline carries `effective_permission_baseline_record`, and the
  install decision carries `manifest_install_decision_record`.
- **Closed vocabularies match the schema verbatim** —
  `publisher_trust_tier_class`,
  `publisher_lifecycle_state_class`,
  `extension_lifecycle_state_class`,
  `manifest_origin_source_class`, `host_contract_family_class`,
  `permission_scope_class`, `effective_permission_diff_class`,
  `manifest_scope_completeness_class`, `install_decision_class`, and
  `install_decision_reason_class` all match the manifest schema's
  `$defs`.
- **Manifest-baseline structural invariants hold** —
  `publisher_identity_required`,
  `publisher_display_label_required`,
  `publisher_signing_key_required`,
  `origin_source_label_required`,
  `extension_identity_unnamespaced`,
  `manifest_baseline_id_unprefixed`,
  `declared_permission_rationale_required`, and
  `declared_permission_scope_target_required`.
- **Effective-permission baseline rules hold** — every
  `effective_permissions` entry's `(scope_class, scope_target)` pair is
  in the manifest's declared set
  (`effective_scope_not_in_declared_set`); and
  `widening_attempted_blocked_count` equals the number of
  `declared_vs_effective_diff` entries with diff_class
  `widening_attempted_blocked`
  (`widening_attempted_blocked_count_mismatch`).
- **Install-decision precedence rules hold** —
  `anonymous_publisher_install_must_be_denied`,
  `quarantined_publisher_install_must_be_denied`,
  `retired_publisher_install_must_be_denied`,
  `retired_extension_install_must_be_denied`,
  `unknown_origin_install_must_be_denied`,
  `effective_permission_widening_attempted`,
  `incomplete_manifest_install_must_be_denied`,
  `step_up_required_install_must_be_admit_with_step_up`,
  `unverified_publisher_install_must_be_review_only`,
  `admittable_manifest_install_must_be_admit`.
- **Required coverage is met** — at least one row exists for each
  member of `required_publisher_trust_tier_coverage` (every
  `publisher_trust_tier_class`) and
  `required_install_decision_coverage` (every
  `install_decision_class`).
- **Failure drills are reproducible** — every row names one drill in
  `failure_drill_id_vocabulary` plus the precise `expected_check_id`
  the runner reproduces when the drill is forced with `--force-drill`.

## Run

```bash
python3 tests/extensions/m1_extension_manifest_baseline_lane/run_m1_extension_manifest_baseline_lane.py --repo-root .
```

The runner emits a deterministic JSON capture to
`artifacts/milestones/m1/captures/extension_manifest_baseline_validation_capture.json`
and exits non-zero on any check failure.

### Force a named failure drill

```bash
python3 tests/extensions/m1_extension_manifest_baseline_lane/run_m1_extension_manifest_baseline_lane.py \
    --repo-root . \
    --force-drill <row_id>:<drill_id>
```

In `--force-drill` mode the runner exits `0` only when the row's
declared `expected_check_id` is reproduced by the forced input.

| Row | Drill | Expected check id |
|---|---|---|
| `extension_manifest:verified_publisher_complete_manifest_admit`               | `verified_publisher_drill.publisher_identity_stripped`               | `manifest_baseline.publisher_identity_required` |
| `extension_manifest:community_publisher_complete_manifest_admit`              | `community_publisher_drill.declared_permission_rationale_stripped`   | `manifest_baseline.declared_permission_rationale_required` |
| `extension_manifest:organisational_publisher_policy_narrowed_step_up`         | `org_publisher_drill.requested_permission_outside_declared_scope`    | `manifest_baseline.effective_permission_widening_attempted` |
| `extension_manifest:unverified_publisher_review_only`                         | `unverified_publisher_drill.install_decision_class_widened_to_admit` | `manifest_baseline.unverified_publisher_install_must_be_review_only` |
| `extension_manifest:quarantined_publisher_install_denied`                     | `quarantined_publisher_drill.install_decision_class_widened_to_admit`| `manifest_baseline.quarantined_publisher_install_must_be_denied` |
| `extension_manifest:anonymous_publisher_install_denied`                       | `anonymous_publisher_drill.install_decision_class_widened_to_admit`  | `manifest_baseline.anonymous_publisher_install_must_be_denied` |

The third row reproduces the canonical failure drill named in
[`/.plans/M01-104.md`](../../../.plans/M01-104.md):

> Request extension permissions not granted by manifest scope and
> confirm effective-permission truth exposes the mismatch.

Optional flags:

- `--matrix <path>` — point at an alternate matrix file.
- `--schema <path>` — alternate manifest schema.
- `--report <path>` — change the capture output path.
- `--build-identity <path>` — change which build identity record is
  embedded in the capture.

## Where the evidence lives

| Artifact | Path |
| --- | --- |
| Reviewer landing page | `docs/extensions/m1_permission_and_publisher_baseline.md` |
| Seed-row matrix | `fixtures/extensions/m1_extension_manifest_baseline_rows/m1_rows.yaml` |
| Manifest schema | `schemas/extensions/m1_extension_manifest.schema.json` |
| Rust source-of-truth crate | `crates/aureline-extensions/src/manifest_baseline/mod.rs` |
| Latest capture | `artifacts/milestones/m1/captures/extension_manifest_baseline_validation_capture.json` |
| Owning proof packet | `artifacts/milestones/m1/proof_packets/extension_manifest_baseline.md` |

The lane is registered in
[`artifacts/milestones/m1/artifact_index.yaml`](../../../artifacts/milestones/m1/artifact_index.yaml)
under `proof_lanes.extension_manifest_baseline` so reviewers can find
the latest capture, owner, and validation-lane reference without
searching ad hoc folders.

## Refresh policy

Re-run the lane (and refresh the capture) when any of the following
change:

- `fixtures/extensions/m1_extension_manifest_baseline_rows/m1_rows.yaml`
- `schemas/extensions/m1_extension_manifest.schema.json`
- `crates/aureline-extensions/src/manifest_baseline/mod.rs`
- `docs/extensions/m1_permission_and_publisher_baseline.md`
- the broader ADR-0012 effective-permission vocabulary it composes with.
