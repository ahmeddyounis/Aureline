# M5 AI connector-detail-row and local-model-pack-card primitive contract

Task: **M05-878** — Ship connector / tool-server detail rows and local model pack cards
with boundary / auth / capability / digest / hardware / offline lifecycle truth across the
claimed M5 AI routing surfaces.

This lane narrows the `connector_detail_row` and `local_model_pack_card` families from the
frozen [AI-execution/replay component matrix](./freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix.md)
(M05-876) into two reusable primitives: a connector resolver, a local-model resolver, and
one shared parity matrix. A user can tell — from the row or the card alone — where a tool
or a model runs, what authority or hardware it depends on, what disk cost and offline
locality apply, and which bounded actions are available, before invocation and before
mistaking a bare `installed` state for a warm, verified, hardware-fit pack.

## Primitives

- Connector resolver: `resolve_connector_detail_row(&M5AiConnectorRowResolutionInput) -> Result<M5ResolvedConnectorDetailRow, M5AiConnectorRowResolutionError>`.
- Local-model resolver: `resolve_local_model_pack_card(&M5AiModelPackResolutionInput) -> Result<M5ResolvedLocalModelPackCard, M5AiModelPackResolutionError>`.
- Parity matrix packet: `M5AiConnectorModelPrimitivePacket`, one row per claimed routing
  consumer, each carrying worked connector and model resolution cases.

### Connector readiness ladder (blocking-first)

1. `policy_blocked` → **policy_blocked** — a blocked connector never reads as ready.
2. not `reachable` → **unavailable**.
3. `session_warmed` → **warm**.
4. otherwise → **cold** (reachable but not yet warmed; still invocable).

A connector that declares any capability beyond `read_only_query` must disclose its side
effects (`side_effecting_capability_undisclosed` otherwise). The row always carries
`requires_authority_before_invocation`, true when the connector has a side-effecting
capability or authenticates as anything other than unauthenticated, so a user can tell
what authority a tool depends on before invoking it.

### Model pack readiness ladder (blocking-first)

1. `quarantined` / `provenance_unverified` pack state, or `provenance_verified == false` →
   **verification_held**.
2. `hardware_unfit` pack state, or a blocking hardware fit (`exceeds_memory` /
   `requires_accelerator`) → **hardware_blocked**.
3. `update_available` → **update_pending**.
4. `offline_only` → **offline_ready**.
5. `mirrored` → **mirrored_ready**.
6. otherwise (`installed`) → **ready_selectable**.

Hardware fit is derived from the required / available memory and the accelerator signals:
a missing required accelerator → `requires_accelerator`; required memory over available →
`exceeds_memory`; required memory over three-quarters of available → `fits_with_swap`;
otherwise `fits`. Offline posture is derived from pack state and the network-fetch signal
(`runs_fully_offline`, `mirror_served`, `requires_network_fetch`, `local_cached`).

### Bounded actions by readiness

| Model pack readiness | Available actions |
| --- | --- |
| `verification_held` | `verify`, `remove` |
| `hardware_blocked` | `run_hardware_fit_check`, `remove` |
| `update_pending` | `select`, `update`, `verify`, `remove` |
| `offline_ready` / `mirrored_ready` / `ready_selectable` | `select`, `verify`, `remove` |

### Resolver errors

- Connector: `empty_canonical_id`, `empty_publisher_source`, `empty_capabilities`,
  `side_effecting_capability_undisclosed`, `forbidden_connector_material`.
- Local model: `empty_model_identity`, `empty_digest`, `empty_hardware_expectation`,
  `zero_disk_size` (disk cost is never hidden), `forbidden_model_material`.

## Claimed consumer surfaces

`ai_settings`, `model_picker`, `route_inspector`, `evidence_view`, and
`cli_support_export`. Every row reuses the shared connector and model anatomy, the same
loci / capabilities / auth postures / readinesses / hardware fits / offline postures /
bounded actions, the same mandatory export fields, and a non-visual accessibility route,
so the boundary / auth / locality vocabulary stays identical across settings, model
pickers, route inspectors, evidence views, and support / help exports.

## Hard invariants (per row, all must be false)

- `masks_execution_locus_or_authority`
- `shows_blocked_connector_as_ready`
- `hides_disk_hardware_or_offline_cost`
- `invents_parallel_connector_or_model_grammar`

## Acceptance-criterion lints

- `connector_locus_and_authority_unproven` — at least one worked connector resolution
  proves a connector that depends on an authority grant before invocation.
- `connector_availability_coverage_unproven` — at least one connector resolution is
  invocable and at least one needs attention (unavailable or policy-blocked).
- `model_readiness_coverage_unproven` — at least one model resolution is selectable and at
  least one needs attention (hardware-blocked or verification-held).
- `offline_locality_unproven` — at least one model resolution proves an offline-capable
  pack that carries a real (non-zero) disk cost.

## Reused vocabulary (frozen in M05-876)

`M5AiConnectorCapability`, `M5AiAuthPosture`, `M5AiModelPackState`, `M5AiSurfaceFamily`,
`M5AiDeploymentLine`, `M5AiConsumerSurface`, `M5AiAccessibilityRoute`,
`M5AiQualificationClass`, and `M5AiExecutionDowngradeTrigger`.

## Artifacts

- Boundary schema: `schemas/ai/m5-ai-connector-detail-row-and-local-model-pack-card.schema.json`.
- Support export (canonical): `artifacts/ai/m5/implement_ai_connector_detail_rows_and_local_model_pack_cards_across_claimed_m5_ai_routing_surfaces/support_export.json`.
- Matrix CSV and Markdown report alongside the support export.
- Narrowed fixtures under `fixtures/ai/m5/implement_ai_connector_detail_rows_and_local_model_pack_cards_across_claimed_m5_ai_routing_surfaces/`.
- Headless emitter: `cargo run -p aureline-ai --bin aureline_ai_connector_detail_row_local_model_pack_card_primitive -- <support-export|report|csv|validate|fixture-...>`.
