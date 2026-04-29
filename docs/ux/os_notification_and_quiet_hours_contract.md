# OS notification and quiet-hours contract: suppression audit, lock-screen privacy, exact reopen, and desktop-summary affordances

This document freezes the cross-surface contract for OS notifications,
companion push, lock-screen summaries, dock / taskbar / system-tray
mirrors, and other desktop summary affordances against quiet-hours
policy. It exists so suppression, privacy, and reopen behavior on every
OS-level surface are never implicit: a suppressed event still leaves an
auditable local trail tied to the same canonical event, an OS-bound
payload is privacy-safe by construction, and an OS-level shortcut can
never complete a high-risk or mutating action without re-entering the
in-product preview, approval, or revalidation flow.

The contract is normative. Where this document disagrees with the UI /
UX Spec or with the upstream attention, notification, notification-
delivery, or durable-job contracts, the source spec wins and this
document, schema, and fixtures must change in the same patch. Where a
downstream surface — OS notification shim, companion push transport,
lock-screen summary renderer, dock / taskbar adapter, system-tray panel
— invents private suppression vocabulary, private privacy payload
shape, private reopen target, or a private mutation shortcut for one of
the frozen affordance classes, this contract wins and the surface is
non-conforming.

## Companion artifacts

- [`/schemas/ux/notification_suppression_record.schema.json`](../../schemas/ux/notification_suppression_record.schema.json)
  defines the `notification_suppression_record`, the
  `privacy_safe_payload_rule_record`, and the
  `desktop_summary_affordance_record` carried at the cross-tool
  boundary.
- [`/fixtures/ux/os_notification_cases/`](../../fixtures/ux/os_notification_cases/)
  contains worked cases covering held quiet-hours bursts, dedupe
  collapse, lock-screen privacy redaction, admin-narrowed durable-only
  delivery, exact-reopen through review, and read-only desktop summary
  affordances.

## Upstream contracts

This contract composes with existing owners and does not replace them:

- [`attention_activity_taxonomy.md`](./attention_activity_taxonomy.md)
  owns delivery-surface class, attention class, interruptibility tier,
  quiet-hours mode, suppression reason, privacy payload class, dedupe
  scheme, dismissal verb, and reopen-target vocabulary.
- [`notification_contract.md`](./notification_contract.md) and
  [`/schemas/ux/notification_event.schema.json`](../../schemas/ux/notification_event.schema.json)
  own the higher-level surface-class, badge, durability, and exact-
  reopen contract every notification event records.
- [`notification_delivery_contract.md`](./notification_delivery_contract.md)
  and [`/schemas/ux/event_lineage.schema.json`](../../schemas/ux/event_lineage.schema.json)
  own canonical event lineage, delivery / dismissal / reopen / release
  steps, durable linkbacks, the action taxonomy, and the
  high-risk-shortcut-no-bypass routing row.
- [`durable_job_envelope_contract.md`](./durable_job_envelope_contract.md)
  and [`/schemas/ux/durable_job_envelope.schema.json`](../../schemas/ux/durable_job_envelope.schema.json)
  own the durable-job envelope a desktop summary affordance mirrors;
  this contract reuses `durable_job_id` and `canonical_event_id`
  rather than re-minting per-affordance progress vocabulary.
- [`/artifacts/ux/quiet_hours_policy_matrix.yaml`](../../artifacts/ux/quiet_hours_policy_matrix.yaml)
  owns per-`quiet_hours_mode` suppression / preservation rules. This
  contract records the per-event suppression decision against those
  rules and does not re-declare which modes suppress which surfaces.
- `docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`
  — every OS notification, lock-screen summary, and companion push
  payload runs through the broker-owned redaction pass before bytes
  reach the sink.
- `docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`
  — admin policy MAY narrow OS surfaces; admin policy MAY NOT silently
  widen and MAY NOT silently block `tier_critical_safety`.

## Scope

This revision freezes:

- the `notification_suppression_record` carried for every notification
  that was held, delayed, collapsed, redacted, or denied — one record
  per `(canonical_event_id, intended_delivery_surface_class,
  client_scope)` triple, tied back to the same canonical event /
  object identity the in-product surface uses;
