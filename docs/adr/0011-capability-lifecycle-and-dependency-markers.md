# ADR 0011 — Capability lifecycle and dependency-marker vocabulary

- **Decision id:** D-0017 (see `artifacts/governance/decision_index.yaml#D-0017`)
- **Status:** Accepted
- **Decision date:** 2026-04-19
- **Freeze deadline:** 2026-09-15
- **Owner:** `@ahmedyounis`
- **Backup owner:** `null` (covered by waiver `single-maintainer-backup` in `artifacts/governance/ownership_matrix.yaml#waivers`)
- **Forum:** product_scope_review
- **Related requirement ids:** none

## Context

Every product surface that offers a capability also implies a
readiness claim: a settings row offers a control; a bundle card
offers an install; a command palette entry offers a command; a
docs page offers a how-to; a support export dumps an observed
feature; a claim manifest publishes a capability to a downstream
consumer; a future SDK or API surface binds a consumer contract.
Without one controlled vocabulary, each surface invents its own
version of "experimental", "preview", "new", "coming soon",
"unsupported", "deprecated", "disabled", "turned off", or
"managed-only", and the reviewer, the support-export consumer,
the docs reader, and the release-council promoter each see a
different cut. Worse, when a capability that renders as "stable"
depends on a sub-capability that is still in preview, or on a
provider whose connection is admin-disabled, or on a freshness
floor the surface cannot currently satisfy, the surface silently
inherits the unstable posture without anyone being able to
point at the dependency.

The source documents
(`.t2/docs/Aureline_PRD.md`,
`.t2/docs/Aureline_Technical_Architecture_Document.md`,
`.t2/docs/Aureline_Technical_Design_Document.md`,
`.t2/docs/Aureline_Milestones_Document.md`) treat lifecycle
posture, support class, release channel, freshness, and client
scope as first-order product contracts that MUST stay separate
even when a single UI chip collapses them. The boundary
manifest at `docs/product/boundary_manifest_strawman.md` and
the settings resolver frozen in ADR 0008 already name settings,
capabilities, and residual dependencies; the connected-provider
vocabulary frozen in ADR 0010 already names actor classes and
grant-resolution reasons. What is still missing is the shared
vocabulary that answers, for any capability on any surface:
how ready is it, what support is committed, which channel ships
it, how fresh is the view, which client classes surface it, and
— critically — which unstable, disabled, deprecated, retired,
or narrowed sub-capabilities the surface is silently inheriting.

The freeze matters now, ahead of the badge, compatibility,
claim-manifest, rollout, support-export, and docs lanes
landing: if those lanes proliferate before a shared capability-
lifecycle and dependency-marker vocabulary is frozen, each will
invent its own cut (the settings resolver will say "preview"
while the docs page says "beta"; the support export will say
"GA" while the claim manifest says "experimental"; a policy-
disabled row will read as "unavailable" on one surface and as a
regular disabled toggle on another), and downstream badge /
compatibility / claim-propagation tooling will have nothing
mechanical to consume. This ADR closes `D-0017` (capability
lifecycle and dependency-marker vocabulary) so the settings,
bundles, command / install review, docs, support-export,
release-evidence, and claim-manifest lanes can instrument
against one contract.

This ADR rides alongside the ADR-0008 settings resolver (the
`lifecycle_label`, `control_stack`, and `capability_dependencies`
fields on the effective-setting record are projections of the
vocabulary frozen here), the ADR-0010 connected-provider
vocabulary (a provider-linked dependency marker references the
grant-resolution reason set and actor-class set frozen there),
the ADR-0001 identity-mode envelope (the managed / self-hosted /
account-free-local identity modes gate the `managed_only_channel`
release channel and the `managed_admin_surface` client scope),
the ADR-0004 RPC transport (lifecycle rows and dependency
markers cross as typed payloads; raw channel tokens and raw
provider scopes never do), and the ADR-0005 subscription
envelope (lifecycle views ride the shared envelope with
authority class `derived_knowledge` on projected surfaces).
This ADR does not redefine those contracts; it defines the
lifecycle-specific and dependency-marker-specific fields they
refer to.

Automated claim publication and full rollout tooling are
explicitly out of scope at this milestone; this freeze
establishes the vocabulary and invariants those later flows
will honour.

## Decision

Aureline freezes five **orthogonal axes** —
`lifecycle_state`, `support_class`, `release_channel`,
`freshness_class`, and `client_scope` — that every capability
row carries independently even when a UI chip collapses them
visually. Aureline also freezes a **dependency-marker record**
that names every unstable, disabled, deprecated, retired, or
narrowed sub-capability a parent flow depends on, a
**downgrade rule** that computes the parent's effective
lifecycle posture from its own state and its dependencies'
states, and a set of **projection requirements** for settings
rows, bundle cards, command / install review, docs / help, and
support / export surfaces so lifecycle posture stays visible
and distinct from support, freshness, channel, and client
scope.

Every capability rendered on a protected surface — settings
row, bundle card, command palette entry, install review step,
docs / help page, support / export row, claim-manifest entry,
and future SDK / API surface — resolves to exactly one
`capability_lifecycle_row_record`, exactly one
`effective_lifecycle_state`, exactly zero-or-more
`dependency_marker_record`s, and exactly one
`projection_envelope` per surface it renders on.

