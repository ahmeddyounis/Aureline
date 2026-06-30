# M5 event-provenance contract

This contract freezes the user / admin / evaluator-facing event-provenance inspector: the one surface
that explains, for each **queued or replayable** M5 action across the AI, provider, remote, and
support flows, **where the event came from, what changed since it was planned, and whether replaying
it is still safe**. It sits beside the
[boundary inspector](m5-boundary-inspector-contract.md): that surface reads "for *this* high-risk
action, where did work run and who approved it"; this inspector reads the deferred layer next to it —
"this action was queued or will replay; has its endpoint, tenant, region, route, or approval drifted
since the plan, and may it still run?"

It does **not** invent a new replay queue or any mutate-on-retry behavior. Every facet derives its
state from the provenance / route / approval / capability-boundary vocabularies the
[assurance / route governance matrix](../../schemas/release/m5-assurance-route-governance.schema.json)
froze, so a facet can never read safer than its proof, and a drifted route or invalidated approval
never renders as a clean pass. The exit-gate it guards: replay or publish-later flows become unsafe
when target, tenant, region, route, or approval facts drift but Aureline continues as if the old
boundary still holds.

- Packet schema: [`schemas/public-truth/m5-event-provenance.schema.json`](../../schemas/public-truth/m5-event-provenance.schema.json)
- Component schemas (validatable on their own):
  [`m5-event-provenance-row.schema.json`](../../schemas/public-truth/m5-event-provenance-row.schema.json),
  [`m5-route-drift-banner.schema.json`](../../schemas/public-truth/m5-route-drift-banner.schema.json),
  [`m5-replay-reapproval-gate.schema.json`](../../schemas/public-truth/m5-replay-reapproval-gate.schema.json)
- Published inventory: [`artifacts/public-truth/m5-event-provenance.json`](../../artifacts/public-truth/m5-event-provenance.json)
- Rendered overview: [`artifacts/public-truth/m5-event-provenance.md`](../../artifacts/public-truth/m5-event-provenance.md)
- Machine-readable event / facet matrix: [`artifacts/public-truth/m5-event-provenance-events.csv`](../../artifacts/public-truth/m5-event-provenance-events.csv)
- Release-grade parity proof: `artifacts/public-truth/m5-event-provenance-proof/event-provenance.json` (+ `.md`)
- Exported redaction-safe preview: `artifacts/public-truth/m5-event-provenance-proof/export-preview.json`
- Per-state fixtures: `fixtures/public-truth/m5-event-provenance/`
- Producer crate / module: `crates/aureline-release` → `m5_event_provenance`
- Headless emitter: `aureline_release_m5_event_provenance`

## What the inspector holds

The packet holds one **deferred event** per queued or replayable M5 action, minted from one source by
the headless emitter — each action's provenance state, route facts and drift, and boundary and
approval state — so the in-code packet, the published artifacts, and the fixtures can never drift. The
eight actions are `queued_prompt_replay`, `deferred_model_download`, `scheduled_credential_rotation`,
`publish_later_data_export`, `queued_control_plane_sync`, `retried_policy_push`,
`deferred_support_handoff`, and `replayed_audit_export`, spanning the `ai`, `provider`, `remote`, and
`support` flows. Each event carries three reusable facets and a verdict that is the **worst gate** of
the three, so it never reads safer than its least-attested facet.

### 1. Event-provenance row

One row per action, attached to the `log`, `diagnostic`, `artifact`, or `audit` **surface** the event
landed on. It names the **event id**, the **mutation / run / session** it links to (refs only), the
**host lane** (`local_machine` → `vendor_edge`), the **retrieval epoch** it was read as-of, and the
**redaction posture** (`metadata_only`, `reference_only`, `redacted_body`, `sealed_local`). Its active
state is read from the matrix provenance vocabulary (`fully_traced` → `provenance_missing`), and the
effective gate folds in evidence freshness so the row can never read more traceable than its proof. A
local event reads `local_only` on its route and `crosses_trust_boundary: false`, so the inspector
proves work that *stayed* on the machine just as clearly as work that left it.

### 2. Route-drift banner

One banner per action — the comparison of the action's current route facts against its **baseline**
(`plan` or `last_success`). It names every route fact that drifted using the controlled facet set, and
each drifted fact carries export-safe category refs for the planned and current value (never raw
identifiers). The banner binds the action's route-hop state (`local_only` → `unattributed_route`); its
effective gate folds in the worst drifted-fact gate and the route evidence freshness:

| Drifted fact | Gate | Cause |
|--------------|------|-------|
| `endpoint` | `narrowed` | the endpoint moved since the baseline |
| `region` | `narrowed` | the region changed since the baseline |
| `proxy` | `narrowed` | the proxy changed since the baseline |
| `certificate` | `narrowed` | the certificate changed from the pinned one |
| `mirror` | `narrowed` | a mirror silently replaced the named target |
| `policy` | `narrowed` | the governing policy changed since the baseline |
| `tenant` | `blocked` | the tenant changed — a hard isolation boundary |

A banner may never read more attributable than its facts: a drifting fact forbids a governed route,
and a tenant drift forces a blocked one.

### 3. Replay / reapproval gate

One gate per action, explaining whether the deferred action may run again. It binds the **deferred
kind** (`replay`, `publish_later`, `approve_again`), the current **capability-boundary state**
(`within_boundary` → `outside_boundary`), the **approval state** (`pre_authorized` →
`approval_denied`), and the **decision** those facts imply. The effective gate folds the boundary
state, the approval state, and the approval evidence freshness; the decision follows the gate:

| Gate | Decision | Meaning |
|------|----------|---------|
| `governed` | `replay_as_is` | current facts match the plan; the action may replay |
| `narrowed` | `require_reapproval` | current facts narrowed the boundary; re-approve first |
| `blocked` | `hold_blocked` | current facts invalidate the action; it is held |

So when current boundary facts invalidate the earlier route or approval assumptions, the gate requires
a replay / publish-later / approve-again decision rather than continuing silently.

## Exported redaction-safe preview

The packet carries an `export_preview` that reduces each deferred event to the exact provenance /
route / approval vocabulary the in-product facets show, so an exported support / audit pack and the
live inspector can never read differently. The export is metadata-only: it preserves event / route /
proof lineage as refs (event id, mutation / run / session, three proof refs per event) and carries no
credential bodies or raw provider payloads. The same desktop, support, and evaluation surfaces can
validate a single provenance row, drift banner, or reapproval gate against the three component schemas
above, so the facets stay reusable without the whole packet.

## Gate behaviour

- Every provenance row links the event to a mutation, run, and session, and declares its host lane,
  retrieval epoch, and redaction posture.
- Every route-drift banner names the changed facts and the baseline they changed from.
- Every reapproval gate binds the deferred kind, boundary state, and approval state.
- An event's effective gate is the worst of its three facets — it never overstates.
- A route-fact drift narrows the banner deterministically; a tenant drift blocks Stable promotion.
- Changed boundary facts force a re-approval or hold instead of a silent replay.
- Event / route / proof lineage stay refs-only; no credential bodies or raw provider payloads cross
  into the export.

The packet's `conformance` block records each of these as a hard invariant; the
`M5EventProvenance::validate` method re-derives every part and fails on any drift, and the headless
emitter refuses to mint a packet that does not validate.
