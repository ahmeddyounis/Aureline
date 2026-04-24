# Transport-posture, environment-status-strip, and repair-action-card contract

This document is the **cross-surface inspectability contract** for
Aureline's transport posture, environment status strip, and repair
actions. It freezes one transport-posture object, one environment-
status-strip cell shape, and one repair-action-card shape so that the
current proxy mode, trust-store source, mirror route, active policy
bundle, deployment profile, identity mode, offline / deny-all state,
and current interpreter / SDK / shell / target truth are inspectable
*before* failures harden into support folklore.

It exists so a status item, a diagnostics card, a support-export
representation, an admin-audit packet, and a boundary-manifest row
all reference one posture object instead of recomputing field names;
so the environment status strip never grows four parallel widgets
for "what interpreter / SDK / shell / target am I on?"; and so a
repair action discloses *what* it changes, *where* it applies,
*who* must hold authority, and *what side effects* it lands without
requiring the user to read the underlying code.

The contract is normative. Where this document disagrees with the
UI / UX Spec sections it quotes, the source spec wins and this
document MUST be updated in the same change. Where this document
disagrees with a downstream surface's private posture, status, or
repair model, this document wins and the surface is non-conforming.

## Companion artifacts

- [`/schemas/ux/transport_posture.schema.json`](../../schemas/ux/transport_posture.schema.json)
  — boundary schema for `transport_posture_record`,
  `environment_status_strip_record`, and
  `repair_action_card_record`, plus the embedded
  `transport_posture` object whose field names and enum values
  match the `transport_posture` already embedded in
  [`/schemas/network/network_attribution_record.schema.json`](../../schemas/network/network_attribution_record.schema.json)
  by-name and by-value.
- [`/fixtures/ux/repair_cards/`](../../fixtures/ux/repair_cards/)
  — worked repair-action-card and supporting status-strip /
  posture fixtures covering the local, remote, managed, and
  policy-gated scope classes plus the offline, mirror-only, and
  deny-all postures the spec calls out by name.

## Upstream contracts this contract rides on

This contract does **not** re-mint vocabulary already frozen in
upstream seeds; it consumes them by name and by value:

- [`/docs/network/transport_governance_seed.md`](../network/transport_governance_seed.md)
  and [`/schemas/network/network_attribution_record.schema.json`](../../schemas/network/network_attribution_record.schema.json)
  — proxy resolution mode, trust-store source, SSH host-key
  provenance, mirror route, offline / deny-all state, and the
  embedded `transport_posture` object. This contract re-exports
  those vocabularies verbatim so one posture object value-matches
  across both schemas.
- [`/docs/execution/context_inspector_packet.md`](../execution/context_inspector_packet.md)
  and [`/schemas/execution/context_snapshot.schema.json`](../../schemas/execution/context_snapshot.schema.json)
  — execution-context snapshot shape the environment status strip
  reads its interpreter / SDK / shell / target cells against.
- [`/docs/identity/offline_entitlement_and_policy_seed.md`](../identity/offline_entitlement_and_policy_seed.md)
  and [`/schemas/identity/policy_bundle.schema.json`](../../schemas/identity/policy_bundle.schema.json),
  [`/schemas/identity/entitlement_snapshot.schema.json`](../../schemas/identity/entitlement_snapshot.schema.json)
  — `active_policy_bundle_ref`, `policy_epoch_ref`, and the
  entitlement / grace state vocabulary repair cards cite when an
  action is policy-gated or entitlement-gated.
- [`/docs/governance/telemetry_and_support_schema_registry.md`](../governance/telemetry_and_support_schema_registry.md)
  — registry rows for the support-bundle, diagnostics, and
  admin-audit packet families that consume this posture.
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  — credential handles cited by repair cards that touch local
  credential storage.
