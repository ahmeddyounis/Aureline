# Extension manifest, effective-permission, and publisher-identity baseline (first ecosystem-bearing lane)

This page is the reviewer-facing entry point for the M1 extension
manifest baseline: the minimum trustworthy ecosystem model the first
ecosystem-bearing lane uses to make extension scope, effective
permissions, and publisher identity explicit before any meaningful
extension-bearing surface expands.

It is the named runtime consumer for the row matrix in
[`fixtures/extensions/m1_extension_manifest_baseline_rows/m1_rows.yaml`](../../fixtures/extensions/m1_extension_manifest_baseline_rows/m1_rows.yaml)
and the typed boundary every non-owning surface (install / review docs,
support exports, runtime truth badges, CI / schema validation) reads.

## Canonical truth sources

| Concern | File |
| --- | --- |
| Cross-tool boundary schema | [`schemas/extensions/m1_extension_manifest.schema.json`](../../schemas/extensions/m1_extension_manifest.schema.json) |
| Rust source-of-truth crate | [`crates/aureline-extensions/src/manifest_baseline/mod.rs`](../../crates/aureline-extensions/src/manifest_baseline/mod.rs) |
| Seed-row matrix (canonical) | [`fixtures/extensions/m1_extension_manifest_baseline_rows/m1_rows.yaml`](../../fixtures/extensions/m1_extension_manifest_baseline_rows/m1_rows.yaml) |
| Unattended validation lane | [`tests/extensions/m1_extension_manifest_baseline_lane/run_m1_extension_manifest_baseline_lane.py`](../../tests/extensions/m1_extension_manifest_baseline_lane/run_m1_extension_manifest_baseline_lane.py) |
| Proof packet | [`artifacts/milestones/m1/proof_packets/extension_manifest_baseline.md`](../../artifacts/milestones/m1/proof_packets/extension_manifest_baseline.md) |
| Latest validation capture | [`artifacts/milestones/m1/captures/extension_manifest_baseline_validation_capture.json`](../../artifacts/milestones/m1/captures/extension_manifest_baseline_validation_capture.json) |

The Rust crate's types are authoritative; the JSON schema is the
typed cross-tool boundary every non-owning surface reads. They are kept
in lock-step — adding a new enum member is additive-minor with a
schema-version bump, and repurposing an existing member is breaking and
requires a new decision row.

## Three records, one boundary

Every row on the matrix pins three records the install / review surface
reads together:

1. **`extension_manifest_baseline_record`** — publisher identity,
   publisher trust tier, publisher lifecycle state, publisher signing
   key, extension lifecycle state, extension identity, extension
   version, host-contract family, manifest origin / source, declared
   permissions (each with a non-empty `rationale_label`), and the row's
   self-declared `manifest_scope_completeness_class`.
2. **`effective_permission_baseline_record`** — the effective
   permission set after policy-pack narrowings and widening blocks;
   carries a per-scope `declared_vs_effective_diff` plus a non-negative
   `widening_attempted_blocked_count`. Any scope an extension actually
   uses whose `(scope_class, scope_target)` is not in the declared set
   is recorded as `widening_attempted_blocked` and dropped from
   `effective_permissions`.
3. **`manifest_install_decision_record`** — the typed install / review
   verdict. One of `admit`, `admit_with_step_up`, `review_only`, or
   `denied`, paired with a typed
   [`install_decision_reason_class`](../../schemas/extensions/m1_extension_manifest.schema.json).

## Closed vocabularies (authoritative)

The schema's `$defs` blocks freeze these closed vocabularies. The matrix
mirrors them verbatim and the unattended lane fails loudly on drift.

- `publisher_trust_tier_class` — `verified_publisher`,
  `community_publisher`, `organisational_publisher`,
  `unverified_publisher`, `quarantined_publisher`,
  `anonymous_publisher_class`. The `anonymous_publisher_class` token is a
  typed terminal class so a row that fails to attribute a publisher
  cannot pretend to be unverified; it is admitted only on the explicit
  denial drill row paired with `install_decision_class = denied` and
  `install_decision_reason_class = publisher_anonymous`.
- `publisher_lifecycle_state_class` — `active`, `preview`,
  `deprecated`, `retired`, `quarantined`.
- `extension_lifecycle_state_class` — `published`, `preview`,
  `deprecated`, `retired`, `quarantined`.
- `manifest_origin_source_class` — `public_registry`,
  `private_registry`, `mirror`, `offline_bundle`, `vendored_local`,
  `unknown_source_class`. The `unknown_source_class` token mirrors the
  publisher-anonymous denial discipline: it is admitted only on the
  explicit denial drill row paired with `install_decision_class = denied`.
- `host_contract_family_class` — `wasm_component_model`,
  `wasm_core_module`, `external_host_process`, `helper_binary`,
  `remote_side_component`, `compatibility_bridge`.
- `permission_scope_class` — `filesystem_read`, `filesystem_write`,
  `shell_execute`, `network_egress`, `ai_provider_access`,
  `connected_provider_access`, `secret_handle_use`,
  `workspace_settings_read`, `workspace_settings_write`,
  `execution_context_bind`, `subscription_subscribe`,
  `ui_command_contribute`, `capability_inherit`.