- the `privacy_safe_payload_rule_record` carried per
  `privacy_payload_class` so lock-screen, companion, and system-
  notification surfaces share one statement of allowed labels,
  allowed actions, and forbidden shortcut action classes;
- the `desktop_summary_affordance_record` carried per OS-notification
  action, dock / taskbar progress mirror, dock / taskbar badge,
  system-tray summary, lock-screen quick action, companion push
  action, or desktop widget summary so the affordance reflects the
  durable-job envelope without becoming a privileged mutation path;
- exact-reopen linkage from every OS-bound payload back to the
  canonical object, route, review context, durable activity row,
  digest group, or announced placeholder — never a generic home
  screen;
- the no-bypass rule that high-risk or mutating actions cannot be
  completed from an OS-level shortcut without re-entering the in-
  product review sheet, approval workflow, or revalidation flow.

Out of scope (frozen on the spec):

- platform notification adapter code (macOS User Notifications, Linux
  XDG `org.freedesktop.Notifications`, Windows Toast and Action
  Center, browser Push API, mobile companion push providers, dock /
  taskbar / system-tray implementation specifics);
- final copy, iconography, animation, layout, and accessible-name
  authoring per platform;
- the eventual notification-router / desktop-summary crate's Rust
  types — the JSON Schema export reserves the boundary shape until
  the crate lands.

## Required suppression-record anatomy

Every `notification_suppression_record` is the auditable trail for a
notification that did not reach the user the way the routing matrix
would otherwise describe. It mints whenever a delivery is held,
delayed, collapsed onto another lineage, redacted to a privacy-safe
payload, or denied entirely under admin or lock-screen policy.

Required fields:

| Field | Required meaning | Non-conforming collapse |
| --- | --- | --- |
| `canonical_event_id` | Same id used by envelopes, lineages, badges, durable rows, system notifications, and history. | A per-surface suppression id that cannot be joined to the canonical event. |
| `event_lineage_id_ref` | Opaque ref to the event-lineage record so the suppression record reuses the lineage's delivery / release / reopen steps. | Re-minting lineage per device or per delivery. |
| `canonical_object_target_ref` | Opaque ref to the canonical object the work concerns. | Raw paths, raw URLs, or display text as identity. |
| `source_subsystem` and `actor_identity_ref` | Stable owner and owner-side actor of record (including system actors). | "System" or "Background task" with no source. |
| `intended_delivery_surface_class` | The surface that would have rendered if the suppression had not applied. | Audit gaps where the suppressed surface is implicit. |
| `intended_privacy_payload_class` | The payload class the surface would have carried if delivered. | A redacted payload that disagrees with the originally-intended class. |
| `intended_interruptibility_tier` | The tier at the time of the suppression decision. | Suppressing `tier_critical_safety` silently. |
| `suppression_class` | One closed value naming why the record exists (held, delayed, collapsed, redacted, denied). | Free-text reasons in copy. |
| `suppression_reasons` | One or more re-exported `suppression_reason` values. | A held envelope with no reason. |
| `active_quiet_hours_modes_at_decision` | Modes active at the time of the decision. | A held delivery with no recorded mode. |
| `suppression_outcome` | What actually happened — held, collapsed, redacted, denied audit-trail-only, released as digest, released on escalation, etc. | Outcome implied by absence. |
| `delay_until_condition_class` | When the held delivery is allowed to release (or `no_delay_collapsed_or_denied`). | A vanishing event with no rule. |
| `preserved_durable_linkback` | The durable row, attention item, history lane, or evidence packet the suppression resolves back to (or `audit_trail_only` when the in-product render is forbidden). | A held or denied delivery with no durable trail. |
| `audit_trail_record_ref` | Opaque ref to the audit row carrying the suppression decision. | Silent drop with no audit row. |
| `privacy_redaction` | Applied payload class, lock-screen / companion / system-notification safety flags, redacted summary label, and stripped label kinds. | A redacted payload that re-includes raw object identity. |
| `exact_reopen_linkage` | Reopen target kind, target ref, canonical route, review-context ref, command id, and proof that the canonical event / object identity is preserved. | Reopen to a generic home screen or with a different canonical id. |
| `forbidden_shortcut_action_classes` | Action classes that MUST NOT be completable from an OS-level shortcut for this suppression record. | A high-risk shortcut that bypasses review. |
| `bypass_protection` | Whether the in-product surface MUST route through a review sheet, approval workflow, or revalidation; carries the refs to those surfaces. | A high-risk action mutated directly from an OS payload. |
| `desktop_summary_affordance_refs` | Opaque refs to every dock / taskbar / system-tray / lock-screen quick-action / companion push action / desktop widget summary that mirrors this event. | Mirrors that drift from the canonical event without a recorded ref. |
| `release_step` | When the held delivery later releases, the trigger class (`mode_exit_grouped_digest`, `user_explicit_show_held`, `escalation_to_critical_safety`, `cross_client_collapse`, `reopen_revalidation_completed`, or `no_release_audit_trail_only`), the surface and payload class it released to, and (for grouped digests) the burst id and member count. | Silent release outside the closed trigger set. |
| `client_scopes` | Every client surface the record applies to. | Per-client suppression that diverges. |
| `policy_context` | Policy epoch, trust state, optional execution context. | Stale policy implied by lack of revalidation. |
| `preserves_durable_history` | Constant `true`. | Suppression that erases durable history. |

