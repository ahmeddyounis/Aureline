# Preview share-sheet, session-visibility, and revoke / expiry contract

This document freezes the **share-surface contract** every preview surface
projects when a user opens the share sheet, mints a share link, or sees an
existing share link transition into a revoked, expired, or otherwise
terminal end state. The goal is to make sure no preview share surface —
browser, native, or embedded — can present a "current live preview" link
without disclosing audience, destination, auth posture, expiry, revoke
path, export posture, current resolved state (live vs. captured vs.
terminal-unavailable), runtime lineage, and continuity on the same record,
and to make sure revocation, expiry, runtime restart, target change, and
stale-capture transitions never leave old links masquerading as current
live previews.

This contract sits **above** the cross-surface preview-snapshot record
and the preview-runtime strip / picker / hot-reload contract, and
**below** any consumer share UX. It does **not** implement sharing
infrastructure, hosting, or external relays; it freezes how a share
sheet, a minted link, and a revoked / expired / terminal link are allowed
to look across the three preview lanes.

Companion artifacts:

- [`/schemas/preview/preview_share_sheet.schema.json`](../../schemas/preview/preview_share_sheet.schema.json)
  — boundary schema for the `preview_share_sheet_record` the chooser
  surface emits.
- [`/schemas/preview/preview_share_link.schema.json`](../../schemas/preview/preview_share_link.schema.json)
  — boundary schema for the `preview_share_link_record` every minted,
  in-flight, expired, revoked, superseded, or terminal-unavailable
  preview share carries.
- [`/fixtures/preview/preview_share_cases/`](../../fixtures/preview/preview_share_cases/)
  — worked corpus of share-sheet and share-link cases.
- [`/docs/network/route_class_matrix.md`](../network/route_class_matrix.md)
  and [`/artifacts/network/route_classes.yaml`](../../artifacts/network/route_classes.yaml)
  — canonical route exposure classes that share links and tunnel endpoints map
  through for support and audit export semantics.
- [`/schemas/preview/preview_snapshot.schema.json`](../../schemas/preview/preview_snapshot.schema.json)
  and [`/docs/architecture/preview_runtime_contract.md`](../architecture/preview_runtime_contract.md)
  — cross-surface preview-snapshot record this contract projects from.
- [`/schemas/preview/preview_runtime_strip.schema.json`](../../schemas/preview/preview_runtime_strip.schema.json)
  and [`/docs/preview/preview_runtime_surface_contract.md`](./preview_runtime_surface_contract.md)
  — preview-runtime strip / picker / hot-reload contract.
- [`/schemas/security/trust_class.schema.json`](../../schemas/security/trust_class.schema.json)
  — safe-preview trust-class, connectivity-state, and downgrade-trigger
  ladder this contract re-exports.

If this document disagrees with the PRD, the TAD, or the UX spec, those
sources win and this document plus the schemas update in the same change.

## Why a separate share-surface contract

The cross-surface `preview_snapshot_record` already reserves a typed
`share_sheet` extension point that names visibility, auth/session, and
revoke posture. It does **not** answer:

- who specifically can open a particular link, for how long, and under
  what auth;
- whether a particular link, when opened **right now**, resolves to a
  live runtime, a captured snapshot, a static export, or a terminal
  end state;
- what happens to an old link after the runtime restarts, the device
  target changes, the captured replay goes stale, the workspace trust
  is revoked, or share policy narrows;
- how a revoked / expired / superseded / terminal link transitions into
  a controlled end state with an explanation and a regenerate path;
- how share semantics reuse preview/runtime identity instead of inventing
  a parallel anonymous-link model.

Without a frozen share-surface contract:

- a share UI can render an existing link as "current live preview" after
  the runtime has restarted, the target has changed, or the captured
  replay has gone stale;
- a public link can be minted without a revoke path or without an
  expiry timestamp;
- a one-time link can be re-used because the schema does not pin the
  `one_time_use_expiry` posture to the audience;
- revocation, expiry, and policy-block paths can collapse into one
  vague "share unavailable" label that hides the regenerate path;
- export posture can drift across audiences (a `full_record_admitted`
  export can leak through an external audience).

This contract closes those gaps with two frozen records:

