# Notification routing seed: reviewer-facing entry to the envelope contract

This document is the **reviewer-facing entry** to the typed notification
envelope used by toasts, banners, status rows, durable activity-center items,
native OS notifications, lock-screen summaries, and companion fanout.

Its job is narrow: show a reviewer where the seed lives, how a single
background event flows through the schema, and how the failure drill — duplicate
or privacy-sensitive deliveries — keeps the **privacy class** and the
**action target** honest. The contract is normative; this seed is the entry
point and does **not** invent additional vocabulary.

Where this document disagrees with the UI / UX Spec or with the upstream
notification-envelope, action-grammar, attention-taxonomy, durable-job,
event-lineage, or OS-notification contracts, the source spec wins and this
seed plus its companion artifacts must change in the same patch.

## Companion artifacts

- [`/schemas/ux/notification_envelope.schema.json`](../../schemas/ux/notification_envelope.schema.json)
  defines `notification_envelope_record`, the privacy-class rule rows, and the
  embedded `fanout_receipt_record`. The schema is the boundary contract.
- [`/docs/ux/notification_envelope_contract.md`](./notification_envelope_contract.md)
  freezes the envelope anatomy, privacy-class rules, fanout-receipt rules, and
  non-conforming cases.
- [`/fixtures/ux/notification_envelope_cases/`](../../fixtures/ux/notification_envelope_cases/)
  contains worked envelope fixtures covering the protected walk and the
  failure drill.

## Upstream contracts (composed, not replaced)

This seed composes with existing owners and does not replace them:

- [`attention_activity_taxonomy.md`](./attention_activity_taxonomy.md) owns the
  delivery-surface, attention, interruptibility, quiet-hours, suppression,
  payload-redaction, dedupe-key, reopen-target, and source-subsystem
  vocabularies the envelope re-uses.
- [`notification_action_grammar.md`](./notification_action_grammar.md) owns the
  stable verbs (`dismiss`, `snooze`, `acknowledge`, `resolve`, `archive`,
  `mute_source`, `reopen`) the envelope's `actions[]` bind to.
- [`notification_delivery_contract.md`](./notification_delivery_contract.md) and
  [`/schemas/ux/event_lineage.schema.json`](../../schemas/ux/event_lineage.schema.json)
  own canonical event lineage and cross-surface delivery steps.
- [`durable_job_envelope_contract.md`](./durable_job_envelope_contract.md) owns
  durable-job rows and their linkage to notification deliveries.
- [`os_notification_and_quiet_hours_contract.md`](./os_notification_and_quiet_hours_contract.md)
  owns suppression-record, payload-redaction, and lock-screen-summary
  affordance vocabularies.

## Who reads this seed

- **Shell, activity-center, OS-shim, and companion authors** reaching for the
  envelope before they wire toasts, banners, status rows, OS notifications, or
  companion mirrors. Bind behavior to `notification_envelope_id`,
  `canonical_event_id`, `dedupe_key_ref`, `reopen_target`, `actions[].action_id`,
  and `actions[].command_id`; never bind to `summary_label` or
  `actions[].label`.
- **Support, evidence, and parity-audit tooling** joining notification truth
  across surfaces by `canonical_event_id` and `event_lineage_id_ref` and
  classifying export posture by `privacy_class`, `privacy_payload_class`, and
  `redaction_class`.
- **Reviewers** confirming that durable attention truth survives quiet hours,
  duplicate emissions, and privacy-sensitive payloads without losing the right
  reopen target.

## The envelope at a glance

A `notification_envelope_record` is the joinable, machine-readable object
behind every notification family surface. Its required fields fall into four
groups:

1. **Identity and lineage.** `notification_envelope_id`, `canonical_event_id`,
   `event_lineage_id_ref`, `source_subsystem`, `source_event_ref`,
   `actor_identity_ref`, and `canonical_object_target_ref` make the same event
   joinable across desktop, OS, and companion surfaces without minting
   per-surface ids.
2. **Privacy and severity.** `privacy_class` is the coarse, stable policy
   gate; `privacy_payload_class` and `redaction_class` are the concrete
   enforcement knobs adapters and exports must respect. `severity_class` is
   the typed severity vocabulary used for routing and escalation.
3. **Routing and dedupe.** `recommended_surfaces[]` is the typed surface set
   routers consume. `dedupe_key_scheme` + `dedupe_key_ref` (and the optional
   `grouped_burst_id_ref`) collapse retries and bursts. `suppression_state`
   carries quiet-hours/admin facts at mint time.
4. **Action target.** `reopen_target` resolves to one canonical target kind
   and identity; `actions[]` carries stable action ids and command ids whose
   labels remain localizable. A surface that activates an action MUST resolve
   the same `reopen_target` regardless of which surface fired it.

Localized copy (`summary_label`, `actions[].label`,
`reopen_target.placeholder_announcement_label`) is constrained only by the
privacy-safe label rules; semantics never come from copy.

## The protected walk

> **Trigger a background event → inspect its typed notification envelope →
> verify privacy class and reopen target are defined.**

1. **Trigger.** A background subsystem (package update, indexer, terminal
   recovery, review-and-diff) emits an event with a stable
   `canonical_event_id` and a closed `source_subsystem`.