Raw paths, raw URLs, raw provider payloads, raw prompt / completion
text, raw command bodies, secret material, and customer-owned
identifiers do not appear in the suppression record. Use opaque refs
and privacy-safe labels.

## Suppression class table

Every `notification_suppression_record` resolves to exactly one
`suppression_class`. The class is a closed list; each class has a
default outcome, a release rule, and rules about which `quiet_hours_mode`
must appear in `active_quiet_hours_modes_at_decision`.

| `suppression_class` | Default outcome | Release rule | Required quiet-hours mode | Notes |
| --- | --- | --- | --- | --- |
| `held_quiet_hours_user_policy` | `not_delivered_held` | `mode_exit_grouped_digest` | `mode_quiet_hours_user` | User scheduled quiet hours; transient toasts / OS / lock-screen / companion suppressed. Durable rows preserved. |
| `held_do_not_disturb_user_policy` | `not_delivered_held` | `mode_exit_grouped_digest` | `mode_do_not_disturb_user` | Narrower than quiet hours: also holds `tier_actionable` unless `tier_blocking_trust` or `tier_critical_safety`. |
| `held_focus_mode_user_policy` | `not_delivered_held` | `mode_exit_grouped_digest` | `mode_focus_mode_user` | App-icon badges and status items remain truthful. |
| `held_presentation_mode` | `not_delivered_held` | `mode_exit_grouped_digest` | `mode_presentation` | Audience-visible surfaces dimmed; durable rows preserved. |
| `held_screen_share` | `not_delivered_held` | `mode_exit_grouped_digest` | `mode_screen_share` | Matches presentation; collaboration pings route to session digest. |
| `held_privacy_mode` | `not_delivered_held` | `mode_exit_grouped_digest` | `mode_privacy_mode` | OS surfaces and lock-screen summaries denied; in-product preserved. |
| `held_admin_suppression` | `delivered_admin_narrowed_durable_only` or `not_delivered_held` | per-policy | `mode_admin_suppression` | Managed policy MAY narrow further; MAY NOT silently widen and MAY NOT silently block `tier_critical_safety`. |
| `held_power_saver_runtime` | `not_delivered_held` | `mode_exit_grouped_digest` | `mode_power_saver_runtime` | Not user-overridable while the runtime power manager holds the mode. |
| `held_reduced_attention_policy` | `not_delivered_held` | `mode_exit_grouped_digest` | `mode_reduced_attention_policy` | Reduced-attention accessibility posture. |
| `delayed_release_pending_next_unsuppressed_surface` | `not_delivered_held` | `mode_exit_grouped_digest` or `escalation_to_critical_safety` | `mode_none` permitted only when a downstream surface is itself suppressed | Cross-surface fanout deferred. |
| `collapsed_dedupe_same_canonical_event` | `not_delivered_collapsed` | `no_release_audit_trail_only` | any | Same canonical event already in-flight; this delivery folds onto it. |
| `collapsed_dedupe_same_grouped_burst` | `not_delivered_collapsed` | `mode_exit_grouped_digest` | any | Folds into a grouped burst with `member_count` evolving. |
| `redacted_for_lock_screen` | `delivered_redacted` | n/a | any (typically `mode_none` plus locked screen) | Lock-screen surfaces carry a privacy-safe payload; high-risk shortcuts forbidden. |
| `redacted_for_companion_push` | `delivered_redacted` | n/a | any | Companion surface carries the same payload class and reopen ref. |
| `redacted_for_system_notification` | `delivered_redacted` | n/a | any | OS notification surface redacts payload to lock-screen-safe class. |
| `denied_policy_forbidden_on_lock_screen` | `delivered_in_product_only` or `not_delivered_audit_trail_only` | `no_release_audit_trail_only` | typically `mode_privacy_mode` or admin policy | Lock-screen delivery refused; in-product durable row remains. |
| `denied_audit_trail_only` | `not_delivered_audit_trail_only` | `no_release_audit_trail_only` | any | User-facing delivery denied entirely; only the audit-trail row is preserved. |

