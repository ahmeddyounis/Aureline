# M5 Reproduction Packets

This document is the contract for the M5 reproduction-packet set: the canonical
source for the structured report a user previews and redacts *before* a report
leaves the machine. Help/About, support, and community-handoff surfaces ingest
the checked-in set so a docs, trust-warning, update, workflow, or other-surface
report is anchored, redaction-previewed, and reviewable before a
public/community/support route opens — without overcapturing secrets.

- Record kind: `m5_reproduction_packet_set`
- Schema: [`schemas/help/m5-reproduction-packet.schema.json`](../../schemas/help/m5-reproduction-packet.schema.json)
- Canonical support export: [`artifacts/help/m5-reproduction-packet-proof/packet_set.json`](../../artifacts/help/m5-reproduction-packet-proof/packet_set.json)
- Governance summary: [`artifacts/help/m5-reproduction-packet-governance.md`](../../artifacts/help/m5-reproduction-packet-governance.md)
- Matrix CSV: [`artifacts/help/m5-reproduction-packet-packets.csv`](../../artifacts/help/m5-reproduction-packet-packets.csv)
- Fixtures: [`fixtures/help/reproduction-packets/`](../../fixtures/help/reproduction-packets/)
- Producer: `aureline_shell::m5_reproduction_packets::current_stable_m5_reproduction_packet_set`
- Headless emitter: `aureline_shell_m5_reproduction_packets`

This lane builds on the M3 repro-packet preview contract
([`schemas/public/repro_packet_preview.schema.json`](../../schemas/public/repro_packet_preview.schema.json))
and feeds the M5 community-handoff target review sheets
([`schemas/help/m5-handoff-target.schema.json`](../../schemas/help/m5-handoff-target.schema.json))
that govern whether a route may open.

## Originating surfaces

One packet is named per surface. Each packet pins the exact object anchor of the
locus of concern, the flow used to build it, the redaction posture, the data-exit
boundary, the full redaction preview, and the redaction-safe diagnostics carried
by ref.

| Packet | Surface | Flow | Posture |
| --- | --- | --- | --- |
| `reproduction_packet:docs_pane` | `docs_pane` | `copy_summary` | Metadata refs only |
| `reproduction_packet:trust_warning` | `trust_warning` | `submit_later` | Security channel only |
| `reproduction_packet:update_screen` | `update_screen` | `save_local` | Metadata refs only |
| `reproduction_packet:workflow_bundle` | `workflow_bundle` | `submit_later` | Redacted, support-scoped |
| `reproduction_packet:other_surface` | `other_surface` | `copy_summary` | Fully redacted, public-safe |

## Controlled vocabularies

- **Originating surface** — `docs_pane`, `trust_warning`, `update_screen`,
  `workflow_bundle`, `other_surface`.
- **Redactable field** — `local_path`, `username`, `hostname`, `token`,
  `extension_inventory`, `deployment_profile`, `linked_diagnostic`.
- **Redaction action** — `removed_entirely`, `redacted_placeholder`,
  `generalized_class`, `included_as_object_ref`. Every action keeps the raw value
  out of the export; they differ only in how much labeled, redaction-safe context
  survives.
- **Redaction posture** — `fully_redacted_public_safe`, `redacted_support_scoped`,
  `security_channel_only`, `metadata_refs_only`.
- **Data-exit boundary** — reused from the About/help/community destination
  vocabulary: `no_payload_leaves_product`, `metadata_safe_object_refs`,
  `proposal_refs_only`, `redacted_support_packet`, `security_payloads_only`,
  `external_public_browse`, `vendor_or_third_party_outbound`.
- **Flow** — `save_local`, `copy_summary`, `submit_later`.
- **Included context** — `build_identity`, `environment_capsule`,
  `redacted_log_tail`, `sanitized_config_snapshot`, `repro_steps_text`,
  `anchor_object_ref`, `performance_trace`.

## Invariants

The producer enforces, and the schema mirrors, the following:

- **Preview before share.** Every captured sensitive field has a redaction-preview
  row naming the proposed `default_action` and the `chosen_action`. A user may
  only *tighten* a row (`chosen_action` never exposes more than `default_action`),
  so the preview can never read safer than what is actually shared. A packet that
  shares — a `copy_summary` or `submit_later` flow, or any non-local data-exit —
  sets `preview_confirmed_before_share = true`.
- **Secrets never leave.** A `token` row is always `removed_entirely` and flagged
  `mandatory_redaction`, and every packet asserts `raw_secrets_excluded`,
  `raw_screenshots_excluded`, `hidden_approvals_excluded`, and
  `unmanaged_capture_excluded`. Raw bearer tokens, hidden approvals, and unmanaged
  capture are never collected or exported just because they appear in a local log.
- **Identifiers are never raw.** Local paths, usernames, and hostnames are shown
  as placeholders, generalized classes, or opaque object refs; the extension
  inventory is a ref or a generalized count; the deployment profile is a class
  label only; linked diagnostics/artifacts are opaque object refs.
- **Enough anchor to be actionable.** Each packet pins the exact object anchor and
  carries redaction-safe diagnostics by ref, so the recipient lands on the same
  locus of concern without a screenshot or a raw payload.
- **Creation is separate from submission.** `auto_submit_on_create_allowed` is
  always `false`. Building a packet — including a `submit_later` packet — never
  uploads a support bundle; submission is a distinct, later step.
- **Saved packets survive offline.** A `save_local` packet obeys
  `no_payload_leaves_product` and stays `offline_reusable`; a `submit_later`
  packet also stays reusable offline until it is submitted.

Raw URLs, raw email addresses, raw local paths, raw usernames, raw hostnames,
tokens, and raw secret material never cross this boundary; the records carry
opaque refs and bounded reviewable sentences only.

## Versioning

Adding a new originating surface, redactable field, redaction action, posture,
flow, data-exit, or included-context class is additive-minor and bumps the
relevant schema version. Repurposing an existing value is breaking and requires a
new decision row.
