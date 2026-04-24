# Distributed-boundary matrix example cases

Example-case fixtures for the distributed-boundary matrix seeded at
[`/artifacts/compat/boundary_matrix.yaml`](../../../artifacts/compat/boundary_matrix.yaml).
The narrative companion lives in
[`/docs/compat/boundary_matrix.md`](../../../docs/compat/boundary_matrix.md).

Each fixture renders one closed `example_case_class` from the matrix
vocabulary. None of the fixtures is a live release packet; they exist
so reviewers can follow the end-to-end story from boundary row through
skew-window declaration, qualification row, and worked
mixed-version envelope fixture without inferring shape.

| Fixture | Example-case class | Boundary | What it exercises |
|---|---|---|---|
| [`coordinated_upgrade_only.json`](./coordinated_upgrade_only.json) | `coordinated_upgrade_only` | `launcher_and_local_sidecars` | In-window coordinated set plus the contract that any mix outside the coordinated set refuses to start. |
| [`additive_compatible.json`](./additive_compatible.json) | `additive_compatible` | `schema_or_state_bundle` | Producer ahead of consumer within one additive epoch; consumer preserves unknown additive fields. |
| [`degrade_to_limited_mode.json`](./degrade_to_limited_mode.json) | `degrade_to_limited_mode` | `managed_control_plane` | Out-of-window client; service retains cached read-only safe operations and refuses privileged writes. |
| [`refuse_start.json`](./refuse_start.json) | `refuse_start` | `launcher_and_local_sidecars` | Worked upgrade-order violation: mixed coordinated set refuses to start with a visible fail-closed reason. |
| [`refuse_attach.json`](./refuse_attach.json) | `refuse_attach` | `desktop_cli_and_remote_agent` | Worked skew-window exceeded: mutating attach refuses with typed repair hints; review-only or file-only fallback remains available. |

Each fixture names:

- the `boundary_row_ref` it extends (from
  `artifacts/compat/boundary_matrix.yaml`),
- the `qualification_row_ref`, `skew_window_declaration_ref`, and
  `version_skew_register_ref` it binds to,
- the `mixed_version_envelope_fixture_ref` from
  `fixtures/compat/mixed_version_cases/` that shows the envelope-level
  story,
- one `owner_ref` and a non-empty
  `source_of_truth_manifest_locations` list,
- one `unsupported_state_behavior` block with a closed
  `behavior_class`, a closed `outside_window_posture`, and a
  `contract_rule` string tooling renders verbatim,
- one `what_happens_when_upgrade_order_violated` narrative,
- one `what_happens_when_skew_window_exceeded` narrative,
- an ordered `repair_hints_expected` list, and
- a `map_back_to` block pointing downstream compatibility reports,
  claim manifests, and release-evidence packets at the same row and
  qualification ids — so reviewers can walk from the matrix row to the
  release-evidence packet and back without alias drift.

The `refuse_start.json` and `refuse_attach.json` fixtures together
satisfy the acceptance criterion that example cases show what happens
when the upgrade order is violated or the supported skew window is
exceeded.