All rules below are stated in terms of contract, vocabulary,
and event names rather than specific crates so surface
changes are hygiene, not re-litigation.

### Orthogonal axes (frozen)

The five axes are independent. A surface MAY render them
together on one chip but MUST preserve them separately in its
underlying record; a surface that collapses two axes in its
record (for example, by renaming a channel `preview_channel`
and using the same token on the `lifecycle_state` field) is
non-conforming.

| Axis                 | Frozen vocabulary                                                                                                                                | What it answers                                                                                                 |
|----------------------|--------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------|
| `lifecycle_state`    | `labs`, `preview`, `beta`, `stable`, `lts_facing`, `deprecated`, `disabled_by_policy`, `retired`                                                 | How ready is the capability itself?                                                                             |
| `support_class`      | `best_effort`, `community_supported`, `standard_support`, `extended_support`, `operator_only_support`, `no_support`                              | What support is committed for this capability?                                                                  |
| `release_channel`    | `nightly_channel`, `experimental_channel`, `preview_channel`, `stable_channel`, `lts_channel`, `managed_only_channel`                            | Which channel delivers the capability to users?                                                                 |
| `freshness_class`    | `authoritative_live`, `warm_cached`, `degraded_cached`, `stale`, `unverified`                                                                    | How recently was the capability row refreshed from its canonical owner?                                         |
| `client_scope`       | `desktop_product`, `cli`, `companion_surface`, `remote_agent`, `sdk_or_api`, `managed_admin_surface`                                             | Which client classes surface this capability?                                                                   |

Rules (frozen):

1. A capability row MUST name exactly one value per axis (or
   a set of `client_scope` values, for capabilities that
   surface on more than one client class). Silent blanks are
   forbidden; a surface that cannot resolve an axis MUST
   render `unresolved_axis` and route to a repair hook
   rather than fall back to a generic "available" chip.
2. A surface MUST NOT use `lifecycle_state` tokens as
   `release_channel` tokens or vice versa. `preview` is a
   readiness claim; `preview_channel` is a distribution
   route. A capability MAY live on `stable_channel` while its
   `lifecycle_state` is `preview` (stable channel carries it
   under the preview posture) and MAY live on
   `preview_channel` while its `lifecycle_state` is `stable`
   (stable capability carried in a preview distribution).
3. `support_class` is the SLO axis. A capability's
   `lifecycle_state = stable` does not imply
   `support_class = standard_support`; managed-only offerings
   may carry `support_class = operator_only_support` even at
   `lifecycle_state = stable`.
4. `freshness_class` is the view axis. A surface that renders
   a lifecycle row MUST name the `freshness_class` of the row
   it is rendering; a row whose canonical owner could not be
   reached for re-verification renders with
   `freshness_class = degraded_cached`, `stale`, or
   `unverified` as appropriate and MUST NOT silently render
   as `authoritative_live`.
5. `client_scope` is the surface axis. A capability whose
   `client_scope` excludes the current surface MUST NOT
   render as available on that surface; at most it renders
   as a tombstone with a typed reason pointing the user at
   the client class that does surface it.

Adding a value to any axis is additive-minor and bumps
`capability_lifecycle_schema_version`; repurposing a value is
breaking and requires a new decision row.

### Lifecycle-state class definitions (frozen)

Each `lifecycle_state` carries a fixed meaning that every
surface renders against.

| State                  | Meaning                                                                                                                                        | Visibility default                               | May be used on `stable_channel`? |
|------------------------|------------------------------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------|----------------------------------|
| `labs`                 | Early experimentation. API unstable; may be removed without deprecation window. Opt-in only.                                                   | Hidden by default; surface only when user opts in. | No (experimental / preview only). |
| `preview`              | Broadly testable. API still unstable; breaking changes announced but not guaranteed to keep a grace window. Support downgraded.               | Visible; chip reads `preview`; action gated.     | Allowed under preview posture.    |
| `beta`                 | Feature-complete within a bounded scope. Breaking changes require a deprecation window. Support still not full.                                | Visible; chip reads `beta`.                       | Allowed under beta posture.       |
| `stable`               | Committed. Breaking changes require a full deprecation window plus a supersession decision row.                                                | Visible; no lifecycle chip required by default.   | Yes.                               |
| `lts_facing`           | Stable with extended-support obligations: longer deprecation window, longer back-port window, narrower breaking-change admissibility.         | Visible; chip reads `lts` where applicable.       | Yes (also `lts_channel`).         |
| `deprecated`           | Still present and functional but scheduled for removal. Surfaces MUST show the deprecation window and the replacement path.                    | Visible; chip reads `deprecated` with horizon.    | Yes, with chip.                   |
| `disabled_by_policy`   | Functionality exists but is gated off by admin policy, feature flag kill-switch, managed-only narrowing, or identity-mode-required cap.        | Visible as a disabled row with a typed reason.    | N/A (policy ceiling applies).     |
| `retired`              | Removed from the product. Surfaces that still reference it render a tombstone plus a link to the replacement or to the retirement notice.     | Visible as a tombstone; action denied.            | No.                                |

Rules (frozen):

