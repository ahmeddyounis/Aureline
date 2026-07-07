# M5 provider offline-capture / privacy-redaction row primitive

This contract implements two reusable M5 provider primitives — the **offline-capture row** and
the **privacy/redaction row** — so a prepared provider handoff stays survivable and
privacy-safe when live provider access narrows or disappears. It narrows the last two families
from the frozen
[provider-account / offline-capture component matrix](../../schemas/ui/m5-provider-account-offline-capture-component-matrix.schema.json)
into two resolvers, and is implemented under
`crates/aureline-provider/src/implement_offline_capture_rows_and_privacy_redaction_rows_with_packet_destination_queued_draft_count_export_clear_actions_and_metadata_safe_boundary_truth_across_claimed_m5_provider_workflows/`.

- Boundary schema: [`schemas/ui/m5-provider-offline-capture-privacy-redaction-row.schema.json`](../../schemas/ui/m5-provider-offline-capture-privacy-redaction-row.schema.json)
- Support export: `artifacts/release/m5-provider-offline-capture-privacy-redaction-row-primitive-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-provider-offline-capture-privacy-redaction-row-primitive-proof/matrix.csv`
- Design report: `artifacts/design/m5-provider-offline-capture-privacy-redaction-row-primitive.md`
- Narrowed fixtures: `fixtures/ui/m5-provider-offline-capture-privacy-redaction-row-primitive/`

## Why

M5 cannot honestly claim provider-backed team-workflow continuity if a loss of provider
connectivity erases prepared handoff state, hides what remains queued locally, or lets a
metadata-safe export/support default go unstated before it leaves the device. These two rows
close that gap on top of the already-claimed M5 provider workflows.

## Offline-capture row

`resolve_offline_capture_row` takes one captured packet's offline-capture state, its capture
kind (bug report, task update, or blocked-work note), its packet-destination class, its
queued-draft state, its redaction default, and its queued-draft count, and derives:

- **Row posture** — one-to-one from the frozen offline-capture state, so the six states never
  collapse into one generic "queued" chip.
- **Publish-later behavior** — `publishes_when_reachable`, `held_locally_until_publish`,
  `held_by_user_choice`, `held_pending_conflict`, `will_discard_on_confirm`, or
  `already_published`, so a user never has to guess what the packet does when connectivity
  returns.
- **Packet destination** — the destination class is `routed_to_provider`, `local_bundle_only`,
  or `unrouted_pending`; `shows_packet_destination` is false only for an unrouted packet, which
  is flagged rather than defaulted (`assumes_default_destination_silently` is always `false`).
- **Queued-draft count** — always carried; `has_queued_drafts` is true whenever the count is
  positive. Prepared handoff state is always retained and the queue is never hidden
  (`retains_prepared_handoff` always `true`, `hides_queued_local_work` always `false`).
- **Export / clear actions** — reveal and export are always offered; defer when the packet is
  queued for publish; retry when a publish is blocked or failed; clear whenever the capture is
  not already synced and cleared. A cleared, already-synced capture reporting queued drafts is
  rejected (`ClearedCaptureHasQueuedDrafts`).

## Privacy/redaction row

`resolve_privacy_redaction_row` takes one provider-linked object's redaction class, its export
boundary, its policy source, and its telemetry/event limit, and derives:

- **Row posture** — one-to-one from the frozen redaction class, so a full-body-visible row
  never reads the same as a metadata-only, redacted, policy-restricted, raw-withheld, or
  no-export row.
- **Copied / exported fields** — the exact field classes that cross the boundary versus those
  withheld. Credentials and endpoints are **never** exported, whatever the class
  (`withholds_credentials_and_endpoints` always `true`).
- **Support-bundle treatment** — how the object appears in a support bundle
  (`includes_full_body`, `metadata_only_in_bundle`, `redacted_in_bundle`,
  `excluded_from_bundle`, or `blocked_from_bundle`).
- **Policy source & telemetry limit** — the row states its `user_default` /
  `workspace_policy` / `org_policy` / `regulatory_policy` / `provider_policy` source and its
  telemetry/event limit explicitly; the metadata-safe default stays explicit before anything
  leaves the device (`metadata_safe_default_explicit` always `true`).
- **Reviewed escalation actions** — reveal, view-policy, and reviewed-escalation are always
  offered; a redacted export unless nothing may be exported; a local adjust only when the
  policy is user-adjustable and the class is not policy-restricted. A wider disclosure always
  requires a reviewed escalation (`escalation_requires_review` always `true`).

## Consumers and parity

One matrix binds five claimed provider surface consumers — the offline-capture panel, the
privacy/redaction panel, the provider status bar, the headless/CLI capture surface, and the
support privacy export — to the same offline/privacy vocabulary, anatomy, export fields, and
non-visual accessibility routes, so the destination, queue, and boundary grammar stays
identical across desktop, headless/export, and support consumers. Two checked-in narrowed
fixtures hold the offline-capture panel at Beta and the privacy/redaction panel at Preview
while keeping every consumer visible.

Raw comment bodies, pasted paths, credentials, and private endpoints stay outside the export
boundary; every packet destination, policy label, and capture/redaction identity is carried
only as an opaque, export-safe representation.
