# Keychain lock-state, denied projection, and secret-repair beta

This document is the reviewer-facing landing page for the beta projection that
makes secret handling recoverable on the four claimed beta profiles. It builds
on the alpha record vocabulary frozen in
[`/docs/security/secret_broker_alpha.md`](../secret_broker_alpha.md) and on
the secret-broker beta projection landed in
[`/docs/security/m3/secret_broker_beta.md`](./secret_broker_beta.md). The
contract is owned by
[`/crates/aureline-auth/src/keychain_state/mod.rs`](../../../crates/aureline-auth/src/keychain_state/mod.rs).

The schema lives at
[`/schemas/security/secret_repair_beta.schema.json`](../../../schemas/security/secret_repair_beta.schema.json)
and the source matrix at
[`/artifacts/security/m3/secret_repair/secret_repair_matrix.yaml`](../../../artifacts/security/m3/secret_repair/secret_repair_matrix.yaml).

## What the projection covers

Every claimed beta page is one
`security_secret_repair_beta_page_record` carrying three record kinds:

- **`security_secret_repair_beta_lock_state_row_record`** — one row per
  observed lock-state of a backing keychain or vault on a claimed profile.
  Each row names:
  - Profile (`connected`, `mirror_only`, `offline`, `enterprise_managed`)
  - Backing store (`os_keychain`, `enterprise_vault_managed`,
    `enterprise_vault_self_hosted_mirror`,
    `enterprise_vault_air_gapped_snapshot`, `platform_agent`,
    `hsm_or_kms_backed`, `session_memory_cache`, `managed_policy_injector`)
  - Lock state (`unlocked`, `locked`, `biometric_required`,
    `user_password_required`, `hardware_token_required`,
    `daemon_unreachable`, `adapter_misconfigured`,
    `vault_snapshot_expired`, `vault_mirror_outage`, `policy_blocked`)
  - Affected consumer (`consumer_id`, `consumer_label`,
    `capability_hash_ref`) and the target/scope refs they were trying to
    reach.
  - Optional back-reference to the originating secret-broker beta row.
  - A typed repair action (`prompt_keychain_unlock`, `prompt_biometric`,
    `prompt_user_password`, `insert_hardware_token`,
    `restart_keychain_daemon`, `restart_credential_agent`,
    `reconfigure_vault_adapter`, `refresh_signed_vault_mirror`,
    `import_signed_vault_snapshot`, `contact_admin_to_unblock_policy`,
    `reauth_and_recover_handles`, `accept_visible_degraded_session_only`,
    or `none_required`) with a user-visible label and a typed
    remediation-path ref.
- **`security_secret_repair_beta_denied_projection_row_record`** — one row
  per blocked secret projection. The row identifies the blocked consumer,
  the target/scope, the requested secret class and projection mode, the
  typed denial reason (`backing_store_locked`,
  `backing_store_unavailable`, `backing_store_signature_missing`,
  `handle_expired`, `handle_revoked`, `lifecycle_state_failed_closed`,
  `policy_blocked`, `plaintext_projection_requested`, `stale_snapshot`,
  `public_endpoint_fallback_requested`, `missing_approval`), and the
  required repair action with a typed remediation-path ref. Store-driven
  denials link the originating lock-state row; row-driven denials link
  the downstream secret-broker beta row.
- **`security_secret_repair_beta_repair_event_record`** — one event per
  repair attempt. The event names the consumer, the originating row(s),
  the typed repair action, and a typed outcome (`awaiting_user`,
  `in_progress`, `resolved`, `user_declined`, `failed_transient`,
  `failed_permanent`). Terminal outcomes declare a `resolved_at`
  timestamp; open outcomes do not.

## Acceptance posture

- **Visible lock-state with a repair action.** Every non-`unlocked`
  lock-state row carries a typed repair action and a remediation-path ref.
  Surfaces never fall through into "unexplained downstream failure" — they
  either name the action the user can take or the action the admin can
  take.
- **Denied projections identify the blocked consumer.** Every denied
  projection row names the blocked consumer (`consumer_id`,
  `consumer_label`, `capability_hash_ref`), the blocked target, the
  blocked workspace scope, the requested secret class, and the required
  repair action. Store-driven denials (`backing_store_locked`,
  `backing_store_unavailable`,
  `backing_store_signature_missing`) link the originating lock-state row
  so a reviewer can pivot from "this consumer is blocked" to "this store
  is the reason."
- **No silent plaintext fallback.** Every lock-state row, denied-projection
  row, and repair event carries the
  `raw_secret_material_present`, `plaintext_fallback_attempted`,
  `plaintext_fallback_offered`, and `plaintext_fallback_taken` invariants
  as `false`. The validator surfaces typed defects when any of those
  flips to `true`. Denied-projection rows also carry
  `public_endpoint_fallback_offered=false`.
