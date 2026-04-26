# Degraded-mode pattern, lifecycle-status cards, and last-failure recovery contract

This document defines the shared degraded-mode template and lifecycle-
status card family for Aureline surfaces that remain partially usable
while a subsystem is warming, reconnecting, narrowed by policy, serving
stale evidence, or otherwise below its ideal posture.

It composes with, and does not replace:

- [`state_and_recovery_taxonomy.md`](./state_and_recovery_taxonomy.md)
  for failure-tier placement, recovery-surface mapping, and the existing
  lifecycle row ids.
- [`truth_and_degraded_state_vocabulary.md`](../governance/truth_and_degraded_state_vocabulary.md)
  for truth classes and degraded-state tokens.
- [`session_authority_contract.md`](../collaboration/session_authority_contract.md)
  for collaboration downgrade behavior and the rule that relay loss does
  not freeze local editing.
- [`live_update_review_contract.md`](./live_update_review_contract.md)
  for stale, frozen, and snapshot review posture.
- [`transport_and_environment_status_contract.md`](./transport_and_environment_status_contract.md)
  for inspectable transport posture and repair cards.

Boundary schema and fixtures:

- [`/schemas/ux/lifecycle_status_card.schema.json`](../../schemas/ux/lifecycle_status_card.schema.json)
  defines the machine-readable card record.
- [`/fixtures/ux/degraded_examples/`](../../fixtures/ux/degraded_examples/)
  contains worked examples for warming, reconnecting, no-recording,
  policy-blocked, stale-evidence, and update recovery states.

## Purpose

A degraded surface is not a generic failure state. It is a truthful
partial-capability state with four obligations:

1. Name what still works.
2. Name what is reduced, paused, read-only, or less certain.
3. Show the safest recovery action.
4. Keep the last-failure reason reachable and exportable.

Any protected surface that collapses those axes into "broken",
"unavailable", "error", or a blank state is non-conforming when a more
precise state is known.

## Degraded-mode template

Every degraded-mode surface renders a lifecycle-status card or a card-
equivalent row with the following slots. The slots may be compressed in
compact chrome, but the backing record keeps them distinct.

| Slot | Required content | Non-conforming collapse |
| --- | --- | --- |
| `status_label` | One controlled user-visible label from this contract. | Replacing `Warming`, `Reconnecting`, `No recording`, `Policy blocked`, or `Stale evidence` with generic `Degraded`. |
| `lifecycle_state` | Object-family state such as `workspace.partially_ready`, `remote.reconnecting`, or `ai.retest_pending`. | Inferring state from side effects or logs without recording the state. |
| `what_still_works` | Preserved capabilities, especially local editing where the source contract says it survives. | Hiding usable local work behind a full-screen failure. |
| `what_is_reduced_or_paused` | Narrowed capability and cause class. | Saying only "some features may not work". |
| `data_or_certainty_effect` | Whether data is cached, stale, partial, inferred, read-only, or missing a recording. | Presenting stale or inferred evidence as current authoritative truth. |
| `last_failure` | Failure class/code ref, safe summary, timestamp pair, and recovery history when a failure exists. | Discarding the reason on dismiss or requiring pointer-only hover to reveal it. |
| `primary_recovery_action` | Safest next action routed through review. | Resetting, deleting caches, dropping session state, or retrying a mutation without review. |
| `inspect_path` | Keyboard-reachable route to details. | Details available only by hover, pointer gesture, raw logs, or support-only export. |
| `support_export` | Export field refs and redaction class. | Exporting prose that loses the status token, reason class, or recovery history. |

## Controlled status labels

The label set below is frozen for lifecycle-status cards. The backing
record carries both the rendered label and a stable token. Labels are
not synonyms; they may not be collapsed into each other when their
semantics differ.

| Label | Token | Use when | Must not collapse when |
| --- | --- | --- | --- |
| `Partially ready` | `partially_ready` | The object is usable but a named scope or provider is still incomplete. | The missing scope affects search, extension activation, remote readiness, or support claims. |
| `Degraded` | `degraded` | The object is operating with a reduced provider, fallback, quarantine, or failed apply path. | A more precise label below applies. |
| `Read-only degraded` | `read_only_degraded` | Reads remain available but authoritative writes are blocked before commit. | Any mutation path would silently no-op or queue without review. |
| `Warming` | `warming` | Background preparation is still in progress and richer results will arrive later. | Partial results, cached data, or progress are visible. |
| `Reconnecting` | `reconnecting` | A transport/session owner is actively trying to restore continuity. | Local work continues but remote/collab propagation is paused. |
| `Policy blocked` | `policy_blocked` | Admin, trust, kill-switch, deployment-profile, or local policy denies an operation. | The policy source, admin route, or allowed local fallback differs from other degraded paths. |
| `No recording` | `no_recording` | A session remains active but recording/transcript/audit capture is absent or disabled. | Presence or local editing still works; missing recording must not be reported as relay failure. |
| `Stale evidence` | `stale_evidence` | The visible decision rests on evidence past its freshness floor. | Stale evidence would affect release, AI, support, or recovery certainty. |

### Collapse rules

Status labels may collapse only when all of the following are true:

- the lifecycle object is healthy or terminal with no degraded user
  choice remaining;
- no local-editing, write-safety, recording, policy, data-freshness, or
  certainty axis changes;
- there is no last-failure reason that the user could act on; and
- the support/export row preserves the more specific status elsewhere.

Cards using any label in the table above set
`collapse_policy.may_collapse_to_generic = false`.

## Lifecycle-status card family

Each long-lived object family has one status card shape. The card may
render as a banner, status item drawer, activity card, repair card, or
workflow sheet, but it uses the same record fields and safety contract.

