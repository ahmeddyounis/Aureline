# Mixed-version negotiation and upgrade-order contract

This document is the narrative companion to the first mixed-version
compatibility seed. It names the generic negotiation envelope, the
supported skew windows, the upgrade and rollback order per named
boundary, and the unsupported-state behavior that keeps every boundary
from inheriting a hand-wavy "best effort" story.

Companion artifacts:

- [`/schemas/compat/mixed_version_envelope.schema.json`](../../schemas/compat/mixed_version_envelope.schema.json)
  — reusable envelope contract for the named distributed or
  file/schema boundaries.
- [`/artifacts/compat/skew_windows.yaml`](../../artifacts/compat/skew_windows.yaml)
  — machine-readable skew-window, upgrade-order, rollback-order, and
  unsupported-state declarations per boundary family.
- [`/fixtures/compat/mixed_version_cases/`](../../fixtures/compat/mixed_version_cases)
  — seed fixtures for compatible, out-of-window, downgrade-required,
  partial-feature-narrowing, repair-or-reattach, and upgrade-order
  violation flows.
- [`/artifacts/compat/qualification_matrix_seed.yaml`](../../artifacts/compat/qualification_matrix_seed.yaml)
  — boundary qualification rows the envelope extends by reference.
- [`/artifacts/compat/version_skew_register.yaml`](../../artifacts/compat/version_skew_register.yaml)
  — supported, best-effort, untested, and unsupported skew cases per
  qualification row.

## Why this exists

Aureline already has:

- boundary qualification rows and skew cases in `artifacts/compat/`,
- exact-build identity as a shared build-truth model, and
- architecture-level compatibility rules in `§26.5 Distributed
  compatibility and version-skew policy` and `§9.9 Mixed-version
  compatibility, negotiation, and upgrade posture`.

What it did not have was one shape that every distributed or file/schema
boundary can speak in, with capability and version fields, fail-closed
state, a declared upgrade and rollback order, an unsupported-state
rule, and typed repair hints. Without that shape, every new boundary
tends to invent its own ad hoc compatibility identity and the release
and support lanes end up paraphrasing each other.

The seed fixes that now:

- the envelope is one contract, not one per boundary;
- the skew-window and upgrade-order declarations are machine-readable
  and binding, not prose-only notes;
- unsupported combinations fail closed with a visible reason, not
  implicit corruption or undefined behavior;
- qualification and compatibility reports consume the same envelope
  instead of minting parallel compatibility identities.

## Envelope model

Every mixed-version envelope carries:

- a stable `envelope_id`,
- a `boundary_family` from the reserved vocabulary,
- a `boundary_row_ref` (`compat_row:*`) and
  `version_skew_register_ref` (`skew_register:*`),
- a `skew_window_declaration_ref` (`skew_window:*`) pointing at the
  binding declaration in `artifacts/compat/skew_windows.yaml`,
- a `producer` and `consumer` participant, each with
  `contract_version`, `schema_epoch`, `min_supported_contract_version`,
  `max_tested_contract_version`, and a typed `capability_set`,
- a `negotiation` block carrying one of the closed outcomes
  (`compatible`, `compatible_with_partial_feature_narrowing`,
  `downgrade_required`, `upgrade_required`, `refused_out_of_window`,
  `refused_upgrade_order_violation`, `refused_unsupported_combination`,
  `repair_required`, `reattach_required`), the
  `negotiated_capability_set`, and an ordered `dropped_capabilities`
  list with typed reason classes,
- a `skew_window` snapshot with class, status, window summary, and
  matching `skew_case_ref` where present,
- an `upgrade_order` block with `declared_order` and a closed
  `compliance` verdict,
- a `rollback_order` block with `declared_order` and a closed
  `compliance` verdict,
- a `downgrade_behavior` block with support class and
  state-preservation note,
- a `fail_closed_state` block with `active`, `posture`, and a visible
  reason,
- an `unsupported_state_behavior` block with the contract rule the
  boundary applies against unsupported combinations,
- an ordered `repair_hints` list whose kinds come from the reserved
  `repair_hint_kind` vocabulary,
- a `reserved_boundary_fields` block with exactly one populated
  sub-block matching the boundary family.

Capability negotiation chooses the intersection, never an optimistic
superset. Every refused or narrowed outcome carries a visible reason.

## Supported skew windows per boundary

The supported windows and their primary unsupported-state behavior are
summarized below. The binding source is
`artifacts/compat/skew_windows.yaml`; this table is a reviewer-facing
projection of the same declarations.

| Boundary family | Skew-window class | Upgrade order | Rollback order | Out-of-window posture |
|---|---|---|---|---|
| `launcher_and_local_sidecars` | `coordinated_artifact_set_only` | coordinated artifact set only | coordinated artifact set only | `fail_closed` |
| `desktop_cli_and_remote_agent` | `declared_adjacent_window` | client → agent | agent → client | `degraded` (file or review-only) |
| `managed_control_plane` | `current_plus_previous_minor_or_lts` | control plane → clients | control plane → clients | `read_only` (cached safe ops) |
| `extension_host_and_sdk` | `published_sdk_support_window` | host/runtime → extension | extension → host/runtime | `explicitly_unsupported` (disable or quarantine) |
| `schema_or_state_bundle` | `same_schema_epoch_additive_only` | producer → consumer | producer → consumer | `fail_closed` (attributed error) |
| `provider_linked_packet` | `current_plus_previous_minor_or_lts` | service family → client | service family → client | `read_only` (cached metadata) |
| `audit_or_event_producer_consumer` | `same_schema_epoch_additive_only` | producer → consumer | producer → consumer | `fail_closed` (no silent truncation) |
| `cli_or_export_boundary` | `same_schema_epoch_additive_only` | producer → reader | producer → reader | `fail_closed` (refuse unknown family) |
| `browser_companion_and_remote_or_review_session` | `current_plus_previous_minor_or_lts` | service → companion | service → companion | `degraded` (desktop handoff) |