1. The **share-sheet record** the chooser surface emits when an operator
   is about to mint a share. The record is a typed proposal: every
   `proposed_share_*` field declares the posture the operator chose, and
   the schema gates `mint_admissible = true` on the proposal being
   admissible against share policy, audience-vs-destination pairings,
   and runtime / target / capture continuity.
2. The **share-link record** every minted, in-flight, expired, revoked,
   superseded, or terminal-unavailable link carries. The record is the
   single source of truth for who can open the link, for how long,
   under what auth, and whether opening it right now resolves to a live
   runtime, a captured snapshot, a static export, or a controlled
   terminal end state.

Both records cite the underlying `preview_snapshot_record` (and
optionally the `preview_runtime_strip_record`, `hot_reload_state_record`,
and `device_target_descriptor_record`) by opaque ref, so share semantics
reuse the preview/runtime identity that already lives on the snapshot
contract. No parallel anonymous-link identity model is admitted.

## Scope

Frozen at this revision:

- the `preview_share_sheet_record` shape, its closed `proposed_share_*`
  vocabulary, the `mint_admissible` gate, the typed
  `mint_blocked_reason_class` set, the closed `sheet_disposition_class`
  vocabulary, and the rule that any `sheet_admitted_regenerate_from_*`
  disposition cites the predecessor `preview_share_link_record_ref`;
- the `preview_share_link_record` shape, the closed six-class
  `share_mode_class` partition (local-only, organization-only,
  temporary-external, one-time-external, policy-blocked,
  revoked-or-expired), the closed `share_audience_class`,
  `share_destination_class`, `share_auth_class`, `share_expiry_class`,
  `share_revoke_path_class`, `share_export_posture_class`,
  `share_resolved_state_class`, `share_runtime_lineage_class`,
  `share_continuity_state_class`, `share_lifecycle_state_class`,
  `share_regenerate_path_class`, `share_terminal_explanation_class`,
  `share_revoke_actor_class`, and `share_revoke_reason_class`
  vocabularies, and the per-state pairings that force each lifecycle
  transition into a controlled terminal end state with a typed
  explanation and a typed regenerate path;
- the rule that `share_resolved_state_class = resolves_to_live_runtime`
  is admissible only when `share_lifecycle_state_class = active_live`
  AND `share_runtime_lineage_class = bound_to_current_live_runtime`
  AND `share_continuity_state_class = continuity_intact` AND the link
  is not in any revoked / expired / policy-blocked / trust-revoked
  posture, so old links cannot masquerade as current live previews;
- the rule that every terminal lifecycle (`expired`, `revoked_by_*`,
  `superseded_by_regenerate`, `terminal_unavailable_*`) cites a typed
  `share_terminal_explanation_class` and a typed
  `share_regenerate_path_class` that is not `no_regenerate_required`;
- the redaction floor on every record: raw URLs, raw absolute paths,
  raw IP addresses, raw hostnames, raw bearer tokens, raw session
  cookies, raw expiring credentials, raw rendered bytes, raw stack
  frames, and raw mock-data bodies never cross either boundary; opaque
  snapshot / runtime / target / link / approval / regenerate handles
  and class labels do.

Out of scope (named explicitly so the schemas do not creep):

- implementing share infrastructure, share-link minting, external
  hosting, or external relays;
- minting framework-specific share-token formats or transport
  protocols;
- the org / tenant policy schema (only opaque approval-ticket and
  policy-record refs are carried);
- the share-sheet UX layout (the schema only freezes the typed proposal
  the sheet emits);
- the export-packet schema (only opaque export-record refs are carried);
- any share-specific telemetry; payloads narrow through the
  telemetry/support registry like every other surface.

## The share-sheet record

The share sheet is the chooser surface a user sees before any preview
share is minted. Its boundary record is `preview_share_sheet_record`.
The record is a typed proposal — every `proposed_share_*` field declares
the posture the operator chose — and the schema gates
`mint_admissible = true` on the proposal being admissible.

### Required disclosure floor

A share-sheet record MUST display all of:

