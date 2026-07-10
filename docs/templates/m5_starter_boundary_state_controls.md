# M5 starter boundary states

The **starter boundary state** is the cross-cutting truth any of the six governed scaffold /
project-entry components frozen by the
[M5 scaffold-component matrix](m5_scaffold_component_matrix.md) — the scaffold template card, the
starter parameter row, the scaffold preflight card, the template health row, the generated-project
diff card, and the scaffold handoff banner — can carry when a scaffold's **source, availability,
trust, or durability is not the plain public-registry default**. This lane implements that state as
one export-safe packet,
[`StarterBoundaryStateControlsPacket`](../../crates/aureline-templates/src/ship_mirror_offline_auth_boundary_and_managed_zone_starter_states_with_no_silent_trust_no_silent_install_and_non_durable_temp_staging_honesty_across_claimed_m5_scaffold_surfaces/mod.rs),
so a claimed M5 start-center, template-gallery, or scaffold-preflight surface can tell a user what a
starter depends on **before any silent trust or install step**, and always keep an explicit recovery
path once a starter partially materializes output.

## What the resolver decides

The module has one derived resolver so the honesty of each boundary state is computed, never
asserted.

### `resolve_starter_disclosure`

Given a boundary state's frozen **boundary kind** and **availability state**, the resolver derives an
**access posture** and an **availability posture**:

- `public_registry` -> `direct_public_access`
- `mirror_only` -> `mirror_mediated` (must carry a mirror / offline-cache note)
- `offline_cache_only` -> `offline_cache_backed` (must carry a mirror / offline-cache note)
- `sign_in_required` -> `auth_gated` (must carry a sign-in note)
- `remote_or_managed_workspace` -> `managed_remote` (must carry a managed / remote note)
- `non_durable_temp_staging` -> `non_durable_staging` (must carry a non-durable note)

- `available` -> `reachable_now`
- `mirror_reachable_only` -> `reachable_via_mirror`
- `cache_only_offline` -> `reachable_from_cache`
- `sign_in_pending` -> `blocked_pending_sign_in`
- `provisioning_pending` -> `blocked_pending_provisioning`
- `unavailable` -> `not_reachable` (must carry an unavailable note)

A sign-in-gated, managed-remote, or non-durable starter can therefore never read as a plain
public-registry create, and an unavailable or blocked starter can never read as ready. The state
independently names its **owner class** (`first_party_registry`, `team_mirror`, `local_cache`,
`managed_service`, or `unknown_owner`) and its **freshness state** (`live`, `mirror_synced`,
`cache_stale`, `ephemeral`, or `freshness_unknown`), so the source, owner, freshness, and
availability cues stay explicit.

## No silent trust, no silent install

Every boundary state carries a **trust-disclosure note** and an **install-disclosure note** that name
the trust prompt and the install / network / provisioning / staging step **before** they run — so a
generic Create can never route a user through a silent trust or install step. The hard invariant
`performs_silent_trust_or_install` must stay `false`.

## Recovery is always preserved

When a starter partially materializes output or cannot complete the full bootstrap path, the state
preserves the acceptance-criteria recovery verbs — `delete_generated`, `reuse_existing`,
`clone_elsewhere`, and `continue_without_starter` — plus `retry_when_available`.
`continue_without_starter` is always present, and every state offers at least one real recovery
route, so failure or partial bootstrap never leaves ambiguous generated output.

## The six seeded boundary states

| Boundary kind | Availability | Access posture | Availability posture | Owner | Freshness |
| --- | --- | --- | --- | --- | --- |
| `public_registry` | `available` | `direct_public_access` | `reachable_now` | `first_party_registry` | `live` |
| `mirror_only` | `mirror_reachable_only` | `mirror_mediated` | `reachable_via_mirror` | `team_mirror` | `mirror_synced` |
| `offline_cache_only` | `cache_only_offline` | `offline_cache_backed` | `reachable_from_cache` | `local_cache` | `cache_stale` |
| `sign_in_required` | `sign_in_pending` | `auth_gated` | `blocked_pending_sign_in` | `managed_service` | `freshness_unknown` |
| `remote_or_managed_workspace` | `provisioning_pending` | `managed_remote` | `blocked_pending_provisioning` | `managed_service` | `live` |
| `non_durable_temp_staging` | `unavailable` | `non_durable_staging` | `not_reachable` | `unknown_owner` | `ephemeral` |

The six states cover every boundary kind, availability state, owner class, freshness state, access
posture, availability posture, and recovery verb.

## Hard invariants

Every boundary state keeps five bools `false`:

- `hides_starter_source_or_owner`
- `hides_mirror_offline_or_managed_dependency`
- `performs_silent_trust_or_install`
- `omits_recovery_or_continue_without_starter_path`
- `invents_alternate_state_label`

## Deep links, not overlays

Every next step names one stable `template_manifest`, `starter_registry_entry`, `docs_anchor`, or
`policy_reference` deep link rather than an ephemeral overlay or hidden route.

## Export safety

Raw file bodies, raw secret values, pasted local paths, repository URLs, credentials, and secrets
never cross this boundary; every note, deep-link reference, and component identity is carried only as
an opaque, export-safe representation. The checked support export
(`artifacts/release/m5-starter-boundary-state-proof/support_export.json`), the machine-readable
matrix CSV, and the two scenario fixtures
(`fixtures/ui/m5-starter-boundary-state-controls/`) are all regenerated deterministically from the
canonical seed builders via `cargo run -p aureline-templates --example dump_starter_boundary_states`.
