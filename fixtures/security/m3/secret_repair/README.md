# Secret-repair beta fixtures

Reviewer-facing fixtures for the beta projection that turns keychain
lock-state observations, denied secret projections, and secret-repair attempts
into one auditable record set across connected, mirror-only, offline, and
enterprise-managed beta profiles.

The canonical record kind is `security_secret_repair_beta_page_record`. The
schema lives at
[`/schemas/security/secret_repair_beta.schema.json`](../../../../schemas/security/secret_repair_beta.schema.json).
The beta module lives at
[`/crates/aureline-auth/src/keychain_state/mod.rs`](../../../../crates/aureline-auth/src/keychain_state/mod.rs)
and the reviewer-facing landing page is
[`/docs/security/m3/secret_repair_beta.md`](../../../../docs/security/m3/secret_repair_beta.md).

## Files

These JSON files are produced by the seed example:

```sh
cargo run -q -p aureline-auth --example dump_secret_repair_beta_fixtures -- page > page.json
cargo run -q -p aureline-auth --example dump_secret_repair_beta_fixtures -- lock-state-rows > lock_state_rows.json
cargo run -q -p aureline-auth --example dump_secret_repair_beta_fixtures -- denied-projection-rows > denied_projection_rows.json
cargo run -q -p aureline-auth --example dump_secret_repair_beta_fixtures -- repair-events > repair_events.json
cargo run -q -p aureline-auth --example dump_secret_repair_beta_fixtures -- defects > defects.json
cargo run -q -p aureline-auth --example dump_secret_repair_beta_fixtures -- support-export > support_export.json
cargo run -q -p aureline-auth --example dump_secret_repair_beta_fixtures -- drill-plaintext-fallback-attempted > drill_plaintext_fallback_attempted.json
cargo run -q -p aureline-auth --example dump_secret_repair_beta_fixtures -- drill-repair-action-missing > drill_repair_action_missing.json
cargo run -q -p aureline-auth --example dump_secret_repair_beta_fixtures -- drill-store-lock-denial-unlinked > drill_store_lock_denial_unlinked.json
cargo run -q -p aureline-auth --example dump_secret_repair_beta_fixtures -- drill-terminal-outcome-missing-resolved-at > drill_terminal_outcome_missing_resolved_at.json
```

| File | Purpose |
| --- | --- |
| `page.json` | Full beta page: lock-state rows, denied projections, repair events, defects, summary. |
| `lock_state_rows.json` | Keychain lock-state rows across all four profiles. |
| `denied_projection_rows.json` | Denied projection rows naming the blocked consumer and required repair action. |
| `repair_events.json` | Repair-attempt events with typed outcomes spanning resolved, in-progress, awaiting-user, user-declined, and failed-permanent. |
| `defects.json` | Defect array; empty on the seeded page. |
| `support_export.json` | Support-export wrapper that preserves consumer and repair lineage and proves the no-plaintext-fallback invariant. |
| `drill_plaintext_fallback_attempted.json` | Drill: a lock-state row flips `plaintext_fallback_attempted=true`; surfaces `plaintext_fallback_attempted`. |
| `drill_repair_action_missing.json` | Drill: a non-unlocked lock-state row drops its repair action; surfaces `repair_action_missing`. |
| `drill_store_lock_denial_unlinked.json` | Drill: a store-driven denial drops its `linked_lock_state_row_ref`; surfaces `linked_lock_state_missing`. |
| `drill_terminal_outcome_missing_resolved_at.json` | Drill: a terminal repair event drops its `resolved_at`; surfaces `terminal_repair_outcome_missing_resolved_at`. |

## Protected states covered

- Keychain or vault lock-state is observable on every claimed beta row,
  carries a user-visible repair action label, and links a typed
  remediation-path ref.
- Denied projection rows identify the blocked consumer (`consumer_id`,
  `consumer_label`, `capability_hash_ref`), the requested secret class and
  projection mode, the typed denial reason, and the required repair action.
  Store-driven denials link the originating lock-state row; row-driven
  denials link the downstream secret-broker beta row.
- Repair-attempt events name the consumer, the originating row(s), the
  typed repair action, and a typed outcome. Terminal outcomes
  (`resolved`, `user_declined`, `failed_permanent`) declare a `resolved_at`
  timestamp; open outcomes (`awaiting_user`, `in_progress`) do not.
- Every record carries `raw_secret_material_present=false`, the
  `plaintext_fallback_*` invariants are false, and `local_editing_preserved`
  is true on every row.
- All four beta profiles (connected, mirror-only, offline, enterprise
  managed) appear on the page; profile drift is surfaced as a typed defect.
- The redaction-safe support export preserves consumer lineage, repair
  lineage, lock-state observations, denial reasons, and remediation-path
  refs.