Rules (frozen):

1. **Held events still leave an auditable local trail.** A held envelope
   records `intended_delivery_surface_class`, `intended_privacy_payload_class`,
   `suppression_reasons`, `active_quiet_hours_modes_at_decision`,
   `audit_trail_record_ref`, and a `preserved_durable_linkback` whose
   `kind` is one of the durable linkback kinds (or `audit_trail_only`
   when the in-product render is forbidden). Silent drop is
   non-conforming.
2. **`canonical_event_id` is invariant under suppression.** A
   suppression record never mints a new canonical event id, never
   splits the canonical object identity, and `preserves_canonical_event_id`
   is `true` on every linkback and reopen entry.
3. **`audit_trail_only` is an explicit class, not a fallback.** A
   record that resolves to `not_delivered_audit_trail_only` MUST set
   `preserved_durable_linkback.kind = audit_trail_only` with
   `is_durable = false`. Silent fallback to nothing is non-conforming.
4. **Admin suppression MAY narrow but MAY NOT silently widen.**
   `held_admin_suppression` cannot be applied to
   `intended_interruptibility_tier = tier_critical_safety`.
5. **Release transitions are recorded.** A held record that later
   releases records a `release_step` whose `release_trigger_class` is
   one of `mode_exit_grouped_digest`, `user_explicit_show_held`,
   `escalation_to_critical_safety`, `cross_client_collapse`, or
   `reopen_revalidation_completed`. Silent release is non-conforming.

## Privacy-safe payload classes

Every OS-bound surface carries a `privacy_payload_class` that names
which label kinds and action kinds the payload may carry. The
`privacy_safe_payload_rule_record` freezes the rule per class so
lock-screen, companion, and system-notification renderers, and any
desktop adapter that reads this contract, share one statement of truth.

| `privacy_payload_class` | Lock-screen | Companion | System notification | Allowed labels | Allowed actions |
| --- | --- | --- | --- | --- | --- |
| `lock_screen_safe_generic` | yes | yes | yes | category-only, severity class, lifecycle phase, member-count, audit-only marker | open in product, snooze until unlocked, dismiss transient |
| `lock_screen_safe_scoped` | yes | yes | yes | category, scoped workspace label, scoped session label, severity class, lifecycle phase, next-action label, summary member count | open in product, open canonical object in product (read-only), snooze until resume condition, acknowledge, dismiss transient |
| `in_product_only` | no | no | no | category, scoped workspace, scoped session, next-action, member count, severity, lifecycle phase, actor role | open in product (in-product surfaces only) |
| `redacted_metadata_only` | no | yes | yes | category, severity class, lifecycle phase, member count, audit-only marker | open in product, dismiss transient |
| `policy_forbidden_on_lock_screen` | no | optional per policy | optional per policy | (none on lock-screen; in-product label only) | (no shortcut actions; reopen requires in-product authentication) |