1. A capability MUST transition through the states monotonically
   except for two admissible moves: `preview` -> `labs` when
   evidence shows the capability is not yet broadly testable
   (announced and logged on the `capability_lifecycle` audit
   stream), and any non-`retired` state -> `disabled_by_policy`
   (which is orthogonal to the readiness progression).
2. A capability that reaches `retired` MUST NOT be brought
   back under the same identity; a new identity is minted and
   the retired row is preserved for history. Reviving a
   capability under the same row is breaking and requires a
   new decision row.
3. A capability in `disabled_by_policy` MUST render with a
   typed disable reason and a repair hook (admin-policy,
   feature-flag kill-switch, managed-only narrowing,
   identity-mode-required cap, unmet capability dependency);
   silent removal is forbidden on every product, CLI,
   support-export, docs, and review-overlay surface.
4. A capability in `deprecated` MUST name a
   `deprecation_window` (dates or milestone refs), a
   `replacement_ref` (null only when retirement is outright),
   and a `migration_hint_ref`. A surface that renders a
   deprecated row without these fields is non-conforming.

### Dependency-marker record (frozen)

Dependency markers exist so that a stable parent whose
sub-capability is unstable, disabled, deprecated, retired, or
narrowed cannot silently inherit those postures. A marker is
the typed answer to "which sub-capability reduced the parent's
effective posture, and why?".

A `dependency_marker_record` carries:

- `dependency_marker_id` — opaque stable id safe to log.
- `dependency_marker_schema_version` — integer, pinned.
- `marker_kind` — one of
  `non_stable_capability_dependency`,
  `disabled_by_policy_dependency`,
  `deprecated_dependency`,
  `retired_dependency`,
  `provider_linked_dependency`,
  `client_scope_restricted_dependency`,
  `freshness_floor_dependency`,
  `kill_switch_dependency`,
  `managed_only_dependency`.
- `parent_capability_ref` — id of the capability the marker
  is attached to.
- `dependency_capability_ref` — id of the sub-capability the
  marker describes. Required when `marker_kind` references a
  capability; set to the provider-record id when
  `marker_kind = provider_linked_dependency`.
- `dependency_lifecycle_state` — one of the frozen
  lifecycle states; names the sub-capability's current state.
  For `provider_linked_dependency` this carries the effective
  grant resolution ("`allowed_with_downgrade`" implies
  `preview`-equivalent posture on the parent; see table below).
- `dependency_support_class`,
  `dependency_release_channel`,
  `dependency_freshness_class`,
  `dependency_client_scope` — the sub-capability's values on
  each orthogonal axis. Null only for provider-linked markers
  where the axis does not apply.
- `effect_on_parent` — one of
  `narrows_effective_lifecycle_state`,
  `narrows_effective_support_class`,
  `narrows_effective_release_channel`,
  `narrows_effective_freshness_class`,
  `narrows_effective_client_scope`,
  `gates_entire_capability` (parent becomes
  `disabled_by_policy` or `retired` because the dependency
  is).
- `reason_code` — one of
  `dependency_state_below_parent_declared_state`,
  `dependency_disabled_by_policy`,
  `dependency_kill_switch_tripped`,
  `dependency_deprecated_within_window`,
  `dependency_retired`,
  `provider_grant_narrowed`,
  `provider_connection_unhealthy`,
  `client_scope_excludes_surface`,
  `freshness_floor_unmet`,
  `managed_only_channel_required`.
- `repair_hook_ref` — stable ref to the recovery path a user
  or admin MUST be able to reach (enable-flag, request-
  provider-link, upgrade-channel, re-verify-freshness,
  migrate-to-replacement, request-managed-access). Null
  markers without a repair hook are forbidden.
- `disclosure_summary` — human-legible paragraph naming the
  sub-capability, the reduced axis, and the user-visible
  consequence. A surface that renders a marker without a
  disclosure summary is non-conforming.
- `declared_at` — monotonic timestamp the marker was minted;
  markers older than their parent's
  `freshness_class = stale` threshold auto-refresh or emit
  `dependency_marker_stale_not_refreshed`.
- `policy_context` — `policy_epoch`, `trust_state`,
  `execution_context_id` (from ADR-0008 / ADR-0009 / ADR-0001
  where applicable).
- `redaction_class` — declared redaction class for the marker
  on logs, traces, support bundles, claim manifests, and
  mutation-journal entries.

Rules (frozen):

1. A surface that renders a capability row MUST also render
   every live dependency marker attached to it. Silent hiding
   of dependency markers on protected surfaces (settings,
   bundle cards, command / install review, docs / help,
   support / export, claim manifests) is forbidden.
2. Dependency markers MUST NOT be collapsed into a generic
   "has caveats" badge; each marker renders its `marker_kind`,
   its `effect_on_parent`, and its `disclosure_summary`.
3. Adding a `marker_kind` or a `reason_code` is additive-minor;
   repurposing a value is breaking and requires a new
   decision row.
4. Dependency markers for `provider_linked_dependency` MUST
   reuse the grant-resolution-reason set frozen in ADR-0010
   rather than mint parallel vocabulary.
5. A dependency marker whose `repair_hook_ref` has no live
   target MUST degrade to
   `reason_code = dependency_kill_switch_tripped` and emit
   `dependency_marker_repair_unavailable` rather than pretend
   the marker is repairable.