- [`/docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md)
  — `client_scope`, `redaction_class`, `freshness_class` re-exported
  without modification.

## Who reads this contract

- **Status item host (desktop), CLI / headless diagnostics line,
  diagnostics-card surface, support-export builder,
  admin-audit-packet emitter, boundary-manifest row writer** — to
  read **one** posture object instead of recomputing fields.
- **Environment status strip (desktop and dev-only inspector)** —
  to render interpreter, SDK, shell, and target cells from one
  shape with one source / pin / state vocabulary and one modal-
  prohibition rule.
- **Doctor / explainers, Project Doctor finding cards, repair
  affordances on diagnostics cards, support-export reviewers,
  admin consoles** — to render one repair-action-card shape with
  scope, authority, consent, and side-effect disclosed before the
  action runs.
- **Reviewers (release, security, accessibility)** — to verify
  that a repair card cannot be promoted to a destructive action
  without naming side effects, and that a status-strip cell
  cannot raise a modal in violation of the modal-prohibition
  rule.

## Two questions the contract answers

Any Aureline surface claiming to expose current transport posture,
environment truth, or a repair affordance MUST answer both
questions mechanically, without relying on per-surface copy:

1. **What is the current posture?** Which proxy resolution mode,
   trust-store source, mirror route, active policy bundle, policy
   epoch, deployment profile, identity mode, and offline / deny-all
   state are in force right now? Which interpreter / SDK / shell /
   target did the execution-context resolver land on, where did
   each value come from, what scope (if any) pins it, and is the
   cell `ok_resolved`, `degraded_resolved`, `stale_unverified`,
   `blocked_*`, or `unknown_repair_required`?
2. **If a repair is offered, what does it touch?** Which scope
   (local, local-and-remote, remote-only, managed-only, policy-
   bundle-only, admin-console-only, cross-machine user profile)
   does the action change? Which authority must hold (user-local,
   workspace-trust, remote-target, managed-provider, policy-bundle
   signer, org-admin, OS, extension owner)? Which consent gate is
   required (none, local confirmation, step-up, approval ticket,
   admin co-sign, blocked)? Which side effects land (writes_*,
   rebinds_*, switches_*, restarts_*, loads / unloads / rotates
   credential or CA bundle, requests_*_refresh, publishes_*_audit,
   inspect-only)? Which evidence records (transport posture,
   network attribution, policy bundle, entitlement snapshot,
   execution-context snapshot, approval ticket, doctor finding,
   support bundle, admin audit packet) does the card cite?

Generic prose like "fix this", "try again", "looks fine", or
"network issue" is forbidden when a more precise state and a
more precise action are knowable. The schema enforces typed
sentences and typed vocabulary; surfaces render those values.

## The transport-posture object

`transport_posture` is the single inspectable object that status
items, diagnostics cards, support exports, admin-audit packets,
and boundary-manifest rows reference without recomputing field
names. The schema declares it once in this contract, and the
network attribution record (which lives upstream of this
contract) embeds the **same field names and the same enum
values** as `transport_posture_at_event`. One object, one
vocabulary, one redaction class.

### Fields

The object carries:

- `proxy_resolution_mode` — one of `system_proxy`, `pac_proxy`,
  `environment_proxy`, `manual_proxy`, `direct_no_proxy`,
  `proxy_unknown`.
- `effective_proxy_ref` — opaque ref to the resolved proxy
  endpoint. Raw URLs and raw proxy credentials never appear.
- `trust_store_source` — one of `os_trust_store`,
  `os_trust_store_plus_org_ca_bundle`,
  `pinned_control_plane_trust_only`,
  `air_gap_offline_trust_root`, `trust_store_unknown`.
- `org_ca_bundle_fingerprint` — opaque fingerprint of the
  enterprise / org CA bundle, non-null when the source is
  `os_trust_store_plus_org_ca_bundle`.
- `pinned_control_plane_trust_refs` — opaque refs for the
  pinned control-plane trust roots, non-empty when the source
  is `pinned_control_plane_trust_only`.
- `ssh_host_key_default_provenance` — one of the six provenance
  classes from the transport governance seed.
- `mirror_route_class` — one of the six mirror-route classes.
- `mirror_endpoint_ref`, `mirror_snapshot_ref` — opaque refs to
  the active mirror endpoint and snapshot.
- `active_deployment_profile` — one of `individual_local`,
  `self_hosted`, `enterprise_online`, `air_gapped`,
  `managed_cloud`.
- `active_identity_mode` — one of `account_free_local`,
  `self_hosted_org`, `managed_workspace` (ADR-0001).
- `active_policy_bundle_ref` — opaque ref to the bound signed
  policy bundle. Null is admissible on `account_free_local`.
- `policy_epoch_ref` — opaque ref to the policy epoch the bundle
  resolved at.
- `offline_or_deny_all_state` — one of `online_live_allowed`,
  `online_mirror_only`, `offline_grace_preserved`,
  `offline_air_gapped`, `deny_all_enforced`,
  `network_disabled_by_user`, `network_degraded_heuristic`.
- `offline_since_at` — when the current offline / degraded
  posture began. Null is admissible whenever the state is
  `online_live_allowed`; the schema enforces that pairing.
- `posture_note` — short reviewable sentence summarising the
  posture in product terms.
- `captured_at` — RFC 3339 UTC timestamp from a monotonic clock
  source.

### Same shape, three consumers

Per the acceptance criterion *one posture object can back status
item, diagnostics card, and support-export representations
without changing field names*, this contract guarantees:

- The status-item host reads `transport_posture` directly to
  render a single glanceable badge with a privacy-safe
  `posture_note` sentence.
- The diagnostics card reads the same object to render a
  rectangular card naming proxy mode, trust-store source,
  mirror route, deployment profile, identity mode, and
  offline / deny-all state with opaque refs for each.
- The support-export representation serializes the same object
  byte-for-byte under the declared `redaction_class`. The export
  redaction discipline never widens redaction relative to the
  live surface.

The `consumer_surfaces` field on `transport_posture_record`
enumerates which consumers are reading a given record so a
reviewer can confirm one record fans out to the expected
surfaces.

### Forbidden raw fields

Raw URLs, raw host names, raw IPs, raw SSH host keys, raw proxy
credentials, raw CA bundle bytes, raw certificate-chain PEM,
raw OAuth tokens, raw policy rule bodies, raw absolute paths,
and raw command lines never cross this boundary. The schema
enforces opaque-ref and opaque-fingerprint patterns; surfaces
that need to render a friendly label resolve it from the
opaque ref locally and never embed raw bytes in the record.

## The environment-status-strip contract

The environment status strip is the ambient cross-surface row
that names the **current interpreter / SDK / shell / target
truth** for the active workspace, workset, or remote session.
It is one row of cells, not four widgets.

### Cell shape

Every cell carries:

- `cell_kind` — one of `interpreter_or_toolchain_status`,
  `sdk_status`, `shell_status`, `target_status`. The four kinds
  cover the four PRD-named environment axes; cells whose kind
  does not apply to the active surface remain present in the
  strip with `status_state = not_applicable_for_surface` so the
  strip never reads as a partial collection.
- `resolved_label_text` — a short privacy-safe label rendered
  on the strip. MUST NOT embed raw absolute paths, raw remote
  host names, or raw build metadata.
- `resolved_value_ref` — opaque ref into the runtime execution-
  context resolver for the value being displayed. Cross-walks
  with the execution-context snapshot.
- `status_state` — one of the ten frozen state classes
  (`ok_resolved`, `ok_with_warning`, `degraded_resolved`,
  `stale_unverified`, `blocked_by_policy`,
  `blocked_by_permission`, `blocked_by_offline_or_deny_all`,
  `blocked_by_target_unreachable`, `unknown_repair_required`,
  `not_applicable_for_surface`).
- `source_class` — one of the ten frozen source classes
  (`workspace_explicit_pin`, `workspace_inherited_default`,
  `user_setting`, `policy_bundle_pin`, `platform_default`,
  `auto_detected_in_workspace`, `auto_detected_on_path`,
  `managed_provider_pin`, `remote_target_reported`,
  `source_unknown_repair_required`). Names where the resolved
  value came from.
- `source_ref` — opaque ref into the originating settings,
  policy, workspace, or detection record.
- `pin_class` — one of the seven frozen pin classes
  (`pinned_workspace_scope`, `pinned_workset_scope`,
  `pinned_user_scope`, `pinned_policy_bundle_scope`,
  `pinned_managed_provider_scope`, `not_pinned_resolved_dynamically`,
  `pin_unknown_repair_required`). Names the binding scope.
- `pin_scope_ref` — opaque ref naming the scope that holds the
  pin (workspace, workset, user-profile, policy-bundle, managed-
  provider). Null is admissible for `not_pinned_resolved_dynamically`.
- `degraded_or_blocked_reason` — short reviewable sentence
  required whenever `status_state` is anything other than
  `ok_resolved` or `not_applicable_for_surface`. The schema
  enforces a non-null sentence on every degraded / blocked /
  stale / unknown cell.
- `repair_action_card_refs` — opaque refs to the repair-action
  cards the strip exposes when state is repairable. Empty when
  no repair action is available.
- `modal_prohibition_reason` — one of the nine frozen modal-
  prohibition classes. Names *why* this cell may not raise a
  modal from the strip itself.
- `captured_at` — RFC 3339 UTC timestamp from a monotonic clock.

### When a modal is prohibited

The status strip is an ambient glanceable surface. A modal
prompt from the strip would interrupt unrelated work and is
**non-conforming on the strip itself**. The
`modal_prohibition_reason` enumerates the nine cases:

- `ambient_glanceable_surface_no_modal` — default class for
  resolved cells. The strip exposes details only on click /
  focus, never modally.
- `non_blocking_warning_no_modal` — `ok_with_warning` cells
  expose the warning in a hovercard / popover, not a modal.
- `stale_state_repair_card_only_no_modal` — `stale_unverified`
  cells offer a repair-card promotion, not a modal.
- `policy_block_explainer_only_no_modal` — `blocked_by_policy`
  cells open a non-modal explainer that cites the policy
  bundle ref; the action itself is not invoked from the strip.
- `offline_or_deny_all_explainer_only_no_modal` —
  `blocked_by_offline_or_deny_all` cells render the offline /
  deny-all posture with a non-modal explainer naming
  `offline_or_deny_all_state` and `offline_since_at`.
- `permission_block_quiet_remediation_no_modal` —
  `blocked_by_permission` cells open the quiet remediation flow
  on click; no modal is raised from the strip.
- `remote_target_unreachable_explainer_only_no_modal` —
  `blocked_by_target_unreachable` cells open a non-modal
  explainer naming the target ref and the last-known
  reachability evidence.
- `modal_admissible_only_for_destructive_repair_confirmation`
  — modals are admissible only via repair-card promotion when
  the action is destructive (rotates / deletes / unloads
  credential or CA-bundle, removes known-hosts entry).
- `modal_admissible_only_for_consent_step_up` — modals are
  admissible only via repair-card promotion when consent
  step-up is required (`step_up_authentication_required`,
  `approval_ticket_required`, `admin_co_sign_required`).

The strip never owns the modal. Promotion to a repair-action
card owns it, and only then only for destructive confirmation
or step-up consent.

### Strip parity across surfaces

`environment_status_strip_record` carries a `client_scope`
field (`desktop_product`, `cli`, `companion_surface`,
`remote_agent`, `sdk_or_api`, `managed_admin_surface`) so the
desktop strip, the CLI / headless diagnostics line, and the
companion-surface row read the same shape. A `cli`-only or
`desktop_product`-only override of cell semantics is non-
conforming.

## The repair-action-card seed

`repair_action_card_record` is one structured card explaining a
single offered repair action. Cards are seeded by this contract
and consumed by status-strip cells, diagnostics cards, doctor
findings, support-export reviewers, and admin consoles.

### Required fields

Every card declares:

- `title_label_text` — short label for the card heading.
- `what_changes_sentence` — short reviewable sentence in
  product terms describing the concrete state the action would
  change. Generic "fix" or "retry" prose is non-conforming.
- `where_it_applies_sentence` — short reviewable sentence
  describing the scope the change lands in.
- `scope_class` — one of `scope_local_only`,
  `scope_local_and_remote_target`, `scope_remote_target_only`,
  `scope_managed_provider_only`, `scope_policy_bundle_only`,
  `scope_org_admin_console_only`,
  `scope_cross_machine_user_profile`. Names where the change
  reaches.
- `authority_class` — one of `user_local_authority`,
  `user_workspace_authority`, `remote_target_authority`,
  `managed_provider_authority`, `policy_bundle_signer_authority`,
  `org_admin_authority`, `platform_os_authority`,
  `extension_owner_authority`. Names who must hold authority.
- `consent_class` — one of `no_consent_required_safe_default`,
  `user_confirmation_required_local`,
  `step_up_authentication_required`,
  `approval_ticket_required`, `admin_co_sign_required`,
  `consent_blocked_by_policy`, `consent_blocked_by_permission`,
  `consent_unknown_repair_required`. Names the consent / permission
  gate.
- `side_effects` — one or more declared effects from the closed
  vocabulary. Inspect-only cards declare exactly
  `no_side_effect_inspect_only` and may not also list any
  `writes_*`, `rebinds_*`, `switches_*`, `restarts_*`,
  `loads_*`, `unloads_*`, `rotates_*`, `deletes_*`, `removes_*`,
  `edits_*`, `requests_*`, or `publishes_*` effect.
- `transport_posture_ref` — opaque ref to the
  `transport_posture_record` the card was rendered against, so
  the same posture that drove status / diagnostics also drove
  the offered repair.
- `evidence_links` — opaque refs to the records the card cites
  (transport posture, network attribution, policy bundle,
  entitlement snapshot, execution-context snapshot, approval
  ticket, doctor finding, support bundle, admin audit packet,
  schema registry entry, settings record, browser handoff
  packet). Cards driven by a deny / block state SHOULD link the
  attribution / bundle / snapshot that explains the deny; the
  schema enforces this for `consent_blocked_*` cards.
- `redaction_class` — one of the four standard classes. Cards
  default to `metadata_safe_default` and never widen redaction
  relative to the underlying evidence.

### Scope and effect, disclosed clearly

Per the acceptance criterion *repair-card fixtures disclose
scope and effect clearly enough that a reviewer can tell whether
the action alters local state, remote state, or managed /
provider state*, the schema enforces:

- `scope_local_only` cards may not declare effects that imply
  remote, managed, policy-bundle, admin-console, or cross-
  machine reach. The `writes_remote_target_setting`,
  `edits_remote_known_hosts_entry`,
  `removes_remote_known_hosts_entry`, `restarts_remote_session`,
  `switches_active_policy_bundle`,
  `requests_policy_bundle_refresh`,
  `requests_entitlement_refresh`, and `publishes_admin_audit_event`
  effects are forbidden on `scope_local_only` cards.
- `org_admin_authority` and `policy_bundle_signer_authority`
  cards may not claim `scope_local_only`. The action's reach
  exceeds the local device by definition.
- `consent_blocked_by_policy` and `consent_blocked_by_permission`
  cards MUST link at least one evidence record so a reviewer can
  read why the action is unavailable.

### Inspect-only cards

A card whose action is to open an inspector, copy an opaque
ref, or surface a non-mutating explainer declares exactly
`no_side_effect_inspect_only` in `side_effects`. The schema
forbids stacking `no_side_effect_inspect_only` with any other
effect; inspect-only is a closed posture.

### Promotion provenance

A card promoted from a status-strip cell MUST set
`originating_status_cell_kind` to the kind of cell that promoted
it. A card promoted from a diagnostics card MUST set
`originating_diagnostics_card_ref` to the diagnostics card's
opaque ref. Cards promoted from doctor findings or support-
export review pages set neither and rely on `evidence_links` to
name the originating record.

## Offline, mirror-only, and deny-all examples remain explicit

Per the acceptance criterion *offline, mirror-only, and deny-all
examples remain explicit rather than being inferred from generic
errors*, this contract treats the three postures as **first-
class postures**, not failure modes:

- **Offline (grace preserved or air-gapped).** The transport
  posture carries `offline_or_deny_all_state =
  offline_grace_preserved` or `offline_air_gapped` with a non-
  null `offline_since_at`. Status-strip cells whose value
  cannot be re-resolved without network resolve to
  `blocked_by_offline_or_deny_all` with a typed
  `degraded_or_blocked_reason` sentence and a
  `modal_prohibition_reason =
  offline_or_deny_all_explainer_only_no_modal`. Repair cards
  offered in this state declare `consent_blocked_by_*` only
  when policy explicitly blocks the action; otherwise they
  remain available with a `where_it_applies_sentence` naming
  the local-only or mirror-only effect.
- **Mirror-only.** The posture carries `mirror_route_class =
  mirror_only_no_direct_allowed` and
  `offline_or_deny_all_state = online_mirror_only`, with a
  non-null `mirror_endpoint_ref` and `mirror_snapshot_ref`.
  Repair cards that would otherwise reach a direct origin
  declare `where_it_applies_sentence` naming the mirror and
  `evidence_links` citing the mirror snapshot. A card that
  cannot route through the mirror declares
  `consent_blocked_by_policy` and links the policy-bundle ref.
- **Deny-all enforced.** The posture carries
  `offline_or_deny_all_state = deny_all_enforced`. Status-strip
  cells that depend on egress collapse to
  `blocked_by_offline_or_deny_all` with the
  `offline_or_deny_all_explainer_only_no_modal` modal-prohibition
  reason. Repair cards that would mint egress declare
  `consent_blocked_by_policy` and link the deny-all policy rule
  ref; only inspect-only or local-only cards remain offered.

The schema enforces the deny-all collapse on the embedded
`transport_posture` (an `online_live_allowed` posture cannot
also carry an `offline_since_at`); the network attribution
record adds the matching collapse on the egress side.

## Cross-surface invariants

The schema enforces the following cross-surface invariants
mechanically:

1. A `transport_posture_record` with
   `offline_or_deny_all_state = online_live_allowed` cannot
   carry an `offline_since_at` timestamp.
2. Every environment-status cell whose `status_state` is anything
   other than `ok_resolved` or `not_applicable_for_surface` MUST
   carry a non-null `degraded_or_blocked_reason` sentence.
3. A repair-action card declaring `no_side_effect_inspect_only`
   MUST list **only** that effect; inspect-only is a closed
   posture.
4. A repair-action card with `scope_class = scope_local_only`
   MUST NOT declare any side effect that implies remote,
   managed, policy-bundle, admin-console, or cross-machine
   reach.
5. A repair-action card with `authority_class` of
   `org_admin_authority` or `policy_bundle_signer_authority`
   MUST NOT claim `scope_local_only`.
6. A repair-action card with `consent_class` of
   `consent_blocked_by_policy` or `consent_blocked_by_permission`
   MUST link at least one evidence record.

These are not suggestions; they are schema-enforced rules. A
fixture that fails any of them is non-conforming.

## Redaction and export posture

Every record (`transport_posture_record`,
`environment_status_strip_record`, `repair_action_card_record`)
carries a `redaction_class` from the four-value standard
vocabulary (`metadata_safe_default`, `operator_only_restricted`,
`internal_support_restricted`, `signing_evidence_only`) and an
`export_safe` flag. Raw URLs, raw host names, raw IPs, raw SSH
host keys, raw proxy credentials, raw CA bundle bytes, raw
certificate chain PEM, raw OAuth tokens, raw policy rule
bodies, raw absolute paths, raw command lines, and raw secret
values MUST NOT appear on this boundary. Diagnostics cards,
status items, support exports, and admin-audit packets reference
the same opaque refs and opaque fingerprints the records expose.

## Adding or changing vocabulary

Adding a value to any vocabulary in this contract is
**additive-minor**:

1. Update the schema enum in
   `schemas/ux/transport_posture.schema.json`.
2. Update this document.
3. Add or update a fixture under
   `fixtures/ux/repair_cards/` exercising the new value.
4. Bump `transport_posture_schema_version`.

Repurposing an existing value is **breaking** and requires:

1. A new decision row in
   `artifacts/governance/decision_index.yaml` co-signed by
   `security_trust_review` and `product_scope_review`.
2. Deprecation of the old value, addition of the new value
   through an additive-minor landing, and a translation pass on
   support exports and admin-audit packets across the
   deprecation window.

## Out of scope at this revision

- Final settings-screen UI for reading or editing pinned
  interpreter / SDK / shell / target values.
- Live policy-bundle backend, signed-policy distribution
  pipeline, and admin-console implementation.
- The full network stack, HTTP client, TLS verifier, SSH client,
  and proxy resolver implementations.
- Pixel-perfect strip layout, animation timings, and the
  cross-platform widget toolkit.
- Localization-ready string tables; the contract carries
  reviewable English sentences and the localization layer is
  consumed separately.
