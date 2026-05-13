# Optional-sync scope bundle and device-registry seed

This document is the human-readable seed for the Aureline
**optional-sync and device-registry contract**: the published family
of JSON Schema boundary records every settings-aware lane reads when
it registers, pauses, resumes, revokes, or forgets a device; mints or
consumes a scope bundle; opens or degrades a sync session; or
previews, reviews, applies, or declines a sync conflict. The ADR
[`docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`](../adr/0008-settings-definition-and-effective-configuration-resolver.md)
freezes the decision; the core vocabulary doc
[`docs/settings/settings_vocabulary.md`](./settings_vocabulary.md)
names the shared tokens; the registry seed
[`docs/settings/schema_registry_seed.md`](./schema_registry_seed.md)
freezes the setting-definition / effective-setting / write-intent
publishing contract; this document freezes the **publishing and
consumption contract** for the optional-sync subset so the settings
UI, CLI `settings sync` command, profile import / export, support-
bundle exporter, mutation-journal renderer, migration center, and the
eventual managed sync service all point to the same device registry,
the same scope-bundle shape, the same session-envelope, and the same
conflict packet.

If this document and the ADR disagree, the ADR wins and this document
must be updated in the same change. The **managed sync service** and
**cloud storage** are explicitly out of scope here; this seed freezes
the local-authoritative contract that such a service would later ride
on top of.

## Why a sync seed now

The ADR freezes the precedence order, the effective-setting record,
and the write-intent packet. Without a sync-side seed that names the
**device registry lifecycle**, the **scope-bundle contents**, the
**session envelope** that carries it, the **conflict packet** that
records disagreements, and the **local-authoritative degrade contract**
that covers transport outages, each lane would invent its own answer:

- the settings UI would render "this machine" without a stable device
  pill, device class, or revocation lifecycle;
- the CLI `settings sync` would emit scope bundles whose omitted
  classes drift between releases;
- the profile importer would accept a synced value that silently
  widens AI egress or workspace trust because no lane stamped the
  scope-broadening verdict on the bundle entry;
- the support exporter would quote raw hostnames or machine-local
  paths rather than export-safe lineage cursors;
- the migration center would render device pause / resume / revoke /
  forget as hidden side effects;
- the conflict review surface would fall back to document-level
  last-writer-wins rather than field-aware, scope-aware paths;
- local-authoritative degrade would look like "sync is broken" instead
  of a typed posture with durable continuity.

This seed closes those gaps before the managed sync service, the live
transport implementation, and the generated settings sync surfaces
land.

## Schema family (boundary of record)

Three JSON Schema files are the cross-tool boundary. The eventual
settings crate's Rust types are the schema of record; the JSON Schemas
are what every non-resolver consumer reads.

| Schema | Envelopes | Purpose |
|---|---|---|
| [`/schemas/settings/sync_device_record.schema.json`](../../schemas/settings/sync_device_record.schema.json) | `sync_device_record` | Opaque `device_id`, user-chosen `device_label`, `device_class`, `os_family_class`, `identity_mode`, `trust_state`, `revocation_state` (active / paused / revoked / forgotten), `revocation_reason`, `revoked_at`, `revoked_by_actor_class`, `capability_states`, `device_secret_binding` (class + broker alias only), `export_safe_lineage` (lineage cursor + registration / last-seen stamps), `redaction_class`, `superseded_by_device_id`. |
| [`/schemas/settings/sync_scope_bundle.schema.json`](../../schemas/settings/sync_scope_bundle.schema.json) | `sync_scope_bundle` / `sync_session_envelope` | Single-scope bundle entries (`setting_id`, `value_preview`, `redaction_class`, `scope_broadening_verdict`, `last_written`), `omitted_classes` block (frozen sixteen-class denylist + non-widening affirmation), `producer_device_id`, monotonic `bundle_epoch`, session-level `session_state`, `transport_state`, `degrade_reasons`, and the `manual_continuity` export/import record. |
| [`/schemas/settings/sync_conflict_packet.schema.json`](../../schemas/settings/sync_conflict_packet.schema.json) | `sync_conflict_packet` | Typed `conflict_class`, field-aware `conflict_delta`, `scope_broadening_verdict`, `offered_resolution_paths` (keep-local, keep-synced, merge-preview, rollback-friendly-review, decline), `resolution_state` lifecycle, `rollback_checkpoint_ref`, `approval_ticket_ref`, `mutation_journal_ref`, `change_preview_ref`, and `remote_origin` lineage. |