### Downgrade rule (frozen)

A parent capability's **declared** lifecycle state is what the
owner asserts. Its **effective** lifecycle state is computed
at render time as the monotonic meet of the declared state and
every live dependency marker's contribution:

```
effective_lifecycle_state =
    min_readiness(
        declared_lifecycle_state,
        min_readiness(
            dependency.dependency_lifecycle_state
            for dependency in live_dependency_markers
            if dependency.effect_on_parent ==
                narrows_effective_lifecycle_state
        )
    )
```

Where `min_readiness` is the monotonic ordering
`labs < preview < beta < stable <= lts_facing`, and
`disabled_by_policy` / `retired` are absorbing: if any live
dependency marker contributes `disabled_by_policy`, the
parent's effective state becomes `disabled_by_policy`; if any
contributes `retired`, the parent's effective state becomes
`retired` (and the parent SHOULD open its own retirement
decision row). The same meet applies, axis by axis, to
`support_class`, `release_channel`, `freshness_class`, and
`client_scope` against their dependency-marker narrowings.

Rules (frozen):

1. Every surface that renders a capability row MUST render
   `effective_lifecycle_state` rather than the
   declared state when the two differ. A surface that
   renders only the declared state while a live dependency
   marker narrows it is non-conforming.
2. `declared_lifecycle_state` remains visible as metadata
   (so a reviewer can see where the row would sit without
   its dependencies); it is not what the user-facing chip
   reads.
3. The effective state is recomputed on every render; it is
   not cached past its `freshness_class` window without a
   re-verify.
4. A parent MAY carry
   `effect_on_parent = gates_entire_capability` markers; in
   that case the capability MUST render as a disabled or
   tombstoned row regardless of its declared state.
5. Downgrade is asymmetric: a parent whose declared state is
   `preview` does NOT get promoted to `stable` by a stable
   dependency. The declared state is an upper bound; the
   dependency markers only narrow it.

### Projection requirements per surface (frozen)

Each surface MUST project the lifecycle axes and dependency
markers into its record using the fields below. A surface that
renders a capability row without the required projection is
non-conforming. `lifecycle_state`, `support_class`,
`release_channel`, `freshness_class`, `client_scope`, and
dependency markers MUST remain separately addressable in the
record even when the surface's chip collapses them visually.

| Surface                              | Required projected fields                                                                                                                                                                                   | Required dependency-marker treatment                                                                                                          |
|--------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------|
| Settings row                         | `lifecycle_state`, `effective_lifecycle_state`, `support_class`, `release_channel`, `freshness_class`, `client_scope`, `lifecycle_label` (renderer hint)                                                   | Every live marker renders inline under the row with its `marker_kind`, `disclosure_summary`, and `repair_hook_ref`; silent hiding forbidden.  |
| Bundle card                          | All five axes plus `install_gating_summary` (which lifecycle + dependency facts affect install eligibility)                                                                                                 | Markers gating install render as a blocking sub-panel; non-blocking markers render as a visible caveat strip.                                 |
| Command / install review             | All five axes plus `proposed_effect_summary` (what the command will do, under which effective state, under which dependency markers)                                                                        | Every live marker renders in the review's typed disclosure list; a review with hidden markers is denied with `review_disclosure_incomplete`. |
| Docs / help page                     | All five axes plus `replacement_ref` (when `deprecated`) and `since_label` (channel / milestone where the capability reached its declared state)                                                            | Markers render as typed call-outs with links to the sub-capability's docs; unresolved markers render a visible gap rather than a generic note.|
| Support / export                     | All five axes plus `declared_lifecycle_state`, `effective_lifecycle_state`, `dependency_marker_count`, `kill_switch_state`, and a per-marker enumeration under a redaction envelope                         | Every live marker is exported with `marker_kind`, `reason_code`, `disclosure_summary`, and `policy_context` (redaction class applied).        |
| Claim manifest (reserved)            | All five axes plus `claim_subject`, `claim_horizon`, and the full list of dependency markers that affect the claim                                                                                          | Claims whose computed effective state is below the declared claim state are refused at publish time with `claim_effective_state_below_declared`. |
| Future SDK / API surface (reserved)  | All five axes plus `binding_kind` (`source_binding`, `wire_binding`, `capability_binding`) and the stability-window label bound by the axis set                                                             | Markers narrowing SDK availability render as typed `sdk_dependency_marker` entries and contribute to the compatibility-propagation record.    |

Rules (frozen):

1. Chip collapsing is a UI freedom, not a record-shape
   freedom. A surface that folds all five axes into one chip
   MUST keep the five fields separately addressable in its
   underlying record so tooling, support bundles, and claim
   propagation can read each axis independently.
2. `disabled_by_policy` and `retired` MUST render with a
   visible chip and a typed repair hook on every surface;
   silent disablement or silent tombstoning is forbidden.
3. Dependency markers MUST be counted and enumerated in
   every support export; a support bundle that reports a
   capability without its marker count is non-conforming.
4. A surface that cannot name a required projected field
   MUST render `unresolved_axis` and route to a repair hook.
5. The `freshness_class` projected to a surface MUST be at
   most as fresh as the surface's own refresh posture; a
   surface cannot claim `authoritative_live` for rows whose
   canonical owner it has not contacted since its last
   refresh window.