| Field group              | Sheet field                                                        |
|--------------------------|--------------------------------------------------------------------|
| Snapshot identity        | `preview_snapshot_record_ref`                                      |
| Lane / mode              | `preview_lane_class`, `preview_mode_class`                         |
| Audience / mode          | `proposed_share_mode_class`, `proposed_share_audience_class`       |
| Destination              | `proposed_share_destination_class`                                 |
| Auth / session           | `proposed_share_auth_class`                                        |
| Expiry                   | `proposed_share_expiry_class` (+ `proposed_expires_at` when typed) |
| Revoke path              | `proposed_share_revoke_path_class`                                 |
| Export posture           | `proposed_share_export_posture_class`                              |
| Resolution / lineage     | `proposed_share_resolved_state_class`, `proposed_share_runtime_lineage_class` |
| Continuity               | `proposed_share_continuity_state_class`                            |
| Regenerate path          | `proposed_share_regenerate_path_class`                             |
| Mint admissibility       | `mint_admissible`, `mint_blocked_reason_class`                     |
| Disposition              | `sheet_disposition_class`                                          |

The schema enforces that `mint_admissible = true` requires
`mint_blocked_reason_class = not_blocked_mint_admissible`, the proposal
to be `continuity_intact`, the resolution and lineage to be in the
non-blocked classes, and the disposition to be one of
`sheet_admitted_minted_link` or any `sheet_admitted_regenerate_from_*`.
A `sheet_admitted_regenerate_from_*` disposition MUST cite the
predecessor `preview_share_link_record_ref` so the regenerate intent is
inspectable.

### Audience-vs-destination partition

`proposed_share_audience_class` is the only field that drives audience
semantics. The schema enforces these audience-vs-destination pairings:

- `workspace_local_only_no_share` → `local_only_share_mode`,
  `local_workspace_anchor_only`, `no_auth_required`,
  `no_expiry_local_only`, `no_revoke_required`, export in
  `{export_disabled_no_share, export_metadata_only}`,
  `proposed_expires_at = null`.
- `workspace_signed_in_only` / `organization_signed_in_only` /
  `tenant_signed_in_only` → `organization_only_share_mode`, destination
  in `{in_app_share_thread, managed_workspace_share_endpoint,
  managed_organization_share_endpoint, managed_tenant_share_endpoint}`,
  auth `!= no_auth_required`, revoke path `!= no_revoke_required`.
- `temporary_external_link_audience` → `temporary_external_share_mode`,
  destination in `{temporary_external_browser_link,
  approved_third_party_relay}`, auth `!= no_auth_required`, revoke path
  `!= no_revoke_required`, expiry in
  `{short_lived_minutes_expiry, short_lived_hours_expiry,
  explicit_timestamp_expiry}` with a non-null `proposed_expires_at`.
- `one_time_external_link_audience` → `one_time_external_share_mode`,
  destination in `{one_time_external_browser_link,
  approved_third_party_relay}`, auth in
  `{step_up_auth_required, one_time_token_required,
  approval_ticket_required}`, revoke path `!= no_revoke_required`,
  expiry `= one_time_use_expiry`.
- `policy_blocked_no_share_audience` → `policy_blocked_share_mode`,
  destination `= no_destination_share_blocked`, export
  `= export_disabled_no_share`, resolution
  `= no_resolution_share_blocked`, lineage
  `= no_lineage_share_blocked`, continuity
  `= continuity_broken_share_blocked`, `mint_admissible = false`,
  `sheet_disposition_class = sheet_refused_policy_blocked`,
  `proposed_share_regenerate_path_class != no_regenerate_required`.
- `not_shareable_inherent_surface` → `policy_blocked_share_mode`,
  destination `= no_destination_share_blocked`, `mint_admissible = false`,
  `sheet_disposition_class = sheet_refused_policy_blocked`.

Export posture is gated separately:
`export_full_record_admitted_internal_only` is admissible only on the
three signed-in audiences. External audiences cannot propose a
full-record export.

### Disposition vocabulary

`sheet_disposition_class` is the closed disposition vocabulary the sheet
records. `sheet_open_proposal` is the in-flight chooser posture before
any decision; `sheet_admitted_minted_link` is the success posture (a
`preview_share_link_record` is minted); the
`sheet_refused_*` postures are the closed refusal set; the
`sheet_admitted_regenerate_from_*` postures cover sheets opened on top
of a revoked / expired / terminal predecessor link.

## The share-link record

The share link is the typed record every minted, in-flight, expired,
revoked, superseded, or terminal-unavailable preview share carries. Its
boundary record is `preview_share_link_record`.