Every row shares the frozen tokens already published by the settings
registry seed (`scope_id`, `redaction_class`, `actor_class`,
`widening_vector`, `preview_class`, `identity_mode`, `trust_state`)
and binds together via the shared `settings_schema_version` integer.

Alpha profile/keymap/saved-view review surfaces also consume
[`schemas/sync/device_registry_alpha.schema.json`](../../schemas/sync/device_registry_alpha.schema.json)
and
[`schemas/sync/conflict_packet_alpha.schema.json`](../../schemas/sync/conflict_packet_alpha.schema.json).
Those schemas project the same device and conflict truth into the profile
export/import lane with artifact owner scope, privacy class, portability labels,
per-source revision attribution, explicit `Keep local`, `Keep synced`, and
`Compare` actions, and local-only fallback when transport, policy, or capability
checks fail.

## Publishing conventions

### `$id` and `$schema` URIs

- Each schema declares a stable absolute `$id` under
  `https://aureline.dev/schemas/settings/<name>.schema.json`. The
  published URI MUST match the on-disk path so offline consumers and
  the eventual distribution service resolve identically.
- Each schema declares `$schema:
  "https://json-schema.org/draft/2020-12/schema"`, matching the rest
  of the settings schema family.
- Payloads carry `settings_schema_version` (currently `1`) and a
  record-level `schema_version` string (currently `"1"`). Envelope
  and row versions bump independently; a consumer reading a
  `sync_scope_bundle` with `settings_schema_version=1` MUST consume
  the matching `sync_device_record` and `sync_conflict_packet` at the
  same envelope version.

### Version URI and payload version rules

- Additive-minor changes (new `revocation_state`, new
  `revocation_reason`, new `device_class`, new `os_family_class`, new
  `capability_state.kind`, new `omitted_record_class`, new
  `local_authoritative_degrade_reason`, new `session_state`, new
  `transport_state`, new `conflict_class`, new `resolution_path`, new
  `resolution_state`, new `delta_kind`) MUST NOT bump the `$id`; they
  MUST bump `settings_schema_version` and add a row here.
- Repurposing an existing enum value, relabeling a token, or removing
  a field is breaking. Breaking evolution mints a new `$id` on a
  superseding schema file, ships a deprecation row under the existing
  `$id`, and requires a new decision row.
- The distribution service MUST be able to render a machine diff
  between any two published versions of the sync registry alongside
  the setting-registry diff; release notes reference the diff rather
  than inventing a parallel "sync changelog".

### Unknown-field policy

- Sync artifacts (scope bundle, session envelope, conflict packet,
  device record) run under the same **closed-envelope** policy as
  the rest of the settings family: every `*_record` / `*_packet` /
  `*_envelope` / `*_bundle` object declares
  `additionalProperties: false`. Unknown envelope fields are refused
  loudly; they never pass silently.
- Unknown **setting values** inside a bundle entry follow the
  registry seed's rules: unknown setting ids are preserved as
  `setting_unknown_preserved` on the receiver side, not dropped. The
  envelope still closes; only the carried setting payload is
  round-trip preserving.
- Stale-payload detection is envelope-level: a bundle whose
  `bundle_epoch` is not strictly greater than the last accepted
  epoch from the same device is refused, not quietly merged.

### Omitted classes (what sync MUST NOT carry)

The scope-bundle schema enumerates a frozen sixteen-class denylist
that every bundle MUST reproduce in its `omitted_classes.classes`
array. Missing a class is an invariant violation; the resolver
refuses the bundle outright. The denylist:

1. `secret_bytes` — raw secret material of any kind.
2. `credential_raw` — raw credentials; only broker aliases cross.
3. `trust_grants` — workspace-trust grants; trust is per-device.
4. `delegated_credentials` — delegated / shared credentials of any
   kind.
5. `machine_local_paths` — filesystem paths, mount points, serials.
6. `policy_caches` — cached admin-policy bundles.
7. `admin_policy_bundles` — signed admin-policy bundles themselves.
8. `session_or_command_override_values` — ephemeral scope values.
9. `approval_tickets` — ADR-0007 tickets never ride sync.
10. `rollback_checkpoints` — checkpoints live on the owning device.
11. `support_bundles` — support bundles are not a sync carrier.
12. `crash_dumps` — crash dumps never travel through sync.
13. `mutation_journal_raw` — only lineage cursors travel; raw
    journals never do.
14. `workspace_specific_overrides` — workspace / folder / language
    scopes below `language_override` never sync by default.
15. `device_secret_raw` — only the device-secret class label and a
    broker alias travel.
16. `ephemeral_operation_tokens` — short-lived op tokens never ride
    sync.

The `omitted_classes.non_widening_affirmed` boolean MUST be `true` on
every emitted bundle. Producers that set it to `false` (or omit it)
see the session refused with `session_state=refused`.

### Non-widening import / apply rules

The optional-sync lane inherits the ADR-0008 scope-broadening
invariants verbatim:

1. A synced entry MUST NOT widen workspace trust, AI egress, network
   egress, extension permissions, managed entitlement, credential
   exposure, or the declared allowed-scope set. Every entry carries a
   `scope_broadening_verdict`; any `would_widen_trust=true` entry is
   refused on apply with `write_denial_reason=scope_broadening_would_widen_trust`
   and a conflict packet whose `conflict_class` is
   `scope_broadening_refusal`.
2. Synced entries MAY narrow a value relative to the local effective
   value; narrowing flows through the same write-intent / preview /
   apply pipeline.
3. A synced entry targeting a non-syncable scope is refused with
   `conflict_class=allowed_scope_mismatch`; sync MAY target only
   `user_global` or `language_override`. `machine_specific`,
   `workspace`, `folder_or_module_override`,
   `session_or_command_override`, `imported_profile_default`,
   `channel_or_experiment_default`, `built_in_default`, and
   `admin_policy_narrowing` never sync.

## Device registry lifecycle

The device record is the durable source of truth for every device
that ever participated in optional sync. The lifecycle is a typed
state machine; pause / resume / revoke / forget are transitions, not
hidden side effects.

| State | Meaning | Entered by | Leaves via |
|---|---|---|---|
| `active` | Normal participating state. | First registration; `resume` from `paused`; re-registration after `migration_superseded` mints a new device_id. | `paused` (user / admin / capability lost), `revoked` (user / admin / policy), `forgotten` (user / admin after revoke; or `migration_superseded`). |
| `paused` | User- or admin-initiated freeze. Device keeps its lineage cursor, emits no bundles, consumes no arrivals, but remains in the registry. Sessions the device participates in immediately enter `local_authoritative_degraded` with `degrade_reasons` including `device_paused`. | `user_paused`, `capability_lost`, `policy_revoked` (when the intent is reversible). | `active` (user resumes), `revoked` (user escalates), `forgotten` (user decides to drop). |
| `revoked` | Durable refusal. The device rejects all sync traffic (push and pull), its historical bundle lineage is retained for audit, and its `device_secret_binding` is severed. Sessions refuse its arrivals with `conflict_class=device_revoked`. | `user_revoked`, `admin_revoked`, `policy_revoked`, `stale_expired`. | `forgotten` (user or retention window expires); NEVER returns to `active` — a revoked device re-registers as a new `device_id` under `migration_superseded`. |
| `forgotten` | Terminal state. Device label and lineage cursor are purged modulo the retention window declared in the record-class registry; `device_id` and revocation stamps remain so the audit stream is complete. | `user_forgot`, automatic progression from `revoked` after the retention window. | No onward transition; a later registration from the same hardware mints a fresh `device_id`. |

