# Distributed-boundary matrix

This document is the narrative companion to the distributed-boundary
matrix. It turns mixed-version compatibility into per-boundary truth
instead of one generic skew note that hides unsafe assumptions. Every
row names the owner, the machine-readable source-of-truth manifest,
the negotiated fields, the supported skew window, the upgrade order,
the rollback order, and the unsupported-state behavior the boundary
applies when the current combination is unsupported.

Companion artifacts:

- [`/artifacts/compat/boundary_matrix.yaml`](../../artifacts/compat/boundary_matrix.yaml)
  — machine-readable one-row-per-boundary matrix. Binds each
  boundary_family to an owner, a source-of-truth manifest list, the
  reserved negotiated fields, the supported skew window, the upgrade
  and rollback order, the downgrade support class, the
  unsupported-state behavior, and the example-case class and fixture.
- [`/artifacts/compat/skew_windows.yaml`](../../artifacts/compat/skew_windows.yaml)
  — machine-readable skew-window, upgrade-order, rollback-order, and
  unsupported-state declarations per boundary family. This matrix
  cites those declarations by `skew_window:*` ref.
- [`/artifacts/compat/qualification_matrix_seed.yaml`](../../artifacts/compat/qualification_matrix_seed.yaml)
  — qualification rows every boundary extends by reference
  (`compat_row:*`).
- [`/artifacts/compat/version_skew_register.yaml`](../../artifacts/compat/version_skew_register.yaml)
  — supported, best-effort, untested, and unsupported skew cases per
  qualification row (`skew_register:*`).
- [`/schemas/compat/mixed_version_envelope.schema.json`](../../schemas/compat/mixed_version_envelope.schema.json)
  — cross-tool envelope contract every later compatibility report and
  release-evidence packet extends by reference.
- [`/fixtures/compat/boundary_cases/`](../../fixtures/compat/boundary_cases)
  — example cases exercising the closed example_case_class vocabulary
  (coordinated-upgrade-only, additive-compatible,
  degrade-to-limited-mode, refuse-start, and refuse-attach).
- [`/fixtures/compat/mixed_version_cases/`](../../fixtures/compat/mixed_version_cases)
  — envelope-level fixtures each boundary row cites through
  `mixed_version_envelope_fixture_ref`.
- [`/docs/compat/upgrade_order_contract.md`](./upgrade_order_contract.md)
  — narrative companion for the envelope, skew-window declaration, and
  upgrade-order contract. This matrix is the reviewer-facing projection
  of those declarations.

## Why this exists

Aureline already has:

- boundary qualification rows in
  `artifacts/compat/qualification_matrix_seed.yaml`,
- explicit supported, best-effort, untested, and unsupported cases in
  `artifacts/compat/version_skew_register.yaml`,
- a mechanical skew-window, upgrade-order, rollback-order, and
  unsupported-state declaration per boundary family in
  `artifacts/compat/skew_windows.yaml`, and
- a reusable mixed-version negotiation envelope in
  `schemas/compat/mixed_version_envelope.schema.json`.

What it did not have was one reviewer-facing matrix that binds each
boundary to one owner, one machine-readable source-of-truth manifest
list, and one worked example case per behavior class. Without that
matrix, later compatibility reports, claim manifests, and
release-evidence packets would each re-derive the cross-boundary story
from scratch and drift against each other in wording, column order, and
scope.

The seed fixes that now:

- every boundary row has one owner drawn from
  `artifacts/governance/ownership_matrix.yaml`;
- every boundary row names at least one machine-readable source of
  truth (the skew-window declaration, the qualification row, the
  version-skew register entry, and the envelope-schema reserved-field
  block), so "source of truth" is a file tooling can read, not a prose
  paragraph;
- every boundary row names one `unsupported_state_behavior` with a
  closed `behavior_class`, a closed `outside_window_posture`, and a
  `contract_rule` that tooling renders verbatim — silent undefined
  behavior is non-conforming;
- every boundary row cites one worked example case in
  `fixtures/compat/boundary_cases/` that shows what happens when the
  upgrade order is violated or the supported skew window is exceeded.

## Row model

Every boundary_matrix row carries:

- a stable `boundary_row_id` (pattern `boundary_row:*`),
- one `boundary_family` from the reserved vocabulary in
  `schemas/compat/mixed_version_envelope.schema.json`,
- a named `producer_surface` and `consumer_surface`,
- one `qualification_row_ref` (`compat_row:*`),
- one `skew_window_declaration_ref` (`skew_window:*`),
- one `version_skew_register_ref` (`skew_register:*`),
- one `artifact_or_protocol_boundary_label` (stable id copied from
  the qualification matrix),
- a non-empty `claimed_deployment_profiles` list drawn from the shared
  deployment-profile vocabulary,