### Required disclosure floor

A share-link record MUST display all of:

| Field group              | Link field                                                                   |
|--------------------------|------------------------------------------------------------------------------|
| Sheet / snapshot identity | `minted_from_share_sheet_record_ref`, `preview_snapshot_record_ref`         |
| Lane / mode              | `preview_lane_class`, `preview_mode_class`                                   |
| Audience / mode          | `share_mode_class`, `share_audience_class`                                   |
| Destination              | `share_destination_class`                                                    |
| Auth / session           | `share_auth_class`                                                           |
| Expiry                   | `share_expiry_class` (+ `expires_at` when typed)                             |
| Revoke path              | `share_revoke_path_class`                                                    |
| Export posture           | `share_export_posture_class`                                                 |
| Current resolution       | `share_resolved_state_class`                                                 |
| Runtime lineage          | `share_runtime_lineage_class`                                                |
| Continuity               | `share_continuity_state_class`                                               |
| Lifecycle                | `share_lifecycle_state_class`                                                |
| Regenerate path          | `share_regenerate_path_class`                                                |
| Terminal explanation     | `share_terminal_explanation_class`                                           |
| Revoke metadata          | `share_revoke_actor_class`, `share_revoke_reason_class`, `revoked_at`        |
| Successor                | `superseded_by_share_link_record_ref` (when `superseded_by_regenerate`)      |

The schema enforces that any `share_lifecycle_state_class` in
`{expired, revoked_by_user, revoked_by_admin, revoked_by_policy,
superseded_by_regenerate, terminal_unavailable_runtime_restart,
terminal_unavailable_target_change, terminal_unavailable_stale_capture,
terminal_unavailable_policy_blocked,
terminal_unavailable_workspace_trust_revoked}` forbids
`share_resolved_state_class` in the three live / captured / static
classes, requires `share_regenerate_path_class != no_regenerate_required`,
and requires `share_terminal_explanation_class != not_terminal_state`.

### The "no old link masquerades as live" rule

The schema gates `share_resolved_state_class = resolves_to_live_runtime`
on **every** of the following holding at the same time:

- `share_lifecycle_state_class = active_live`
- `share_runtime_lineage_class = bound_to_current_live_runtime`
- `share_continuity_state_class = continuity_intact`
- `share_terminal_explanation_class = not_terminal_state`
- `share_revoke_actor_class = not_revoked`
- `share_revoke_reason_class = not_revoked`
- `share_regenerate_path_class = no_regenerate_required`

Conversely, any of `share_lifecycle_state_class` in
`{expired, revoked_by_user, revoked_by_admin, revoked_by_policy,
superseded_by_regenerate, terminal_unavailable_*}` forces
`share_resolved_state_class` out of `resolves_to_live_runtime` and into
the corresponding `terminal_unavailable_*` class. This is the
schema-enforced version of the spec rule that old links never present
as current live previews.

### Lifecycle-to-terminal-explanation pairings

The schema enforces these per-state pairings so every terminal record
carries an inspectable terminal explanation:

