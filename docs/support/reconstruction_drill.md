# Cross-packet reconstruction drill

This document freezes the cross-packet reconstruction drill Aureline
runs over its exported support and release artifacts. The drill proves
that one export packet family can explain a user-visible action — what
command ran, where it ran, which docs and source version applied, which
exact build produced it, which claim row governed it, and which
known-limit note caveats it — **without reading raw source code,
without replaying the live system, and without relying on tribal memory
or oral history**.

If this document disagrees with the contracts it projects from, the
schema and artifact contracts win and this document plus the companion
checklist and fixture cases update in the same change.

## Companion artifacts

- [`/artifacts/support/reconstruction_checklist.yaml`](../../artifacts/support/reconstruction_checklist.yaml)
  — machine-readable checklist defining the mandatory, derivable, and
  intentionally-absent field sets per scenario class, the ordered
  reconstruction steps, the typed escalation rules when a correlation
  is missing, and the supportability / release / security review
  linkages.
- [`/fixtures/support/reconstruction_cases/`](../../fixtures/support/reconstruction_cases/)
  — worked reconstruction cases covering local-only, provider-bearing,
  mirrored / offline-import, and wrong-target or degraded-state
  actions. Each case names the exact artifacts a reviewer must open,
  the field-level joins between them, the reconstructed sentence the
  drill produces, and the escalation a missing correlation would
  trigger.

## Contracts this drill projects from

The drill does not mint new vocabulary. Every field the drill asserts
on must come from an already-frozen contract. If a field is missing
the fix is a schema or packet follow-up, not a drill-local label.

- Command and invocation truth
  - [`/docs/commands/command_descriptor_contract.md`](../commands/command_descriptor_contract.md)
  - [`/schemas/commands/command_descriptor.schema.json`](../../schemas/commands/command_descriptor.schema.json)
  - [`/schemas/commands/command_registry_entry.schema.json`](../../schemas/commands/command_registry_entry.schema.json)
  - [`/schemas/commands/diagnostic_projection.schema.json`](../../schemas/commands/diagnostic_projection.schema.json)
- Target, origin, route, and exposure truth
  - [`/docs/runtime/origin_target_route_taxonomy.md`](../runtime/origin_target_route_taxonomy.md)
  - [`/artifacts/runtime/action_origin_target_labels.yaml`](../../artifacts/runtime/action_origin_target_labels.yaml)
- Docs and source-version applicability
  - [`/docs/docs/help_about_service_health_routes.md`](../docs/help_about_service_health_routes.md)
  - [`/schemas/docs/destination_descriptor.schema.json`](../../schemas/docs/destination_descriptor.schema.json)
  - [`/schemas/docs/docs_pack_manifest.schema.json`](../../schemas/docs/docs_pack_manifest.schema.json)
- Exact-build identity
  - [`/docs/build/exact_build_identity_model.md`](../build/exact_build_identity_model.md)
  - [`/schemas/build/exact_build_identity.schema.json`](../../schemas/build/exact_build_identity.schema.json)
- Support bundle and object-handoff packets
  - [`/docs/support/support_bundle_contract.md`](./support_bundle_contract.md)
  - [`/schemas/support/support_bundle.schema.json`](../../schemas/support/support_bundle.schema.json)
  - [`/docs/support/object_handoff_packet.md`](./object_handoff_packet.md)
  - [`/schemas/support/object_handoff_packet.schema.json`](../../schemas/support/object_handoff_packet.schema.json)
  - [`/schemas/support/support_packet_index.schema.json`](../../schemas/support/support_packet_index.schema.json)
- Claim rows and known-limit notes
  - [`/docs/release/assurance_claim_matrix.md`](../release/assurance_claim_matrix.md)
  - [`/schemas/release/assurance_claim.schema.json`](../../schemas/release/assurance_claim.schema.json)
  - [`/docs/product/known_limits_contract.md`](../product/known_limits_contract.md)
  - [`/schemas/product/known_limit_note.schema.json`](../../schemas/product/known_limit_note.schema.json)
- Redaction, record-class, and evidence id conventions
  - [`/artifacts/governance/record_class_registry.yaml`](../../artifacts/governance/record_class_registry.yaml)
  - [`/artifacts/governance/evidence_id_conventions.md`](../../artifacts/governance/evidence_id_conventions.md)
  - [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)

## Why this exists

The adjacent contracts already cover each individual piece of truth a
reviewer needs. What stayed implicit was whether a reviewer opening an
export for the first time — months later, without the original
operator, without shell access to the target, and without the
in-memory state of the producing IDE — could reconstruct what actually
happened end-to-end.

