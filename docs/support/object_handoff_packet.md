# Object-specific issue and support handoff contract

This document freezes the object-specific handoff packet Aureline uses
when a user reports an issue, files docs feedback, or hands work to
support from a launch-critical surface. The goal is to preserve the
failing object's stable identity, current state vocabulary, build truth,
route truth, boundary cues, and redaction/destination choices in one
packet family instead of falling back to surface-local forms.

Companion artifacts:

- [`/schemas/support/object_handoff_packet.schema.json`](../../schemas/support/object_handoff_packet.schema.json)
  — machine-readable boundary for both the concrete handoff packet and
  the reusable route record that packets cite.
- [`/fixtures/support/object_handoff_examples/`](../../fixtures/support/object_handoff_examples/)
  — worked route and packet examples covering extension, workflow,
  update, trust, docs/help, migration, command-detail, generated, and
  imported-object launches.
- [`/schemas/support/support_packet_index.schema.json`](../../schemas/support/support_packet_index.schema.json)
  — support packet family registry that now includes the
  `object_issue_handoff` family.
- [`/docs/support/support_center_concept.md`](./support_center_concept.md)
  — product-facing concept note that defines object-specific issue
  handoff as part of the Support Center.
- [`/docs/support/support_bundle_contract.md`](./support_bundle_contract.md)
  and
  [`/schemas/support/support_bundle.schema.json`](../../schemas/support/support_bundle.schema.json)
  — support-bundle linkage, redaction posture, recovery-rung, and
  exact-build vocabulary the handoff packet reuses rather than
  flattening privately.
- [`/docs/docs/help_about_service_health_routes.md`](../docs/help_about_service_health_routes.md)
  and
  [`/schemas/docs/destination_descriptor.schema.json`](../../schemas/docs/destination_descriptor.schema.json)
  — source and destination descriptor family for docs/help/support
  routes and browser/device-code disclosures.
- [`/artifacts/governance/issue_routing.yaml`](../../artifacts/governance/issue_routing.yaml)
  — canonical issue class, privacy, disclosure, summary, and owning
  forum seed the route record snapshots.
- [`/schemas/security/incident_workspace_packet.schema.json`](../../schemas/security/incident_workspace_packet.schema.json)
  — trust-sensitive incident linkage the handoff packet references by
  stable id rather than inlining private evidence.
- [`/docs/migration/migration_center_object_model.md`](../migration/migration_center_object_model.md)
  — migration packet/report linkage vocabulary reused for migration
  launches.
- [`/docs/commands/palette_row_and_modifier_contract.md`](../commands/palette_row_and_modifier_contract.md)
  — command-row, modifier-action, automation-cue, and degraded-state
  projection vocabulary command detail sheets preserve when an issue or
  support packet starts from palette, docs, CLI discovery, or automation
  surfaces.

Normative sources this contract projects from:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` supportability,
  recovery-ladder, route-truth, and exact-build passages.
- `.t2/docs/Aureline_Technical_Design_Document.md` offline handoff,
  browser handoff, admin handoff, and support/export packet passages.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` structured handoff
  packets, support/admin handoff packets, offline handoff packet
  banner, and object-linked issue drill coverage.

If this document disagrees with those sources, those sources win and
this document plus the schema must be updated in the same change.

## Why this exists

The repository already had adjacent contracts for:

- support bundles;
- incident packets;
- browser handoff packets;
- docs/help destination descriptors;
- migration support-packet refs; and
- issue-routing lanes.

What stayed implicit was the packet a user actually creates when they
choose `Report issue`, `Send to support`, `Save handoff packet`, or a
similar action from a concrete object surface. Without this contract:

- extension pages, workflow bundles, update screens, trust warnings,
  docs/help rows, migration flows, command/detail sheets, and artifact
  rows would each invent different report fields;
- packets would keep free-text titles but lose the object's stable id,
  build id, or current state tokens;
- support bundles, incident packets, repair refs, docs descriptors, and
  generated/imported provenance would be attached ad hoc or not at all;
- offline capture would preserve text but silently drop route,
  boundary, redaction, or publish-later truth.

This contract closes that gap with one packet family:
`object_issue_handoff`.

## Scope

Frozen at this revision:

- one `object_handoff_packet_record` that captures source surface,
  source object identity, current state rows, build/docs truth, route
  snapshot, scope/boundary/path-truth cues, typed provenance bindings,
  support-bundle and incident refs, recovery-ladder refs, destination
  routing, redaction choice, and offline delivery state;
- one `object_handoff_route_record` that snapshots the route lane,
  privacy/disclosure posture, public-summary expectation, transport,
  destination descriptor, and evidence expectations the packet was built
  for;
- one shared state-row grammar so surfaces can preserve their exact
  state vocabulary without inventing a packet-local flat enum; and
- one shared provenance-binding grammar so generated/imported,
  migration, docs-anchor, browser-handoff, support-bundle, incident,
  repair, and checkpoint links stay attributable.

Out of scope:

- live network submission or tracker API integration;
- ticket deduplication;
- raw evidence-body transport;
- automatic route selection based on classifier heuristics alone.

## Record kinds

| Record kind | Job |
|---|---|
| `object_handoff_route_record` | define the issue lane, privacy/disclosure posture, transport, destination descriptor, and evidence expectations a packet can target |
| `object_handoff_packet_record` | preserve one user-initiated object report or handoff with the object's stable identity, current state, route/build/boundary truth, and explicit delivery/redaction choices |

## Packet overview

