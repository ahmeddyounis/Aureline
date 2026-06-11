# Usage-Export, Offboarding, Grace-Window, and Org-Switch Fixtures

These fixtures are generated deterministically from the first-consumer surface builder
in `aureline-companion` and validate against
`schemas/companion/implement-usage-export-and-offboarding-packages-grace-window-state-org-switch-semantics-and-deletion-export-ho.schema.json`.

## managed_service_degraded_surface.json

A surface where the managed service is degraded, so every section narrows one
qualification step (beta → preview, preview → experimental) and one rollout step, and
every live/cached item is forced to `stale` with `stale_label_shown` set.
`degraded_labels` records `managed_service_degraded` and `freshness_downgraded_to_stale`.
Demonstrates that a degraded managed service narrows the claim and downgrades freshness
honestly while the local-first paths remain and local work is never stranded.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_usage_export_offboarding_surface -- managed_service_degraded
```

## export_assembler_down_surface.json

A surface where the managed export assembler is unavailable, so every
provider-assembled package (`requires_provider_assembly`) narrows to `unavailable`
while every `local_ready`/`local_staging` package keeps working, and the usage-export
and offboarding sections narrow one step. `degraded_labels` records
`export_assembler_unavailable` and `package_narrowed_to_local_path`. Demonstrates that
losing the export assembler narrows the managed package while the local-first path keeps
the user able to export.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_usage_export_offboarding_surface -- export_assembler_down
```

## completeness_unverified_surface.json

A surface where export completeness is unverified, so every verified completeness claim
(in usage-export and offboarding packages) downgrades to `complete_unverified`, sets
`proof_label_shown`, clears `claim_verified`, and the usage-export and offboarding
sections narrow one step. `degraded_labels` records `completeness_unverified` and
`completeness_claim_downgraded`. Demonstrates that a completeness claim is shown as
proven only when verifiable, and otherwise labeled as unverified.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_usage_export_offboarding_surface -- completeness_unverified
```

## deletion_service_down_surface.json

A surface where the deletion service is unavailable, so every `closing_reversible`
grace window is held open again to `open_reversible` (widening the reversible window),
an already-committed deletion stays `committed_irreversible` and labeled, and the
grace-window section narrows one step. `degraded_labels` records
`deletion_service_unavailable` and `grace_window_held_open`. Demonstrates that losing
the deletion service never strands the user: it only widens the reversible window and
never silently commits a deletion.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_usage_export_offboarding_surface -- deletion_service_down
```

## admin_continuity_lost_surface.json

A surface where managed-tenant admin continuity is unavailable, so the
offboarding-package, grace-window, and org-switch sections narrow one step while the
usage-export section is untouched. `degraded_labels` records
`admin_continuity_unavailable`. Demonstrates that managed offboarding bundles, managed
deletions, and managed org-switch migrations depend on admin continuity, and the
local-first usage export never does.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_usage_export_offboarding_surface -- admin_continuity_lost
```
