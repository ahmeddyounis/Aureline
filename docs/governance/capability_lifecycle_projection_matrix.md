# Capability lifecycle projection matrix

This document publishes the cross-surface projection rules that keep
capability lifecycle posture, dependency markers, and certification /
retest posture consistent across product UI, docs/help/About, diagnostics,
support exports, and policy explainers.

It exists so that no launch-bearing surface can render a capability as
`stable` (or “Certified”) while another surface renders the same
capability as `preview`, `RetestPending`, policy-blocked, or sunset
without triggering a review failure.

Machine-readable companions:

- [`/schemas/governance/capability_projection_row.schema.json`](../../schemas/governance/capability_projection_row.schema.json)
  — projection-row envelope used by later badge linting and
  claim-publication checks.
- [`/artifacts/governance/capability_projection_examples.yaml`](../../artifacts/governance/capability_projection_examples.yaml)
  — worked examples spanning preview, sunset, policy blocks,
  managed-only, mixed bundles, and retest-pending certification.
- [`/artifacts/governance/lifecycle_fail_gate.yaml`](../../artifacts/governance/lifecycle_fail_gate.yaml)
  — closed fail-gate rules that reject stable / certified wording when a
  narrowing marker or stale / retest posture is present.

Upstream contracts (re-exported, not re-minted):

- [`/docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md)
  and
  [`/schemas/governance/capability_lifecycle.schema.json`](../../schemas/governance/capability_lifecycle.schema.json)
  — canonical axis vocabularies and dependency-marker record shape.
- [`/docs/governance/capability_axis_matrix.md`](./capability_axis_matrix.md)
  and
  [`/artifacts/governance/capability_badge_axes.yaml`](../../artifacts/governance/capability_badge_axes.yaml)
  — seven-axis separation rules and badge-propagation constraints.
- [`/docs/governance/truth_and_degraded_state_vocabulary.md`](./truth_and_degraded_state_vocabulary.md)
  — degraded tokens including `PolicyBlocked` and `RetestPending` and
  their composition with lifecycle and freshness axes.

If this document and ADR 0011 disagree, ADR 0011 wins and this document
updates in the same change. If this document and the projection examples
disagree, this document wins and the examples update in the same change.

## Terms

- **Canonical lifecycle row.** The authoritative axis set and live marker
  list for one capability (`capability_lifecycle_row_record` plus
  `dependency_marker_record`s).
- **Projection row.** A surface-authored (or generator-authored) snapshot
  that names the *same* axes and markers in a shape that is convenient
  for linting and for review of surface-local copy. Projection rows are
  projections, not sources: they MUST NOT widen above the canonical
  effective posture.
- **Launch-bearing surface.** Any surface whose default presentation can
  be interpreted as a readiness claim (settings, bundle cards, command
  palette + install review, docs/help/About, support exports, release
  evidence, marketplace discovery).

## Separation rules (normative)

Surfaces may collapse badges visually, but they MUST NOT collapse axis
truth structurally.

1. A projection MUST keep these values separately addressable:
   `effective_lifecycle_state`, `support_class`, `release_channel`,
   `freshness_class`, `client_scopes`, and the full dependency-marker
   list (or an explicit `no_live_markers` sentinel).
2. Dependency markers are first-class truth. A surface MUST NOT replace
   a concrete marker list with generic copy such as “Unavailable”,
   “Having trouble”, “Try again”, “Blocked”, or “Not supported”.
3. When any dependency marker narrows an axis, the projection MUST carry
   the narrowed effective value and MUST carry the marker entry that
   caused the narrowing.
4. `disabled_by_policy`, `managed_only`, `deprecated`, and `retired`
   postures MUST render with typed reason and a repair route or
   successor route. Silent disappearance is non-conforming.
5. `RetestPending` is a degraded / freshness posture (not a marketing
   synonym). A surface that is retest-pending MUST NOT present certified
   wording or stable-by-omission wording.

## Projection rows (what every surface exports)

Every surface that renders a capability in a badge-bearing way emits one
`capability_projection_row_record` (or a list of them) carrying:

- capability identity (`capability_lifecycle_row_id` + `capability_ref`)
- the canonical declared + effective lifecycle state
- the six other axes used for badge rendering (support, channel, client
  scope set, freshness, certification axis snapshot)
- an explicit dependency-marker summary list (or `no_live_markers`)
- the surface’s own **wording claims** (what the surface copy implies)
  so fail-gates can flag “Stable” / “Certified” wording that survived a
  narrowing marker.

See the schema for the precise field list.

## Surface matrix

The table below extends ADR 0011’s per-surface projection requirements
to additional launch-bearing surfaces (About, diagnostics, policy
explainers). It does not weaken ADR 0011 requirements.

| Surface | Projection row requirements | Marker treatment requirement |
|---|---|---|
| Settings row | All axes, declared + effective lifecycle, and surface wording claims. | Every live marker renders inline and remains inspectable via an axis inspector; generic “unavailable” copy forbidden. |
| Bundle card | All axes, declared + effective lifecycle, plus bundle-specific wording claims for installability and managed-only posture. | Gating markers render as blocking reasons; non-gating markers render as explicit caveats with inspect affordance. |
| Command palette entry | All axes plus wording claims about readiness and certification. | Markers render in the command details / inspector; a hidden marker set is treated as disclosure-incomplete. |
| Install review step | All axes plus wording claims about “what will happen” and under what narrowed state. | Every live marker is listed in the review disclosure list with a repair hook or successor route. |
| Docs page | All axes plus `since_label` and successor fields when sunset applies; wording claims must match the axis values. | Markers render as typed call-outs with links to the dependent capability docs or repair route; unresolved markers render as a visible gap. |
| Help / About | All axes plus version-match and freshness disclosures required by the help/about truth contract; wording claims must fail closed when stale. | Marker lists remain inspectable from the help/about details path; policy-blocked rows route to the policy explainer. |
| Diagnostics (e.g., doctor) | All axes plus explicit freshness and retest posture; wording claims are conservative by default. | Diagnostics MUST enumerate markers instead of flattening to generic failures; exports preserve the marker list. |
| Support export | All axes, declared + effective lifecycle, plus captured timestamps and redaction class for safe export. | Every live marker is exported with kind, reason, effect, and disclosure summary; redaction applies before serialization. |
| Policy explainer | All axes plus the explicit policy / kill-switch / managed-only reason path and the surface’s wording claims. | Markers drive the explainer: each policy-blocking marker maps to a typed explanation + repair route; absence of marker detail is non-conforming. |

## Fail-gate intent (stable / certified wording)

Projection rows exist so review and automation can reject inconsistent
claims early. The fail-gate rules (see the companion artifact) treat the
following as release-blocking overclaims on launch-bearing surfaces:

- “Stable” wording when `effective_lifecycle_state` is not `stable` or
  `lts_facing`.
- “Stable” or stable-by-omission wording when any live marker indicates a
  preview / managed-only / policy-blocked dependency or when freshness is
  stale/unverified.
- “Certified” wording when the certification freshness posture is
  `retest_pending`/`stale`/`unknown`, or when any live marker narrows the
  capability below the certified floor.
- Cross-surface drift where one projection claims stable while another
  claims preview, retest-pending, policy-blocked, or sunset for the same
  capability id.

