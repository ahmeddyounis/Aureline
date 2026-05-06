# Transport remediation card, repair-hint catalog, and export packet contract

This contract freezes how Aureline transport failures and denials map to
actionable, screenshot-safe remediation guidance without hiding trust
boundaries. It connects:

- transport inspector truth (`transport_decision_record`,
  `effective_route_state_record`, `transport_denial_record`)
- remediation surfaces (remediation cards + hint sentences)
- export/support packets (portable evidence links, redaction posture)

The contract is a projection layer only. It does not implement a network
stack, proxy configuration, certificate management, SSH key management, or
automatic remediation.

If this document and a schema disagree, the schema wins and this document
updates in the same change. If this document and the transport governance
seed disagree on enum values, the seed wins.

## Companion artifacts

Transport governance and inspector sources:

- `docs/network/transport_governance_seed.md` — shared proxy/trust/egress/mirror
  vocabulary.
- `docs/network/transport_governance_packet_seed.md` — `transport_decision_record`
  boundary and repair-hint rules.
- `docs/network/transport_inspector_contract.md` — `effective_route_state_record`
  and `transport_denial_record` inspector projections.
- `docs/network/transport_explainability_surface_contract.md` — summary strip,
  endpoint row, certificate detail card, and denied-attempt history surfaces.

Remediation and evidence packet shapes:

- `schemas/network/transport_denial.schema.json` — typed denials with required
  remediation actions and admin-vs-user repair boundary.
- `schemas/network/network_remediation_card.schema.json` — remediation cards
  that bind trust failures to export actions and enterprise/offline review.
- `schemas/network/trust_proof_packet.schema.json` — verifiable trust evidence
  packets linked 1:1 from remediation cards.
- `schemas/network/transport_remediation_card.schema.json` — wrapper schema for
  export-safe remediation cards used by support/export tooling.

Copy governance (portable and screenshot-safe wording):

- `artifacts/network/repair_hint_catalog.yaml` — canonical remediation-hint and
  denial-remediation sentences keyed by stable reason codes.

Worked examples:

- `fixtures/network/transport_inspector_cases/` — end-to-end inspector cases
  (effective route + denial where applicable).
- `fixtures/network/network_trust_cases/` — remediation card + trust-proof
  packet cases for proxy/CA/SSH/mirror trust failures.
- `fixtures/network/transport_remediation_cases/` — remediation-card focused
  cases keyed to the repair-hint catalog and export packet linkage.

Normative source anchors:

- `.t2/docs/Aureline_PRD.md` — network egress, proxy and trust stores, support
  bundles, and redaction defaults.
- `.t2/docs/Aureline_Technical_Design_Document.md` — transport governance,
  denial vocabulary, inspector rules, and no-bypass posture.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — transport explainability and
  bounded repair action card rules (network/proxy/certificate section).

## Remediation surfaces

Aureline uses two export-safe remediation card shapes that project from the
same transport-governance truth:

1. `transport_denial_record` (typed denial surface)
2. `network_remediation_card_record` (trust-focused remediation card surface)

Support/export tooling treats both as “transport remediation cards” by
validating them against `schemas/network/transport_remediation_card.schema.json`.

### `transport_denial_record`

This is the inspector’s typed denial surface. It is emitted when an action is
denied or deferred in a way that must remain legible offline. A denial record:

- carries one typed denial category (e.g. blocked fallback, proxy conflict,
  stale mirror, certificate trust failure, SSH host proof failure, offline-only)
- preserves route/origin truth via `denial_subject` and linked decision/route
  refs
- enumerates required remediation actions (`required_remediation`) with
  `repair_boundary_class` separating user-fixable from admin-fixable paths
- exposes bounded export actions, including an `export_packet` action that links
  to a portable packet identifier (the packet content is defined below)

### `network_remediation_card_record`

This remediation card is the trust-focused export surface used when the failure
is best explained as a trust/proxy/PAC/client-certificate/SSH/mirror evidence
chain with a linked verifiable packet.

Rules:

- every remediation card MUST link exactly one `trust_proof_packet_record`
  (`linked_trust_proof_packet_ref`)
- the card MUST remain usable as a standalone, screenshot-safe summary without
  embedding raw URLs, hostnames, IPs, credentials, PAC bodies, PEM material, or
  private keys
- export actions MUST state availability and unavailable reasons per profile

## Repair-hint catalog

All remediation guidance rendered as a short sentence (UI banners, cards,
support exports, or screenshots) SHOULD come from the catalog:

- `artifacts/network/repair_hint_catalog.yaml`

The catalog provides:

- stable reason codes keyed to existing inspector vocabularies (hint kind/class
  and remediation kind/action)
- screenshot-safe, export-safe sentences that do not leak secrets or endpoint
  identity
- the admin-vs-user repair boundary expected for the hint

Implementations MAY localize or shorten sentences, but MUST preserve:

- the same reason codes
- the same repair boundary class
- the same “no bypass” posture (no insecure fallback, no silent direct origin
  fallback, no hidden proxy/trust overrides)

## Export packet contract

The `export_packet` action on transport inspector surfaces (effective route and
denial records) produces a **metadata-safe transport inspector packet**.

Minimum packet contents:

- the source `transport_decision_record`(s) for the attempt
- the linked `effective_route_state_record`
- the linked `transport_denial_record` when the attempt was denied or deferred
- any explainability surface records that were available at capture time (summary
  strip, endpoint row, certificate detail card, denied-attempt history row)
- the redaction posture and any unavailability notes (air-gapped, managed policy,
  offline-only) that affected capture

Packet rules:

- packets MUST preserve route and origin lineage (origin scope, egress class,
  route class, proxy source class, trust-store source) using the canonical field
  names from the inspector and decision schemas
- packets MUST be safe to export by default: raw URLs, raw hostnames, raw IPs,
  proxy credentials, PAC bodies, PEM/private key material, raw SSH keys, and
  request/response bodies do not appear
- packets MUST NOT invent a second vocabulary; they embed or reference the same
  records the inspector surfaces are derived from

The packet identifier referenced by `export_packet.action_target_ref` is an
opaque handle owned by the export surface (support/export bundles, file export,
or admin handoff). The packet’s internal manifest MUST list the embedded record
schema refs so offline tooling can validate each record independently.