### Audit events (frozen)

Every lifecycle transition, every dependency-marker mint /
update / clear, and every disable or kill-switch flip emits
a structured event on the `capability_lifecycle` audit stream.
Events carry the capability id, the surface id, the observer
subject, the axis affected, the previous and next values,
and a typed reason where relevant. Events MUST NOT carry raw
provider tokens, raw policy-bundle bytes, or raw
kill-switch-material.

| Event id                                           | Fires when                                                                                      |
|----------------------------------------------------|-------------------------------------------------------------------------------------------------|
| `capability_lifecycle_row_created`                 | A capability row is minted in the registry.                                                     |
| `capability_lifecycle_row_updated`                 | Any axis (`lifecycle_state`, `support_class`, `release_channel`, `freshness_class`, `client_scope`) changes. |
| `capability_lifecycle_row_deprecated`              | Row transitions to `deprecated` (names `deprecation_window` and `replacement_ref`).             |
| `capability_lifecycle_row_retired`                 | Row transitions to `retired`.                                                                   |
| `capability_lifecycle_row_disabled_by_policy`      | Row transitions to `disabled_by_policy` (names reason code and policy-bundle ref).              |
| `capability_lifecycle_row_re_enabled`              | Row leaves `disabled_by_policy` (never reached from `retired`).                                 |
| `dependency_marker_minted`                         | A dependency marker is attached to a capability row.                                            |
| `dependency_marker_updated`                        | A dependency marker's lifecycle or effect on parent changes.                                    |
| `dependency_marker_cleared`                        | A dependency marker is cleared because the dependency resolved back to parent's declared state. |
| `dependency_marker_repair_unavailable`             | A marker's `repair_hook_ref` has no live target.                                                |
| `dependency_marker_stale_not_refreshed`            | A marker crossed its freshness threshold without refreshing.                                    |
| `effective_lifecycle_state_narrowed`               | Render-time meet produced an effective state below the declared state.                          |
| `effective_lifecycle_state_absorbed`               | Render-time meet produced `disabled_by_policy` or `retired` because a marker absorbed parent.   |
| `kill_switch_tripped`                              | A kill switch flipped a capability or a marker.                                                 |
| `lifecycle_claim_refused`                          | A claim-manifest publish was refused because effective state was below declared claim state.    |
| `capability_lifecycle_schema_version_bumped`       | `capability_lifecycle_schema_version` was bumped.                                               |

### Denial posture (frozen)

When a surface cannot render a lifecycle row safely it
denies. Denial is typed, visible, auditable, and repairable.
Silent downgrade to a generic "unavailable" chip is forbidden.

The denial-reason set:

- `lifecycle_state_unresolved`
- `axis_value_not_in_vocabulary`
- `declared_state_below_dependency_ceiling`
- `dependency_marker_repair_unavailable`
- `effective_state_absorbed_by_retirement`
- `freshness_floor_unmet`
- `client_scope_excludes_surface`
- `disabled_by_policy_no_repair_hook`
- `kill_switch_tripped`
- `managed_only_channel_required`
- `claim_effective_state_below_declared`
- `review_disclosure_incomplete`

Denials fail closed. They MUST NOT silently retry, MUST NOT
downgrade a capability to a neighbouring axis (a
`client_scope_excludes_surface` denial does NOT auto-route to
another client), and MUST emit the corresponding audit event.

### Process-boundary constraints (frozen)

1. `capability_lifecycle_row_record`s and
   `dependency_marker_record`s cross the RPC boundary as
   typed payloads (ADR-0004). Raw provider tokens, raw
   policy-bundle bytes, and raw kill-switch material never
   cross.
2. The capability registry is authoritative in the host
   process; extensions and AI tool calls read lifecycle rows
   only through the shared subscription envelope (ADR-0005)
   with authority class `derived_knowledge` and a declared
   freshness hint.
3. Remote-agent attach surfaces carry a remote-scoped
   capability view whose client scope is
   `remote_agent`; the host surface renders a
   `client_scope_restricted_dependency` marker for any
   capability that does not surface on the remote agent.
4. Crash dumps and core files MUST NOT inherit unresolved
   lifecycle projections; a crash that lands mid-render
   discards the projection rather than persisting a partial
   axis set.
5. Mutation-journal entries, save manifests, and support
   bundles name `capability_lifecycle_row_id`,
   `effective_lifecycle_state`, and
   `dependency_marker_id`s only; they MUST NOT embed raw
   provider tokens, raw policy-bundle bytes, or raw
   kill-switch material.
6. Claim manifests reference lifecycle rows and dependency
   markers by id; the bodies stay in their respective
   registries so the claim propagator can re-verify rather
   than re-parse a captured snapshot.

### Redaction defaults (frozen)

Every surface that emits observable state declares a
redaction class; the broker-owned redaction pass (ADR-0007)
runs before bytes reach any persistent or exportable sink.