| Lifecycle                                     | Resolved state                              | Continuity                              | Terminal explanation                                                                                                                          | Required side fields |
|-----------------------------------------------|---------------------------------------------|-----------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------|----------------------|
| `expired`                                     | `terminal_unavailable_expired`              | `continuity_broken_expired`             | `{terminal_explanation_automatic_expiry, terminal_explanation_one_time_use_consumed}` + `share_revoke_actor_class = automatic_expiry_actor`  | non-null `expired_at` |
| `revoked_by_user`                             | `terminal_unavailable_revoked`              | `continuity_broken_revoked`             | `terminal_explanation_user_self_revoke` + user reasons                                                                                       | non-null `revoked_at` |
| `revoked_by_admin`                            | `terminal_unavailable_revoked`              | `continuity_broken_revoked`             | admin terminal explanations + admin reasons                                                                                                  | non-null `revoked_at` |
| `revoked_by_policy`                           | `terminal_unavailable_revoked`              | `continuity_broken_policy_changed` or `continuity_broken_workspace_trust_revoked` | policy terminal explanations + policy reasons                                              | non-null `revoked_at` |
| `superseded_by_regenerate`                    | `terminal_unavailable_revoked`              | one of the five continuity-broken classes | (caller picks)                                                                                                                              | non-null `superseded_by_share_link_record_ref` + non-null `regenerate_target_share_sheet_record_ref` |
| `terminal_unavailable_runtime_restart`        | `terminal_unavailable_runtime_restarted`    | `continuity_broken_runtime_restarted`   | `{terminal_explanation_runtime_restarted, terminal_explanation_runtime_unavailable, terminal_explanation_managed_runtime_decommissioned}`    | runtime lineage out of three runtime-restart classes |
| `terminal_unavailable_target_change`          | `terminal_unavailable_target_changed`       | `continuity_broken_target_changed`      | `terminal_explanation_target_changed`                                                                                                        | runtime lineage `= bound_to_target_now_changed` |
| `terminal_unavailable_stale_capture`          | `terminal_unavailable_stale_capture`        | `continuity_broken_capture_stale`       | `terminal_explanation_capture_stale`                                                                                                         | runtime lineage in `{bound_to_captured_snapshot_only, bound_to_runtime_now_unavailable}` |
| `terminal_unavailable_policy_blocked`         | `terminal_unavailable_policy_blocked`       | `continuity_broken_policy_changed`      | `{terminal_explanation_policy_blocked_at_mint, terminal_explanation_policy_changed_post_mint, terminal_explanation_workspace_trust_revoked}` |  |
| `terminal_unavailable_workspace_trust_revoked`| `terminal_unavailable_workspace_trust_revoked`| `continuity_broken_workspace_trust_revoked` | `terminal_explanation_workspace_trust_revoked`                                                                                              |  |

### Active-link invariants

The schema forbids active links from carrying terminal metadata: any
`share_lifecycle_state_class` in `{active_live, active_captured}` forces
`share_terminal_explanation_class = not_terminal_state`,
`share_revoke_actor_class = not_revoked`,
`share_revoke_reason_class = not_revoked`,
`share_continuity_state_class = continuity_intact`, and
`share_regenerate_path_class = no_regenerate_required`.

### Audience-driven floors (re-stated for the link record)

The same audience-vs-destination floors that gate the share-sheet
proposal are re-asserted on the share-link record so a minted link
cannot drift from the proposal it was minted under:

- `workspace_local_only_no_share` pins the link to `local_only_share_mode`,
  `local_workspace_anchor_only`, `no_auth_required`,
  `no_expiry_local_only`, `no_revoke_required`, and export in
  `{export_disabled_no_share, export_metadata_only}` with
  `expires_at = null`.
- The three signed-in audiences pin the link to
  `organization_only_share_mode` and an in-app or managed-share
  destination, with auth `!= no_auth_required` and revoke path
  `!= no_revoke_required`.
- `temporary_external_link_audience` pins the link to
  `temporary_external_share_mode`, an external destination, auth
  `!= no_auth_required`, revoke path `!= no_revoke_required`, and a
  short-lived or explicit-timestamp expiry with a non-null `expires_at`.
- `one_time_external_link_audience` pins the link to
  `one_time_external_share_mode`, an external destination, auth in
  the three step-up / one-time-token / approval-ticket classes, revoke
  path `!= no_revoke_required`, and `share_expiry_class = one_time_use_expiry`.
- `policy_blocked_no_share_audience` pins the link to
  `policy_blocked_share_mode`, `no_destination_share_blocked`,
  `export_disabled_no_share`,
  `share_resolved_state_class = terminal_unavailable_policy_blocked`,
  `share_lifecycle_state_class = terminal_unavailable_policy_blocked`,
  a typed regenerate path in
  `{regenerate_blocked_policy, regenerate_admitted_with_policy_review,
  regenerate_admitted_admin_only, regenerate_blocked_workspace_trust_revoked}`,
  a typed terminal explanation in
  `{terminal_explanation_policy_blocked_at_mint,
  terminal_explanation_policy_changed_post_mint,
  terminal_explanation_workspace_trust_revoked}`, and
  `denial_reason_class != no_denial`.

### Approval and one-time-token gating

`share_auth_class = approval_ticket_required` requires a non-null
`approval_ticket_ref`. `share_auth_class = one_time_token_required` is
admissible only on `share_audience_class = one_time_external_link_audience`
together with `share_expiry_class = one_time_use_expiry`.

