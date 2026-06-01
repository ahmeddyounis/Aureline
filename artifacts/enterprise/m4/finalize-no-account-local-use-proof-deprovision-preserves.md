# No-Account Local-Use Proof, Managed-Exit Truth, Deprovision-Preserves-Local-Work, and Org-Switch Semantics — Beta Packet

- Packet: `auth:deprovision_preserves_local_work:beta:0001`
- Schema version: `1`
- Contract ref: `auth:deprovision_preserves_local_work:v1`
- Qualification: `beta` (derived, not asserted)
- Silent-purge defects: 0
- Blocking-exit defects: 0
- Data-bearing-without-notice defects: 0
- Withdrawn rows: 0
- Stable rows: all

## Lane coverage

| Exit event | Local editing | Export paths | Org affordances | Profile coverage |
|------------|--------------|-------------|-----------------|-----------------|
| `sign_out` | `preserved_unchanged` | `preserved_unchanged` | removed/narrowed with notice | connected, mirror_only, offline, enterprise_managed |
| `org_switch` | `preserved_unchanged` | `preserved_unchanged` | removed/narrowed with notice | connected, mirror_only, offline, enterprise_managed |
| `seat_loss` | `preserved_unchanged` | `preserved_unchanged` | removed/narrowed with notice | connected, mirror_only, offline, enterprise_managed |
| `deprovision` | `preserved_unchanged` | `preserved_unchanged` | removed/narrowed with notice | connected, mirror_only, offline, enterprise_managed |
| `account_free_local_no_managed_exit` | `preserved_unchanged` | `preserved_unchanged` | not_applicable | connected |

## Evidence sources

- No-account local-use proof module:
  `auth:deprovision_preserves_local_work:v1`
  — `crates/aureline-auth/src/finalize_no_account_local_use_proof_deprovision_preserves/mod.rs`

## Key invariants verified

1. Local editing (buffer, file system, undo history) is `preserved_unchanged`
   for every managed-exit event across all four required deployment profiles.
2. User-owned local export paths (file exports, git commits) are `preserved_unchanged`
   — they are never silently purged on managed exit.
3. Every managed-exit event row carries `prior_export_opportunity: true` —
   the user is offered an export before any affordance closes.
4. Org-scoped affordance removal (collab, seat-bound extensions, managed secret
   broker, policy enforcement) is disclosed with explicit notice before the
   exit event completes.
5. Managed AI narrows to account-free BYOK with notice — it never blocks the
   local BYOK lane.
6. The account-free local lane is never blocked by any managed exit event.

## Hard guardrails — withdrawal conditions

Both of the following force `Withdrawn` and cannot be overridden:

- Any `LocalWorkSilentlyPurged` defect (local editing or export paths silently
  removed without an export opportunity).
- Any `ManagedExitBlocksLocalCore` defect (managed exit event reduces local
  editing to something less than `preserved_unchanged`).

## Canonical paths

- Doc: `docs/enterprise/m4/finalize-no-account-local-use-proof-deprovision-preserves.md`
- Runtime owner: `aureline_auth::finalize_no_account_local_use_proof_deprovision_preserves`