| Surface                              | Default inclusion (lifecycle / dependency-marker fields)                                                                                                                                               |
|--------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `logs_local`                         | `capability_lifecycle_row_id`, `effective_lifecycle_state`, `marker_kind` counts, event ids. Raw policy bytes / raw provider tokens / raw kill-switch material excluded.                               |
| `traces_local`                       | Same as `logs_local`; span names MUST NOT include raw policy identifiers or raw provider host strings beyond canonical hostnames.                                                                     |
| `support_bundle`                     | Full per-axis values, declared vs. effective state, full dependency-marker enumeration with `reason_code`, `repair_hook_ref`, `disclosure_summary`. Raw tokens / raw bundle bytes excluded.            |
| `evidence_packet`                    | Release-relevant lifecycle fields: `declared_lifecycle_state`, `effective_lifecycle_state`, `release_channel`, `support_class`, full marker list. Raw tokens never included.                          |
| `ai_context_capture`                 | `effective_lifecycle_state` and `disclosure_summary` per marker only; no raw repair-hook payloads and no raw policy-bundle bytes.                                                                     |
| `recipe_manifest`                    | `capability_lifecycle_row_id`, `dependency_marker_id`s only. Raw tokens and raw hooks forbidden.                                                                                                      |
| `profile_export` / `sync`            | Same as `recipe_manifest`.                                                                                                                                                                             |
| `crash_dump`                         | Opt-in only; redaction scan precedes packaging; denied by default for rows whose `disabled_by_policy` reason references a managed policy bundle.                                                      |
| `mutation_journal_entry`             | `capability_lifecycle_row_id`, `effective_lifecycle_state`, marker ids, event id. No raw tokens or raw policy material.                                                                               |
| `save_manifest` (ADR-0006)           | Same as `mutation_journal_entry`.                                                                                                                                                                      |
| `claim_manifest`                     | Full per-axis values and full marker enumeration required; manifests refuse publish when effective state is below the declared claim state.                                                           |
| `terminal_transcript`                | Capability ids and marker ids only; raw repair-hook payloads require boundary-labelled confirmation before capture.                                                                                  |

Overrides are narrowing only; admin policy MAY reduce
inclusion further, but MAY NOT widen beyond the frozen
exclusion rules.

### Schema-of-record posture (frozen)

Rust types in the eventual capability-lifecycle crate are the
source of truth. The JSON Schema export at
`schemas/governance/capability_lifecycle.schema.json` is the
cross-tool boundary every non-owning surface reads. Adding a
new lifecycle state, support class, release channel,
freshness class, client scope, marker kind, reason code,
effect-on-parent value, audit-event id, or denial reason is
additive-minor and bumps
`capability_lifecycle_schema_version`; repurposing a value is
breaking and requires a new decision row.

There is no external IDL or code-generator toolchain at this
milestone; this mirrors ADR 0004, ADR 0005, ADR 0006, ADR
0007, ADR 0008, ADR 0009, and ADR 0010.

### Non-goals at this decision

Out of scope until a superseding decision row opens:

- Automated claim publication (the claim-manifest packet
  family already exists as a placeholder; this ADR freezes
  the lifecycle and dependency-marker vocabulary the claim
  propagator will consume but does not implement the
  propagator).
- Full rollout tooling (experiment-rollout bundles,
  progressive-deploy percentage ramps, cohort assignment).
- Badge visual design. The vocabulary here names the chips
  surfaces render; the visual treatment lands with the
  design-system lane.
- Per-surface compatibility-propagation rules beyond the
  projection requirements named here. The compatibility lane
  rides this contract.
- Provider-specific lifecycle mapping tables (mapping
  upstream provider lifecycle labels to Aureline lifecycle
  states). The importer lane rides this contract.
- Live SDK / API surfaces. The vocabulary reserves the
  future surface under `client_scope = sdk_or_api`; the
  binding contract lands with the SDK / API freeze.

These lines move only by opening a new decision row, not by
editing this ADR.

### Tradeoff summary

| Axis                          | Chosen stack                                                                                                                                                   | Best rejected alternative                                               | Why chosen wins                                                                                                                         |
|-------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------|
| **Lifecycle state space**     | Eight explicit states (`labs`, `preview`, `beta`, `stable`, `lts_facing`, `deprecated`, `disabled_by_policy`, `retired`) with frozen transition rules          | Three-state `experimental / stable / deprecated`                        | Three states collapse Labs / preview, LTS / stable, and disabled / retired; support, docs, and claim surfaces cannot tell them apart.  |
| **Axis separation**           | Five orthogonal axes (lifecycle / support / channel / freshness / client scope) kept separately addressable even when the chip collapses them                  | One `status` field with pre-computed combined enum                       | A combined enum hides the axis that actually changed; support exports and claim propagation cannot re-verify what the chip asserts.    |
| **Dependency markers**        | Typed `dependency_marker_record` with kind, reason, effect-on-parent, repair hook, and disclosure summary                                                      | Generic "has caveats" badge plus free-form tooltip                       | A tooltip is not machine-readable; badge lanes, support exports, and claim propagators cannot consume it mechanically.                |
| **Downgrade rule**            | Effective lifecycle state = monotonic meet of declared state and live dependency markers' contributions, recomputed on every render                            | Static declared state, with caveats surfaced out-of-band                 | A static chip lies when a dependency narrows; the meet makes the narrowing visible on the primary chip.                                |
| **Disable / kill-switch**     | `disabled_by_policy` is a first-class lifecycle state with a typed reason code and a required repair hook                                                       | Render disabled rows the same as "not installed"                         | Collapsing disabled and not-installed hides admin policy and kill switches; users and admins cannot tell what repair is possible.     |
| **Schema of record**          | Rust types in the eventual capability-lifecycle crate; JSON Schema export at `schemas/governance/capability_lifecycle.schema.json`                              | External IDL + codegen at this milestone                                | No second-language consumer yet; the JSON Schema export reserves a clean integration point.                                            |
| **Projection discipline**     | Each protected surface declares the fields it projects; chip collapse is a UI freedom, not a record-shape freedom                                              | Let each surface invent its own record shape                             | Inventing per-surface records recreates the drift this ADR is closing.                                                                  |