Every transition:

- carries a typed `revocation_reason` and a `revoked_by_actor_class`;
- records `revoked_at` (monotonic);
- emits a settings-audit event (reuse the existing audit stream
  vocabulary);
- is durable (survives restart);
- is reflected in the support-bundle exporter and the offboarding
  packet.

Silent revocation, silent pause, or silent forget is a bug. The
settings UI MUST name the state and the reason in-line.

## Scope bundle and session envelope

### Scope bundle rules

- A scope bundle carries exactly one `scope` (`user_global` or
  `language_override`). Multi-scope payloads are refused: split them
  into separate bundles.
- `producer_device_id` MUST resolve to a `sync_device_record` whose
  `revocation_state` is `active` or `paused`. A `paused` producer may
  only emit heartbeat bundles with zero entries.
- `bundle_epoch` is monotonic per device. The receiver refuses
  bundles whose epoch is `<=` the last accepted epoch from the same
  device; refusal triggers `conflict_class=stale_payload`.
- Entries whose declared `redaction_class` is `exclude_from_export`
  are filtered out of the bundle before emission; the producer emits
  a settings-audit event and the bundle MUST NOT carry them.
- Entries carrying a `credential_alias` value carry the broker alias
  only. The receiver resolves the alias through the ADR-0007 broker;
  raw secret bytes never cross the sync boundary.

### Session envelope rules

- Every exchange rides inside a `sync_session_envelope`. The envelope
  names the local device, the remote peers, the session state, the
  transport state, and the active degrade reasons.
- A session with `session_state != open` and `session_state != refused`
  MUST list at least one `degrade_reason`. A "degraded with no named
  reason" envelope is a bug.
- `manual_continuity` is the canonical name for the user-initiated
  export / import path. The envelope records the `source_device_id`,
  the `target_device_id`, the `carried_at` monotonic stamp, and an
  opaque `export_artifact_ref`; hostnames and filesystem paths never
  appear.

## Local-authoritative degrade contract

The degrade contract is the floor: whenever the transport is
unavailable, the payload is stale, encryption fails, a capability is
missing, policy blocks, or the user carried continuity by hand, the
session enters `local_authoritative_degraded` and the resolver keeps
serving local values. No lane may silently replay stale, encrypted-
failed, or capability-missing data.

| Degrade reason | Trigger | Receiver posture |
|---|---|---|
| `transport_unavailable` | No transport at all (account-free local with no sync endpoint, network down). | Serve local values; emit no bundles; `session_state=local_authoritative_degraded`. |
| `transport_degraded` | Transport up but round-trips exceed the resolver's freshness budget. | Serve local values; continue to read heartbeats; transition back to `open` on a fresh successful round. |
| `stale_payload` | `bundle_epoch <= last_accepted_epoch` from the same device, or bundle `expires_at` has elapsed. | Refuse the bundle; emit a `conflict_class=stale_payload` packet; remain in `local_authoritative_degraded`. |
| `encryption_failure` | Transport-level encryption check failed or keys are missing. | Refuse all arrivals; emit `conflict_class=encryption_failure` for any replayed entry; never fall through to plaintext. |
| `missing_capability` | One or more `capability_state` rows are unsatisfied (device secret absent, encryption key missing, policy epoch too low, identity mode mismatched, sync transport unreachable). | Session enters `local_authoritative_degraded`; bundle emission stops; resume requires the capability to recover. |
| `policy_block` | Admin policy prohibits optional sync for this identity mode or policy epoch. | Refuse all arrivals; conflict packets carry `conflict_class=policy_block`; revocation may follow under `policy_revoked`. |
| `manual_export_import` | User carries continuity by hand via profile export / import. | The envelope records `manual_continuity`; arriving entries are treated as sync entries subject to the same scope-broadening / preview / approval pipeline. |
| `device_paused` / `device_revoked` | The participating device is paused or revoked. | Session enters `local_authoritative_degraded` (paused) or refuses the device's arrivals outright (revoked). |
| `identity_mode_mismatch` | The producer device's `identity_mode` differs from the receiver's in a way that policy forbids. | Refuse the bundle; emit `conflict_class=allowed_scope_mismatch` or `policy_block` as appropriate. |
| `schema_version_incompatible` | Bundle `schema_version` cannot be migrated in place. | Refuse the bundle; routing to the migration center is the only safe path. |