The reconstruction drill closes that gap. It proves the packets join
cleanly, that no join is only discoverable by reading running code, and
that when a join is missing the export is honest about the gap instead
of leaving reviewers to guess.

## Scope

Frozen at this revision:

- the six correlation axes every reconstruction must resolve
  (command, route and target and origin and exposure, docs and source
  version applicability, exact-build identity, claim row, known-limit
  note);
- the four scenario classes the drill covers (local-only,
  provider-bearing, mirrored or offline, wrong-target or
  degraded-state);
- the per-field mandatory / derivable / intentionally-absent
  classification the drill reviewer applies;
- the typed escalation vocabulary when reconstruction fails;
- the three review paths that cite this drill
  (supportability, release, security).

Out of scope:

- automated support tooling beyond the drill assets;
- hosted ticket submission or live replay;
- runtime reconstruction services;
- any packet family not already frozen in the contracts above.

## The six correlation axes

Every reconstruction joins these six axes. Reviewers MUST be able to
point at one field or typed ref per axis, or record a typed gap and
route the gap to the matching escalation class.

| Axis | Canonical source | Join field(s) reviewers read |
|---|---|---|
| Command identity | `command_descriptor_record` and `command_registry_entry_record` | `command_id`, `command_descriptor_ref`, and `invocation_session_id` from the object-handoff `route_context` and support-bundle `route_and_execution_context` |
| Route, target, origin, and exposure | origin/target/route/exposure taxonomy | `action_origin_class`, `action_target_class`, `action_route_class`, `action_exposure_class`, `target_identity_ref`, and `route_summary` |
| Docs and source-version applicability | destination-descriptor and docs-pack manifest | `docs_pack_ref`, `docs_version_match_state`, `source_destination_descriptor_ref` |
| Exact-build identity | exact-build identity contract | `exact_build_identity_ref`, `related_exact_build_identity_refs`, `install_mode_class`, `install_channel_class` |
| Claim row | assurance-claim matrix | `claim_row_refs` on the object handoff and the linked packet header `coverage.requirement_ids` |
| Known-limit note | known-limit note contract | `known_limit_refs` on the object handoff and the claim row's bound known-limit refs |

A reconstruction is complete when each axis resolves to a typed id, a
typed ref, or a typed-absent token whose absence is itself admissible
under the checklist.

## Reconstruction procedure

Every drill case follows the same five steps. The steps are ordered
narrowest-safe-first: the reviewer reads typed ids before any summary
prose, and never reads prose that contradicts a typed field.

1. **Open the root packet.** The root is the exported
   `object_handoff_packet_record` or the exported
   `support_bundle_record`. The reviewer reads the packet header,
   record kind, and packet family class first so the later joins are
   scoped correctly.
2. **Resolve command and invocation.** Read `route_context.command_id`
   and `route_context.invocation_session_id`. Follow the
   `command_descriptor_ref` binding and the
   `invocation_session_ref` binding from `provenance_bindings` to the
   command descriptor and the invocation-session packet. Record the
   typed command-id and invocation-id as axis 1 resolved.
3. **Resolve route, target, origin, and exposure.** Read
   `route_context.action_origin_class`, `.action_target_class`,
   `.action_route_class`, and `.action_exposure_class` and the
   scope-and-boundary block. Cross-check against the linked
   route-packet ref (if present) to confirm the export mirrors the
   route truth rather than summarising it from memory.
4. **Resolve docs, build, and claim.** Read `build_context` for
   `exact_build_identity_ref`, `install_mode_class`,
   `install_channel_class`, `docs_pack_ref`,
   `docs_version_match_state`, and `claim_row_refs`. Follow each typed
   ref to the exact-build identity record, the docs-pack manifest, the
   destination descriptor, and the claim row. Record axes 3, 4, and 5
   as resolved.
5. **Resolve known-limit caveats and redaction.** Read
   `build_context.known_limit_refs` and the claim row's bound
   known-limit refs. Read `evidence_and_recovery_context.redaction_choice_class`
   and `withheld_artifact_refs`. Confirm every withheld item has a
   typed reason and that no known-limit narrowing is silently
   implied. Record axis 6 as resolved.

If any step resolves to an unexpected typed-unknown token or to no
token at all, the reviewer records the gap on the checklist and routes
the case to the matching escalation class below. A gap is never
resolved by invention, by reading source, or by paging an original
operator.

## Scenario classes

The drill covers four scenario classes. Every class carries its own
required-field set, derivable-field set, intentionally-absent set, and
escalation rule in the checklist. Each class is seeded with at least
one worked case under
[`/fixtures/support/reconstruction_cases/`](../../fixtures/support/reconstruction_cases/).

