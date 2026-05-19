# Capability Dependency Marker Beta

Capability dependency markers are read-only records that any
capability-sensitive artifact persists when behavior narrows or
portability changes on targets lacking the required capability. They
exist so stable- or beta-facing artifacts and product surfaces cannot
silently depend on Labs, Preview, Beta-only, policy-gated, or
host-specific capabilities — every claimed artifact carries an explicit
dependency marker, fallback note, and typed import/downgrade behavior.

The boundary schemas live at:

- [`/schemas/capabilities/capability_record.schema.json`](../../../schemas/capabilities/capability_record.schema.json)
- [`/schemas/capabilities/artifact_dependency_marker.schema.json`](../../../schemas/capabilities/artifact_dependency_marker.schema.json)

The runtime projection lives at:

- [`/crates/aureline-capabilities/src/dependency_markers/mod.rs`](../../../crates/aureline-capabilities/src/dependency_markers/mod.rs)

Canonical fixtures live at:

- [`/fixtures/capabilities/m3/dependency_markers/`](../../../fixtures/capabilities/m3/dependency_markers/)

The release-evidence packet lives at:

- [`/artifacts/release/m3/capability_dependency_marker_packet.md`](../../../artifacts/release/m3/capability_dependency_marker_packet.md)

## Contract

Every artifact in the closed [`ArtifactClass`] set MUST persist one
`capabilities_artifact_dependency_marker` record per capability it
depends on when behavior narrows or portability changes on a target
that lacks the capability. The closed artifact classes are:

| Artifact Class | Where it travels |
| --- | --- |
| `settings_export` | Settings export bundle (effective values, overlays, lock state). |
| `profile` | User or workspace profile. |
| `workflow_bundle` | Workflow bundle (recorded macros, command compositions). |
| `portable_state_package` | Portable workspace state, restore payload, handoff bundle. |
| `recipe` | Saved recipe (parameterized command + inputs). |
| `saved_view` | Saved graph, search, or navigation view. |
| `migration_packet` | Settings, profile, or capability migration payload. |
| `support_export` | Support-export packet attached to a support bundle. |
| `sync_artifact` | Cross-device sync payload. |

Every marker carries:

| Block | Purpose |
| --- | --- |
| `marker_id` | Stable opaque marker identity. |
| `artifact_class` + `artifact_ref` | Which artifact the marker is attached to. |
| `required_capability_id` | Capability the artifact depends on. |
| `required_lifecycle_state` | Lifecycle state the producer observed at mint time. |
| `dependency_class` | `labs`, `preview`, `beta_only`, `policy_gated`, or `host_specific`. |
| `support_promise` | Support promise recorded on the marker. |
| `effect_on_import` | Typed import-behavior class. Every variant preserves user-authored data. |
| `behavior_on_missing.summary` | Reviewer-facing "what changed" copy. |
| `behavior_on_missing.fallback_path` | Bounded recover / dismiss / wait path. |
| `kill_switch_active` | True when an active kill switch / policy disable narrowed the dependency at mint time. |
| `host_scope` | Closed host list for `host_specific` markers. |
| `docs_ref` | Optional docs / help ref the surface can open. |

## UX Rules

- **Same vocabulary everywhere.** Settings inspectors, import-review
  sheets, bundle detail pages, downgrade flows, headless / CLI inspect
  output, and docs / help pages all consume the same projection. The
  `dependency_class`, `required_lifecycle_state`, `support_promise`,
  and `effect_on_import` tokens never differ across surfaces.
- **No silent drop.** The closed `effect_on_import` vocabulary forbids
  variants that discard user-authored data. `block_apply`,
  `narrow_behavior`, `emulated_downgrade`, `hold_for_later`, and
  `render_tombstone` all carry `_preserve_data` in the token name so a
  reviewer reading the marker sees the contract directly.
- **Apply is held until disclosure.** Import-review sheets and
  downgrade flows MUST disclose every live marker before apply. The
  `MarkerHostProjection.blocks_apply_until_disclosed` flag drives the
  gate.
- **Recover is always one click.** Settings inspectors, import-review
  sheets, bundle detail pages, and downgrade flows render the
  `fallback_path` as an inline recover affordance
  (`MarkerHostProjection.requires_recover_affordance = true`). CLI
  inspect output and docs / help pages quote the same fallback path as
  context but do not require a button.
- **Kill switches and policy disables still produce guidance.** When
  `kill_switch_active = true`, the marker MUST carry a non-empty
  `fallback_path`; the validator rejects the artifact otherwise. User
  data is preserved.
- **Host-specific markers name their hosts.** When `dependency_class =
  host_specific`, the marker MUST carry at least one entry in
  `host_scope` (e.g. `managed_admin_surface`, `desktop_product`).
- **No drift between marker and capability record.** The validator
  cross-checks `dependency_class`, `support_promise`, and
  `required_lifecycle_state` against the producer-side
  `capability_record` and rejects drift before the artifact ships.

## Fixture Coverage

The fixtures under
[`/fixtures/capabilities/m3/dependency_markers/`](../../../fixtures/capabilities/m3/dependency_markers/)
exercise every artifact class against every dependency class. The
fixture-replay test [`dependency_markers_fixtures.rs`](../../../crates/aureline-capabilities/tests/dependency_markers_fixtures.rs)
loads every JSON fixture and proves that:

1. Each marker passes `validate_artifact_markers`.
2. Each marker projects to identical vocabulary on every host surface.
3. Every artifact class and dependency class is exercised at least
   once.
4. Every projection reports `user_authored_data_preserved = true`.

## Out Of Scope

- Hosted experimentation analytics or broad experimentation governance
  beyond marker fidelity, portability, and downgrade honesty.
- Live RPC of raw provider tokens, raw policy-bundle bytes, or raw
  kill-switch material; the marker carries ids and typed vocabulary
  only.
- Rewriting the governance-owned `capability_lifecycle` vocabulary; the
  marker mirrors that vocabulary instead of replacing it.
