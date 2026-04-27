# Workspace bootstrap packet contract

This document freezes the packet Aureline emits after every clone,
import, open, restore, resume, template, prebuild, or recovery-checkpoint
entry path once the path has enough state to explain what happened. The
packet is the reconstructable join between:

- the source being acquired
  ([`source_locator_record`](../../schemas/workspace/source_locator.schema.json));
- the checkout/trust plan
  ([`checkout_plan_record`](../../schemas/workspace/checkout_plan.schema.json));
- any setup work
  ([`bootstrap_queue_item_record`](../../schemas/workspace/bootstrap_queue_item.schema.json));
- workspace entry-route truth
  ([`entry_route.schema.json`](../../schemas/workspace/entry_route.schema.json));
- route-change and mirror/public-route posture
  ([`action_origin_target_labels.yaml`](../../artifacts/runtime/action_origin_target_labels.yaml));
- typed reason codes
  ([`bootstrap_reason_codes.yaml`](../../artifacts/workspace/bootstrap_reason_codes.yaml)).

The machine-readable schema is
[`/schemas/workspace/bootstrap_packet.schema.json`](../../schemas/workspace/bootstrap_packet.schema.json).
Worked packet fixtures live beside the existing source-acquisition
fixtures under
[`/fixtures/workspace/bootstrap_cases/`](../../fixtures/workspace/bootstrap_cases/).

If this contract disagrees with the PRD, architecture, design, UI/UX
spec, or the source-acquisition seed, those sources win and this
contract plus the schema are updated in the same change.

## Why this exists

Locator, checkout-plan, and queue-item records already make source
acquisition inspectable. They do not, by themselves, give support,
automation, migration, and issue-handoff surfaces one object that says:

- which entry route the user took;
- which source identity and trust stage were current;
- whether acquisition used public origin, a customer mirror, an offline
  bundle, air-gapped media, managed cloud, or only local files;
- which credential handles were requested or consumed;
- which checkpoint makes resume or rollback possible;
- why the path failed, resumed, narrowed, or continued offline;
- which prerequisites still block trusted bootstrap after open;
- what may travel into support bundles, migration diff views, route
  audits, or object handoff packets.

Without this packet, each surface would reconstruct those answers from
different joins and would drift back toward free-form error strings such
as "setup failed" or "try again."

## Packet Shape

Every `bootstrap_packet_record` is summary-first and reference-first.
It carries stable refs and typed tokens, not raw source bodies.

| Block | Required job |
|---|---|
| `entry_context` | Entry verb, target kind, source surface, source locator ref, optional command or route-case ref. |
| `source_identity` | Locator ref, locator class, acquisition posture, freshness, signer continuity, digest/local/endpoint refs. |
| `checkout_context` | Checkout-plan ref, trust state, trust stage, browse-safe actions, blocked execution paths, destination and target refs. |
| `route_context` | Mirror/public route class, public-route posture, action-route class, route-change reason, mirror/manual-import/network refs. |
| `credential_handle_refs` | Opaque handle refs, handle class, projection mode, redaction class, and consumer ref. Raw credentials are forbidden. |
| `resumability` | Resume state, checkpoint refs, rollback refs, read-only partial roots, available actions, and reason codes. |
| `partial_failure` | Outcome class, failed stage, affected refs, and reason codes for partial, recoverable, denied, or mismatched paths. |
| `post_open_prerequisites` | Typed prerequisites that still block open, mutation, or trusted bootstrap. |
| `export_rules` | Destination-specific export posture and forbidden field classes for support, issue handoff, migration, route audit, CLI, and automation. |
| `reason_codes` | Packet-wide reason-code set from the governed artifact. |

The packet does not replace the underlying locator, plan, or queue items.
It is the durable reconstruction spine that carries their ids through
support and automation boundaries.

## Route And Source Identity

The packet preserves both the user-visible source class and the route
that actually served or resumed it.

`source_identity.locator_class` and
`source_identity.acquisition_posture` are copied from the
`source_locator_record`. `checkout_context.trust_stage` is copied from
the `checkout_plan_record`. Consumers must treat these as projections of
the authoritative records, not as a second source of truth.

`route_context.mirror_public_route_class` is required on every packet:

- `public_origin` means the acquisition used the public upstream route.
- `customer_managed_mirror` and `private_proxy` mean the route was
  narrowed through an organization-controlled route.
- `offline_bundle` and `air_gapped_media` mean no live origin freshness
  is implied.
- `managed_cloud` means live authority must be revalidated before a
  resume can attach.
- `local_filesystem` and `no_network_route` make local-only entry
  explicit.

`route_context.public_route_posture` is also required so mirror-only,
air-gapped, and offline continuations do not look like ordinary missing
network metadata.

## Reason Codes