Upgrade and rollback order are mechanical declarations, not advisory
recommendations. Violations surface as `refused_upgrade_order_violation`
(or a `violated_producer_behind_consumer`/`violated_consumer_behind_producer`
compliance verdict where the boundary can still continue in a narrowed
mode), with a visible reason and a typed repair hint.

## Downgrade behavior

Downgrade is a declared support class per boundary (see
`downgrade_support_class_vocabulary` in `skew_windows.yaml`), not a
silent capability:

- `downgrade_supported` — additive-only downgrade preserves state.
- `downgrade_best_effort` — cached read paths preserved; mutating
  paths narrow with a visible reason.
- `downgrade_untested` — explicitly reserved; not a silent support
  claim.
- `downgrade_unsupported` — downgrade refuses rather than partially
  loading in ambiguous mode.
- `downgrade_requires_coordinated_artifact_set` — downgrade is always
  a coordinated rollback; individual components cannot downgrade
  independently.

Every boundary declaration names exactly one support class and a
state-preservation note when durable state is involved.

## Unsupported-state behavior

Unsupported combinations never render as undefined behavior. Each
boundary declaration names a `contract_rule` the boundary applies
whenever the current combination is unsupported. Tooling renders that
rule verbatim in desktop UI, CLI diagnostics, admin docs, and support
exports. The rule is machine-readable and stable; user-facing wording
stays localizable without changing the contract.

## Repair hints

Repair hints are closed and typed. The reserved kinds are:

- `upgrade_component`
- `downgrade_component`
- `rollback_artifact_set`
- `reattach_session`
- `refresh_policy_bundle`
- `refresh_capability_manifest`
- `reissue_skew_snapshot`
- `import_compatible_bundle`
- `use_file_or_review_only_mode`
- `use_cached_read_only_mode`
- `disable_or_quarantine_extension`
- `wait_for_staged_rollout`
- `contact_admin_or_support`

Each hint names the `target_component`, a reviewer-facing
`instruction`, and a `reversible` flag. Hints are ordered so the
first entry is the safest or most likely to succeed on the current
boundary.

## How later artifacts use the envelope

- **Compatibility reports** extend the envelope by reference. They add
  release-specific evidence refs, migration notes, and a verdict, but
  they do not rename boundary families or invent a parallel envelope
  shape.
- **Claim manifests** cite the envelope id and the named boundary
  row; public wording resolves through the envelope first.
- **Release-evidence packets** carry envelope ids for each boundary
  under qualification; qualification and compatibility reports
  consume the same ids.
- **Support bundles** quote envelope ids, visible reasons, and repair
  hints verbatim so user-visible state, CLI diagnostics, and admin
  docs tell one story.

## Fixtures

The seed fixtures in
[`/fixtures/compat/mixed_version_cases/`](../../fixtures/compat/mixed_version_cases)
exercise the closed vocabulary directly:

- `compatible.json` — schema-or-state-bundle producer and consumer
  both in the same additive epoch; `outcome = compatible`.
- `out_of_window.json` — managed-control-plane client past the
  declared previous-minor window; `outcome = refused_out_of_window`
  and `fail_closed_state.posture = read_only`.
- `downgrade_required.json` — desktop-CLI client ahead of the
  declared adjacent window against the remote agent;
  `outcome = downgrade_required` and `downgrade_behavior.support_class
  = downgrade_best_effort`.
- `partial_feature_narrowing.json` — extension host and extension
  both in-window but the extension advertises a permission the host
  does not implement; `outcome =
  compatible_with_partial_feature_narrowing` with the dropped
  capability recorded.
- `repair_reattach.json` — remote agent attach drifts out of the
  session envelope after a network partition;
  `outcome = reattach_required` with a typed `reattach_session`
  repair hint.
- `upgrade_order_violation.json` — launcher-and-local-sidecars boundary
  starts with a mixed artifact set;
  `outcome = refused_upgrade_order_violation` and
  `fail_closed_state.posture = fail_closed`. This is the worked
  example showing how a boundary fails closed when the upgrade order
  is violated, and it is the fixture the acceptance criterion for
  upgrade-order violation cites directly.

## Relationship to earlier work

This seed does not replace the boundary qualification matrix or the
version-skew register. It supplements them:

- qualification matrix: which rows exist, who owns them, what they
  claim;
- version-skew register: which supported, best-effort, untested, and
  unsupported skew cases each row carries;
- skew-window and upgrade-order contract (this seed): the mechanical
  declaration every mixed-version envelope binds to, and the
  reusable envelope shape that release, support, admin, and
  ecosystem surfaces all consume.
