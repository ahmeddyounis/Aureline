# Offline policy-bundle and entitlement verifier beta

The beta offline-entitlement verifier turns the signed policy-bundle and
entitlement-snapshot contract into a single inspectable, locally-runnable
verifier that produces one record per bundle, per profile, per kind. It
covers the connected, mirror-only, offline, and enterprise-managed beta
profiles for both `policy_bundle` and `entitlement_snapshot` kinds, so
admin, support, shell, and headless surfaces share one verification truth
and never silently fall back to a public endpoint or accept an
unverifiable bundle.

## Contract

The shared contract ref is `security:offline_entitlement_verifier_beta:v1`.

The auth-owned source of truth is
[`crates/aureline-auth/src/offline_entitlements/mod.rs`](../../../crates/aureline-auth/src/offline_entitlements/mod.rs).
The shell consumer and headless inspector live at
[`crates/aureline-shell/src/offline_entitlement_beta/mod.rs`](../../../crates/aureline-shell/src/offline_entitlement_beta/mod.rs)
and
[`crates/aureline-shell/src/bin/aureline_shell_offline_entitlement_beta.rs`](../../../crates/aureline-shell/src/bin/aureline_shell_offline_entitlement_beta.rs).

The page exports:

- `security_offline_entitlement_verifier_beta_row_record` for one verifier
  row covering one signed bundle on one profile, with subject (bundle
  ref, kind, version, epoch, signer, signed-at, valid-until), resolved
  trust anchor, typed verifier outcome, managed-capability impact,
  local-editing preservation, and recovery action.
- `security_offline_entitlement_verifier_beta_defect_record` for validator
  findings.
- `security_offline_entitlement_verifier_beta_summary_record` for the
  page-level summary (row counts, profiles, kinds, outcomes, defect
  counts).
- `security_offline_entitlement_verifier_beta_page_record` for the page
  itself and
  `security_offline_entitlement_verifier_beta_support_export_record` for
  the support-export wrapper.

## Required behavior

Offline entitlement and policy verification succeeds without live vendor
calls on every row that claims it:

- Connected, mirror, offline, and enterprise-managed rows for both kinds
  resolve to `verified_live`, `verified_mirror`, `verified_air_gapped`,
  or `verified_manual_import` against their declared trust anchor.
- The `unsigned_local_advisory` outcome is admissible only with a
  `local_advisory_no_root` trust anchor; pairing it with a managed
  anchor triggers `unsigned_local_advisory_on_managed_anchor`.

Expired, missing, or unverifiable bundles downgrade managed capability
authority without blocking local editing:

- `expired`, `signature_missing`, `signature_invalid`, `untrusted_signer`,
  `bundle_not_present`, and `revoked` outcomes MUST resolve to a
  narrowed, paused, or blocked managed impact (never
  `full_authority_active`).
- The same outcomes MUST resolve to `preserved` or
  `preserved_with_advisory` local-editing preservation; the validator
  rejects rows that block local editing on a failed verify.
- Downgrade rows MUST declare a recovery action other than
  `no_action_verified` so a product surface or admin runbook can offer
  the next step.

Verifier results are inspectable in product surfaces and export packets:

- The shell consumer renders a compact summary; the headless inspector
  emits the same records as JSON.
- The support-export wrapper carries per-row support projections plus
  metadata-safe defect roll-ups; the validator never permits raw
  private/secret material on the row or the export.

## Validation

The validator emits typed
`OfflineEntitlementVerifierBetaDefectKind` records:

| Defect | When it appears |
| --- | --- |
| `untrusted_signer_accepted` | An untrusted-signer outcome resolves to `full_authority_active`. |
| `expired_bundle_accepted_without_downgrade` | An expired outcome resolves to `full_authority_active`. |
| `verified_outcome_without_trust_anchor` | A verified outcome resolves to the `local_advisory_no_root` anchor. |
| `outcome_impact_mismatch` | An outcome and impact pairing is not allowed by the contract. |
| `outcome_token_drift` / `profile_token_drift` / `bundle_kind_token_drift` / `trust_anchor_token_drift` / `impact_token_drift` / `local_editing_token_drift` / `recovery_action_token_drift` | A stable token does not match its enum value. |
| `local_editing_blocked_on_failed_verification` | A failed-verify row blocks local editing. |
| `hidden_public_endpoint_fallback` | A row permits undeclared public endpoint fallback. |
| `raw_private_material_exposed` | A row would expose raw private or secret material. |
| `profile_coverage_missing` / `bundle_kind_coverage_missing` | A required profile or bundle kind is missing from the page. |
| `unsigned_local_advisory_on_managed_anchor` | An unsigned local advisory outcome pairs with a managed trust anchor. |
| `downgrade_missing_recovery_action` | A failed-verify row offers `no_action_verified`. |

The seeded page has zero defects. The failure drills under
[`fixtures/security/m3/offline_entitlement_verifier/`](../../../fixtures/security/m3/offline_entitlement_verifier/)
prove the validator catches an expired bundle accepted with full
authority, an untrusted signer accepted with full authority, and a
failed-verify row that blocks local editing.

## Reproduce

```sh
cargo run -q -p aureline-shell --bin aureline_shell_offline_entitlement_beta -- validate
cargo test -p aureline-auth --lib offline_entitlements
cargo test -p aureline-shell --lib offline_entitlement_beta
cargo test -p aureline-shell --test offline_entitlement_beta_fixtures
```