- one `owner_ref` resolvable against
  `artifacts/governance/ownership_matrix.yaml`,
- a non-empty `source_of_truth_manifest_locations` list with `kind`,
  `path`, and optional `ref` per entry,
- a `negotiated_fields` list aligned with the envelope-schema reserved
  fields for that family,
- a `supported_skew_window` (class + summary),
- an `upgrade_order` declaration (ordered component list + notes),
- a `rollback_order` declaration (ordered component list + notes),
- a `downgrade_behavior` block (support class + state-preservation
  note),
- an `unsupported_state_behavior` block (behavior_class +
  outside_window_posture + contract_rule),
- one `example_case_class` from the closed vocabulary
  (`coordinated_upgrade_only`, `additive_compatible`,
  `degrade_to_limited_mode`, `refuse_start`, `refuse_attach`),
- one `example_case_fixture_ref` pointing at a
  `fixtures/compat/boundary_cases/*.json` file whose
  `example_case_class` matches the row,
- one `mixed_version_envelope_fixture_ref` pointing at the worked
  envelope fixture in `fixtures/compat/mixed_version_cases/`,
- a `downstream_consumers` list drawn from the shared vocabulary.

## Boundary-family reading guide

| Boundary family | Owner | Supported window | Upgrade order | Rollback order | Unsupported-state behavior class |
|---|---|---|---|---|---|
| `launcher_and_local_sidecars` | `lane:release_evidence` | `coordinated_artifact_set_only` | coordinated artifact set only | coordinated artifact set only | `coordinated_rollback_or_refuse_start` (`fail_closed`) |
| `desktop_cli_and_remote_agent` | `lane:release_evidence` | `declared_adjacent_window` | client → agent | agent → client | `refuse_attach_or_degrade_to_review_only` (`degraded`) |
| `managed_control_plane` | `lane:release_evidence` | `current_plus_previous_minor_or_lts` | control plane → clients | control plane → clients | `read_only_cached_safe_operations` (`read_only`) |
| `extension_host_and_sdk` | `lane:governance_packets` | `published_sdk_support_window` | host/runtime → extension | extension → host/runtime | `disable_or_quarantine_extension` (`explicitly_unsupported`) |
| `schema_or_state_bundle` | `lane:support_export` | `same_schema_epoch_additive_only` | producer → consumer | producer → consumer | `attributed_error_refuse_ingest` (`fail_closed`) |
| `provider_linked_packet` | `lane:release_evidence` | `current_plus_previous_minor_or_lts` | service family → client | service family → client | `privileged_write_refuse_cached_read_remains` (`read_only`) |
| `audit_or_event_producer_consumer` | `lane:support_export` | `same_schema_epoch_additive_only` | producer → consumer | producer → consumer | `attributed_error_refuse_ingest` (`fail_closed`) |
| `cli_or_export_boundary` | `lane:shell_command_system` | `same_schema_epoch_additive_only` | producer → reader | producer → reader | `refuse_export_unknown_family` (`fail_closed`) |
| `browser_companion_and_remote_or_review_session` | `lane:release_evidence` | `current_plus_previous_minor_or_lts` | service → companion | service → companion | `hand_off_to_desktop_or_review_only` (`degraded`) |

Upgrade and rollback order are mechanical declarations, not advisory
recommendations. Violations surface as `refused_upgrade_order_violation`
(or a `violated_producer_behind_consumer` /
`violated_consumer_behind_producer` compliance verdict where the
boundary can still continue in a narrowed mode), with a visible reason
and a typed repair hint.

## Example-case classes

Every boundary row names one `example_case_class` from the closed
vocabulary and cites one worked case under
`fixtures/compat/boundary_cases/`.

- `coordinated_upgrade_only` — the boundary only promotes the whole
  artifact graph together; rolling upgrades are not a claim. The
  companion fixture shows the in-window coordinated state and names
  what happens if the set is partially promoted. Used by
  `boundary_row:launcher_and_local_sidecars` in-window review; paired
  with `refuse_start` for the violation path.
- `additive_compatible` — the boundary tolerates additive-field
  evolution inside one schema epoch. Producers may lead consumers
  provided consumers preserve unknown additive fields. Used by
  `boundary_row:schema_or_state_bundle`,
  `boundary_row:audit_or_event_producer_consumer`, and
  `boundary_row:cli_or_export_boundary`.
- `degrade_to_limited_mode` — once outside the supported window, the
  boundary degrades to a read-only or review-only fallback rather than
  partially loading. Used by `boundary_row:managed_control_plane`,
  `boundary_row:provider_linked_packet`, and
  `boundary_row:browser_companion_and_remote_or_review_session`.
- `refuse_start` — the boundary fails closed at startup when the
  coordinated artifact set is mixed. Used by the upgrade-order
  violation path on `boundary_row:launcher_and_local_sidecars`.
