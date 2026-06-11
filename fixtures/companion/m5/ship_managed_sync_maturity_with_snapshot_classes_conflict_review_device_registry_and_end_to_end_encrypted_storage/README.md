# Managed Sync Maturity Surface Fixtures

These fixtures are generated deterministically from the first-consumer surface
builder in `aureline-companion` and validate against
`schemas/companion/ship-managed-sync-maturity-with-snapshot-classes-conflict-review-device-registry-and-end-to-end-encrypted-storage.schema.json`.

## provider_unavailable_surface.json

A surface where the managed sync provider is unavailable, so every section narrows one
qualification step (beta → preview, preview → experimental) and one rollout step, and
every live/cached item is forced to `stale` with `stale_label_shown` set. The scope,
inspectability, stale-state honesty, security review, and consumer projection blocks
stay fully satisfied, and `degraded_labels` records `sync_provider_unavailable` and
`freshness_downgraded_to_stale`. Demonstrates that losing the provider narrows the
claim and downgrades freshness honestly instead of showing stale state as live.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_managed_sync_maturity_surface -- provider_down
```

## sync_uninspectable_surface.json

A surface where managed sync can no longer be inspected or reconciled, so every
snapshot, conflict, and device record narrows to `unreconcilable` and the three
managed-sync sections narrow one step. The encrypted-storage section is untouched.
`degraded_labels` records `sync_inspection_unavailable` and
`reconciliation_downgraded`. Demonstrates that the surface never claims a
reconciliation it can no longer establish.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_managed_sync_maturity_surface -- sync_uninspectable
```

## residency_unverified_surface.json

A surface where the residency or encryption claim is unverified, so every verified
encryption claim downgrades to `claimed_unverified`, sets `proof_label_shown`, clears
`claim_verified`, and the encrypted-storage section narrows one step. `degraded_labels`
records `residency_or_encryption_unverified` and `encryption_claim_downgraded`.
Demonstrates that a customer-managed-key or end-to-end-encryption claim is shown as
proven only when verifiable, and otherwise labeled as unverified.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_managed_sync_maturity_surface -- residency_unverified
```

## device_trust_narrowed_surface.json

A surface where device trust narrowed, so trusted devices narrow to `pending_approval`
and the conflict-review and device-registry sections narrow one step. `degraded_labels`
records `device_trust_narrowed`. Demonstrates that device trust is tracked and
narrowing trust narrows the sections that depend on it.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_managed_sync_maturity_surface -- device_trust_narrowed
```

## admin_continuity_lost_surface.json

A surface where managed-tenant admin continuity is unavailable, so the device-registry
and encrypted-storage sections narrow one step while the snapshot-class and
conflict-review sections are untouched. `degraded_labels` records
`admin_continuity_unavailable`. Demonstrates that managed-tenant device approval and
residency depend on admin continuity, and the local core never does.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_managed_sync_maturity_surface -- admin_continuity_lost
```