Every `object_handoff_packet_record` carries these major blocks:

| Block | Job |
|---|---|
| `source_object` | stable object kind/id, label, version, anchor, and immediate summary |
| `current_state_rows` | machine-readable state vocabulary rows (`state_family_class`, `state_value`, source field/schema refs, summary) |
| `build_context` | exact-build identity, install mode/channel, docs-pack and docs-version truth, known-limit refs, and claim refs |
| `route_context` | command/invocation/execution ids plus action origin/target/route/exposure, source docs/help descriptor, and browser-handoff linkage |
| `scope_and_boundary_context` | host boundary, target/workspace identity, deployment/locality/tenant/key posture, and path-truth cue |
| `provenance_bindings` | typed refs to docs anchors, workflow runs, migration packets, generated/imported lineage, support bundles, incidents, repairs, checkpoints, and similar context |
| `evidence_and_recovery_context` | selected evidence refs, support-bundle refs, incident refs, recovery rung, repair refs, checkpoint refs, redaction choice, and withheld refs |
| `destination_context` | route ref, route lane, issue class, privacy/disclosure posture, target destination descriptor, transport, delivery state, and publish-later target |

The packet family is intentionally summary-first. It preserves stable ids
and typed refs instead of raw logs, raw URLs, raw docs bodies, or raw
provider payloads.

## State rows

Different source surfaces expose different state tokens. A docs/help row
may care about `freshness` and `version_match`; a trust warning may care
about `trust_posture` and `security_posture`; a generated artifact row
may care about `generated_lineage` and `path_truth`.

Instead of collapsing those into one overfit enum, the packet carries
`current_state_rows[]`:

- `state_family_class` names the axis (`freshness`, `route_posture`,
  `migration_posture`, `generated_lineage`, and so on);
- `state_value` carries the stable upstream token;
- `source_schema_ref`, `source_field`, and `source_ref` optionally point
  back to the contract or record that minted the token; and
- `summary` keeps the row reviewable in export previews.

This lets consumers preserve exact source vocabulary without inventing
surface-local prose-only packets.

## Route and destination model

The route record and packet destination snapshot keep three distinct
layers explicit:

1. `handoff_route_lane_class`
   public tracker, private partner lane, private security lane,
   governance queue, local-only export, and similar issue-routing lanes.
2. `handoff_transport_class`
   whether the packet stays local, opens a browser review, falls back to
   device code, attaches by reference to an incident workspace, or
   expects manual ticket entry.
3. `target_destination_descriptor_ref`
   the concrete destination descriptor when the handoff crosses a docs,
   support, or community route boundary.

That split matters because a local-only packet and a browser-first
packet can target the same logical issue lane while having very
different disclosure and replay behavior.

## Linkage rules

The packet preserves adjacent support objects by typed ref instead of
flattening them:

- support bundles via `support_bundle_refs` and
  `provenance_bindings.support_bundle_ref`;
- incident workspaces via `incident_workspace_packet_refs` and
  `provenance_bindings.incident_workspace_packet_ref`;
- recovery-ladder state via `recovery_rung_class`,
  `repair_transaction_refs`, and `checkpoint_refs`;
- docs/help source descriptors via
  `route_context.source_destination_descriptor_ref`;
- destination/support-route descriptors via
  `destination_context.target_destination_descriptor_ref` and
  `destination_context.support_route_ref`;
- generated/imported provenance via `generated_artifact_lineage_ref` and
  `import_source_ref`; and
- migration flows via `migration_session_ref`,
  `importer_outcome_packet_ref`, and `migration_report_ref`.
- workspace bootstrap provenance via `bootstrap_packet_ref` so clone,
  import, open, and resume failures preserve source identity, checkout
  plan, trust stage, mirror/public route, resumability, prerequisites,
  and reason codes without copying raw secrets or source bodies.
- command palette rows via the combined row contract so handoff packets
  preserve command ID, modifier action, disabled reason, automation cue,
  origin badge, lifecycle cue, and target/authority hint without
  rewording the palette row locally.

If a linked record is unavailable, the packet may omit the ref, but it
must keep the relevant state row and summary honest about the gap.

## Covered surfaces

This revision seeds examples for the required launch surfaces:

| Surface | Minimum preserved context |
|---|---|
| Extension page | extension/runtime host id, version, health/trust state, repair/support-bundle linkage |
| Workflow bundle | bundle id, route or publish context, browser-handoff/offline state, publish-later target |
| Update screen | update candidate id, build/channel state, docs/help truth, support route |
| Trust warning | warning id, target/boundary/trust cues, incident linkage, redaction choice |
| Docs/help surface | source destination descriptor, docs anchor, freshness/version state, docs-feedback route |
| Migration flow | migration session/outcome/report refs, import-fidelity and migration state, recovery/support linkage |
| Command detail sheet | command id, invocation id, route truth, target identity, authority/boundary cues |
| Generated artifact row | lineage ref, path-truth cue, workspace/object refs, selected evidence |
| Imported artifact row | import-source ref, partial/normalized state, path-truth cue, route/destination choice |

## Delivery and redaction

The packet does not pretend submission already happened. Every packet
names:

- the chosen `redaction_choice_class`;
- any `withheld_artifact_refs`;
- the current `delivery_state_class`; and
- `publish_later_target_ref` when the route is deferred.

This keeps offline capture, blocked browser launch, local-only review,
manual security escalation, and support-bundle-by-reference flows
machine-readable instead of hidden in toast copy.