Forbidden shortcut action classes (see also §No-bypass on high-risk
shortcuts) apply uniformly: a privacy-safe payload MUST NOT expose
`destructive_publish_or_apply`, `secret_or_credential_reveal`,
`irreversible_high_blast`, `bypass_review_sheet`,
`bypass_approval_workflow`, `cross_workspace_mutation`,
`direct_mutation_from_lock_screen`, `direct_mutation_from_companion_push`,
`direct_mutation_from_dock_or_taskbar`, `direct_mutation_from_system_tray`,
`policy_override_from_os_shortcut`, `trust_state_change_from_os_shortcut`,
or `provider_grant_change_from_os_shortcut` as a completable shortcut.

Forbidden label kinds — `raw_path`, `raw_url`, `raw_email`,
`raw_provider_payload`, `raw_secret_material`, `raw_prompt_text`,
`raw_completion_text`, `raw_command_body`, `customer_owned_identifier`,
`destructive_intent_phrase`, `object_identity_on_lock_screen`,
`actor_real_name_on_lock_screen`, `review_diff_excerpt_on_lock_screen` —
are stripped before bytes reach the OS sink. Stripped kinds appear in
`privacy_redaction.stripped_label_kinds` so the audit row records what
was removed.

`redaction_template_tokens` are a closed substitution set —
`{category}`, `{workspace_label}`, `{session_label}`, `{member_count}`,
`{actor_role}`, `{severity_class}`, `{lifecycle_phase}`,
`{audit_only_marker}` — so a lock-screen / companion / system-
notification renderer can fill the redacted summary template
mechanically.

Rules (frozen):

1. **`in_product_only` never reaches lock-screen surfaces.** A
   `privacy_safe_payload_rule_record` whose
   `applies_to_surface_classes` includes `lock_screen_summary` while
   `privacy_payload_class = in_product_only` is non-conforming.
2. **`policy_forbidden_on_lock_screen` exposes no shortcut actions.**
   `allowed_action_kinds` is empty for this class; the user MUST open
   the in-product surface to act.
3. **`lock_screen_safe_generic` carries category-class labels only.**
   The allowed-label set is restricted to the category-and-severity
   subset; workspace and session labels are forbidden.
4. **Stripped kinds are recorded.** A privacy-redaction step that
   strips a forbidden label kind MUST list it on
   `privacy_redaction.stripped_label_kinds`. Silent strip is
   non-conforming.

## Exact-reopen linkage

Activating a notification, badge, dock progress mirror, lock-screen
quick action, companion push action, or system-tray summary MUST
resolve through one of the closed `reopen_target_kind` values and MUST
land on the same canonical event / object identity the in-product
surface uses:

- `canonical_object` — exact review item, build, branch, artifact,
  session, provider grant, route object;
- `canonical_route` — stable product route with target arguments;
- `review_context` — review sheet, diff view, approval, evidence
  context;
- `durable_activity_row` — durable activity-center row;
- `digest_group` — digest group row with member list and source
  filters;
- `placeholder_announced` — target missing, moved, blocked by
  policy, extension unavailable, or display topology lost;
- `denied_requires_revalidation` — wake-from-sleep, display
  reconnect, policy-epoch change, provider-grant narrowing, or
  trust-state change requires fresh user intent.

Every `exact_reopen_linkage` block carries:

- `reopen_target_kind` — the closed kind;
- `target_identity_ref` — the canonical object / route / review
  context / durable row / digest group ref (required for every kind
  that names a target);
- `command_id_ref` — the command the user invokes; the command
  itself owns preview / approval / revalidation logic;
- `returns_to_invoking_window` — whether the reopen returns to the
  invoking window or preserves an existing owner window;
- `preserves_canonical_event_id` and
  `preserves_canonical_object_identity` — both constant `true`;
- `must_resolve_through_in_product_surface` — constant `true`. An
  OS-level surface never owns the destination directly; the user
  always lands on the in-product surface that owns the canonical
  object.

A reopen-linkage that resolves to a generic home screen, a different
canonical event id, or an OS-owned destination is non-conforming.

## No-bypass on high-risk shortcuts

OS notifications, lock-screen quick actions, companion push actions,
dock / taskbar shortcuts, and system-tray summaries MUST NOT complete
high-risk or mutating actions inside the OS shortcut path. The
`bypass_protection` block on the suppression record names the
in-product surface the user MUST re-enter:

- `must_route_through_review_sheet = true` with a non-null
  `review_sheet_ref` — the user lands on the in-product review sheet
  before any apply / publish / mutate action runs;
- `must_route_through_approval_workflow = true` with a non-null
  `approval_workflow_ref` — the user lands on the approval workflow
  surface for collaboration / authority changes;
- `requires_revalidation = true` with a `revalidation_reason_label`
  — fresh user intent is required (wake-from-sleep, display
  reconnect, policy-epoch change, provider-grant narrowing,
  trust-state change);
- `interaction_safety_packet_id_ref` — opaque ref to the interaction-
  safety packet that owns authority, consequence, preview / apply /
  revert, and focus-return posture for the action.

The `forbidden_shortcut_action_classes` array is the mechanical no-
bypass review surface. The thirteen frozen classes are:

- `destructive_publish_or_apply`
- `secret_or_credential_reveal`
- `irreversible_high_blast`
- `bypass_review_sheet`
- `bypass_approval_workflow`
- `cross_workspace_mutation`
- `direct_mutation_from_lock_screen`
- `direct_mutation_from_companion_push`
- `direct_mutation_from_dock_or_taskbar`
- `direct_mutation_from_system_tray`
- `policy_override_from_os_shortcut`
- `trust_state_change_from_os_shortcut`
- `provider_grant_change_from_os_shortcut`

A `desktop_summary_affordance_record` whose `effect_class` is
`forbidden_mutation_shortcut` MUST list at least one forbidden class so
review tooling can see precisely which class triggered the rule.

Rules (frozen):

1. **A high-risk action cannot be completed from an OS payload.** A
   record whose `effect_class` is `mutation_via_review_path_only` or
   `mutation_via_in_product_only` MUST carry a non-null
   `review_sheet_ref`; the OS payload itself never holds the
   completion path.
2. **Lock-screen affordances are read-only.** `affordance_class =
   lock_screen_quick_action` restricts `effect_class` to the read-
   only-reopen / read-only-progress / read-only-badge / read-only-
   summary variants or `forbidden_mutation_shortcut`. Mutation
   classes are non-conforming.
3. **Dock / taskbar / system-tray / desktop widget mirrors are read-
   only.** Same restriction applies; these surfaces mirror durable-
   job phase, badge counts, or summary counts but never complete a
   mutation.
4. **Forbidden shortcuts are explicit.** A privacy-safe payload that
   would otherwise expose a forbidden class MUST list it on
   `forbidden_shortcut_action_classes` so the audit row records the
   refusal.

## Desktop summary affordances

A `desktop_summary_affordance_record` describes one dock / taskbar /
system-tray / lock-screen quick action / companion push action /
desktop widget summary that mirrors a durable-job envelope. The
record exists so the affordance:

- mirrors the same `canonical_event_id` and `canonical_object_target_ref`
  the in-product surface uses;
- mirrors `durable_job_id` when the underlying work is a durable job
  (read-only progress mirror);
- declares its `effect_class` from the closed set so review tooling
  can confirm it is read-only or routes through a review path;
- carries a `privacy_payload_class` that matches the surface's
  privacy posture;
- carries a `progress_value_basis_class` so a progress mirror's value
  is traceable to the durable-job phase, the envelope's progress
  field, the grouped-burst member count, the canonical object's
  state, or `not_a_progress_source`;
- carries an `exact_reopen_linkage` so activating the affordance
  resolves into the in-product surface that owns the canonical
  object;
- assigns `preserves_durable_history`, `preserves_canonical_event_id`,
  and `preserves_canonical_object_identity` constant `true` so the
  affordance never mints private chronology or identity.

Affordance classes are a closed set:

- `os_notification_action`
- `dock_taskbar_progress`
- `dock_taskbar_badge`
- `system_tray_summary`
- `lock_screen_quick_action`
- `companion_push_action`
- `desktop_widget_summary`

`progress_value_basis_class` is one of:

- `derived_from_durable_job_phase`
- `derived_from_envelope_progress`
- `derived_from_grouped_burst_member_count`
- `derived_from_canonical_object_state`
- `not_a_progress_source`