### Local-only action

The action ran entirely on-device with no managed control plane,
provider call, mirror, or offline import in scope. Route class is one
of the local-only origin/target/route values. Support-bundle
`support_export_posture` defaults to `local_only`. The reviewer MUST
resolve the command, invocation, local target identity, exact-build
identity, docs-pack ref (even when
`docs_version_match_state = local_offline_cache_current`), claim row,
and known-limit refs from the exported packet alone. The reviewer MUST
NOT expect a route-packet reference to a remote control plane.

### Provider-bearing action

The action crossed one or more provider boundaries (AI broker, managed
cloud, auth identity, remote connector, device-code browser handoff).
The route packet MUST be exported by reference or embedded with
redaction. The reviewer MUST resolve the provider boundary through the
typed `handoff_route_lane_class`, `handoff_transport_class`, and
`target_destination_descriptor_ref`, not through free-text prose. The
reviewer MUST confirm the claim row and known-limit note align with
the provider's support class — a provider-bearing case whose claim row
quotes a higher support class than the provider is typed as a support-
window alignment failure (see escalation rules).

### Mirrored or offline-import action

The action consumed an artifact that arrived through an approved
mirror, offline bundle, manual-import, or trust-root-rotation
distribution path. The reviewer MUST resolve the original
`exact_build_identity_ref`, the mirror-snapshot or manual-import
receipt ref, the docs-version-match state (expected to be
`mirror_snapshot_*` or `offline_bundle_*`), and the corresponding
emergency-distribution or revocation refs when the import is a
security-triage distribution. The reviewer MUST confirm the
known-limit note covers the narrowed support window of the imported
snapshot.

### Wrong-target or degraded-state action

The action was invoked against the wrong target, ran under a degraded
route posture, failed on helper attach, collided with a cached
trust-state approval, or produced a corrected-target event. The
reviewer MUST read the route packet's `route_change_reason_code` or
the object-handoff `current_state_rows` for the `route_posture`,
`trust_posture`, or `health` family rows, and MUST confirm that the
corrected target identity is typed, not summarised. Support-bundle
`recovery_rung_class` and `repair_transaction_refs` MUST be present
when the packet claims a repair path was offered.

## Field classification

Each scenario class classifies every reconstruction field as
**mandatory**, **derivable**, or **intentionally absent**. The
checklist is the machine form; the narrative rules below govern the
classes.

### Mandatory

Mandatory fields MUST be present on the exported packet. Absence is
an export defect and routes to
`reconstruction_gap_export_defect`. Mandatory fields include, at
minimum:

- `exact_build_identity_ref` on every scenario class;
- `command_id` and `invocation_session_id` on every scenario class
  whose command axis is not `command_id_not_applicable`;
- `docs_pack_ref` and `docs_version_match_state` on every scenario
  class;
- `redaction_choice_class` and the support-bundle
  `support_export_posture` for the packet class;
- at least one `claim_row_ref` OR a typed `claim_row_not_applicable`
  token whose absence is admitted by the checklist.

### Derivable

Derivable fields are not required to be present inline on the root
packet provided a typed ref resolves to the field on a linked packet.
Example: `action_target_class` on a mirror-import case is derivable
from the manual-import receipt when the object handoff cites the
receipt by ref and the receipt carries the target class. A
reconstruction does not fail when a derivable field is resolved
through exactly one hop of typed refs, but it does fail if the hop
requires reading prose, raw logs, or source code.

### Intentionally absent

Intentionally absent fields are excluded by contract. Examples:

- `invocation_session_id` on a docs-feedback handoff whose
  `report_intent_class` is `docs_feedback` and whose
  `object_kind_class` is `docs_pack_row` may resolve to
  `invocation_session_id_not_applicable`;
- `target_identity_ref` on a local-only action may resolve to
  `target_identity_ref_not_applicable`;
- `claim_row_ref` on a packet whose `support_packet_family_class` is
  explicitly admitted as claim-free by the support-packet index.

Intentionally-absent fields MUST carry the typed absent token. Silent
null or missing keys route to `reconstruction_gap_typed_absence_missing`.

## Escalation classes

When a reconstruction fails, the reviewer assigns one of the
following escalation classes and files it through the supportability,
release, or security review path named on the checklist. Every
escalation is a schema or packet follow-up, never an ad hoc
interpretation.

