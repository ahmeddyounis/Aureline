# Policy-pack beta

The beta policy-pack projection turns the effective policy view into a single
inspectable record consumed by the admin/settings center, support export,
shell summary, headless inspector, and reviewer fixtures. It covers the
connected, mirror-only, offline, and enterprise-managed beta profiles without
allowing each surface to invent its own `is_policy_signed` check or silently
fall back to a public endpoint.

## Contract

The shared contract ref is `security:policy_pack_beta:v1`.

The auth-owned source of truth is
[`crates/aureline-auth/src/policy_packs/mod.rs`](../../../crates/aureline-auth/src/policy_packs/mod.rs).
The shell consumer and headless inspector live at
[`crates/aureline-shell/src/policy_pack_beta/mod.rs`](../../../crates/aureline-shell/src/policy_pack_beta/mod.rs)
and
[`crates/aureline-shell/src/bin/aureline_shell_policy_pack_beta.rs`](../../../crates/aureline-shell/src/bin/aureline_shell_policy_pack_beta.rs).

The page exports:

- `security_policy_pack_beta_pack_record` for inspected packs (effective,
  staged, replaced, revoked, or never applied).
- `security_policy_pack_beta_rule_record` for per-surface effective rules.
- `security_policy_pack_beta_diff_record` and
  `security_policy_pack_beta_diff_entry_record` for before/after pack diffs
  per profile.
- `security_policy_pack_beta_denial_trace_record` for explain traces tying a
  product-row denial back to the originating pack and rule.
- `security_policy_pack_beta_import_receipt_record` for mirror, manual signed
  file, and air-gapped signed-transfer receipts that preserve signatures,
  provenance, and rule explanations verbatim.
- `security_policy_pack_beta_defect_record` for validator findings.
- `security_policy_pack_beta_page_record` and
  `security_policy_pack_beta_support_export_record` for the complete page and
  its export wrapper.

## Required behavior

Admins and support can inspect effective policy, source, diff, and signature
state before or after application:

- Every pack record carries its `source_class`, `signature_state`,
  `apply_state`, signer, signed-at and fetched-at timestamps, signature blob
  ref, and the rules that contribute to a managed authority narrowing.
- A `diff` record exists for every profile transition between the recorded
  baseline pack and the effective target, with one entry per added, removed,
  effect-changed, scope-changed, or reason-changed rule.

Mirror and manual-import policy packs preserve signatures, provenance, and
explanation fields:

- Mirror, manual signed-file import, and air-gapped signed-transfer receipts
  must keep `preserves_signature_blob`, `preserves_provenance`, and
  `preserves_explanation` all set, and must copy `signature_blob_ref`,
  `signer_id`, `signed_at`, and rule explanations from the upstream pack
  verbatim. The validator emits
  `mirror_or_import_signature_blob_dropped`,
  `mirror_or_import_provenance_dropped`, or
  `mirror_or_import_explanation_dropped` when any of these promises drift.

Policy denial reasons in product surfaces can be traced back to the same pack
and rule identifiers:

- Denial traces carry `pack_id`, `rule_id`, `surface_family_token`,
  `reason_token`, and the rule's explanation string. The validator rejects
  traces that cannot resolve to a pack/rule pair, point at non-denial effects,
  or reference a surface family the rule does not cover.

Connected, mirror-only, offline, and enterprise-managed profiles are present
across the page. Every pack record refuses undeclared public endpoint fallback
and excludes raw private/secret material from the record itself.

## Validation

The validator emits typed `PolicyPackBetaDefectKind` records:

| Defect | When it appears |
| --- | --- |
| `unsigned_managed_authority` | A managed-authority pack lacks a verified signature. |
| `source_token_drift` | Source token does not match source class. |
| `signature_token_drift` | Signature-state token does not match signature state. |
| `mirror_or_import_provenance_dropped` | Mirror or manual-import receipt drops upstream provenance. |
| `mirror_or_import_signature_blob_dropped` | Mirror or manual-import receipt drops signature blob ref. |
| `mirror_or_import_explanation_dropped` | Mirror or manual-import receipt drops the rule explanation. |
| `hidden_public_endpoint_fallback` | A pack permits undeclared public endpoint fallback. |
| `profile_coverage_missing` | Required connected, mirror-only, offline, or enterprise-managed coverage is missing. |
| `diff_entry_effect_mismatch` | A diff entry's before/after tokens do not match the referenced packs. |
| `denial_trace_unresolvable` | A denial trace cannot resolve to a pack/rule pair. |
| `denial_trace_effect_not_denial` | A denial trace points at a rule whose effect does not surface denial. |
| `denial_trace_surface_missing` | A denial trace surface does not match its rule. |
| `raw_private_material_exposed` | A record would expose raw private or secret material. |

The seeded page has zero defects. The failure drills under
[`fixtures/security/m3/policy_packs/`](../../../fixtures/security/m3/policy_packs/)
prove the validator catches a dropped signature blob, an unresolvable denial
trace, and a hidden public-endpoint fallback.

## Reproduce

```sh
cargo run -q -p aureline-shell --bin aureline_shell_policy_pack_beta -- validate
cargo test -p aureline-auth --lib policy_packs
cargo test -p aureline-shell --lib policy_pack_beta
cargo test -p aureline-shell --test policy_pack_beta_fixtures
```