Each row carries reopen triggers. A support-bundle finding
that a capability renders as `stable` while a live
non-stable dependency marker goes unreported, a docs finding
that a deprecated row lacks a `replacement_ref`, a
support-export finding that `disabled_by_policy` collapsed
into a generic "unavailable" chip, a review finding that a
command / install review hid dependency markers, or a claim-
manifest finding that a publish was accepted with effective
state below declared state reopens the relevant row.

## Consequences

- **Frozen:** the five orthogonal axes — `lifecycle_state`,
  `support_class`, `release_channel`, `freshness_class`,
  `client_scope` — and their frozen value sets. Every
  capability row names exactly one value per axis; silent
  blanks are forbidden.
- **Frozen:** the `lifecycle_state` vocabulary (`labs`,
  `preview`, `beta`, `stable`, `lts_facing`, `deprecated`,
  `disabled_by_policy`, `retired`) with monotonic transition
  rules and two admissible non-monotonic moves
  (preview -> labs announced on the audit stream; any
  non-retired -> disabled_by_policy as an orthogonal move).
- **Frozen:** the `dependency_marker_record` shape — kind,
  parent ref, dependency ref, per-axis values, effect on
  parent, reason code, repair hook, disclosure summary,
  declared-at timestamp, policy context, redaction class —
  and the `marker_kind` / `reason_code` / `effect_on_parent`
  vocabularies.
- **Frozen:** the render-time downgrade rule. Every surface
  renders `effective_lifecycle_state` (and the parallel
  per-axis effective values) rather than the declared state
  when a live dependency marker narrows it.
- **Frozen:** the projection requirements for settings rows,
  bundle cards, command / install review, docs / help,
  support / export, claim manifests, and future SDK / API
  surfaces; chip collapsing is a UI freedom but record
  addressability is mandatory.
- **Frozen:** the audit-event ids on the
  `capability_lifecycle` audit stream and the denial-reason
  set. Silent downgrade to a generic "unavailable" chip is
  forbidden; denials fail closed.
- **Frozen:** process-boundary constraints. Raw provider
  tokens, raw policy-bundle bytes, and raw kill-switch
  material never cross RPC; lifecycle rows and dependency
  markers cross as typed payloads.
- **Frozen:** the schema of record is Rust types in the
  eventual capability-lifecycle crate; the boundary schema
  lives at
  `schemas/governance/capability_lifecycle.schema.json`; no
  external IDL or codegen toolchain at this milestone.
- **Permitted:** adding a new lifecycle state, support
  class, release channel, freshness class, client scope,
  marker kind, reason code, effect-on-parent value,
  audit-event id, or denial reason is additive-minor with a
  schema bump. Repurposing any existing value is breaking
  and requires a new decision row.
- **Permitted:** admin policy MAY narrow lifecycle surfaces
  further — pin a capability to `disabled_by_policy`, forbid
  a release channel, require step-up for marker repair, or
  narrow client scope beyond the declared set. Policy MAY
  NOT silently widen beyond the frozen rules.
- **Permitted:** surfaces MAY collapse the five axes into a
  single chip for dense rendering, provided the underlying
  record retains each axis as a separately addressable field.
- **Follow-up:** the settings, bundle, command / install
  review, docs / help, support / export, release-evidence,
  claim-manifest, and (reserved) SDK / API lanes instrument
  against this vocabulary before claiming lifecycle
  guarantees.
- **Follow-up:** the `lifecycle_label` enum on the ADR-0008
  settings schemas at
  `schemas/settings/setting_definition.schema.json` and
  `schemas/settings/effective_setting.schema.json` currently
  carries the narrower pre-freeze vocabulary
  (`experimental`, `beta`, `stable`, `deprecated`,
  `retired`). A follow-up change aligns those enums to the
  frozen `lifecycle_state` vocabulary here
  (`labs`, `preview`, `beta`, `stable`, `lts_facing`,
  `deprecated`, `disabled_by_policy`, `retired`); the
  settings resolver's rendering hint is a projection of the
  lifecycle vocabulary this ADR freezes.
- **Follow-up:** the badge / compatibility / claim-
  propagation work opened at later milestones consumes this
  vocabulary mechanically rather than re-freezing it.
- **Ratifies:** the ADR-0008 settings resolver's
  `lifecycle_label`, `capability_dependencies`, and
  `control_stack` fields name the vocabulary frozen here.
  The ADR-0010 connected-provider vocabulary supplies the
  grant-resolution-reason set that
  `provider_linked_dependency` markers quote. The ADR-0001
  identity modes gate the `managed_only_channel` release
  channel and the `managed_admin_surface` client scope. The
  ADR-0005 subscription envelope carries lifecycle views as
  `derived_knowledge` projections with declared freshness.