Reason codes are the only admissible explanation for interrupted,
narrowed, offline, policy-denied, target-mismatch, missing-prerequisite,
or partial-failure bootstrap paths. Product copy, logs, support packets,
CLI summaries, issue handoffs, and automation reports resolve from the
same token set in
[`/artifacts/workspace/bootstrap_reason_codes.yaml`](../../artifacts/workspace/bootstrap_reason_codes.yaml).

Required families at this revision:

- interrupted clone/fetch;
- interrupted import/archive/bundle handling;
- interrupted live or cached resume;
- mirror-only, private-proxy, offline, or air-gapped fallback;
- offline local-only continuation;
- policy denial;
- target mismatch;
- missing post-open prerequisites;
- partial bootstrap queue failure.

A surface may add a redaction-safe summary, but it may not replace the
reason code with an untyped string. Adding a code is additive-minor;
repurposing a code is breaking.

## Resumability

Interrupted bootstrap paths expose explicit branches:

- `resume_acquisition` when a durable checkpoint can continue the same
  target;
- `discard_and_restart` when retry needs staging cleanup or rollback;
- `open_read_only_partial` when already materialized content is safe to
  inspect;
- `continue_offline` or `continue_in_restricted_mode` when local work
  can proceed without hidden authority;
- `locate_missing_target`, `reauth_required`, `refresh_mirror`, or
  `switch_to_live_origin` when a typed prerequisite must be resolved.

The packet records checkpoint refs and rollback refs as opaque ids.
It never records raw staging paths, raw pack contents, or raw archive
bytes.

## Post-Open Prerequisites

Post-open prerequisites are not failures by themselves. They are the
remaining blockers that separate source materialization from trusted
workspace readiness.

Examples:

- trust or signer review;
- credential-handle refresh or reauthentication;
- mirror refresh;
- target identity review;
- submodule init;
- LFS or partial-clone hydration;
- package restore, toolchain install, devcontainer attach, or extension
  restore;
- policy review, disk-space repair, or schema upgrade.

Each prerequisite names whether it blocks open, mutation, trusted
bootstrap, or only a deferred optional step.

## Export And Issue Handoff

Every packet carries destination-specific export rules. A packet may
export by reference, metadata-only, local-only ref, or review-required
posture, but it must preserve the same typed source, trust, route,
reason-code, checkpoint, and prerequisite truth across these surfaces:

- support bundle artifact manifest;
- object-specific issue handoff packet;
- migration diff and import-result views;
- workspace-entry route audit;
- CLI/headless export;
- automation run record;
- local debug packet.

Forbidden field classes are explicit: raw secrets, raw absolute paths,
raw remote URLs with credentials, raw archive bytes, raw provider
payloads, and raw policy bundles do not cross the packet boundary.
Credential data travels only as handle refs with handle class and
projection mode.

Object handoff packets cite a bootstrap packet through
`bootstrap_packet_ref`; support bundles cite it as a
`bootstrap_packet_reference` artifact row.

## Worked Cases

The seeded packet fixtures exercise the acceptance-critical branches:

- `resume_interrupted_mirror_clone__packet.yaml` — interrupted mirror
  clone with resume checkpoint, read-only partial root, mirror freshness,
  and submodule prerequisite.
- `live_resume_managed_workspace__packet.yaml` — managed resume blocked
  on expired authority and credential-handle refresh.
- `snapshot_archive_import__packet.yaml` — unsigned offline snapshot
  import with read-only staging and signer/attestation prerequisites.
- `policy_guided_deployment_generators_blocked__packet.yaml` — policy
  narrowed bootstrap queue with item-level failure refs.
- `target_mismatch_managed_resume__packet.yaml` — managed resume denied
  because the revalidated remote witness does not match the requested
  target.

## Acceptance

- Support and automation can reconstruct how a workspace entered
  Aureline from one packet id plus the referenced locator, plan, and
  queue items.
- Interrupted bootstrap paths resume, roll back, open read-only, or
  continue offline through typed reason codes and action classes.
- Mirror/public route, source class, trust stage, credential-handle refs,
  checkpoint refs, and post-open prerequisites survive support exports,
  issue handoffs, migration diffs, and route audits.
- Redaction removes raw secret or source bodies without erasing the
  reason-code, route, target, and prerequisite truth needed for repair.

## Source Anchors

- `.t2/docs/Aureline_Technical_Design_Document.md` Appendix BW and BZ
  for repository acquisition, bootstrap queue, mirror, resume, and
  hydration rules.
- `.t2/docs/Aureline_Technical_Design_Document.md` verification lane
  for bootstrap truth across local, mirror, and air-gap profiles.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` transport,
  mirror/offline, bootstrap executor, and credential-handle sections.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` entry-flow and
  mirror-first/offline vocabulary rules.
- `.t2/docs/Aureline_PRD.md` air-gapped and enterprise diagnostics
  requirements.