### Reusing preview/runtime identity

Every share-link record cites the underlying `preview_snapshot_record`
by `preview_snapshot_record_ref`. The link does **not** mint a parallel
anonymous-link identity; instead, when a consumer surface opens the
link, it reads the snapshot the link cites and then composes the
strip / picker / hot-reload state from the snapshot. The link only
narrows the snapshot with the audience / destination / auth / expiry /
revoke / export posture, the current resolution, the runtime lineage,
the continuity state, the lifecycle, the regenerate path, and (where
applicable) the typed terminal explanation.

The share sheet is the only authority that may mint a link. The link's
`minted_from_share_sheet_record_ref` cites the sheet that authorised
the link's posture; if a consumer surface needs to know why a link has
the audience / auth / expiry it does, it reads the sheet.

## Continuity rules

The schema enforces these continuity rules on the link record:

- `share_continuity_state_class = continuity_intact` is admissible only
  on `share_lifecycle_state_class` in `{active_live, active_captured}`.
- A runtime restart that breaks the link's lineage forces
  `share_lifecycle_state_class = terminal_unavailable_runtime_restart`,
  `share_resolved_state_class = terminal_unavailable_runtime_restarted`,
  `share_continuity_state_class = continuity_broken_runtime_restarted`,
  and `share_runtime_lineage_class` in the three runtime-restart classes.
- A device-target change forces
  `share_lifecycle_state_class = terminal_unavailable_target_change`,
  `share_resolved_state_class = terminal_unavailable_target_changed`,
  `share_continuity_state_class = continuity_broken_target_changed`,
  and `share_runtime_lineage_class = bound_to_target_now_changed`.
- A captured-replay invalidation forces
  `share_lifecycle_state_class = terminal_unavailable_stale_capture`,
  `share_resolved_state_class = terminal_unavailable_stale_capture`,
  `share_continuity_state_class = continuity_broken_capture_stale`,
  and `share_runtime_lineage_class` in
  `{bound_to_captured_snapshot_only, bound_to_runtime_now_unavailable}`.
- A workspace-trust revocation forces
  `share_lifecycle_state_class = terminal_unavailable_workspace_trust_revoked`,
  `share_resolved_state_class = terminal_unavailable_workspace_trust_revoked`,
  `share_continuity_state_class = continuity_broken_workspace_trust_revoked`,
  and `share_terminal_explanation_class = terminal_explanation_workspace_trust_revoked`.
- A share-policy narrowing or destination-no-longer-admitted observation
  forces `share_lifecycle_state_class = revoked_by_policy` with
  `share_revoke_actor_class = policy_revoke_actor` and a typed policy
  reason / explanation.

In every case, the regenerate path is a typed
`share_regenerate_path_class` (not `no_regenerate_required`) and the
record cites a `regenerate_target_share_sheet_record_ref` that the
operator can open to mint a fresh share. The share sheet that handles
the regeneration carries
`sheet_disposition_class = sheet_admitted_regenerate_from_*` and cites
the predecessor link by `predecessor_share_link_record_ref`.

## Composition with the cross-surface preview-snapshot record

Every `preview_share_sheet_record` and every `preview_share_link_record`
carries `preview_snapshot_record_ref` pointing at the underlying
snapshot. The share records never re-assert source-of-truth fields the
snapshot already owns (preview-mode, runtime identity, mapping
confidence, hot-reload state, transform manifest); they carry the share
vocabularies above and an opaque ref to the snapshot.

When a share record refers to the strip / hot-reload / device-target
surface contracts, it does so through the strip / hot-reload / device-
target opaque refs, so the share contract layers cleanly on top of
those surface contracts and never duplicates their source-of-truth
fields.

## Composition with the safe-preview, artifact-edit-posture, and execution-context boundaries

- The execution-context boundary stays the source of truth for runtime
  target identity, sandbox posture, and policy epoch. The share contract
  cites `execution_context_record_ref` only through the snapshot.
- The safe-preview boundary stays the source of truth for trust class,
  connectivity state, and the downgrade-trigger ladder. Workspace-trust
  revocation observed on the safe-preview boundary forces the link into
  `terminal_unavailable_workspace_trust_revoked`.
