# Browser handoff, incident viewer, and admin-surface parity contract

This contract prevents browser or vendor-console surfaces from quietly
becoming the real product. It freezes:

- what a browser/console handoff must explain;
- which parity gaps are allowed (and how they are disclosed); and
- how evidence continuity survives a transition into and back out of an
  external surface.

Browser and vendor-console surfaces are permitted as **explicit overlays**
or **explicit escape hatches**. They are not permitted as **hidden control
planes** or as the only practical way to answer contractual questions
about policy, advisories/incidents, audit truth, or deployment truth.

## Companion artifacts

- [`/docs/admin/browser_handoff_and_surface_parity_contract.md`](./browser_handoff_and_surface_parity_contract.md)
  — this contract.
- [`/schemas/admin/browser_handoff_record.schema.json`](../../schemas/admin/browser_handoff_record.schema.json)
  — boundary schema for `browser_handoff_record`, the export-safe handoff
  record used by product surfaces, support packets, and admin exports.
- [`/fixtures/admin/browser_handoff_cases/`](../../fixtures/admin/browser_handoff_cases/)
  — worked YAML cases for incident drill handoff, admin policy review
  handoff, procurement/support packet handoff, and explicit non-parity
  disclosure on offline/self-hosted profiles.

Related upstream contracts this contract composes over:

- [`/schemas/integration/browser_handoff_packet.schema.json`](../../schemas/integration/browser_handoff_packet.schema.json)
  — provider/browser handoff packet vocabulary and redaction posture.
- [`/docs/ops/incident_workspace_contract.md`](../ops/incident_workspace_contract.md)
  — incident workspace chronology, attributable browser/console exits,
  and frozen evidence-handoff bundles.
- [`/docs/admin/admin_audit_export_contract.md`](./admin_audit_export_contract.md)
  — vendor-console independence rules for admin audit exports.
- [`/docs/admin/policy_explainability_contract.md`](./policy_explainability_contract.md)
  — policy explainability vocabulary and admin handoff export linkage.
- [`/docs/policy/admin_policy_and_bundle_cache_contract.md`](../policy/admin_policy_and_bundle_cache_contract.md)
  — signed policy bundle cache and offline explainability posture.
- [`/docs/security/advisory_surface_contract.md`](../security/advisory_surface_contract.md)
  — advisory/incident publication posture and evidence linkage.
- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  — “optional services are additive”, “provider overlay”, and “deep links
  into vendor consoles are valid handoff targets, but they are explicit
  control-plane escapes, not hidden dependencies or substitute truths.”

If this document disagrees with `.t2/docs/`, `.t2/docs/` is authoritative
and this contract plus companion artifacts must be updated in the same
change.

## Scope

Frozen at this revision:

- one `browser_handoff_record` shape that every surface uses to disclose
  a browser/vendor-console pivot without losing target identity, authority
  scope, return anchor, parity disclosure, or evidence continuity; and
- the “no hidden control plane” parity rules for incident-viewer, admin,
  and policy surfaces.

Out of scope:

- implementing a browser companion, hosted admin console, or external
  incident tooling;
- defining raw URLs, raw tokens, raw provider payloads, or vendor-console
  page structures; and
- replacing the provider/browser packet contract. This contract consumes
  it by reference when present.

## Core principles

1. **No hidden control plane.** Browser/vendor-console surfaces MAY
   exist, but the product MUST NOT rely on them to satisfy contractual
   questions that the product promises locally or via export.
2. **Authority scope is explicit.** Every pivot states what authority
   the external surface can exercise and what remains governed locally.
3. **Target identity is explicit.** The handoff binds to stable object
   refs (tenant/org, workspace, incident, policy, audit slice) instead of
   a free-floating deep link.
4. **Return is explicit.** The record pins a return anchor so the user
   can re-enter the product with continuity.
5. **Parity gaps are disclosed before launch.** A non-parity state is
   shown *in-product* and recorded, not discovered after context is lost.
6. **Evidence continuity is stable.** Product, external surface handoff,
   and exported evidence point to the same underlying object refs and
   history rows.

## Browser handoff record (`browser_handoff_record`)

Every browser or vendor-console pivot produces (or updates) one
`browser_handoff_record` conforming to
`schemas/admin/browser_handoff_record.schema.json`.

The record is **export-safe**: it carries opaque refs, typed vocabulary,
and reviewable summaries. Raw URLs, raw tokens, raw tenant-private
payloads, raw device fingerprints, raw emails/names, raw policy bodies,
and raw secret material are forbidden.

Minimum required blocks:

- `reason` — why the pivot exists (typed `reason_code` plus a
  reviewable `reason_note`).
- `destination` — what kind of surface is being opened (`destination_class`)
  plus an export-safe destination ref/label.
- `target_identity` — the stable target object the external surface is
  about.
- `authority_transfer` — what authority is being exercised externally
  and how the transfer is justified.
- `return_anchor` — how the user returns to the product and what object
  they land back on.
- `evidence_continuity` — the object refs and history refs that must
  remain stable across product ↔ handoff ↔ export.
- `parity_disclosure` — whether the external surface is parity, partial
  parity, or explicitly non-parity, and what gaps exist.

### Authority transfer rules

- A handoff record MUST name an `authority_scope_class` (read-only,
  limited mutation, or admin mutation) and MUST include an
  `authority_transfer_note` explaining what is and is not permitted.