- **Local editing preserved.** Every claimed row carries
  `local_editing_preserved=true`. A row that flips this to `false`
  surfaces a `local_editing_not_preserved` defect.
- **Lineage-preserved support and audit packets.**
  [`SecretRepairBetaSupportExport`](../../../crates/aureline-auth/src/keychain_state/mod.rs)
  wraps the page in a redaction-safe envelope that preserves consumer
  lineage (blocked consumer, target, scope, secret class, projection
  mode) and repair lineage (lock-state row, denied-projection row,
  repair event) verbatim. The export proves the no-plaintext-fallback
  invariant.
- **Fail-closed profile coverage.** All four beta profiles (`connected`,
  `mirror_only`, `offline`, `enterprise_managed`) must have at least one
  claimed row; missing coverage surfaces a `profile_coverage_missing`
  defect.

| Profile               | First-claim authority shape                                                              |
| --------------------- | ---------------------------------------------------------------------------------------- |
| `connected`           | OS credential store or live enterprise vault available.                                  |
| `mirror_only`         | Signed enterprise-vault mirror is the only authority.                                    |
| `offline`             | Air-gapped enterprise vault snapshot only.                                               |
| `enterprise_managed`  | Managed policy injector materialises narrow authority per call.                          |

## Failure-mode drills

The seed example regenerates each drill fixture under
[`/fixtures/security/m3/secret_repair/`](../../../fixtures/security/m3/secret_repair/):

- `drill_plaintext_fallback_attempted.json` — a lock-state row sets
  `plaintext_fallback_attempted=true`; the validator surfaces
  `plaintext_fallback_attempted`.
- `drill_repair_action_missing.json` — a non-unlocked lock-state row
  drops its repair action; the validator surfaces
  `repair_action_missing`.
- `drill_store_lock_denial_unlinked.json` — a store-driven denied
  projection drops its `linked_lock_state_row_ref`; the validator
  surfaces `linked_lock_state_missing`.
- `drill_terminal_outcome_missing_resolved_at.json` — a terminal repair
  event drops its `resolved_at` timestamp; the validator surfaces
  `terminal_repair_outcome_missing_resolved_at`.

## Regeneration

```sh
cargo run -q -p aureline-auth --example dump_secret_repair_beta_fixtures -- page > fixtures/security/m3/secret_repair/page.json
cargo run -q -p aureline-auth --example dump_secret_repair_beta_fixtures -- lock-state-rows > fixtures/security/m3/secret_repair/lock_state_rows.json
cargo run -q -p aureline-auth --example dump_secret_repair_beta_fixtures -- denied-projection-rows > fixtures/security/m3/secret_repair/denied_projection_rows.json
cargo run -q -p aureline-auth --example dump_secret_repair_beta_fixtures -- repair-events > fixtures/security/m3/secret_repair/repair_events.json
cargo run -q -p aureline-auth --example dump_secret_repair_beta_fixtures -- defects > fixtures/security/m3/secret_repair/defects.json
cargo run -q -p aureline-auth --example dump_secret_repair_beta_fixtures -- support-export > fixtures/security/m3/secret_repair/support_export.json
cargo run -q -p aureline-auth --example dump_secret_repair_beta_fixtures -- drill-plaintext-fallback-attempted > fixtures/security/m3/secret_repair/drill_plaintext_fallback_attempted.json
cargo run -q -p aureline-auth --example dump_secret_repair_beta_fixtures -- drill-repair-action-missing > fixtures/security/m3/secret_repair/drill_repair_action_missing.json
cargo run -q -p aureline-auth --example dump_secret_repair_beta_fixtures -- drill-store-lock-denial-unlinked > fixtures/security/m3/secret_repair/drill_store_lock_denial_unlinked.json
cargo run -q -p aureline-auth --example dump_secret_repair_beta_fixtures -- drill-terminal-outcome-missing-resolved-at > fixtures/security/m3/secret_repair/drill_terminal_outcome_missing_resolved_at.json
```

## Verification

- Unit tests at
  [`crates/aureline-auth/src/keychain_state/mod.rs`](../../../crates/aureline-auth/src/keychain_state/mod.rs)
  cover the seeded page, each typed defect kind, the no-plaintext-fallback
  invariant, terminal/open `resolved_at` rules, store-driven denial
  linkage, profile coverage, and the support-export redaction posture.
- Fixture-driven coverage at
  [`crates/aureline-auth/tests/secret_repair_beta_cases.rs`](../../../crates/aureline-auth/tests/secret_repair_beta_cases.rs)
  parses every fixture under
  `/fixtures/security/m3/secret_repair/` and verifies the seeded page,
  the drill defects, and the support-export wrapper.