A surface that increments a progress mirror without naming an
upstream basis class is non-conforming.

`announce_actions_to_assistive_tech` is a boolean; OS shortcut
buttons and lock-screen quick actions MUST be announced to assistive
technology so a user with screen reading or switch control sees the
same actions a sighted user does.

## Audit-stream events

Audit events on the suppression-record stream complement the lineage
audit stream named in `notification_delivery_contract.md`:

| Audit-event id                                | Fires when                                                                                                  |
|-----------------------------------------------|-------------------------------------------------------------------------------------------------------------|
| `suppression_record_opened`                   | A new suppression record is minted for a held / delayed / collapsed / redacted / denied delivery.           |
| `suppression_record_release_recorded`         | A held record releases under a closed `release_trigger_class`.                                              |
| `suppression_record_redaction_recorded`       | A privacy-redaction step strips one or more forbidden label kinds.                                          |
| `suppression_record_audit_trail_only_emitted` | A record resolves to `not_delivered_audit_trail_only` because lock-screen or admin policy denied delivery.  |
| `suppression_record_bypass_blocked`           | A high-risk OS shortcut would have completed a forbidden class; the action is refused and routed to review. |
| `suppression_record_reopen_resolved`          | An OS / dock / taskbar / lock-screen / companion / system-tray reopen resolves to a canonical destination.  |
| `desktop_summary_affordance_synced`           | A dock / taskbar / system-tray / desktop widget summary syncs to the underlying durable-job envelope.       |
| `desktop_summary_affordance_drift_denied`     | An affordance update would have widened privacy posture or assumed a private mutation path; the update is refused. |

## Schema-of-record posture

The eventual notification-router / desktop-summary crate's Rust types
are the source of truth. The JSON Schema export at
`schemas/ux/notification_suppression_record.schema.json` is the cross-
tool boundary every non-owning surface reads. Adding a new
suppression class, suppression outcome, desktop affordance class,
desktop affordance effect class, forbidden shortcut action class,
allowed label kind, allowed action kind, redaction template token,
release trigger class, or progress-value basis class is additive-
minor and bumps `notification_suppression_schema_version`;
repurposing an existing value is breaking and requires a new decision
row.

There is no external IDL or code-generator toolchain at this
milestone; this mirrors the upstream notification, notification-
delivery, and durable-job contracts.

## Reuse guarantee

This contract is reusable by every owning subsystem and every desktop
adapter author without redefining core suppression, privacy, reopen,
or no-bypass semantics. A subsystem that mints a notification MUST:

1. Open one `notification_suppression_record` per `(canonical_event_id,
   intended_delivery_surface_class, client_scope)` triple whenever a
   delivery is held, delayed, collapsed, redacted, or denied; never
   silently drop the delivery.
2. Re-export the suppression record's `canonical_event_id`,
   `event_lineage_id_ref`, `canonical_object_target_ref`,
   `source_subsystem`, and `actor_identity_ref` from the upstream
   lineage and envelope; never split canonical object identity across
   suppression records.
3. Resolve the suppression to one of the frozen `suppression_class`
   rows and honor the default outcome, release rule, and required
   quiet-hours mode for that row.
4. Carry a non-empty `preserved_durable_linkback` with
   `is_durable = true` for every class except `denied_audit_trail_only`,
   which carries `kind = audit_trail_only` and `is_durable = false`.
5. Carry a `privacy_redaction` block whose `applied_payload_class`
   matches the rendered surface's payload class and whose
   `stripped_label_kinds` records every forbidden label kind removed
   on the way to the OS sink.
6. Carry an `exact_reopen_linkage` block whose
   `reopen_target_kind` is one of the closed kinds and whose
   `must_resolve_through_in_product_surface` is `true`.
7. Carry a `bypass_protection` block whose flags reflect whether the
   in-product surface MUST route through a review sheet, approval
   workflow, or revalidation; never invent surface-local mutation
   shortcuts.
8. Mint a `desktop_summary_affordance_record` for every dock /
   taskbar / system-tray / lock-screen quick action / companion push
   action / desktop widget summary that mirrors the durable-job
   envelope, and carry the same `canonical_event_id` and
   `canonical_object_target_ref` the in-product surface uses.
