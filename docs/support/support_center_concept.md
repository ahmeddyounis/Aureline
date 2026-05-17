# Support Center concept

The Support Center is Aureline's local-first home for diagnosis,
recovery, evidence review, and escalation. It is not a generic "reset"
drawer, not a background upload wizard, and not a second settings page.
Its job is to make blocked-user recovery truthful, previewable, and
auditable across desktop and headless flows.

This note reserves the concept and object model. It does not promise a
fully integrated UI in one step. Early implementation can land as linked
inspectors, commands, and export flows so long as they preserve the same
contracts described here.

## Purpose

The Support Center exists so Aureline can answer, in-product and without
guesswork:

- what is broken;
- why Aureline believes that;
- what can be repaired safely right now;
- what evidence will leave the machine if the user escalates; and
- which build, docs, route, target, and known-limit truths apply to the
  current failure.

## Non-goals

- a hidden factory-reset button as the primary recovery story;
- automatic upload of diagnostics or code-bearing evidence;
- ad hoc troubleshooting text that cannot be tied back to stable finding
  codes, repair IDs, or export manifests;
- a support surface that works only when the full UI is healthy.

## Design principles

- Diagnosis is read-only by default.
- The narrowest safe repair comes first.
- Local-only recovery is equal in prominence to export or escalation.
- Safe mode, bisect, quarantine, cache reset, restricted reopen, and
  rollback are different recovery rungs and must stay distinct.
- Exact-build, docs/help version truth, route truth, and known limits are
  first-class support surfaces.
- Every export or handoff shows included, excluded, and redacted classes
  before it leaves the machine.

## Core areas

| Area | Primary job | Must surface |
|---|---|---|
| **Project Doctor** | run versioned probes and explain findings | stable finding code, probe version, severity, confidence, scope, evidence refs, repair-candidate IDs |
| **Recovery ladder** | reopen or continue work with the smallest safe blast radius | safe mode, bisect, quarantine, cache-reset candidate, restricted reopen, rollback candidate |
| **Repair preview** | turn "fix" into a reviewed transaction | changed and unchanged state classes, preconditions, checkpoint path, reversal class, verification plan |
| **Support-bundle preview** | preview evidence before export | file/section list, data class, redaction state, origin of each artifact, exact-build identity, finding IDs |
| **Artifact fidelity** | explain build, symbolication, docs, and claim truth | exact-build identity, channel/profile, docs/help version-match state, symbol/source-map state, known limits |
| **Issue handoff** | create contextual escalation packets from the object that failed | object ID, route or target context, build/docs truth, related findings or repair history |

## Project Doctor

Project Doctor is the Center's primary diagnosis surface. It should:

- use stable probe families and finding codes;
- preserve the same machine-readable fields in desktop and headless
  flows;
- emit repair candidates without mutating anything during diagnosis; and
- keep unsupported or low-confidence states explicit instead of hiding
  them behind generic advice.

The Center should treat Doctor output as the canonical diagnosis layer.
Other panes may quote or filter it, but should not mint parallel finding
names.

## Safe mode and the recovery ladder

Safe mode is a published runtime profile, not a vague fallback state.
Inside the Support Center it should appear as one rung in a broader
recovery ladder:

1. Safe mode
2. Extension bisect
3. Quarantine
4. Cache-reset candidate
5. Restricted reopen
6. Rollback or reinstall candidate

For each rung, the Center should show:

- entry reason;
- disabled or narrowed classes;
- preserved capabilities;
- whether the current session is already inside that rung; and
- the next safer or broader step.

## Support-bundle preview

Bundle export should begin with preview, not upload. The preview should
show:

- what will be included;
- what is excluded by default;
- what was redacted;
- which artifacts were collected locally versus generated on demand;
- which shared `record_class_id` each artifact resolves to, and whether
  the item is local-only, a managed reference, an export packet, or a
  receipt reference;
- which build, finding, and repair identifiers remain visible even after
  redaction.

The governed machine-readable bundle contract for that preview lives in
[`docs/support/support_bundle_contract.md`](./support_bundle_contract.md)
and
[`schemas/support/support_bundle.schema.json`](../../schemas/support/support_bundle.schema.json).

The Center should treat bundle preview as a review surface, not a final
confirmation dialog. If policy prevents adding or removing an item, the
user should see that in the preview itself. The preview quotes the
shared record-class registry
([`docs/governance/record_class_governance.md`](../governance/record_class_governance.md))
so support exports do not invent their own retention or offboarding
labels.

## Support intake, scenario picker, and escalation packet

The Support Center's symptom-to-evidence routing runs through the
support-intake contract in
[`docs/support/support_intake_and_escalation_contract.md`](./support_intake_and_escalation_contract.md)
and the schemas
[`schemas/support/scenario_picker.schema.json`](../../schemas/support/scenario_picker.schema.json)
and
[`schemas/support/escalation_packet.schema.json`](../../schemas/support/escalation_packet.schema.json).
The picker pins six scenario families, the four required intake
surfaces (picker, builder, review, timeline), the seven capability
cards (Doctor, safe mode, bisect, support bundle, crash triage,
guided repair, issue escalation), the closed approved/forbidden fix
sets, the local-only delivery prominence pin, and per-environment
parity rows. The escalation packet preserves stable scenario family,
finding ids, build/profile identity, deployment class, evidence ids,
reproduction steps, recommended-repair review rows, and per-context
parity so a user does not restate their case after handoff.