- `refuse_attach` — the boundary refuses to establish or maintain a
  mutating session when the current skew is outside the declared
  window or the required SDK / permission vocabulary is absent. Used
  by `boundary_row:desktop_cli_and_remote_agent` and
  `boundary_row:extension_host_and_sdk`.

## What happens when upgrade order is violated

Upgrade-order violation is never silent on any boundary in this seed:

- `launcher_and_local_sidecars` refuses mixed startup. The fail-closed
  reason points at the coordinated artifact set and offers a rollback
  repair hint. The worked example is
  [`fixtures/compat/boundary_cases/refuse_start.json`](../../fixtures/compat/boundary_cases/refuse_start.json)
  which cites the envelope fixture
  [`fixtures/compat/mixed_version_cases/upgrade_order_violation.json`](../../fixtures/compat/mixed_version_cases/upgrade_order_violation.json).
- `desktop_cli_and_remote_agent` refuses the mutating attach and
  surfaces a file-or-review-only fallback with a typed downgrade /
  upgrade repair hint. The worked example is
  [`fixtures/compat/boundary_cases/refuse_attach.json`](../../fixtures/compat/boundary_cases/refuse_attach.json).
- `extension_host_and_sdk` disables or quarantines the extension
  rather than partially loading it.
- `managed_control_plane`, `provider_linked_packet`, and
  `browser_companion_and_remote_or_review_session` fall back to the
  declared read-only or review-only posture and refuse privileged or
  mutating paths until the service-side rollback completes.
- `schema_or_state_bundle`, `audit_or_event_producer_consumer`, and
  `cli_or_export_boundary` refuse ingest or export with an attributed
  error rather than silently truncating unknown required fields.

## What happens when the supported skew window is exceeded

Out-of-window behavior is likewise declared per boundary, never
undefined:

- Coordinated-artifact-set-only boundaries (`launcher_and_local_sidecars`)
  fail closed at startup.
- Adjacent-window boundaries (`desktop_cli_and_remote_agent`) degrade
  to file-or-review-only mode; mutating attach is refused.
- Published-SDK-window boundaries (`extension_host_and_sdk`) quarantine
  or disable the affected extension.
- Additive-epoch boundaries (`schema_or_state_bundle`,
  `audit_or_event_producer_consumer`, `cli_or_export_boundary`) refuse
  ingest or export with an attributed error.
- Service-family-window boundaries (`managed_control_plane`,
  `provider_linked_packet`,
  `browser_companion_and_remote_or_review_session`) degrade to cached
  read-only safe operations or hand off to desktop; privileged writes
  are refused.

The worked fixture under `fixtures/compat/boundary_cases/` cites a
concrete mixed-version envelope fixture so reviewers can trace the
story end-to-end without inferring the shape of the envelope.

## Map-back to qualification, claim, and release evidence

The `projection_contract` block in `boundary_matrix.yaml` makes the
following structural promises:

- **Compatibility reports** extend rows by `boundary_row_id`,
  `boundary_family`, `qualification_row_ref`, and
  `skew_window_declaration_ref`. They do not rename boundary families
  or collapse mixed-version status into ad hoc prose.
- **Claim manifests** cite the `boundary_row_id` and
  `qualification_row_ref` before publishing public wording. A
  claim-manifest row without these refs is non-conforming.
- **Release-evidence packets** carry `boundary_row_id` and a
  `mixed_version_envelope_fixture_ref` per boundary under
  qualification so release, support, and docs lanes can consume the
  same ids.
- **Support bundles** quote `unsupported_state_behavior.contract_rule`
  and envelope repair hints verbatim so user-visible state, CLI
  diagnostics, and admin docs tell one story.

Reviewers can follow the `source_of_truth_manifest_locations` list on
each row to reach the binding declarations without alias drift.

## Relationship to earlier work

This seed does not replace the boundary qualification matrix, the
version-skew register, the skew-window declarations, or the
mixed-version envelope contract. It is the reviewer-facing projection
that binds them:

- qualification matrix — which rows exist, who owns them, what they
  claim;
- version-skew register — which supported, best-effort, untested, and
  unsupported cases each row carries;
- skew-window and upgrade-order contract — the mechanical declaration
  every envelope binds to;
- mixed-version negotiation envelope — the reusable runtime shape every
  envelope instance carries;
- **boundary matrix (this seed)** — the reviewer-facing
  one-row-per-boundary projection binding owner, source-of-truth
  manifest locations, negotiated fields, supported skew window,
  upgrade order, rollback order, unsupported-state behavior, and
  worked example case into one inspectable row.

## Out of scope at this revision

- exhaustive multi-version test automation;
- post-M0 protocol rollout plans;
- live release-time compatibility reports (those extend this matrix by
  reference from `artifacts/release/` at release time).