| Family | Typical labels | Preserved capability | Reduced or paused capability | Recovery examples |
| --- | --- | --- | --- | --- |
| Workspace | `Partially ready`, `Warming`, `Degraded`, `Read-only degraded` | Editing already-open trusted files, save journal, palette, local Git when available. | Index, watcher fidelity, extension host, managed workspace attach, or write authority. | Repair cache/index, enter safe mode, continue restricted, open without restore. |
| Extension | `Warming`, `Degraded`, `Policy blocked` | Shell stability and local workspace state. | Extension activation, privileged permission, network egress, or contributed commands. | Reload extension, quarantine, open extension details, request policy review. |
| Remote session | `Reconnecting`, `Read-only degraded`, `Policy blocked` | Local buffers, recovery journal, cached/read-only remote inspection where safe. | Remote writes, task runner, forwarded ports, remote VCS refs, or authority renewal. | Reconnect, reauthenticate, continue locally, open without restore. |
| Collaboration session | `Reconnecting`, `No recording`, `Degraded`, `Policy blocked` | Local editing for every participant, local drafts, visible session history markers. | Presence, relay propagation, follow state, recording, transcript, or session archive completeness. | Rejoin, continue without recording, export diagnostics, escalate policy. |
| AI action | `Warming`, `Stale evidence`, `Degraded`, `Policy blocked` | Draft prompt, cited context already gathered, patch review surface, local editing. | Context completeness, evidence freshness, provider/tool execution, or apply authority. | Refresh evidence, retry narrower scope, reveal citations, review or reject patch. |
| Update / rollback | `Warming`, `Degraded`, `Stale evidence`, `Policy blocked` | Current working binary, workspace data, rollback checkpoint, release notes inspection. | Forward apply, confirmation, evidence freshness, staged update, or channel action. | Retry, roll back, boot previous version, enter safe mode, reveal last failure. |

### Local-editing survival

Local editing remains available whenever the source contract says it
survives degradation. In particular:

- workspace `partially_ready` and `degraded` states keep editing of
  already-open trusted files available;
- remote `reconnecting` and `read_only_degraded` states preserve local
  buffers and local-only continuation;
- collaboration `participant_degraded`, `shared_degraded`, and
  `no_recording` states must not freeze participant buffers;
- AI degraded states cannot block unrelated local editing or bypass
  patch review to regain full capability; and
- update or rollback failures preserve the current working binary and
  workspace data until the user reviews a recovery path.

If a local edit path is actually unsafe, the card must say which path is
blocked and why. It must not imply that all local work is unavailable.

## Last-failure recovery contract

Every lifecycle-status card follows the same last-failure contract:

1. **Visibility.** The primary state surface either shows the last-
   failure summary directly or exposes a keyboard-reachable
   `Reveal last failure` / inspect action that opens an immediate
   detail surface.
2. **Attribution.** The record carries a typed reason class, reason-code
   ref, monotonic and wall-clock timestamp pair where available, and a
   redacted summary. Raw credentials, raw logs, raw prompts, raw paths,
   and raw external payloads do not cross this boundary.
3. **Recovery history.** Recovery attempts are preserved by rung or
   action ref so support, timeline, and export readers can reconstruct
   what already happened.
4. **Safe review.** Recovery actions that mutate state route through a
   review surface. Retrying a failing operation, clearing caches,
   quarantining an extension, applying an AI patch, or rolling back an
   update cannot discard current work as a shortcut.
5. **Preservation.** Recovery actions set
   `preserves_current_work = true`,
   `routes_through_safe_review = true`, and
   `discards_session_state = false` in the schema.

## Inspect path and keyboard access

The inspect path is part of the contract, not a convenience affordance.
It names:

- the detail surface class;
- the command id or focus target that opens it;
- the accessible name announced for the status row;
- whether the detail surface opens without pointer input; and
- the support/export fields that preserve the same facts.

Pointer-only hover text, hidden debug logs, toast-only recovery, or
support-only diagnostics do not satisfy the inspect-path requirement.

## Data and certainty disclosure

A degraded card must say whether the user is seeing:

- live authoritative data;
- cached data and its freshness;
- partial scope;
- stale evidence;
- AI-inferred content whose citations are incomplete;
- read-only data because write authority is missing;
- a recording/transcript gap; or
- policy-withheld data.

This disclosure is required even when the visual surface is compact. The
backing record carries `data_or_certainty_effect` so support exports and
automation do not infer stronger certainty than the UI showed.

## Fixture requirements

Fixture examples under
[`/fixtures/ux/degraded_examples/`](../../fixtures/ux/degraded_examples/)
must:

- validate as `lifecycle_status_card_record` examples;
- keep `warming`, `reconnecting`, `no-recording`, `policy-blocked`, and
  `stale evidence` as distinct user-visible states (with stable
  `status_label_token` values in the schema);
- name preserved and reduced capability sets;
- expose a keyboard-reachable inspect path;
- preserve the last-failure reason on support export; and
- assert that recovery actions preserve current work and route through
  safe review.

## Acceptance checklist

A reviewer can accept a lifecycle-status card when all of these are true:

1. The rendered label is one of the controlled status labels and is not
   collapsed into a broader label when the axes differ.
2. `what_still_works` explicitly names local editing or local-only
   continuation whenever it survives the degraded state.
3. `what_is_reduced_or_paused` names the narrowed capability and cause.
4. `data_or_certainty_effect` says whether data or certainty changed.
5. Last-failure details are visible on the primary surface or reachable
   by keyboard from an immediate detail surface.
6. The primary recovery action preserves current work, routes through
   review, and does not discard session state.
7. The support-export fields preserve status token, reason class,
   reason code ref, recovery history, and redaction class.