- `effective_permission_diff_class` — `unchanged`, `narrowed`,
  `denied`, `step_up_required`, `widening_attempted_blocked`.
- `manifest_scope_completeness_class` — `complete`,
  `incomplete_publisher_missing`, `incomplete_origin_missing`,
  `incomplete_permission_rationale_missing`,
  `incomplete_lifecycle_unknown`. Only `complete` may be paired with
  `install_decision_class = admit`.
- `install_decision_class` — `admit`, `admit_with_step_up`,
  `review_only`, `denied`.
- `install_decision_reason_class` — `admitted_no_violation`,
  `step_up_required_by_policy_pack`,
  `review_only_unverified_publisher`, `publisher_identity_required`,
  `publisher_anonymous`, `publisher_quarantined`,
  `publisher_lifecycle_retired`, `extension_lifecycle_retired`,
  `manifest_scope_incomplete`, `manifest_origin_unknown`,
  `declared_permission_rationale_required`,
  `effective_permission_widening_attempted`,
  `lifecycle_state_unknown_class`.

## Protected walk

To review a row on the first ecosystem-bearing lane:

1. Read the `extension_manifest_baseline_record` block. Confirm
   `publisher_identity_ref` is non-empty, `publisher_display_label` is
   present, `publisher_trust_tier_class` is in the closed vocabulary,
   and every entry in `declared_permissions` carries a non-empty
   `rationale_label`. A row whose `manifest_scope_completeness_class`
   is anything other than `complete` MUST be denied install.
2. Read the `effective_permission_baseline_record` block. Confirm every
   entry in `effective_permissions` has its
   `(scope_class, scope_target)` pair in the manifest's
   `declared_permissions` set. Confirm
   `widening_attempted_blocked_count` equals the number of entries in
   `declared_vs_effective_diff` whose `diff_class` is
   `widening_attempted_blocked`. A non-zero
   `widening_attempted_blocked_count` MUST be paired with
   `install_decision_class = denied` and
   `install_decision_reason_class = effective_permission_widening_attempted`.
3. Read the `manifest_install_decision_record` block. Confirm the
   `install_decision_class` and `install_decision_reason_class` are in
   the closed vocabularies, the `decision_summary` is human-legible,
   and the precedence rules in the
   [Rust `decide_manifest_install`](../../crates/aureline-extensions/src/manifest_baseline/mod.rs)
   contract hold:

   1. anonymous publisher → `denied` (`publisher_anonymous`)
   2. quarantined publisher → `denied` (`publisher_quarantined`)
   3. retired publisher → `denied` (`publisher_lifecycle_retired`)
   4. retired or quarantined extension → `denied` (`extension_lifecycle_retired`)
   5. unknown manifest origin → `denied` (`manifest_origin_unknown`)
   6. effective permission widening attempted → `denied`
      (`effective_permission_widening_attempted`)
   7. manifest scope incomplete → `denied` (the most-specific
      `install_decision_reason_class`)
   8. policy-pack step-up required → `admit_with_step_up`
   9. unverified publisher → `review_only`
   10. otherwise → `admit`.

## Failure drill

Per [`/.plans/M01-104.md`](../../.plans/M01-104.md), the canonical
failure drill is:

> Request extension permissions not granted by manifest scope and
> confirm effective-permission truth exposes the mismatch.

The matrix carries one named drill per row, plus the canonical
widening-blocked drill on the
`extension_manifest:organisational_publisher_policy_narrowed_step_up`
row. The unattended lane reproduces the drill on demand:

```bash
python3 tests/extensions/m1_extension_manifest_baseline_lane/run_m1_extension_manifest_baseline_lane.py \
    --repo-root . \
    --force-drill extension_manifest:organisational_publisher_policy_narrowed_step_up:org_publisher_drill.requested_permission_outside_declared_scope
```

The runner exits `0` only when the row's declared `expected_check_id`
(here, `manifest_baseline.effective_permission_widening_attempted`) is
reproduced by the forced input, so it fails loudly on real regressions.

## Closure rule

The row stays open until the latest validation capture lands under the
governed proof root and every row reports PASS for:

- closed-vocabulary membership (every enum mirrored from the schema's
  `$defs`),
- manifest-baseline structural invariants
  (`publisher_identity_required`,
  `declared_permission_rationale_required`,
  `extension_identity_unnamespaced`,
  `manifest_scope_completeness_class`),
- effective-permission baseline rules
  (`effective_permission_widening_attempted`,
  `widening_attempted_blocked_count_mismatch`,
  `effective_scope_not_in_declared_set`),
- install-decision precedence rules
  (`anonymous_publisher_install_must_be_denied`,
  `quarantined_publisher_install_must_be_denied`,
  `unknown_origin_install_must_be_denied`,
  `incomplete_manifest_install_must_be_denied`,
  `unverified_publisher_install_must_be_review_only`,
  `step_up_required_install_must_be_admit_with_step_up`),
- and the row's named failure drill.

The six required publisher trust tier classes plus the four required
install decision classes are all observed.