## Object-specific issue handoff

"Report issue" should preserve the object that failed. The Support Center
should prefer contextual handoff packets over generic free-text reports.

Recommended object types:

| Object type | Minimum handoff context |
|---|---|
| **Command or task** | `command_id`, `invocation_session_id`, palette row or modifier-action ref, route context, target identity, exact-build identity, finding codes |
| **Extension or runtime host** | extension/host id, version, permission or runtime class, quarantine/bisect state, exact build |
| **Route or remote target** | origin/target/route/exposure classes, target identity ref, approval or authority linkage, drift or expiry state |
| **Docs/help or known-limit row** | docs-pack ref, docs/help version-match state, page or citation ref, exact build |
| **Benchmark or compatibility claim** | packet/report ref, reference-workspace report row ref when applicable, freshness timestamp, exact build, known-limit ref, docs-version truth |
| **Crash or repair object** | crash id or repair id, checkpoint refs, reversal class, symbol/source-map fidelity state |

The goal is not to auto-classify everything perfectly. The goal is to
preserve enough object identity that the next human does not start from a
blank page.

The concrete packet and route contract for these handoffs lives in
[`docs/support/object_handoff_packet.md`](./object_handoff_packet.md)
and
[`schemas/support/object_handoff_packet.schema.json`](../../schemas/support/object_handoff_packet.schema.json).
Command-origin handoffs also preserve the combined palette row contract
from
[`docs/commands/palette_row_and_modifier_contract.md`](../commands/palette_row_and_modifier_contract.md)
so support packets can reconstruct origin badge, disabled reason,
automation posture, shortcut hint, and no-bypass modifier semantics
without inventing support-only wording.

## Repair-preview direction

Every repair launched from the Center should use one transaction grammar:

- review;
- preview;
- checkpoint;
- apply;
- verify;
- rollback or compensate.

The preview must show both changed and unchanged state classes. When no
exact rollback exists, the Center should say so before apply and prefer
export or escalation over bluffing a safe reset.

## Exact-build and docs/version truth

Support work routinely fails when build truth and docs truth drift apart.
The Support Center should keep both visible on the same surface.

Minimum build and docs truth surfaces:

- exact-build identity ref;
- channel and install/profile class;
- symbol/source-map fidelity or mismatch state;
- docs-pack or help-manifest revision;
- docs/help version-match state;
- known-limit refs that already cover the current behavior;
- proof refs for benchmark, compatibility, reference-workspace, or
  migration claims when the issue touches a claim-bearing surface.

This lets the Center answer "is the product broken?" separately from "is
the current docs/help truth stale?".

## Minimum reconstruction path

Any escalation packet or exported issue handoff should preserve the
minimum path needed to reconstruct what happened:

1. **What was attempted:** `command_id`, surface/object id, and
   `invocation_session_id` where applicable.
2. **Where it ran:** `action_origin_class`, `action_target_class`,
   `action_route_class`, `action_exposure_class`, and target identity.
3. **What build was involved:** `exact_build_identity_ref`, channel, and
   install/profile identity.
4. **What truth surfaces applied:** docs/help revision, docs-version
   match state, known-limit refs, and dependency-marker refs.
5. **What Aureline believed:** finding codes, probe versions, severity,
   confidence, and evidence refs.
6. **What the user tried next:** recovery rung entered, repair IDs,
   checkpoint refs, and reversal class.
7. **What proof or claim context mattered:** benchmark packet,
   compatibility report, reference-workspace report row,
   compatibility-row refs, migration session / outcome packet / report
   refs, or claim packet refs when relevant.

If any field is unknown, the packet should carry a typed unknown value
and the export path should stay honest about that gap.

## Delivery direction

The Center can mature in layers:

- first as stable headless and inspector contracts for Doctor, bundle
  preview, and recovery rungs;
- then as linked desktop surfaces with shared identifiers and previews;
- finally as a consolidated support surface once the underlying contracts
  are already trustworthy.

That sequence keeps Aureline from shipping a polished shell over
non-reconstructable support flows.

## Information architecture and route table

The Support Center information architecture, capability-card field
contract, and symptom-to-module route table live in
[`docs/support/support_center_information_architecture.md`](./support_center_information_architecture.md),
[`schemas/support/support_center_capability_card.schema.json`](../../schemas/support/support_center_capability_card.schema.json),
and
[`artifacts/support/support_center_routes.yaml`](../../artifacts/support/support_center_routes.yaml).
Together they pin the closed nine top-level Support Center modules
(Project Doctor, safe mode, bisect or quarantine, support bundle,
crash triage, guided repair, issue or escalation handoff, advisory
or incident history, field diagnostics), the closed six symptom-
surface classes (error, blocked-action, crash-loop, update-failure,
policy-denial, transport-failure), the closed five deployment-context
classes (local-only, managed, self-hosted, mirrored, offline), the
no-upload-first first-action invariant, and the rule that every
route preserves stable evidence-id classes for later escalation
packets. The seven intake-bound capability cards listed under
[`support_intake_and_escalation_contract.md`](./support_intake_and_escalation_contract.md)
align 1:1 with the IA modules' `aligned_intake_capability_class`;
advisory or incident history and field diagnostics are IA-only
modules that have no intake-bound capability and resolve to read-
only review and read-only diagnosis surfaces.