In every degrade posture, local values remain authoritative, the
`effective_setting_record.offline_fallback` is `authoritative_local`,
and the user can continue to edit, preview, and apply local changes.
`manual_export_import` is a first-class continuity path, not a
fallback: the user exports a profile on one device and imports it on
another; the envelope records the lineage exactly so the support and
audit lanes see the same trace a live session would produce.

## Conflict packet (field-aware, scope-aware, rollback-friendly)

Conflicts are never resolved by document-level last-writer-wins. The
packet is **field-aware** (the delta names which fields diverged),
**scope-aware** (the packet names the target scope and carries a
`scope_broadening_verdict`), and **rollback-friendly** (every apply
that crosses a preview / rollback / approval bar is gated by
checkpoint and approval refs).

### Offered resolution paths

| Path | Meaning | When offered |
|---|---|---|
| `keep_local` | Retain the local value. The refusal is recorded in the audit stream; no write happens. | Always. Even when the widening verdict denies, `keep_local` remains so the user always has a safe out. |
| `keep_synced` | Adopt the synced value. Routes through a `write_intent_packet` with `reason_class=sync`; the preview / checkpoint / approval class is honoured. | When the entry did not widen trust and does not violate allowed-scope rules. |
| `merge_preview` | Run a merge against a field-aware preview. Offered only when `delta_kind` is merge-safe (`array_append_only`, `object_field_add`, `redacted_structural`). Gated by a `change_preview_packet`. | When the setting's shape permits safe structural merge. |
| `rollback_friendly_review` | Create an ADR-0006 rollback checkpoint before any apply. Default path for `preview_required`, `rollback_checkpoint_required`, and `rollback_checkpoint_and_approval_required` settings. | Whenever the setting's preview class requires a checkpoint. |
| `decline` | Terminal refusal of the packet. Neither value applies; the device records the refusal and rejects further arrivals for this setting/scope pair until the user chooses another path. | Always available. |

### Resolution-state lifecycle

`pending` → (`previewed`) → (`acknowledged`) → `resolved` /
`declined` / `expired` / `withdrawn`. `withdrawn` fires when the
producer device is paused / revoked / forgotten before apply;
`expired` fires when an acknowledgement times out and the packet
must be re-presented. The mutation journal never contains an applied
sync resolution without a matching acknowledgement.

## Support and diagnostics projection rules

The support-bundle exporter and diagnostics surfaces project the
device registry, scope bundles, session envelopes, and conflict
packets into read-only views. The rules are:

1. **Device records** appear as lineage rows. The support exporter
   quotes `device_id`, `device_class`, `os_family_class`,
   `identity_mode`, `trust_state`, `revocation_state`,
   `revocation_reason`, `revoked_at`, and the `export_safe_lineage`
   cursor. `device_label` is redacted per the record's
   `redaction_class` (default `redact_to_class_label` on
   cross-org bundles). Raw hostnames, IPs, serials, and
   machine-local paths MUST NOT appear.
2. **Scope bundles** appear as summary rows: `producer_device_id`,
   `scope`, `bundle_epoch`, `emitted_at`, and the count and the
   `omitted_classes` block verbatim. Individual entry values
   inherit the setting's declared `redaction_class`; values whose
   class is `exclude_from_export` are absent.
3. **Session envelopes** appear with their `session_state`,
   `transport_state`, `degrade_reasons`, `opened_at`,
   `last_reconciled_at`, and (when present) the `manual_continuity`
   block. Raw transport diagnostics never appear; only the typed
   state.
4. **Conflict packets** appear as rows keyed by `packet_id` with
   `setting_id`, `target_scope`, `conflict_class`, `delta_kind`,
   `widening_vector`, `chosen_resolution_path`, `resolution_state`,
   and the `rollback_checkpoint_ref` / `approval_ticket_ref` /
   `mutation_journal_ref` handles. Old and new value previews
   inherit the setting's redaction class.