- When a provider/browser packet exists, the record SHOULD cite
  `destination.browser_handoff_packet_ref` so the handoff shares the same
  destination class, actor class, and redaction posture as the provider
  contract.
- “Open browser” without a typed record is non-conforming for incident,
  policy, audit, or deployment-truth pivots.

### Return anchor rules

- The return anchor MUST be stable and object-scoped (e.g. incident
  workspace id ref, policy card id ref, audit export id ref), not a
  “go back to where you were” heuristic.
- If return requires a manual step (e.g. offline/self-hosted console
  absent), the record MUST still carry a return anchor plus a parity gap
  describing the blocked path and the safe fallback (export / local
  inspection).

### Evidence continuity rules

- `evidence_continuity.primary_object_ref` is the object the user is
  actually reasoning about (incident, audit export, policy decision
  row, deployment profile slice). All surfaces cite it.
- `evidence_continuity.history_row_refs` carry stable refs for the
  chronology a reviewer would inspect (audit rows, decision rows,
  incident action-ledger entries, evidence bundle ids).
- External surfaces may add convenience overlays, but they must not be
  the only place where the stable ids and minimum history rows exist.

## Incident viewer and admin/policy surface parity

External surfaces are permitted, but they are constrained by parity
rules. The goal is to keep “what is true” resolvable locally or via
export even when an external surface is unavailable.

### Action matrix (frozen intent)

This contract freezes the *classes* of incident-viewer/admin/policy
actions that may appear in browser/vendor-console surfaces on claimed
profiles, and which are explicitly handoff-only.

| Action family | Product surface | Export surface | Browser/vendor-console surface | Notes |
|---|---|---|---|---|
| Inspect incident identity, evidence bundle lineage, and chronology | MUST be available (at least summary + refs) | MUST be available via evidence-handoff bundle | MAY be present as a read-only overlay | Overlay requires `browser_handoff_record`; must cite the same incident/evidence refs. |
| Run incident mitigation that mutates an external target | MUST NOT be silent; if supported, MUST be attributable | MUST preserve minimum history rows and evidence refs | MAY be handoff-only | Requires explicit authority scope; must cite typed handoff refs and remain reconstructable from stable ids. |
| Inspect effective policy, locks, and escalation path | MUST be available | MUST be available via policy/audit exports | MAY be present as an overlay | Console-only detail is permitted only when disclosed as partial parity before launch. |
| Inspect audit truth (who/what/scope/epochs/reason) | MUST preserve minimum field set | MUST be available via admin audit export | MAY be present as an overlay | Vendor-console dependence is only allowed for additional detail, never for the minimum field set. |
| Inspect “deployment truth layers” (desired vs observed vs provider overlay) | MUST keep truth layers distinct and labeled | MUST preserve declared truth class and refs | MAY be present as a provider overlay | Provider overlays are allowed but may not collapse into “the” canonical truth without a local/exportable anchor. |
| Deliver a support/procurement packet to an external portal | MUST provide local export copy | MUST preserve export-safe packet refs | MAY be present as a delivery endpoint | This is handoff-only; the portal is never the canonical evidence store. |

### Allowed external-surface actions

Allowed in browser/vendor-console surfaces (with a `browser_handoff_record`
and parity disclosure):

- **Read-only overlays**: incident timeline browsing, policy inspection,
  audit filtering, deployment environment overlays, and runbook viewing
  that cite stable refs also available locally.
- **Mutations that are explicitly handoff-only**: actions that the
  product does not claim to implement locally yet (for example, a
  vendor-console-only administrative mutation), provided:
  - the handoff is recorded with explicit authority scope, target
    identity, and return anchor; and
  - exported evidence still preserves the minimum field set and stable
    ids needed to reconstruct what changed.

### Explicitly out of scope (handoff-only)

The following are permitted only as explicit handoff-only escapes and
MUST NOT be represented as “supported in-product” without a parity row
and local/exportable evidence path:

- vendor-console-only annotations or timelines that cannot be exported as
  stable ids and minimum history rows;
- console-only policy authorship workflows whose only durable record is
  a provider page; and
- incident-response tooling that mutates external targets without
  producing attributable handoff refs and evidence bundle continuity.

### No-hidden-control-plane rules

For these contractual questions, the product (or its exports) must
remain a viable source of truth:

- **Policy**: effective state, source freshness, owner/lock reason, and
  escalation path (see `policy_explainability_contract`).
- **Advisory/incident**: incident identity, severity, evidence bundle
  lineage, and mutation chronology (see `incident_workspace_contract` and
  `advisory_surface_contract`).
- **Audit**: who changed what at what scope under which epochs, with
  minimum history rows preserved (see `admin_audit_export_contract`).
- **Deployment truth**: desired vs observed vs provider overlay truth
  classes remain distinct and labeled (see the architecture “truth
  layer” rules).

If an external surface is required for *additional detail*, the handoff
record MUST declare that as a parity gap before launch. If an external
surface is required for a *mutation*, the product MUST still retain
stable ids and minimum history rows locally or in export so the mutation
does not become “hidden console state”.

## Worked fixtures

See [`/fixtures/admin/browser_handoff_cases/`](../../fixtures/admin/browser_handoff_cases/)
for worked cases exercising this contract.
