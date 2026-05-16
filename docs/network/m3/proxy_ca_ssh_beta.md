# Enterprise proxy, CA, SSH, and client-certificate beta

The beta enterprise network-trust projection turns the effective proxy,
trust-store, SSH host-proof, and client-certificate state into a single
inspectable record consumed by the admin/settings center, support export,
shell network summary, headless inspector, and reviewer fixtures. It covers
the connected, mirror-only, offline, and enterprise-managed beta profiles
without allowing each surface to invent its own `is_proxy_set` or `has_org_ca`
check or silently fall back to a public endpoint.

## Contract

The shared contract ref is `network:network_trust_beta:v1`.

The auth-owned source of truth is
[`crates/aureline-auth/src/network_trust/mod.rs`](../../../crates/aureline-auth/src/network_trust/mod.rs).
The shell consumer and headless inspector live at
[`crates/aureline-shell/src/network_trust_beta/mod.rs`](../../../crates/aureline-shell/src/network_trust_beta/mod.rs)
and
[`crates/aureline-shell/src/bin/aureline_shell_network_trust_beta.rs`](../../../crates/aureline-shell/src/bin/aureline_shell_network_trust_beta.rs).

The page exports:

- `network_network_trust_beta_row_record` for each facet (proxy, trust store,
  SSH host proof, client certificate).
- `network_network_trust_beta_profile_binding_record` for the per-profile
  effective value, source, lock reason, and authority.
- `network_network_trust_beta_support_row_record` for the export-safe support
  projection of each live row.
- `network_network_trust_beta_defect_record` for validator findings.
- `network_network_trust_beta_page_record` and
  `network_network_trust_beta_support_export_record` for the complete page and
  its export wrapper.

## Required behavior

Users and admins can inspect active proxy, CA, client-cert, and SSH settings
with source and lock reasons:

- Each profile binding carries `source_class`, `source_token`, `lock_class`,
  `lock_token`, `authority`, `authority_token`, a reviewable
  `effective_value_label`, a `source_label`, a `lock_reason`, and — when the
  source is managed — a `managed_attribution_ref` pointing at the signature
  or attribution artifact.
- The validator rejects bindings whose `source_token` does not match
  `source_class`, whose `lock_token` does not match `lock_class`, whose
  managed lock is not backed by a managed source, or whose published authority
  carries no reviewable label.

Enterprise network settings are reused consistently across runtime,
extension, AI, provider, and update lanes:

- Every row declares all of `runtime`, `extension`, `ai`, `provider`, and
  `update` in `consumer_lanes` / `consumer_lane_tokens`. The validator emits
  `missing_consumer_lane_coverage` when any lane is missing, preventing a
  silent split where one surface reads a different effective proxy or CA
  bundle than another.

Support exports can explain network posture without dumping raw secrets or
private keys:

- The support-row projection mirrors source tokens, lock tokens, authority
  tokens, effective-value labels, and the consumer-lane fan-out without
  carrying raw private-key material; both the row and the support-export
  wrapper expose `raw_secret_or_private_material_excluded = true`.
- The validator rejects rows or bindings that permit undeclared public
  endpoint fallback (`hidden_public_endpoint_fallback`) or that mark
  `raw_secret_or_private_material_excluded = false`.

Connected, mirror-only, offline, and enterprise-managed profiles are present
on every row. Bindings that fail closed because the required input is
unavailable (for example, mTLS enrollment on the offline profile) report
`blocked_missing_input` instead of silently widening authority.

## Validation

The validator emits typed `NetworkTrustBetaDefectKind` records:

| Defect | When it appears |
| --- | --- |
| `missing_facet_coverage` | A required facet (proxy, trust_store, ssh_host_proof, client_certificate) is missing. |
| `missing_profile_coverage` | A row is missing the connected, mirror-only, offline, or enterprise-managed binding. |
| `facet_token_drift` | A row's `facet_token` does not match `facet`. |
| `source_token_drift` | A binding's `source_token` does not match `source_class`. |
| `lock_token_drift` | A binding's `lock_token` does not match `lock_class`, or `authority_token` does not match `authority`. |
| `lock_inconsistent_with_source` | A binding declares a managed lock without a managed source. |
| `unsigned_managed_authority` | A managed-source binding presents no signature or attribution pointer. |
| `hidden_public_endpoint_fallback` | A row or binding permits undeclared public endpoint fallback. |
| `raw_secret_or_private_material_exposed` | A row marks `raw_secret_or_private_material_excluded = false`. |
| `missing_consumer_lane_coverage` | A row does not declare every required consumer lane. |
| `support_row_vocabulary_drift` | The support/export row drifted from the live row vocabulary. |
| `empty_effective_value_label` | A `published` authority binding has no reviewable label. |

The seeded page has zero defects. The failure drills under
[`fixtures/network/m3/network_trust/`](../../../fixtures/network/m3/network_trust/)
prove the validator catches an unsigned managed binding, a silent public
fallback, and a lock/source mismatch.

## Reproduce

```sh
cargo run -q -p aureline-shell --bin aureline_shell_network_trust_beta -- validate
cargo test -p aureline-auth --lib network_trust
cargo test -p aureline-shell --lib network_trust_beta
cargo test -p aureline-shell --test network_trust_beta_fixtures
```
