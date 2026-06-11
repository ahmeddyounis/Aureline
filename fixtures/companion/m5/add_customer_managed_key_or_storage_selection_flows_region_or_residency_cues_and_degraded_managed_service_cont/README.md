# Key/Storage Selection, Residency Cues, and Degraded Continuity Fixtures

These fixtures are generated deterministically from the first-consumer surface
builder in `aureline-companion` and validate against
`schemas/companion/add-customer-managed-key-or-storage-selection-flows-region-or-residency-cues-and-degraded-managed-service-cont.schema.json`.

## managed_service_degraded_surface.json

A surface where the managed service is degraded, so every section narrows one
qualification step (preview → experimental, beta → preview) and one rollout step,
every live/cached item is forced to `stale` with `stale_label_shown` set, and every
non-local continuity capability is marked `degraded` while `local_work_preserved`
stays set. `degraded_labels` records `managed_service_degraded` and
`freshness_downgraded_to_stale`. Demonstrates that a degraded managed service narrows
the claim and downgrades freshness honestly while the local core continues and local
work is never stranded.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_key_storage_residency_continuity_surface -- managed_service_degraded
```

## key_management_unavailable_surface.json

A surface where the customer-managed-key key-management service is unavailable, so
every non-local key-custody option narrows to `requires_admin_approval` and the
key-custody section narrows one step. The `local_only_no_key_escrow` option stays
`available`. `degraded_labels` records `key_management_unavailable` and
`selection_narrowed_to_local_fallback`. Demonstrates that losing key management
narrows the non-local selection while the local-only key fallback keeps working.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_key_storage_residency_continuity_surface -- key_management_down
```

## storage_provider_unavailable_surface.json

A surface where the managed storage provider is unavailable, so every
non-local-fallback storage option narrows to `requires_admin_approval` and the
storage section narrows one step. The `local_only` and `hybrid_local_first` options
remain available. `degraded_labels` records `storage_provider_unavailable` and
`selection_narrowed_to_local_fallback`. Demonstrates that losing the storage provider
narrows the managed storage selection while the local-first storage fallback keeps
working.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_key_storage_residency_continuity_surface -- storage_provider_down
```

## residency_unverified_surface.json

A surface where the residency claim is unverified, so every verified residency pin
(in the cues and the storage options) downgrades to unverified, sets
`proof_label_shown`, clears `claim_verified`, and the residency-cue and storage
sections narrow one step. `degraded_labels` records `residency_unverified` and
`residency_claim_downgraded`. Demonstrates that a region-residency claim is shown as
proven only when verifiable, and otherwise labeled as unverified.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_key_storage_residency_continuity_surface -- residency_unverified
```

## admin_continuity_lost_surface.json

A surface where managed-tenant admin continuity is unavailable, so the key-custody,
residency-cue, and managed-service-continuity sections narrow one step while the
storage section is untouched. `degraded_labels` records
`admin_continuity_unavailable`. Demonstrates that customer-managed-key custody,
region pinning, and managed-tenant continuity depend on admin continuity, and the
local core never does.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_key_storage_residency_continuity_surface -- admin_continuity_lost
```