## Alternatives considered

- **Three-state `experimental / stable / deprecated`
  vocabulary.** Rejected: collapses labs and preview, LTS
  and stable, and disabled and retired; support, docs, and
  claim surfaces cannot tell them apart and downstream
  lanes (badge, compatibility, claim propagation) end up
  re-introducing the missing distinctions ad hoc.
- **Single `status` field carrying a pre-computed combined
  enum.** Rejected: hides which axis actually changed when
  the chip flips. Support exports and claim propagators
  cannot re-verify a combined enum; they need the axes
  independently addressable.
- **Generic "has caveats" badge plus free-form tooltip for
  dependency disclosure.** Rejected: free-form text is not
  machine-readable, so badge lanes, support exports, and
  claim propagators cannot consume it mechanically. The
  typed dependency-marker record makes each dependency
  enumerable and repairable.
- **Static declared state, caveats surfaced out-of-band.**
  Rejected: a static chip lies when a dependency narrows.
  Computing effective state as the monotonic meet at render
  time keeps the primary chip honest.
- **Render `disabled_by_policy` the same as "not
  installed".** Rejected: collapsing disabled and
  not-installed hides admin policy and kill switches; users
  and admins cannot tell what repair is possible without a
  typed reason and a repair hook.
- **Let each surface invent its own lifecycle record.**
  Rejected: this ADR exists to stop that drift. Per-surface
  records force downstream badge, compatibility, and claim-
  propagation tooling to write adapters per surface instead
  of reading one vocabulary.
- **External IDL + generator for lifecycle / marker
  payloads.** Rejected: same argument ADR 0004 through ADR
  0010 make — an IDL without a second-language consumer
  costs more than it buys; the JSON Schema export reserves
  the integration point.
- **Defer to a later milestone.** Rejected: the
  default-if-unresolved narrowing on `D-0017`
  (`experimental`/`stable` toggle only, no dependency
  markers, no separate support / channel / freshness /
  client-scope axes, no `disabled_by_policy` state, no
  retired state) would force the settings, bundle,
  command / install review, docs / help, support-export,
  release-evidence, and claim-manifest lanes to land with
  incompatible assumptions that downstream tooling could not
  reconcile.

The `D-0017` `narrow` default-if-unresolved posture would
have locked the product to an `experimental`/`stable` toggle
with no dependency markers, no separate support / channel /
freshness / client-scope axes, no `disabled_by_policy`
state, and no `retired` state; bundles, settings, docs, and
support exports would have collapsed into per-surface
lifecycle vocabularies. Accepting this ADR replaces that
narrowing with the five frozen axes, the eight-state
lifecycle vocabulary, the typed dependency-marker record,
the downgrade rule, the per-surface projection requirements,
the audit-event list, and the denial posture above; the
narrowing default does not apply.

## Source anchors

- `.t2/docs/Aureline_PRD.md` — normative MUST / SHOULD
  language on lifecycle posture, support class, and
  release channel discipline (see the lifecycle, release,
  and support sections).
- `.t2/docs/Aureline_Technical_Architecture_Document.md` —
  the architecture discussion of how capability lifecycle,
  support class, release channel, freshness, and client
  scope decompose across product, docs, support, and release.
- `.t2/docs/Aureline_Technical_Design_Document.md` — the
  detailed component discussion of capability surfaces,
  dependency propagation, and disabled / deprecated /
  retired posture on settings, bundles, commands, and
  support exports.
- `.t2/docs/Aureline_Milestones_Document.md` — the
  milestone discussion of which capabilities land in which
  channels, which carry extended support, and which
  explicitly stay in labs / preview / beta posture.
- `docs/product/boundary_manifest_strawman.md` — the
  residual-dependency and self-hostable narrowing posture
  the `provider_linked_dependency` and
  `managed_only_dependency` marker kinds inherit.

## Linked artifacts

- Decision register row:
  `artifacts/governance/decision_index.yaml#D-0017`
- RFC: none.
- Capability-lifecycle and dependency-marker schema:
  `schemas/governance/capability_lifecycle.schema.json`
- Worked examples across settings, bundles, commands,
  provider-linked features, policy-disabled rows, and
  future SDK / API surfaces:
  `artifacts/governance/dependency_marker_examples.yaml`
- Settings resolver whose `lifecycle_label`,
  `control_stack`, and `capability_dependencies` fields
  project this vocabulary:
  `docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`.
- Connected-provider vocabulary whose grant-resolution
  reasons `provider_linked_dependency` markers reuse:
  `docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`.
- Identity-mode envelope that gates `managed_only_channel`
  and `managed_admin_surface`:
  `docs/adr/0001-identity-modes.md`.
- Subscription-envelope authority class under which
  projected lifecycle views ride:
  `docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`.
- Affected lanes: `governance_lane:product_scope_review`,
  `governance_lane:release_council`,
  `governance_lane:docs_public_truth`,
  `governance_lane:support_export`,
  `governance_lane:governance_packets`.

## Supersession history

First acceptance. No supersession.
