# M5 build / remote boundary component contract

This contract freezes Aureline's reusable **build / remote / managed-workspace boundary** UI
component family so target discovery, host ownership, and managed-workspace lifecycle stop drifting
across M5 execution surfaces. It is the shared component-honesty layer on top of the already-claimed
M5 execution and managed-workspace systems — it does **not** re-architect the remote control planes,
the target-discovery engines, or the managed-workspace orchestration backends.

The authoritative gate is the Rust validator in
`crates/aureline-remote` (module
`freeze_the_m5_adapter_confidence_chip_..._and_local_safe_continuation_card_component_matrix`). The
JSON Schemas under `schemas/ui/` document the shape; the checked-in artifacts under
`artifacts/release/m5-build-remote-boundary-proof/` and the fixtures under
`fixtures/ui/m5-build-remote-boundary-components/` are minted from the seed builders by the
`dump_m5_build_remote_boundary_component_matrix` example.

## The eight governed components

| Component | Names | Canonical schema |
| --- | --- | --- |
| `adapter_confidence_chip` | build/runtime adapter confidence + claim ceiling | `schemas/ui/m5-adapter-confidence-chip.schema.json` |
| `discovery_diff_card` | discovery confidence + heuristic-vs-resolved drift | `schemas/ui/m5-discovery-diff-card.schema.json` |
| `host_boundary_strip` | which host kind the work ran on | `schemas/ui/m5-host-boundary-strip.schema.json` |
| `execution_origin_receipt_row` | origin locus where work actually ran | `schemas/ui/m5-execution-origin-receipt-row.schema.json` |
| `managed_workspace_lifecycle_card` | managed-workspace lifecycle state | `schemas/ui/m5-managed-workspace-lifecycle-card.schema.json` |
| `suspend_resume_rebuild_review_sheet` | lifecycle transition + changed persistence + claimed continuity | `schemas/ui/m5-suspend-resume-rebuild-review-sheet.schema.json` |
| `workspace_expiry_banner` | expiry timing governing the workspace | `schemas/ui/m5-workspace-expiry-banner.schema.json` |
| `local_safe_continuation_card` | continuity class + local-safe / companion handoff | `schemas/ui/m5-local-safe-continuation-card.schema.json` |

## The one controlled boundary-disposition vocabulary

Every component binds to a single controlled disposition vocabulary. No build/remote surface invents
a parallel word for any of these:

`local_execution`, `ssh_execution`, `container_execution`, `devcontainer_execution`,
`managed_workspace`, `browser_bridge`, `service_plane`, `suspended`, `rebuilt`, `recreated`,
`expired`, `local_safe_continuation`, `not_evaluated`.

Only `local_execution` is fresh first-party local execution. `rebuilt`, `recreated`, `expired`, and
`local_safe_continuation` materially break exact continuity — a reused card must never present them
as exact continuity.

## Bound object models (reuse, do not fork)

The confidence, host, and lifecycle vocabularies are **bound** from the existing M5 object models
rather than re-minted, so a later consumer cannot fork its own wording:

- `adapter_confidences` ← `aureline_execution::m5_build_and_host_governance::AdapterConfidence`
- `discovery_confidences` ← `aureline_execution::m5_target_discovery::DiscoveryConfidence`
- `host_kinds` / `origin_loci` ← `aureline_execution::m5_host_boundary::{HostKind, OriginLocus}`
- `lifecycle_states` / `persistence_classes` / `continuity_classes` / `expiry_classes` ←
  `aureline_remote::managed_workspace_lifecycle::{LifecycleStateClass, PersistenceClass,
  ContinuityClass, ExpiryClass}`

## Hard invariants (guardrails)

Every row carries three guardrail booleans that MUST be `false`:

1. `implies_exact_continuity_after_material_change` — a reused card never implies exact continuity
   when target identity, image, template, or persistence class changed materially.
2. `hides_local_safe_or_companion_handoff_in_overflow_only` — local-safe continuation and
   browser/companion handoff are never hidden behind overflow-only affordances.
3. `lower_confidence_overwrites_resolved_target_without_review` — lower-confidence discovery never
   overwrites a higher-confidence resolved target without an explicit review state.

## CLI / export / non-visual parity

Every component declares a non-visual accessibility route (keyboard-focusable, screen-reader
announced, non-hover reachable, pointer-optional, high-contrast safe, support-exportable) and is
present in the support/export packet — these components are never shell-only affordances.

## Acceptance criteria

- Design, schema, QA, and release owners share **one** matrix for build/remote boundary primitives.
- Every claimed M5 consumer points at one canonical component contract instead of rewording
  discovery or lifecycle state locally.
- Future implementation rows have an agreed field/state baseline and no open ambiguity about
  confidence, host ownership, or continuity vocabulary.
