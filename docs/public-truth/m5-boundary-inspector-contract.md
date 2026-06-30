# M5 boundary inspector contract

This contract freezes the user / admin / evaluator-facing boundary inspector: the one surface that
explains, for each consequential M5 action, **where execution and data went, which host / service
hops carried the work, and which approval authority was in effect**. It sits beside the
[assurance center](m5-assurance-center-contract.md) and the
[governance dashboard](m5-governance-dashboard-contract.md): those read "what does Aureline claim"
and "which governance functions are passing"; this inspector reads the runtime layer next to them —
"for *this* high-risk action, did it stay local or cross a boundary, what route did it take, and who
granted it, for how long?"

It does **not** mint new runtime-approval semantics or a second route ledger. Every card derives its
state from the boundary / route / approval vocabularies the
[assurance / route governance matrix](../../schemas/release/m5-assurance-route-governance.schema.json)
froze, so a card can never read safer than its proof, and a drifted route or expired approval never
renders as a clean pass.

- Packet schema: [`schemas/public-truth/m5-boundary-inspector.schema.json`](../../schemas/public-truth/m5-boundary-inspector.schema.json)
- Component schemas (validatable on their own):
  [`m5-boundary-summary-card.schema.json`](../../schemas/public-truth/m5-boundary-summary-card.schema.json),
  [`m5-route-hop-timeline.schema.json`](../../schemas/public-truth/m5-route-hop-timeline.schema.json),
  [`m5-approval-ticket-inspector.schema.json`](../../schemas/public-truth/m5-approval-ticket-inspector.schema.json)
- Published inventory: [`artifacts/public-truth/m5-boundary-inspector.json`](../../artifacts/public-truth/m5-boundary-inspector.json)
- Rendered overview: [`artifacts/public-truth/m5-boundary-inspector.md`](../../artifacts/public-truth/m5-boundary-inspector.md)
- Machine-readable action / facet matrix: [`artifacts/public-truth/m5-boundary-inspector-actions.csv`](../../artifacts/public-truth/m5-boundary-inspector-actions.csv)
- Release-grade parity proof: `artifacts/public-truth/m5-boundary-inspector-proof/boundary-inspector.json` (+ `.md`)
- Exported evaluation packet: `artifacts/public-truth/m5-boundary-inspector-proof/evaluation-packet.json`
- Per-state fixtures: `fixtures/public-truth/m5-boundary-inspector/`
- Producer crate / module: `crates/aureline-release` → `m5_boundary_inspector`
- Headless emitter: `aureline_release_m5_boundary_inspector`

## What the inspector holds

The packet holds one **action inspector** per consequential M5 action, minted from one source by the
headless emitter — each action's boundary state, route state and hops, and approval state and expiry
— so the in-code packet, the published artifacts, and the fixtures can never drift. The eight actions
are `local_model_execution`, `remote_model_inference`, `provider_credential_rotation`,
`workspace_data_export`, `control_plane_sync`, `offline_model_acquisition`, `admin_policy_push`, and
`support_bundle_handoff`. Each inspector carries three reusable cards and a verdict that is the
**worst gate** of the three, so it never reads safer than its least-attested facet.

### 1. Boundary summary card

One card per action. It declares the execution / data **boundary class** (`local_execution`,
`local_to_remote_provider`, `local_to_control_plane`, `vendor_handoff`), who initiated it (the
**actor** and source locality), the **target class**, the **sensitive data classes** that crossed
(category labels only — never the data itself), the **approving authority**, and an export-safe
one-line summary. Its active state is read from the matrix capability-boundary vocabulary
(`within_boundary` → `outside_boundary`), and the effective gate folds in evidence freshness so the
card can never read further within boundary than its proof. A local-only action declares
`local_execution` and `crosses_trust_boundary: false`, so the inspector proves work that *stayed* on
the machine just as clearly as it explains work that left it.

### 2. Route-hop timeline

One ordered timeline per action — the hops the work passed through, in route order. Each hop names
its **locality** (`local_machine` → `vendor_edge`), **role** (`origin` / `proxy` / `mirror` /
`target`), **certificate context** (pinned, mirror, control-plane, local trust, or none on-device),
and any **drift marker**. The timeline binds the action's route-hop state (`local_only` →
`unattributed_route`); its effective gate folds in the worst hop drift and the route evidence
freshness:

| Drift marker | Gate | Cause |
|--------------|------|-------|
| `none` | `governed` | the hop matched the expected route |
| `locality_drift` | `narrowed` | the hop reached an unexpected locality |
| `certificate_drift` | `narrowed` | the hop's certificate changed from the pinned one |
| `mirror_substitution` | `narrowed` | a mirror silently replaced the named target |
| `unattributed_hop` | `blocked` | the hop cannot be attributed at all |

A route state may never read more attributable than its hops: a drifting hop forbids a governed route
state, and an unattributed hop forces a blocked one.

### 3. Approval-ticket inspector

One ticket per action, explaining who granted what and for how long using the same runtime authority
vocabulary the lower-level objects use. It binds the **capability class** the ticket grants, the
**approving authority** (`standing_policy`, `user_consent`, `workspace_admin`, `security_officer`,
`runtime_broker`), the **scope**, the **approval state** (`pre_authorized` → `approval_denied`), the
**expiry** and its standing (`active` / `expiring_soon` / `expired`), and the **revoke / renew
actions** an operator can take. The effective gate folds the approval state and expiry standing, so an
expiring ticket narrows and an expired one blocks. The offered actions follow the gate: a governed
ticket can be revoked or renewed; a blocked one requires reapproval or a tighter scope.

## Exported evaluation packet

The packet carries an [`evaluation_packet`] export that reduces each action inspector to the exact
boundary / route / approval vocabulary the in-product cards show, so an exported evaluation pack and
the live UI can never read differently. The export is metadata-only: it preserves route and proof
lineage as refs and carries no credential bodies or raw provider payloads. The same desktop, support,
and evaluation surfaces can validate a single boundary card, route timeline, or approval ticket
against the three component schemas above, so the cards stay reusable without the whole packet.

## Gate behaviour

- Every boundary card declares its class, actor, target, data classes, authority, and an
  export-safe summary; every route timeline is ordered and names locality / role / certificate per
  hop; every approval ticket binds capability, authority, scope, and expiry.
- An inspector's effective gate is the worst of its three facets — it never overstates.
- A route drift narrows the timeline deterministically.
- An unattributed route hop or an expired approval blocks Stable promotion
  (`blocks_stable_promotion`).
- Route and proof lineage stay refs-only; no credential bodies or raw provider payloads cross into
  the export.

The packet's `conformance` block records each of these as a hard invariant; the
`M5BoundaryInspector::validate` method re-derives every part and fails on any drift, and the headless
emitter refuses to mint a packet that does not validate.
