# M5 Handoff Continuity

This document is the contract for the M5 handoff-continuity scenario set: the
canonical source for the persisted draft state a user keeps when a
public/community/support handoff is delayed or fails. Help/About, support, and
community-handoff surfaces ingest the checked-in set so a browser-blocked,
offline, policy-denied, launch-failed, or unsupported-profile handoff preserves
the user's drafted text, chosen attachments, redaction choices, and intended
target trust class — instead of forcing them to recreate the work from scratch.

- Record kind: `m5_handoff_continuity_scenario_set`
- Schema: [`schemas/help/m5-handoff-draft-state.schema.json`](../../schemas/help/m5-handoff-draft-state.schema.json)
- Canonical support export: [`artifacts/help/m5-handoff-continuity-proof/draft_state_set.json`](../../artifacts/help/m5-handoff-continuity-proof/draft_state_set.json)
- Governance summary: [`artifacts/help/m5-handoff-continuity-governance.md`](../../artifacts/help/m5-handoff-continuity-governance.md)
- Matrix CSV: [`artifacts/help/m5-handoff-continuity-drafts.csv`](../../artifacts/help/m5-handoff-continuity-drafts.csv)
- Fixtures: [`fixtures/help/handoff-continuity/`](../../fixtures/help/handoff-continuity/)
- Producer: `aureline_shell::m5_handoff_continuity::current_stable_m5_handoff_continuity_scenario_set`
- Headless emitter: `aureline_shell_m5_handoff_continuity`

This lane reuses the redaction vocabulary of the M5 reproduction-packet contract
([`schemas/help/m5-reproduction-packet.schema.json`](../../schemas/help/m5-reproduction-packet.schema.json))
and the destination/trust vocabulary of the M5 community-handoff target contract
([`schemas/help/m5-handoff-target.schema.json`](../../schemas/help/m5-handoff-target.schema.json)),
and binds to the frozen M5 public-handoff matrix
([`schemas/help/m5-public-handoff-matrix.schema.json`](../../schemas/help/m5-public-handoff-matrix.schema.json))
that governs whether a route may eventually open.

## Failure scenarios

One draft is named per failure scenario. Each draft pins why the first launch
attempt failed, the explicit continuity state, the intended destination trust
class and visibility boundary, the data-exit boundary the handoff will obey once
it succeeds, the preserved drafted text and attachments, and the preserved
redaction choices.

| Draft | Failure | Intended trust | State |
| --- | --- | --- | --- |
| `handoff_draft:browser_blocked_public_issue` | `browser_blocked` | Official public | Captured offline |
| `handoff_draft:offline_community_support` | `no_network_offline` | Community | Captured offline |
| `handoff_draft:policy_denied_security` | `policy_denied` | Private / security | Staged for later |
| `handoff_draft:launch_failed_official_support` | `handoff_launch_failed` | Official authenticated | Awaiting retry |
| `handoff_draft:unsupported_profile_local` | `unsupported_profile` | Local only | Exported locally |
| `handoff_draft:cleared_public_issue` | `browser_blocked` | Official public | Cleared |

## Controlled vocabularies

- **Failure class** — `browser_blocked`, `no_network_offline`, `policy_denied`,
  `handoff_launch_failed`, `unsupported_profile`.
- **Continuity state** — `captured_offline`, `awaiting_retry`, `staged_for_later`,
  `exported_locally`, `cleared`.
- **Continuity action** — `retry`, `export_packet`, `open_target_later`,
  `switch_target_class`, `clear_draft`.
- **Destination trust class** — reused from the community-handoff target
  vocabulary: `official_public`, `official_authenticated`, `community`,
  `private_security`, `local_only`.
- **Visibility boundary** — reused from the community-handoff target vocabulary:
  `world_readable_public`, `official_account_visible`, `community_visible`,
  `private_security_channel`, `local_never_leaves`.
- **Data-exit boundary** — reused from the About/help/community destination
  vocabulary: `no_payload_leaves_product`, `metadata_safe_object_refs`,
  `proposal_refs_only`, `redacted_support_packet`, `security_payloads_only`,
  `external_public_browse`, `vendor_or_third_party_outbound`.
- **Redactable field** / **redaction action** / **redaction posture** — reused
  from the reproduction-packet vocabulary.
- **Retention scope** — `session_only`, `until_user_clears`,
  `profile_scoped_window`, `declared_retention_window`.
- **Attachment class** — `log_excerpt`, `redacted_screenshot`, `config_snapshot`,
  `diagnostic_bundle`, `repro_steps_note`, `other_artifact`.

## Invariants

The producer enforces, and the schema mirrors, the following:

- **Drafts survive failure.** A live (non-cleared) draft preserves the drafted
  text (by opaque ref and character count), the chosen attachments, and every
  redaction choice the user made. A handoff failure never forces the user to
  recreate the draft or the redaction work.
- **Offline capture is first-class.** Every draft sets
  `offline_capture_first_class`, every live draft is `draft_reusable_offline`,
  and `current_data_exit_boundary` is always `no_payload_leaves_product` — nothing
  has left the product while a draft is held, so a blocked or offline handoff
  degrades to a labeled, reusable local draft, never a dead-end error.
- **Target-class truth is preserved.** Each draft pins the intended
  `intended_trust_class`, its `visibility_boundary`, and the
  `intended_data_exit_boundary` consistent with that class.
  `preserves_target_class_on_retry` and `preserves_visibility_boundary_on_export`
  are always `true`, so retry and export keep the official / community /
  security / local truth intact.
- **No silent redirection.** `auto_redirect_to_reachable_target_allowed` is always
  `false`: a failed security/private route is never quietly rerouted to a
  more-reachable public/community target. Switching target class is always an
  explicit user action (`target_switch_requires_explicit_user_action`).
- **Every failure offers a way forward.** A live draft offers the full set of
  continuity actions — `retry`, `export_packet`, `open_target_later`,
  `switch_target_class`, `clear_draft` — so a failure never strands the user.
- **Secrets never persist.** A `token` redaction row is always `removed_entirely`
  and flagged `mandatory_redaction`; preserved text and attachments are carried by
  opaque ref with redaction applied.
- **Nothing persists invisibly.** Every draft is `persisted_state_visible_to_user`
  under a declared `retention_scope` with a `clear_draft` action, so a persisted
  draft never outlives its declared retention or profile scope without visible
  state and a clear action. A `cleared` draft retains no text, attachments,
  redaction choices, or actions, and is no longer reusable.

Raw URLs, raw email addresses, raw local paths, raw usernames, raw hostnames,
tokens, and raw secret material never cross this boundary; the records carry
opaque refs and bounded reviewable sentences only. The drafted text body lives in
local storage and is named here by opaque ref and character count, never inlined
raw.

## Versioning

Adding a new failure class, continuity state, continuity action, retention scope,
or attachment class is additive-minor and bumps the relevant schema version.
Repurposing an existing value is breaking and requires a new decision row.
