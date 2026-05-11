# Proof packet: extension manifest, effective-permission, and publisher-identity baseline (first ecosystem-bearing lane)

Purpose: anchor proof captures for the unattended M1 lane that
validates the extension-manifest baseline shape, the
effective-permission baseline summary, and the install / review decision
record on the first ecosystem-bearing lane against the canonical
schema and the Rust source-of-truth crate, and proves the seed is
consumable by a named docs / review reviewer surface and a Rust
validator without re-encoding the publisher-identity, permission-scope,
or install-decision vocabularies.

Reviewer entry point:
[`/docs/extensions/m1_permission_and_publisher_baseline.md`](../../../docs/extensions/m1_permission_and_publisher_baseline.md).

## Canonical sources

- `fixtures/extensions/m1_extension_manifest_baseline_rows/m1_rows.yaml`
  — seed-row matrix the runner consumes; one row per protected
  install / review path with a typed
  `publisher_trust_tier_class`, `install_decision_class`,
  `manifest_scope_completeness_class`, and a named failure drill.
- `schemas/extensions/m1_extension_manifest.schema.json` — the
  extension-manifest baseline, effective-permission baseline, and
  install / review decision vocabulary. Freezes closed enums for
  `publisher_trust_tier_class`,
  `publisher_lifecycle_state_class`,
  `extension_lifecycle_state_class`,
  `manifest_origin_source_class`, `host_contract_family_class`,
  `permission_scope_class`, `effective_permission_diff_class`,
  `manifest_scope_completeness_class`, `install_decision_class`, and
  `install_decision_reason_class`.
- `crates/aureline-extensions/src/manifest_baseline/mod.rs` — the
  Rust source-of-truth types, the `validate_manifest_baseline_record`
  validator, the `compute_effective_permission_baseline` projection,
  and the `decide_manifest_install` decision-class precedence rules.
  The schema is kept in lock-step with this crate; the crate is
  authoritative.
- `schemas/extensions/effective_permission.schema.json` — the broader
  ADR-0012 effective-permission seed the M1 lane composes with. The M1
  lane does not redefine its publisher-continuity, policy-pack, or
  capability-lifecycle vocabularies; it pins the M1-bearing subset.
- `tests/extensions/m1_extension_manifest_baseline_lane/run_m1_extension_manifest_baseline_lane.py`
  — unattended runner that replays every row, asserts schema
  membership, manifest-baseline structural invariants,
  effective-permission baseline rules, and install-decision precedence
  rules, and emits the durable JSON capture.

## Named runtime consumer

- `docs/extensions/m1_permission_and_publisher_baseline.md` —
  reviewer-facing landing page. Wired as the M1 named consumer through
  `consumer_bindings.named_runtime_consumer` on the matrix; consumed
  fields include
  `extension_manifest_baseline_record.publisher_identity_ref`,
  `extension_manifest_baseline_record.publisher_display_label`,
  `extension_manifest_baseline_record.publisher_trust_tier_class`,
  `extension_manifest_baseline_record.publisher_lifecycle_state_class`,
  `extension_manifest_baseline_record.extension_lifecycle_state_class`,
  `extension_manifest_baseline_record.manifest_origin_source_class`,
  `extension_manifest_baseline_record.declared_permissions`,
  `extension_manifest_baseline_record.manifest_scope_completeness_class`,
  `effective_permission_baseline_record.effective_permissions`,
  `effective_permission_baseline_record.declared_vs_effective_diff`,
  `effective_permission_baseline_record.widening_attempted_blocked_count`,
  `manifest_install_decision_record.install_decision_class`,
  `manifest_install_decision_record.install_decision_reason_class`,
  and `manifest_install_decision_record.decision_summary`.

## Live runtime consumers (read-only)

- `crates/aureline-extensions/src/manifest_baseline/mod.rs` — Rust
  validator the install / review surface, support exports, and CI
  runners use. Wired as the M1 second consumer through
  `consumer_bindings.rust_validator_consumer` on the matrix; consumed
  fields include
  `extension_manifest_baseline_record.publisher_identity_ref`,
  `extension_manifest_baseline_record.declared_permissions.rationale_label`,
  `effective_permission_baseline_record.widening_attempted_blocked_count`,
  `manifest_install_decision_record.install_decision_class`, and
  `manifest_install_decision_record.install_decision_reason_class`.
  Unit-test coverage on the same precedence rules lives at
  `crates/aureline-extensions/src/manifest_baseline/tests.rs`
  (run via `cargo test -p aureline-extensions`).
- `artifacts/build/build_identity.json` — exact-build identity that the
  capture embeds for cross-artifact traceability.

## Validation captures

- `artifacts/milestones/m1/captures/extension_manifest_baseline_validation_capture.json`

## Refresh policy

Re-run the validation lane after a change to:

- the seed-row matrix,
- the manifest schema,
- the Rust source-of-truth crate,
- the reviewer-facing landing page,
- the broader ADR-0012 effective-permission vocabulary the M1 lane
  composes with.

## Closure rule

The lane stays open until the latest capture lands under the governed
proof root and every row reports PASS for:

- closed-vocabulary membership
  (`publisher_trust_tier_class`,
  `publisher_lifecycle_state_class`,
  `extension_lifecycle_state_class`,
  `manifest_origin_source_class`, `host_contract_family_class`,
  `permission_scope_class`, `effective_permission_diff_class`,
  `manifest_scope_completeness_class`, `install_decision_class`,
  `install_decision_reason_class`),
- manifest-baseline structural invariants
  (`publisher_identity_required`,
  `publisher_display_label_required`,
  `publisher_signing_key_required`,
  `origin_source_label_required`,
  `extension_identity_unnamespaced`,
  `manifest_baseline_id_unprefixed`,
  `declared_permission_rationale_required`,
  `declared_permission_scope_target_required`),
- effective-permission baseline rules
  (`effective_scope_not_in_declared_set`,
  `widening_attempted_blocked_count_mismatch`),
- install-decision precedence rules
  (`anonymous_publisher_install_must_be_denied`,
  `quarantined_publisher_install_must_be_denied`,
  `retired_publisher_install_must_be_denied`,
  `retired_extension_install_must_be_denied`,
  `unknown_origin_install_must_be_denied`,
  `effective_permission_widening_attempted`,
  `incomplete_manifest_install_must_be_denied`,
  `step_up_required_install_must_be_admit_with_step_up`,
  `unverified_publisher_install_must_be_review_only`,
  `admittable_manifest_install_must_be_admit`),
- and the row's named failure drill —

and the six required publisher trust tier classes plus the four
required install decision classes are all observed.
