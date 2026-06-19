# Key-mode and storage-posture inspectors

This contract turns key and trust-root posture from a hidden deployment detail
into explicit continuity state a person can read directly in the product and in
exportable evidence. For every claimed managed, self-hosted, and sovereign row
that protects durable state it produces two things:

1. A **key-mode descriptor** — plain-language key mode, trust-root posture (OS
   store, vendor-managed, customer-managed, or offline trust root), the runtime
   key-availability state, the local keystore store-lock state, the freshness of
   the key/trust evidence, and the typed degraded state when a key fails.
2. A **storage-posture descriptor** — plain-language encryption-at-rest posture
   that names the specific key mode protecting durable storage, so "encrypted" is
   never accepted as sufficient product truth on its own.

The packet is produced by
`aureline_continuity::m5_key_mode_and_storage_posture`. It sits on top of the
frozen continuity-claim matrix
(`aureline_continuity::m5_locality_tenant_keymode_and_drill_matrix`) and reuses
that matrix's `KeyModeClass` vocabulary so there is exactly one key-mode
vocabulary across the product. This lane adds the trust-root posture,
key-availability, store-lock, encryption, and evidence truth the matrix does not
carry, plus the per-surface projection that keeps the vocabulary identical across
desktop, CLI/headless inspect, service-health, support-center exports, About/Help,
and docs/public-truth pages.

## What every surface answers the same way

- Which key mode and trust root protect this row's durable state?
- Is the key or trust material available right now, or is it locked, lost,
  unavailable, or mismatched?
- Is durable storage encrypted, and with which specific key mode?
- When a key fails, what typed degraded state results, and does local-safe work
  still continue?

## Stable conditions

A page qualifies `stable` only when all of the following hold at once:

1. Every claimed row discloses an encryption-at-rest posture that names its
   specific key mode (a bare "encrypted" claim narrows).
2. Every row is projected onto every surface it is required to reach (managed
   rows reach all six surfaces; local-core rows reach all but support-center).
3. Every managed-scope row names an explicit key mode and declares its
   trust-root posture.
4. Every managed-scope row's key and trust material is available, its local store
   is not locked, and its key/trust evidence is current (or stale within grace).
5. No self-hosted or sovereign row leans on vendor-managed keys or a
   vendor-managed trust root.
6. The key and storage vocabulary is identical across every surface projection.

## Fail-closed managed lane

The protected managed lane **fails closed** when a key or trust failure is
detected at runtime: a customer-managed key is unavailable, the running trust
root mismatches the declared one, or durable key material is lost. The affected
row's claim is **withdrawn** and the failure is recorded as the typed degraded
state `managed_lane_fail_closed` — never a generic network error. Crucially, only
the protected managed lane narrows: local-core continuity is preserved
(`local_core_preserved` stays true on every row, including withdrawn ones), so
local-safe editing and version-control work continues.

A locked local store on a managed-scope row is a softer, recoverable degraded
state (`store_locked_degraded`) and holds the claim at `preview` until the store
is unlocked.

## Narrowing reasons

| Reason | Effect |
|---|---|
| `customer_key_unavailable` | withdrawn (fail closed) |
| `trust_root_mismatch` | withdrawn (fail closed) |
| `key_material_lost` | withdrawn (fail closed) |
| `key_mode_undisclosed` | preview |
| `trust_root_posture_undisclosed` | preview |
| `store_locked_on_managed_lane` | preview |
| `key_evidence_stale` | preview |
| `profile_key_mode_mismatch` | preview |
| `key_storage_vocabulary_drift` | preview |
| `local_only_key_overclaimed` | preview |
| `encryption_posture_opaque` | beta |
| `encryption_posture_undisclosed` | beta |
| `key_posture_evidence_missing` | beta |
| `surface_reuse_incomplete` | beta |

## Export safety

The packet is metadata-only. It carries closed-vocabulary tokens, export-safe
plain-language labels, and opaque evidence refs. Raw KMS handles, raw trust
roots, raw key bytes, and any secret material never cross this boundary; the
summary and support-export records both assert `raw_key_material_excluded`.

## Inspect and validate

```sh
# Emit the canonical fixtures.
cargo run -q -p aureline-continuity --example dump_m5_key_mode_storage_posture_fixtures -- page

# Re-audit a page and emit a redaction-safe support export.
cargo run -q -p aureline-continuity --bin aureline_key_posture_inspect -- \
  fixtures/continuity/key_mode_failure_cases/page.json

# Validate the fixtures against the schema.
python3 tools/validate_m5_key_mode_storage_posture_fixtures.py
```

## Related contracts

- Schema: `schemas/continuity/key_mode_descriptor.schema.json`
- Fixtures: `fixtures/continuity/key_mode_failure_cases/`
- Artifact: `artifacts/m5/continuity/key_mode_and_storage_posture.md`
- Truth source for the reused key-mode vocabulary:
  `docs/m5/continuity/locality_tenant_keymode_and_drill_matrix.md`