| Escalation class | Trigger | Routing |
|---|---|---|
| `reconstruction_gap_export_defect` | a mandatory field is missing from an exported packet | supportability: opens a support-bundle / object-handoff schema follow-up |
| `reconstruction_gap_typed_absence_missing` | an intentionally-absent field lacks its typed absent token | governance: opens a schema gate follow-up so the null becomes a typed token |
| `reconstruction_gap_join_unresolvable` | a typed ref cannot be resolved inside the export set | release-evidence: opens a packet-linkage follow-up so the referenced record travels with the export |
| `reconstruction_gap_claim_row_mismatch` | the resolved claim row contradicts the packet's support-class or channel posture | release: opens a claim-row or support-window alignment follow-up |
| `reconstruction_gap_known_limit_mismatch` | a known-limit note is superseded, stale, or missing for the bound claim | product / known-limits: opens a known-limit review rubric follow-up |
| `reconstruction_gap_route_truth_drift` | the route packet and the object handoff disagree on origin / target / route / exposure | runtime / commands: opens a route-taxonomy follow-up |
| `reconstruction_gap_exact_build_unresolvable` | the `exact_build_identity_ref` does not resolve to a known build identity record | build / release: opens a release-evidence follow-up so the build identity record travels with the export |
| `reconstruction_gap_redaction_unexplained` | `withheld_artifact_refs` or `redaction_choice_class` cannot be explained by the record-class registry or ADR-0007 | security: opens a redaction-profile or record-class follow-up |
| `reconstruction_gap_provider_boundary_drift` | the provider boundary claimed by the packet is not represented in the destination descriptor | security / release: opens a destination-descriptor follow-up |
| `reconstruction_gap_mirror_chain_broken` | the mirror snapshot, offline-bundle, or manual-import receipt chain cannot be walked end-to-end | security / support: opens an emergency-distribution or manual-import receipt follow-up |

A case that triggers two or more escalation classes MUST file one
follow-up per class. Escalations are not bundled.

## Integration with review paths

The drill is linked into three review paths and MUST be cited by name
whenever a reviewer produces one of the review outputs below.

- **Supportability review.** Support bundles, object-handoff packets,
  and field runbooks cite this drill when they claim an export can be
  reconstructed by a first-time reviewer. The drill is the honest
  demo. Missing mandatory fields or unresolvable joins surface as
  supportability follow-ups rather than unreviewable exports. See
  [`/docs/support/support_bundle_contract.md`](./support_bundle_contract.md)
  and [`/docs/support/object_handoff_packet.md`](./object_handoff_packet.md).
- **Release review.** Release evidence packets and shiproom review
  cite this drill when a claim row's support class, support window,
  or known-limit narrowing depends on a concrete reconstruction
  working from the released artifacts. Claim-row or known-limit
  mismatches discovered during the drill are release follow-ups. See
  [`/docs/release/assurance_claim_matrix.md`](../release/assurance_claim_matrix.md)
  and
  [`/docs/release/release_evidence_packet_template.md`](../release/release_evidence_packet_template.md).
- **Security review.** Incident workspaces, emergency-action
  distributions, manual-import receipts, and redaction profiles cite
  this drill when confirming an exported trust, provider-boundary,
  mirror-import, or redaction case can be reviewed from the export
  alone. Provider-boundary drift, mirror-chain breaks, and
  redaction-unexplained gaps route to security follow-ups. See
  [`/docs/security/emergency_action_model.md`](../security/emergency_action_model.md),
  [`/docs/security/emergency_distribution_policy.md`](../security/emergency_distribution_policy.md),
  and
  [`/docs/security/intake_and_triage.md`](../security/intake_and_triage.md).

## Acceptance for this revision

This revision seeds:

- four worked reconstruction cases under
  [`/fixtures/support/reconstruction_cases/`](../../fixtures/support/reconstruction_cases/),
  one per scenario class;
- one machine-readable checklist at
  [`/artifacts/support/reconstruction_checklist.yaml`](../../artifacts/support/reconstruction_checklist.yaml)
  with the field classification, escalation rules, and review-path
  bindings;
- one integration point per review path naming the drill as the
  honest reconstruction demo rather than a private script.

Each of the four seeded cases is reconstructable end-to-end from the
cited export artifacts without reading source code, replaying the
live system, or paging an original operator. Each case also declares
at least one deliberate gap (typed-absent token, redaction-by-design,
or by-reference linkage) so reviewers see how gaps remain honest
instead of becoming invisible.

## Change discipline

Adding a new scenario class, escalation class, or reconstruction axis
is additive-minor and requires a companion checklist update plus at
least one fixture case. Repurposing an existing axis or escalation
class is breaking and requires a new decision row in
[`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).

If a reconstruction gap recurs, the fix is a schema, packet, or
publication follow-up in the originating family — not a drill-local
label. The drill remains deliberately thin so escalations stay
visible.
