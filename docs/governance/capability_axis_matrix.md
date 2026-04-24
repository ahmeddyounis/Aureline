# Capability / support-class / channel / client-scope / freshness / certified-archetype / dependency-marker axis matrix

This document is the authoritative narrative for the seven independent
axes every capability-bearing badge in Aureline must render separately.
It exists so that product surfaces, docs, support exports, claim
manifests, compatibility and certified-archetype reports, and
About/service-health panels never collapse lifecycle readiness, support
tier, release channel, client scope, freshness, certified-archetype
posture, and dependency markers into one ambiguous "available" or
"stable" chip.

Machine-readable companion:

- [`/artifacts/governance/capability_badge_axes.yaml`](../../artifacts/governance/capability_badge_axes.yaml)
  — frozen axes, per-axis downgrade ordering, badge-propagation channel
  rules, forbidden cross-axis combinations, downgrade-propagation
  effects, and fixture refs.

Upstream contracts (re-exported, not re-minted):

- [`/docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md)
  and
  [`/schemas/governance/capability_lifecycle.schema.json`](../../schemas/governance/capability_lifecycle.schema.json)
  — canonical `lifecycle_state`, `support_class`, `release_channel`,
  `freshness_class`, `client_scope`, `marker_kind`, `effect_on_parent`,
  and `reason_code` enums.
- [`/docs/adr/0013-docs-help-service-health-truth.md`](../adr/0013-docs-help-service-health-truth.md)
  and
  [`/schemas/docs/help_status_badge.schema.json`](../../schemas/docs/help_status_badge.schema.json)
  — `source_class`, `version_match_state`, `service_contract_state`,
  `degraded_state_cause`, and `surface_class` vocabularies.
- [`/schemas/release/compatibility_row.schema.json`](../../schemas/release/compatibility_row.schema.json)
  and
  [`/docs/release/compatibility_report_template.md`](../release/compatibility_report_template.md)
  — compatibility-row `support_class`
  (`certified`/`supported`/`community`/`experimental`) and
  `current_state` (`supported`/`best_effort`/`untested`/`degraded`/
  `unsupported`).
- [`/docs/release/certified_archetype_report_template.md`](../release/certified_archetype_report_template.md)
  — certified-archetype report shape consumed by the
  certified-archetype axis.
- [`/schemas/governance/claim_manifest.schema.json`](../../schemas/governance/claim_manifest.schema.json)
  and
  [`/artifacts/governance/claim_manifest_seed.yaml`](../../artifacts/governance/claim_manifest_seed.yaml)
  — claim-row publication contract; the worst-supporting-axis rule
  below composes with the `active_downgrade_reasons` set already
  frozen there.
- [`/artifacts/governance/truth_class_matrix.yaml`](../../artifacts/governance/truth_class_matrix.yaml)
  and
  [`/artifacts/governance/claim_propagation_rules.yaml`](../../artifacts/governance/claim_propagation_rules.yaml)
  — freshness composition and worst-supporting-truth-wins rule this
  matrix reuses unchanged.

## Purpose

A reviewer inspecting any capability badge (settings row, bundle card,
command palette entry, install review step, docs pane, support-export
line, claim-manifest entry, compatibility report row, About packet
line) must be able to answer **seven** questions mechanically without
parsing surface-local copy:

1. **Lifecycle state.** What readiness posture has the owning lane
   declared (labs / preview / beta / stable / lts_facing / deprecated
   / disabled_by_policy / retired), and what is the effective state
   after dependency-marker narrowing?
2. **Support class.** What support tier is committed
   (best_effort / community_supported / standard_support /
   extended_support / operator_only_support / no_support), independent
   of lifecycle readiness?
3. **Release channel.** On which channel does the shipping build
   carry this capability
   (nightly / experimental / preview / stable / lts / managed_only),
   independent of declared lifecycle?
4. **Client scope.** Which rendering clients does this capability
   expose on (desktop_product / cli / companion_surface / remote_agent
   / sdk_or_api / managed_admin_surface)?
5. **Freshness.** How current is the evidence backing this badge
   (authoritative_live / warm_cached / degraded_cached / stale /
   unverified)?
6. **Certified-archetype status.** Does a live compatibility or
   certified-archetype report row back this capability for the
   claimed tuple, and at what state (supported / best_effort /
   untested / degraded / unsupported / not_applicable)?
7. **Dependency markers.** Which live sub-capability dependencies
   are narrowing the parent capability, with what `effect_on_parent`,
   `reason_code`, and `repair_hook_ref`?

A generic "available" / "unavailable" / "stable" / "beta" chip that
collapses two or more of these axes into a single label is
**forbidden on every protected surface**. When an axis is not yet
known, the surface renders that axis's `unresolved_axis` token and
routes to the owning repair hook, rather than fabricating a synthetic
positive chip.

## The seven axes

Each axis has its own frozen vocabulary, its own narrowing rule, and
its own downgrade ordering. The vocabularies are drawn from the
upstream schemas above; this matrix composes them, it does **not**
mint parallel vocabularies.

| Axis | Vocabulary source | Values | Narrowing rule |
|---|---|---|---|
| Lifecycle state | `capability_lifecycle.schema.json#/$defs/lifecycle_state` | `labs`, `preview`, `beta`, `stable`, `lts_facing`, `deprecated`, `disabled_by_policy`, `retired` | Monotonic meet of declared state and every live dependency marker's contribution. `gates_entire_capability` absorbs to `disabled_by_policy` or `retired`. |
| Support class | `capability_lifecycle.schema.json#/$defs/support_class` | `best_effort`, `community_supported`, `standard_support`, `extended_support`, `operator_only_support`, `no_support` | Narrowed by markers whose `effect_on_parent` is `narrows_effective_support_class`. Narrowed value MUST NOT exceed declared value. |
| Release channel | `capability_lifecycle.schema.json#/$defs/release_channel` | `nightly_channel`, `experimental_channel`, `preview_channel`, `stable_channel`, `lts_channel`, `managed_only_channel` | Narrowed by markers whose `effect_on_parent` is `narrows_effective_release_channel` (for example, `managed_only_dependency`). Widening above declared channel is forbidden. |
| Client scope | `capability_lifecycle.schema.json#/$defs/client_scope` | `desktop_product`, `cli`, `companion_surface`, `remote_agent`, `sdk_or_api`, `managed_admin_surface` | Set subtraction by live `client_scope_restricted_dependency` markers. Empty set absorbs lifecycle into `disabled_by_policy` or `retired`. |
| Freshness class | `capability_lifecycle.schema.json#/$defs/freshness_class` | `authoritative_live`, `warm_cached`, `degraded_cached`, `stale`, `unverified` | Worst-supporting-truth-wins composition per `claim_propagation_rules.yaml`. An owner outside its refresh window MUST NOT render `authoritative_live`. |
| Certified-archetype status | `compatibility_row.schema.json#/$defs/current_state` | `not_applicable`, `supported`, `best_effort`, `untested`, `degraded`, `unsupported` | Derived from the linked `compatibility_row` / `certified_archetype_report` row. A stale backing row narrows through `supported` → `best_effort` → `degraded`. A capability with no compatibility-row coverage renders `not_applicable`. |
| Dependency markers | `capability_lifecycle.schema.json#/$defs/marker_kind` | `no_live_markers`, `non_stable_capability_dependency`, `disabled_by_policy_dependency`, `deprecated_dependency`, `retired_dependency`, `provider_linked_dependency`, `client_scope_restricted_dependency`, `freshness_floor_dependency`, `kill_switch_dependency`, `managed_only_dependency` | Every live marker renders with `marker_kind`, `effect_on_parent`, `reason_code`, and `repair_hook_ref`. Absence renders the explicit `no_live_markers` sentinel. |

### Labeling rules (MUST)

1. **No collapses.** A surface MUST NOT collapse two axes into one
   label. For example, a "stable" chip that asserts both
   `lifecycle_state = stable` and `support_class = standard_support`
   without naming both tokens is non-conforming.
2. **Effective vs declared.** When `effective_lifecycle_state`,
   `effective_support_class`, `effective_release_channel`, or
   `effective_client_scopes` differ from their declared values, the
   surface renders the **effective** value and cites the narrowing
   marker. A declared-only badge over a narrower effective state
   overclaims.
3. **Mapping between support-class vocabularies.** The compatibility
   row schema uses a parallel four-value support_class vocabulary
   (`certified` / `supported` / `community` / `experimental`). A
   compatibility or certified-archetype report that quotes a
   capability-lifecycle `support_class` outside the frozen mapping
   (see `capability_badge_axes.yaml#compatibility_row_support_class_mapping`)
   MUST cite the mapping in the row `notes` field or open a
   decision row.
4. **Compatibility-row `support_class` and the support-class axis
   are separate.** A compatibility row may mark a tuple as
   `certified` / `supported` (compat) while the capability-lifecycle
   `support_class` axis renders `standard_support`; the certified-
   archetype axis carries the compat verdict and the support axis
   carries the ongoing commitment. They are independent axes.
5. **Dependency markers render, never summarise.** The marker list
   is not optional and is not collapsible into a generic "has
   caveats" chip. Empty means `no_live_markers`.
6. **`unresolved_axis`, not fabricated chips.** Any axis that is
   not yet resolved renders its `unresolved_axis` token and routes
   to the owning repair hook. Synthetic "available" / "ok" / "live"
   chips over unresolved axes are a release-blocking overclaim.
7. **Worst-supporting-axis downgrade.** When any one supporting
   axis goes red or stale, the rendered badge narrows *within that
   axis* and MUST NOT compensate by widening another axis. The
   matrix is intentionally seven-dimensional; there is no scalar
   "overall status" chip.

## Per-axis downgrade ordering

Each axis has its own intra-axis ordering (rightmost is strongest
when rendered, so surfaces pick the leftmost that applies under
narrowing). Ordering across axes is intentionally not defined;
axes are independent.

- **Lifecycle readiness:**
  `labs > preview > beta > stable <= lts_facing`; overlay states
  `deprecated`, `disabled_by_policy`, and `retired` are orthogonal
  and absorb as specified in ADR-0011.
- **Support class:**
  `no_support > best_effort > community_supported > standard_support
  > extended_support`; overlay `operator_only_support` re-projects
  rather than widens.
- **Release channel:**
  `experimental_channel > nightly_channel > preview_channel >
  stable_channel > lts_channel`; overlay `managed_only_channel`
  narrows without widening the base.
- **Freshness class:**
  `unverified > stale > degraded_cached > warm_cached >
  authoritative_live` (worst on the left under narrowing); composes
  with worst-supporting-truth-wins.
- **Certified-archetype status:**
  `unsupported > degraded > untested > best_effort > supported`;
  `not_applicable` is an overlay for capabilities outside the
  archetype program.
- **Client scope:** set subtraction, not a linear ordering.
- **Dependency markers:** unordered; every live marker renders.

See
[`artifacts/governance/capability_badge_axes.yaml#per_axis_downgrade_ordering`](../../artifacts/governance/capability_badge_axes.yaml)
for the machine-readable ordering blocks.

## Badge-propagation rules

Each propagation channel names the axes it MUST render, the axes it
MAY omit, and the forbidden generic chips it fails closed on. A
channel MAY be narrower than the upstream row; it MUST NOT be
broader.

| Channel | Consumer surfaces | Required axes | Conditional axes | Forbidden generic chips |
|---|---|---|---|---|
| `product_surface` | settings row, bundle card, command palette entry, install review step, provider-linked feature card, extension capability row | Lifecycle, Support, Channel, Client scope, Dependency markers | Freshness (when capability truth class is runtime-observed or derived-indexed); Certified archetype (when capability has a linked compatibility row) | "available", "unavailable", "temporarily unavailable", "having trouble", "beta", "coming soon", "stable" (as a lone chip) |
| `docs` | docs pane, docs browser, generated reference | Lifecycle, Support, Channel, Freshness, Client scope, Dependency markers | Certified archetype (when the page quotes a certified-archetype or compatibility row) | "out of date", "current", "available", "unavailable" |
| `support_export` | support summary, support bundle, project-doctor packet | All seven axes | — | "error", "failed", "unavailable", "try again" |
| `claim_manifest` | claim-manifest packet | Lifecycle, Support, Channel, Client scope, Freshness, Dependency markers | Certified archetype (when the claim row cites a compatibility or certified-archetype row) | "error", "temporarily unavailable" |
| `compatibility_report` | compatibility report packet, certified-archetype report packet | All seven axes | — | "certified" (as a bare chip), "supported" (as a bare chip), "failing", "works" |
| `about_service_health` | Help/About, service-health, About packet | Lifecycle, Support, Channel, Client scope, Freshness, Dependency markers | Certified archetype (when row is a certified archetype or launch-wedge) | "error", "failure", "unavailable", "having trouble", "try again", "healthy", "all good" |

### Worst-supporting-axis downgrade behaviour

Surfaces render the **effective** value per axis. When a supporting
axis narrows, downstream channels preserve the narrowing rather
than compensating by widening an adjacent axis.

- **Support narrowing.** `effective_support_class` narrower than
  declared → product surfaces render the effective class and a
  repair hook; docs cite the narrowing marker; support exports
  preserve the effective class and marker list; claim manifests
  downgrade `effective_claim_posture` to `limited` when support
  falls below the claim floor; compatibility reports narrow
  `current_state` toward `best_effort` or `degraded`; About and
  service-health render the effective class and name the marker.
- **Freshness falls to stale.** Product surfaces render the
  freshness badge and refresh hook; docs render the stale-pack
  disclosure and refresh route; support exports preserve the
  captured freshness class and timestamp; claim manifests route
  the `stale_evidence_inputs` active downgrade reason;
  compatibility reports set `freshness_state` to `caveated` or
  `retest_pending`; About / service-health render the typed
  degraded-state token (per `truth_class_matrix.yaml`) with the
  typed cause.
- **Certified-archetype backing row stale.** Product surfaces
  narrow the certified badge to `best_effort` and cite the stale
  row; docs narrow the certified wording and link the stale row;
  support exports preserve the narrowed status and retest route;
  claim manifests downgrade `effective_claim_posture` to
  `limited` with the compatibility-row cite; compatibility
  reports narrow `current_state` through `supported` →
  `best_effort` → `degraded`; About/service-health render
  `not_applicable` when the surface does not carry archetype
  claims.
- **Dependency marker minted.** Product surfaces render every
  live marker with its effect and repair hook; docs render the
  marker narrative and cite the repair hook; support exports
  preserve every marker; claim manifests attach the marker to
  `active_downgrade_reasons` when it narrows the claim;
  compatibility reports attach the marker to `known_deviations`
  when it narrows the row; About/service-health render the
  marker chip adjacent to the service-contract state.
- **Client scope excluded.** Product surfaces render a tombstone
  with the `client_scope_excludes_surface` reason; docs render a
  redirect to the supported surface; support exports preserve
  the excluded scope; claim manifests refuse the claim render;
  compatibility reports narrow `claimed_deployment_profiles`;
  About/service-health render a redirect rather than a generic
  "unavailable" chip.
- **Kill switch tripped.** Product surfaces absorb lifecycle
  into `disabled_by_policy` and cite the kill switch; docs
  render `PolicyBlocked` and cite the source; support exports
  preserve the kill-switch source and policy context; claim
  manifests force `effective_claim_posture` to `policy_disabled`;
  compatibility reports narrow `current_state` to `unsupported`
  for affected profiles; About/service-health render
  `PolicyBlocked` with the explainer route.

See
[`artifacts/governance/capability_badge_axes.yaml#downgrade_propagation_rules`](../../artifacts/governance/capability_badge_axes.yaml)
for the machine-readable effect table.

## Forbidden cross-axis combinations

The matrix exists to prevent ambiguous label pairings. Every
combination below is **machine-checkable**: a rendered badge that
produces the pair is non-conforming and fails drift-blocking rules.

| Forbidden combination | Why it is forbidden |
|---|---|
| `lifecycle = stable` + `support_class = no_support` | A stable capability with no support is a contradiction. Either narrow support to `best_effort`/`community_supported` and render it, or narrow lifecycle to `deprecated` with a deprecation window and replacement. |
| `lifecycle = retired` + `freshness_class = authoritative_live` | Retired overrides freshness; the row renders the retired badge and cites `replacement_ref`. |
| `lifecycle = disabled_by_policy` + `dependency_markers = no_live_markers` | A `disabled_by_policy` row MUST carry a typed `disabled_by_policy_reason` and at least one live marker (or a tripped kill switch) citing the source. Silent `disabled` chips are forbidden. |
| `lifecycle = preview` + `certified_archetype_status = supported` | `certified_archetype_status = supported` requires `lifecycle ≥ beta`. Preview capabilities render `best_effort`, `untested`, or `not_applicable`. |
| `lifecycle = labs` + `certified_archetype_status > untested` | Labs is below the certified-archetype floor; overclaiming narrows publication rights. |
| `freshness = authoritative_live` + live `freshness_floor_dependency` marker | The marker narrows `effective_freshness_class`; the badge renders `warm_cached`, `degraded_cached`, `stale`, or `unverified`. |
| Rendering surface not in `effective_client_scopes` + any positive lifecycle token | The capability renders a tombstone with `client_scope_excludes_surface`. |
| `kill_switch_state = tripped` + lifecycle not in `{disabled_by_policy, retired}` | Tripped kill switches MUST absorb lifecycle into `disabled_by_policy`. |
| Live `retired_dependency` marker + lifecycle not in `{retired, disabled_by_policy, deprecated}` | A retired dependency gates the parent; the parent MUST narrow. |
| `release_channel = managed_only_channel` on unmanaged client scopes without managed-admin context | The surface renders a tombstone citing `managed_only_channel_required`. |
| `lifecycle = deprecated` without `deprecation_window` | Non-conforming per ADR-0011 / `capability_lifecycle.schema.json`. |
| Any freshness below `authoritative_live` + copy asserting "current" / "up to date" / "live" | Copy overclaim; surfaces render the typed freshness token and the refresh route. |
| Single chip collapsing two or more axes into "available", "unavailable", "healthy", "having trouble", "on", "off", "beta", "stable" | Ambiguous single-status chips are the exact pattern this matrix exists to prevent. |

See
[`artifacts/governance/capability_badge_axes.yaml#forbidden_combinations`](../../artifacts/governance/capability_badge_axes.yaml)
for the full machine-checkable set, including the axes-active
predicates and required repair-hook kinds.

## Seed combinations

The seed fixtures under
[`fixtures/governance/badge_combinations/`](../../fixtures/governance/badge_combinations)
cover allowed combinations, forbidden combinations, and
downgrade-propagation behaviour so conformance checkers, docs,
support exports, claim manifests, and compatibility reports can
be validated against one shared corpus. Each fixture names the
scenario, the expected badge axes, the worst-supporting-axis
outcome per channel, and (for forbidden rows) the expected
denial reason.

Seeded fixtures:

- `stable_capability_authoritative_live.yaml` — allowed: stable,
  standard-support, stable-channel capability with no markers.
- `preview_capability_warm_cached_with_marker.yaml` — allowed:
  preview capability with a `non_stable_capability_dependency`
  marker narrowing support to `best_effort`.
- `deprecated_capability_with_replacement.yaml` — allowed:
  deprecated row carrying a populated `deprecation_window` and
  `replacement_ref`.
- `disabled_by_policy_kill_switch_tripped.yaml` — allowed:
  kill-switch-tripped absorption with a populated disabled
  reason.
- `managed_only_channel_on_managed_admin_surface.yaml` — allowed:
  managed-only capability rendered on the managed-admin surface.
- `client_scope_excluded_on_cli.yaml` — allowed: desktop-only
  capability rendering a typed tombstone on the CLI surface.
- `certified_archetype_supported_row.yaml` — allowed: certified
  archetype with fresh backing rows.
- `certified_archetype_backing_row_stale.yaml` — downgrade:
  stale reference-workspace row narrows the archetype to
  `best_effort` and the claim posture to `limited`.
- `forbidden_stable_with_no_support.yaml` — forbidden: stable
  capability rendered with `no_support` and no narrowing marker.
- `forbidden_ambiguous_available_chip.yaml` — forbidden: single
  "available" chip collapsing multiple axes.

## Acceptance checklist

A reviewer auditing a protected badge can confirm conformance in
three passes:

1. **Axis-presence pass.** Open the badge. Can you read one value
   per axis (or the `unresolved_axis` token and a repair hook) for
   every required axis on the rendering channel? If any axis is
   unrendered, the badge fails acceptance.
2. **Narrowing pass.** For every axis where the effective value
   differs from the declared value, is the narrowing marker
   cited? Is the downgrade-propagation effect honoured (docs
   narrate the marker, support preserves it, claim manifests
   attach it to `active_downgrade_reasons`, etc.)? If not, the
   badge fails acceptance.
3. **Forbidden-combination pass.** Run the rendered axes against
   the forbidden-combination set in
   [`artifacts/governance/capability_badge_axes.yaml#forbidden_combinations`](../../artifacts/governance/capability_badge_axes.yaml).
   Any match is a fail-closed finding under
   [`docs/governance/drift_blocking_rules.md`](./drift_blocking_rules.md).

## Out of scope

Per the originating spec, this matrix freezes the axis set, the
badge-propagation rules, and the forbidden cross-axis
combinations. **Public marketing copy generation is out of
scope.** Wiring every live surface to the matrix is pulled
forward through each surface's own freeze, referencing this file
and the machine-readable companion.