- The generated-artifact lineage / edit-posture boundary stays the
  source of truth for `artifact_origin_class = preview_projection`. The
  share contract does not invent edit-posture vocabulary; it carries
  the export posture for share scope only.
- The integration approval-ticket family stays the source of truth for
  approval tickets cited under `share_auth_class = approval_ticket_required`
  and under audiences whose policy posture requires policy-managed
  grants.

## Per-lane field requirements

| Field group                          | Browser preview lane | Native preview lane | Embedded preview lane |
|--------------------------------------|----------------------|---------------------|------------------------|
| Share-sheet required disclosure floor| required             | required            | required               |
| Share-link required disclosure floor | required             | required            | required               |
| Audience-vs-destination pairings     | required             | required            | required               |
| Lifecycle-to-terminal-explanation    | required             | required            | required               |
| Continuity rules                     | required             | required            | required               |

The same record shapes render in all three lanes; lane-specific adapters
do not redefine the share vocabulary, the audience partition, or the
lifecycle-to-terminal-explanation map.

## Forbidden collapses

The schemas forbid the following collapses; fixtures and consumer
surfaces that produce any of them are rejected at the boundary:

- rendering a `revoked_by_*` link as `resolves_to_live_runtime` or any
  other live / captured / static class;
- rendering an `expired` link as `resolves_to_live_runtime` or
  `resolves_to_captured_snapshot`;
- rendering a `terminal_unavailable_runtime_restart` /
  `terminal_unavailable_target_change` /
  `terminal_unavailable_stale_capture` link as
  `resolves_to_live_runtime`;
- rendering a `policy_blocked_share_mode` sheet with `mint_admissible = true`;
- rendering a `not_shareable_inherent_surface` audience with anything
  other than `policy_blocked_share_mode` and a refusal disposition;
- rendering a `temporary_external_link_audience` link with
  `share_auth_class = no_auth_required`, `share_revoke_path_class = no_revoke_required`,
  or no `expires_at`;
- rendering a `one_time_external_link_audience` link with
  `share_expiry_class != one_time_use_expiry`;
- rendering a `workspace_local_only_no_share` link with any external
  destination or any non-`no_auth_required` auth;
- rendering an `export_full_record_admitted_internal_only` posture on
  any external audience;
- collapsing `expired`, `revoked_by_user`, `revoked_by_admin`,
  `revoked_by_policy`, `terminal_unavailable_runtime_restart`,
  `terminal_unavailable_target_change`,
  `terminal_unavailable_stale_capture`,
  `terminal_unavailable_policy_blocked`, and
  `terminal_unavailable_workspace_trust_revoked` into one vague
  "share unavailable" label;
- omitting `share_terminal_explanation_class` or
  `share_regenerate_path_class` on any terminal lifecycle;
- exposing raw URLs, raw absolute paths, raw IP addresses, raw
  hostnames, raw bearer tokens, raw session cookies, raw expiring
  credentials, raw rendered bytes, raw stack frames, or raw mock-data
  bodies through either record;
- minting a share-link record without a `minted_from_share_sheet_record_ref`,
  or minting a `superseded_by_regenerate` record without
  `superseded_by_share_link_record_ref` and
  `regenerate_target_share_sheet_record_ref`.

## Change discipline

Adding a new `share_mode_class`, `share_audience_class`,
`share_destination_class`, `share_auth_class`, `share_expiry_class`,
`share_revoke_path_class`, `share_export_posture_class`,
`share_resolved_state_class`, `share_runtime_lineage_class`,
`share_continuity_state_class`, `share_lifecycle_state_class`,
`share_regenerate_path_class`, `share_terminal_explanation_class`,
`share_revoke_actor_class`, `share_revoke_reason_class`,
`mint_blocked_reason_class`, `sheet_disposition_class`, or
`denial_reason_class` value is additive-minor and bumps the
corresponding `*_schema_version` const. Repurposing an existing value
is breaking and requires a new decision row.

Re-exporting a vocabulary from another schema is preferred over minting
a parallel one. Where this contract narrows or projects a re-export
(notably the audience-vs-destination partition and the
lifecycle-to-terminal-explanation map), the gate is documented above; if
a future contributor needs to widen the projection, that change lands
here and on the link / sheet schemas together.