2. **Mint.** The notification router mints a `notification_envelope_record`
   with the event's lineage, an explicit `privacy_class` (and concrete
   `privacy_payload_class` / `redaction_class`), a typed
   `recommended_surfaces[]` set, a `reopen_target` whose
   `reopen_target_kind` names a canonical-object/route/review/durable-row
   destination (never a generic home screen), and at least one
   command-backed `stable_action`.
3. **Fanout.** Each delivery attempt emits a `fanout_receipt_record` linked
   back to the envelope by `source_notification_envelope_id_ref`,
   `canonical_event_id`, and `event_lineage_id_ref`. Held, suppressed,
   deduped, and no-route outcomes still emit a receipt — silent drops are
   non-conforming.
4. **Reopen.** Activating any surface (toast, OS notification, lock-screen
   summary, companion push, durable row) resolves the same
   `reopen_target.exact_target_identity_ref`. A receipt whose
   `reopen_target_ref` lands on a generic home surface is non-conforming.

The worked example backing this walk is
[`/fixtures/ux/notification_envelope_cases/simple_cross_surface_completion.json`](../../fixtures/ux/notification_envelope_cases/simple_cross_surface_completion.json):
a completed package update with `privacy_class = summary_safe`, a
`canonical_object` reopen target, and delivered fanout receipts on the toast
and OS-notification surfaces.

## The failure drill

> **Emit duplicate or privacy-sensitive notifications and confirm the envelope
> still carries the right privacy class and action target.**

The drill exercises two compounding hazards the envelope must absorb without
losing routing truth:

### 1. Duplicate emissions

A retry storm or repeat event MUST collapse on `dedupe_key_scheme` +
`dedupe_key_ref` (or `grouped_burst_id_ref` for bursts) without minting a new
`canonical_event_id`. The collapsed envelope MUST keep the same
`privacy_class`, `privacy_payload_class`, `redaction_class`, and
`reopen_target`. Each suppressed delivery STILL records a
`fanout_receipt_record` whose `receipt_state` is `deduped_canonical_event` or
`deduped_grouped_burst` and whose `dedupe_key_scheme` matches the envelope —
so "deduped" remains visible truth.

The worked example is
[`/fixtures/ux/notification_envelope_cases/duplicate_dedupe_envelope.json`](../../fixtures/ux/notification_envelope_cases/duplicate_dedupe_envelope.json):
four indexer-warning emissions collapse to one envelope with stable
`privacy_class = workspace_sensitive` and one `canonical_object` reopen
target; deduped-receipt rows preserve the join keys.

### 2. Privacy-sensitive payloads

A `security_critical` or `managed_sensitive` envelope MUST select a
narrower `privacy_payload_class` (typically `redacted_metadata_only` or
`policy_forbidden_on_lock_screen`) and a narrower `redaction_class`
(typically `internal_support_restricted` or `signing_evidence_only`).
Adapters MUST NOT widen the rendered payload past the envelope's class;
support exports MUST NOT export past the envelope's `redaction_class`.
Lock-screen and companion surfaces denied by policy still emit a receipt
with `receipt_state = suppressed_policy` and the suppression reason — silent
denial is non-conforming.

The worked example is
[`/fixtures/ux/notification_envelope_cases/security_critical_redacted_lock_screen.json`](../../fixtures/ux/notification_envelope_cases/security_critical_redacted_lock_screen.json):
a security advisory with `privacy_class = security_critical`,
`privacy_payload_class = redacted_metadata_only`, delivered banner + OS
notification, and a denied lock-screen receipt.

A held-by-quiet-hours companion case is covered by
[`/fixtures/ux/notification_envelope_cases/held_quiet_hours_companion_fanout.json`](../../fixtures/ux/notification_envelope_cases/held_quiet_hours_companion_fanout.json).

## Background-task and recovery seeds

The acceptance shape — *at least the first background-task and recovery
notifications can be represented through the schema* — is met by:

- **Background task.** `simple_cross_surface_completion.json` — package
  update completion routed to toast, status item, and OS notification with
  matching delivered receipts.
- **Recovery.** `recovery_after_terminal_reconnect.json` — a terminal pane
  recovers from a lost transport on the same opaque session id; the envelope
  carries a `success` severity, a `summary_safe` privacy class, and a
  `durable_activity_row` reopen target so the recovery survives a reload
  without replaying side effects.

## Non-conforming patterns

The following remain non-conforming under this seed:

- per-surface notification ids that cannot be joined across desktop, OS, and
  companion;
- OS, lock-screen, or companion payloads that widen beyond the envelope's
  `privacy_class` / `privacy_payload_class`;
- support-export sinks that export beyond the envelope's `redaction_class`;
- silent drops on quiet hours, admin policy, dedupe, or no-route delivery;
- repeated deliveries that mint new `canonical_event_id`s instead of deduping;
- reopen activation that lands on a generic home screen rather than the
  canonical object/route/review/durable-row target;
- actions whose behavior is bound to `summary_label` or `actions[].label`
  rather than `action_id` + `command_id`.

## Evidence

Validation-lane evidence for this seed lives under
[`/artifacts/milestones/m1/`](../../artifacts/milestones/m1/) and is registered
in
[`artifact_index.yaml`](../../artifacts/milestones/m1/artifact_index.yaml)
under the `notification_envelope_seed` lane. Refresh whenever the schema, the
contract doc, the protected-walk fixture, the failure-drill fixtures, or the
recovery fixture change.