5. **Conflict decisions** are durable. An exported support bundle
   after a fleet-wide sync regression shows the same
   `conflict_class` and `chosen_resolution_path` the settings UI
   showed in-line; there is no parallel "sync log" taxonomy.
6. **Scope-safe merges** are projected through the `change_preview_delta`
   shape already frozen in the write-intent schema family; support
   renders "N fields merged (redacted)" on `redacted_structural`
   rather than inventing a parallel diff format.
7. **Hidden defaults never appear.** The effective-setting record's
   `shadow_chain`, `control_stack`, and `source_label` rules
   already forbid silent defaults; the sync projection inherits the
   rule. A support bundle that would expose a default as "synced"
   is a bug.

## Consumer checklist

Before a sync-aware lane ships, it confirms:

1. It reads the **canonical** `setting_id` from the setting-
   definition row. Legacy ids arrive via the alias table; redirects
   are recorded, not silent.
2. It respects the syncable-scope allow-list (`user_global`,
   `language_override`). Bundles carrying any other scope are
   refused.
3. It reproduces the full `omitted_classes.classes` denylist on
   every bundle; a missing class refuses the bundle.
4. It stamps a `scope_broadening_verdict` on every bundle entry and
   routes any `would_widen_trust=true` entry into a conflict packet
   with `conflict_class=scope_broadening_refusal`.
5. It honours the device revocation lifecycle: `paused` device
   bundles are not consumed as values, `revoked` devices' arrivals
   are refused, `forgotten` devices are purged per the retention
   window.
6. It opens every exchange inside a `sync_session_envelope` and
   lists at least one `degrade_reason` whenever `session_state` is
   not `open`.
7. It supports `manual_continuity` as a first-class path; user
   export / import is not a fallback, it is a typed envelope.
8. It emits a `sync_conflict_packet` for every non-trivial
   disagreement. `value_equal_no_op` is the only no-write outcome
   that is safe to suppress from the UI, and even then the packet
   is recorded in the audit stream.
9. It routes `Undo` for rollback-class settings through the
   `rollback_checkpoint_ref`; it never invents its own checkpoint.
10. It quotes the declared `redaction_class` on every exportable
    surface and the `record_class_id` when the export is
    class-bearing (support bundle, offboarding packet, usage
    export).

## Where related artifacts live

- Core settings schema family and registry seed:
  [`docs/settings/schema_registry_seed.md`](./schema_registry_seed.md).
- Companion settings vocabulary:
  [`docs/settings/settings_vocabulary.md`](./settings_vocabulary.md).
- Sync conflict fixtures (six scenarios exercising keep-local,
  keep-synced, merge-preview, rollback-friendly-review,
  scope-broadening refusal, and manual continuity):
  [`fixtures/settings/sync_conflicts/`](../../fixtures/settings/sync_conflicts/).
- Decision register row `D-0014`:
  [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
- Record-class registry:
  [`artifacts/governance/record_class_registry.yaml`](../../artifacts/governance/record_class_registry.yaml).

## Change management

- Adding a new `revocation_state`, `revocation_reason`,
  `device_class`, `os_family_class`, `capability_state.kind`,
  `omitted_record_class`, `local_authoritative_degrade_reason`,
  `session_state`, `transport_state`, `conflict_class`,
  `resolution_path`, or `resolution_state` is additive-minor: bump
  `settings_schema_version`, add a row in the relevant vocabulary
  section, and extend the schemas.
- Repurposing any existing value (for example, reusing `policy_block`
  for a different block semantics) is breaking and requires a new
  decision row.
- Renaming a device is never done by rewriting the record in place;
  it is done by revoking the old `device_id` under
  `migration_superseded` and re-registering with a new id that
  points back through `superseded_by_device_id`.
- Publishing a new release of the sync registry MUST emit a machine
  diff alongside the setting-registry diff; release notes reference
  the diff rather than a parallel "sync changelog".
