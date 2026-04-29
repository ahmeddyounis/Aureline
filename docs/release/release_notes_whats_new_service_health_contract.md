# Release-notes, what's-new, and service-health communication contract

This document is the contract layer that turns the product's release
and service communication surface into product-bound state. It freezes
the surfaces and the language used before upgrade, after upgrade, and
during managed-feature degradation so the same record family answers
the same questions a reviewer asks every time:

- *what build are we now running, and what changed since the previous
  one?*
- *what action does this release ask of me or of my admins, and what
  evidence backs that ask?*
- *which service is degraded, on whose machine, and what still works
  locally?*
- *am I looking at live truth, or at a cached, mirrored, stale, or
  policy-limited snapshot?*

The card surface does not invent a second vocabulary for release
identity, channel, support window, deprecation, advisory, outage,
maintenance, or boundary change. It projects the records already frozen
elsewhere into one place so the user does not have to chase scattered
release prose to learn what the build is doing.

Companion artifacts:

- [`/schemas/release/whats_new_card.schema.json`](../../schemas/release/whats_new_card.schema.json)
  - boundary schema for `whats_new_card_record`.
- [`/fixtures/release/communication_cases/`](../../fixtures/release/communication_cases/)
  - worked cases for a post-upgrade what's-new card, a breaking-change
    notice before restart, a stale service-health banner, a cached
    release-note feed, and an admin note with affected groups.

Upstream sources this contract projects from rather than restating:

- [`/schemas/release/release_candidate_card.schema.json`](../../schemas/release/release_candidate_card.schema.json)
  and
  [`/docs/release/release_status_surface_contract.md`](./release_status_surface_contract.md)
  - release-candidate, version-bump, channel-identity, support-window,
    compatibility-posture, and rollback vocabulary the card's
    `subject` and breaking-change rules quote.
- [`/schemas/release/update_manifest.schema.json`](../../schemas/release/update_manifest.schema.json)
  and
  [`/docs/release/update_and_rollback_contract.md`](./update_and_rollback_contract.md)
  - update, rollback, repair, and helper-negotiation vocabulary the
    card's `subject.update_manifest_ref` resolves into.
- [`/schemas/release/promotion_timeline_entry.schema.json`](../../schemas/release/promotion_timeline_entry.schema.json)
  - promotion-timeline vocabulary the change-entry evidence rows can
    quote when a what's-new entry corresponds to a stage movement.
- [`/schemas/build/exact_build_identity.schema.json`](../../schemas/build/exact_build_identity.schema.json)
  and
  [`/docs/build/exact_build_identity_model.md`](../build/exact_build_identity_model.md)
  - exact-build identity vocabulary the card's
    `subject.exact_build_identity_ref` resolves into.
- [`/schemas/security/advisory_record.schema.json`](../../schemas/security/advisory_record.schema.json)
  - advisory vocabulary every `change_class=security` entry's
    `advisory_ref` resolves into.
- [`/schemas/ops/outage_notice.schema.json`](../../schemas/ops/outage_notice.schema.json)
  and
  [`/schemas/ops/maintenance_notice.schema.json`](../../schemas/ops/maintenance_notice.schema.json)
  - outage and maintenance notice contracts every degraded service-tier
    row links back to.
- [`/schemas/docs/destination_descriptor.schema.json`](../../schemas/docs/destination_descriptor.schema.json)
  - destination-descriptor refs the access-point rows resolve to when
    they reach docs, help, or support routes.

Normative sources this contract projects from:

- `.t2/docs/Aureline_PRD.md` sections on release-note discipline,
  what's-new disclosure, breaking-change communication, advisory
  surfacing, deprecation timing, service-health surfacing, cached and
  mirrored data discipline, and admin-side communication.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` sections on
  release publication, advisory disclosure, control-plane and
  data-plane separation, and managed-service boundary truth.
- `.t2/docs/Aureline_Technical_Design_Document.md` sections on the
  release-center surface, update review, the about / help surfaces,
  and the service-health surface.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` sections on what's-new
  cards, update-detail views, breaking-change notices, service-health
  panels, stale-status banners, and admin notes.

If this document disagrees with those sources, those sources win and
this document, its companion schema, and fixtures update in the same
change.

## Scope

Frozen here:

- one `whats_new_card_record` shape that projects across six surfaces:
  the post-upgrade what's-new card, the update-detail view, the
  breaking-change or migration notice, the service-health panel, the
  stale-status banner, and the admin note;
- a closed `change_class` vocabulary
  (`new`, `changed`, `deprecated`, `security`, `migration_required`,
  `admin_action_required`) and the evidence-link rules that gate every
  breaking-change or behavior-changing entry on at least one
  `evidence_refs` ref and at least one `known_limit_refs` ref;
- a closed `service_tier_class` vocabulary
  (`local_machine`, `remote_target`, `enterprise_control_plane`,
  `optional_vendor_hosted_service`) the service-health panel MUST keep
  separate so a degraded vendor add-on cannot collapse into a
  banner that reads as a local outage;
- a closed `data_source_class` plus `freshness_class` vocabulary the
  card uses to mark cached, mirrored, stale, offline, or policy-limited
  snapshots so users do not read them as live truth;
- the `non_blocking_invariants` block whose `blocks_typing`,
  `blocks_save`, `blocks_recovery_critical_flow`, and
  `reopenable_from_stable_navigation` flags are structurally locked so
  the surface cannot block the editor or hide reopen access; and
- the `admin_note` block that resolves audience classes and affected
  group rows when the card is rendered to admins.

Out of scope:

- authoring release prose. Concrete what's-new copy, marketing copy,
  and migration narrative are owned outside this milestone; the card
  freezes the shape that carries them.
- running a status page or operating an incident-management backend.
  The card surfaces refs into outage, maintenance, and migration notices
  that own the underlying truth.
- the surface UI implementation (layout, copy polish, motion). This
  contract freezes the record shape and the rendering invariants;
  surface owners design the UI on top.
- raw release prose paragraphs, raw URLs, raw advisory payloads, raw
  incident bodies, raw status-page hostnames, raw policy bodies, raw
  tenant identifiers, raw user identifiers, and raw paths. The card
  boundary forbids them.

## Surfaces

Six surfaces are admitted for this card family. Each one is one
projection of the same record; the schema's `card_class` selects the
projection and gates the additional invariants for that surface:

1. **`whats_new_post_upgrade`.** The post-upgrade what's-new card. One
   per upgrade event. Lists `change_entries` for the new build and
   exposes a `service_health_panel` snapshot. MUST be reopenable later
   from a stable navigation surface, and MUST NOT block typing, save,
   or recovery-critical flows.
2. **`update_detail_view`.** The update-detail view rendered before
   accepting an update or in update review. Same `change_entries` and
   `subject` as the post-upgrade card; the projection differs in how
   prominently the next-restart and migration callouts render.
3. **`breaking_change_notice`.** The breaking-change or migration
   notice rendered before restart, before applying a schema change, or
   before an irreversible-after-restart upgrade. MUST contain at least
   one entry whose `change_class` is `migration_required`,
   `deprecated`, `security`, or `admin_action_required`, or whose
   `behavior_change_kind_class` names a non-`no_behavior_change`
   kind, AND MUST resolve a `release_candidate_card_ref` so the notice
   projects from a frozen release-candidate card.
4. **`service_health_panel`.** The service-health panel rendered in
   the dedicated health surface. Always carries the four-tier
   `service_health_panel` block.
5. **`stale_status_banner`.** The stale-status banner rendered when
   the snapshot is cached, mirrored, expired, imported, or unknown.
   MUST resolve `freshness_state.freshness_class` to a non-`current`
   value and carry a non-null `stale_label`.
6. **`admin_note`.** The admin note rendered to fleet admins, tenant
   admins, security officers, compliance officers, or release operators.
   MUST resolve at least one `audience_classes` entry other than
   `not_admin_audience` and at least one `affected_group_row`.

Two access points are required, at least one of which MUST be on a
stable navigation surface (`stable_navigation=true`) so the card can be
reopened later.

## Subject

The card's `subject` block names what build the card is communicating
about through the same vocabulary frozen by the release-candidate card,
exact-build identity, and update manifest:

- `product_name_class` — frozen at `aureline`.
- `channel_class` — `dev_local`, `nightly`, `preview`, `beta`, `stable`,
  `lts`, `hotfix`. Aligned with the release-candidate card vocabulary.
- `version_label` — the human-facing version label of the new build.
- `prior_version_label` — the prior label, when known. Allowed to be
  null for first-run builds and for service-health or stale-status
  cards that do not represent a version transition.
- `exact_build_identity_ref` — opaque ref into
  `/schemas/build/exact_build_identity.schema.json`. Allowed to be null
  on stale-status banners or admin notes that do not pin a build.
- `release_candidate_card_ref` — opaque ref into the release-candidate
  card the change entries quote from. Required on breaking-change
  notices, recommended on what's-new and update-detail cards.
- `update_manifest_ref` — opaque ref into the update manifest that
  carries the rollback path and helper-negotiation rules. Allowed to
  be null when the card is not rendered against a manifest.
- `subject_disclosure` — short reviewable sentence explaining what
  build the card is communicating about.

A card MUST NOT render upgrade copy that names a version, channel, or
exact-build identity it cannot resolve through these refs.

## Change classes and evidence

`change_entries` is the field that prevents release prose from
asserting behavior changes the rest of the release evidence cannot
back. Every entry resolves to one closed `change_class`:

| `change_class`            | Meaning |
|---------------------------|---------|
| `new`                     | A purely additive entry. No prior behavior changes; no migration is implied. |
| `changed`                 | Existing behavior changed. Must link evidence and a known-limit ref so reviewers can see what is documented as not-yet-fixed alongside the change. |
| `deprecated`              | A surface or capability is on a sunset path. Must resolve a `deprecation_replacement_ref` so the deprecation cannot be communicated without naming what to use instead. |
| `security`                | The entry projects an advisory. Must resolve `advisory_ref` so the security copy quotes the same advisory the rest of the supply-chain surfaces use. |
| `migration_required`      | The build asks the user to perform a deliberate migration. Must resolve `migration_guide_ref`. |
| `admin_action_required`   | The build asks an admin to perform a deliberate response (rotate a key, accept a new boundary, run a fleet command). Must resolve `admin_action_ref`. |

Every entry also carries a closed `behavior_change_kind_class` that
names *what kind* of behavior change is involved:
`no_behavior_change`, `default_changed`, `schema_changed`,
`shortcut_or_command_changed`, `ux_layout_changed`,
`policy_default_changed`, `removal`, `rename`, or
`deprecation_warning_only`. Any non-`no_behavior_change` kind triggers
the same evidence + known-limit linkage rule, regardless of
`change_class`. A change cannot be communicated as merely "polish" when
the kind class names something users will notice.

Every entry also resolves:

- `scope_class` from the closed
  artifact-family / managed-feature vocabulary so the entry pins the
  surface that changed (`ide_binary`, `cli`, `remote_agent`,
  `extension_sdk`, `extension_package`, `managed_service_contract`,
  `marketplace_metadata`, `policy_bundle`, `docs_pack`, `schema_export`,
  `release_evidence_packet`, `supply_chain_artifact`, `managed_feature`,
  `optional_managed_addon`, `platform_compatibility`).
- `reversibility_class` from the closed vocabulary
  (`reversible_via_rollback`, `reversible_via_repin`,
  `reversible_via_user_setting`, `irreversible_after_restart`,
  `irreversible_after_migration`, `not_applicable`) so users know
  whether the change can be undone after the upgrade applies.
- `applies_before_restart` boolean so the breaking-change notice can
  be rendered before the restart that lands the change.
- `evidence_refs` and `known_limit_refs`. Breaking and
  behavior-changing entries MUST resolve at least one of each.

The schema's `allOf` gates encode these structurally. A
"breaking-change-feeling" entry that lists no evidence and no
known-limit refs is rejected.

## Service-health tiers

`service_health_panel.rows` is the field that prevents service-health
copy from collapsing four very different machines into one banner.
Every panel MUST contain one row per tier below:

| `service_tier_class`              | What the row describes |
|-----------------------------------|------------------------|
| `local_machine`                   | The running build on the user's local machine. |
| `remote_target`                   | A customer-operated remote target the user is attached to (SSH host, dev container, customer-operated remote agent, customer-operated service). |
| `enterprise_control_plane`        | The customer-operated or vendor-operated enterprise control plane the deployment is bound to (sync, registry, identity, policy, catalog, merge queue). |
| `optional_vendor_hosted_service`  | An opt-in vendor-hosted add-on (telemetry, hosted AI, hosted symbol service, hosted browser handoff). |

Each row carries:

- `health_state_class` — `healthy`, `degraded_partial`,
  `degraded_read_only`, `outage_blocking`, `scheduled_maintenance`,
  `tenant_migration`, `boundary_recheck_required`, `policy_limited`,
  `unknown_requires_review`, `not_applicable`. Aligned with the
  outage and maintenance notice vocabulary.
- `data_source_class` — `live`, `warm_cached`, `stale_cached`,
  `mirrored_official`, `offline_bundle`, `policy_limited`,
  `unavailable_local_only`. The row pins the source explicitly so a
  cached snapshot cannot read as live truth.
- `freshness_class` — `current`, `warm_cached`, `stale_requires_review`,
  `expired`, `imported_historical`, `unknown`, `not_applicable`.
- `outage_notice_ref` and `maintenance_notice_ref` — opaque refs into
  the outage or maintenance notice that owns the underlying truth.
  When the row reports a managed-degradation state
  (`degraded_partial`, `degraded_read_only`, `outage_blocking`,
  `scheduled_maintenance`, `tenant_migration`,
  `boundary_recheck_required`), at least one of these refs MUST be
  resolved.
- `boundary_truth_disclosure` — short reviewable sentence stating
  whose machine the issue is on and what still works locally. This is
  the local-vs-service boundary truth requirement: a managed
  degradation that fails to disclose what is local is non-conforming.
- `row_disclosure` — short reviewable sentence describing the row's
  effect.

The schema's `contains` gates encode the four-tier presence rule
structurally. A panel that lists three tiers, or that buries
`enterprise_control_plane` and `optional_vendor_hosted_service` in one
row, is rejected.

## Cached, mirrored, stale, and policy-limited data

`freshness_state` is the card-level snapshot label. It pins:

- `data_source_class` — same closed vocabulary as service-health rows.
- `freshness_class` — same closed vocabulary.
- `last_refreshed_at` — UTC timestamp of the last successful refresh,
  or null when no refresh has happened.
- `stale_after` — ISO 8601 duration string indicating when the snapshot
  goes stale, or null when not applicable.
- `stale_label` — a short reviewable sentence stating that the
  snapshot is cached, mirrored, stale, expired, imported, or unknown.
  MUST be non-null whenever the freshness class is anything other than
  `current` or `not_applicable`.
- `boundary_truth_disclosure` — short reviewable sentence stating what
  the snapshot can and cannot prove right now (for example "shown from
  the last successful sync; no live policy state is implied").

A `stale_status_banner` card MUST resolve `freshness_class` to a
non-`current` value and MUST carry a non-null `stale_label`. The
schema's `allOf` gates encode this rule.

## Non-blocking invariants

`non_blocking_invariants` is the structural lock that prevents the
release-communication surface from blocking the editor:

- `blocks_typing` is `const false`. The card cannot block typing in
  the running editor session.
- `blocks_save` is `const false`. The card cannot block save,
  autosave, recovery snapshotting, or local export.
- `blocks_recovery_critical_flow` is `const false`. Recovery-critical
  flows (crash recovery, decode-recovery review, save-consequence
  resolution) cannot be gated behind reading the card.
- `reopenable_from_stable_navigation` is `const true`. The card MUST
  remain reopenable later from a stable navigation surface
  (Help menu, command palette, or settings pane).
- `user_dismissible_without_reading` is left to the surface; it stays
  `true` for `whats_new_post_upgrade` so reviewing release notes never
  blocks getting back to work, but service-health and breaking-change
  surfaces MAY set this to `false` when the user must acknowledge a
  managed degradation or an irreversible-after-restart action before
  proceeding.

The schema's `const` gates make these invariants impossible to relax
in a runtime record.

## Access points

Every card MUST list at least two access points and at least one of
them MUST live on a stable navigation surface
(`stable_navigation=true`). The closed `access_point_class` vocabulary
admits:

- `help_whats_new`
- `command_palette_whats_new`
- `settings_release_pane`
- `release_notes_export`
- `support_bundle`
- `diagnostics_export`
- `admin_console_release_pane`
- `service_health_pane`
- `cli_whats_new`
- `offline_review`

`reachability_class` mirrors the broader status-surface vocabulary
(`available`, `available_read_only`, `unavailable_visible`,
`policy_hidden_visible`, `not_applicable_visible`).

## Admin note

`admin_note` is the field rendered when the card is consumed by fleet
admins, tenant admins, workspace admins, security officers, compliance
officers, support operators, or release operators. It carries:

- `audience_classes` — closed list of admin audience tokens. An
  `admin_note` card MUST contain at least one entry that is not
  `not_admin_audience`.
- `affected_groups` — array of `affected_group_row` objects pinning
  the deployment profile, tenant, org, workspace, ring, fleet segment,
  policy class, role class, or extension audience the note targets.
  Each row resolves an opaque `group_ref` and a reviewable
  `row_disclosure`. An `admin_note` card MUST contain at least one
  affected-group row.
- `admin_action_summary` — short reviewable sentence describing the
  admin response the note asks for.

Non-admin cards (`whats_new_post_upgrade`, `update_detail_view`,
`breaking_change_notice`, `service_health_panel`,
`stale_status_banner`) MAY still carry an `admin_note` block when an
admin response is implicated; the schema only requires the audience
and affected-group constraints when `card_class=admin_note`.

## Linkage rules

- A card's `subject.exact_build_identity_ref` MUST resolve to a record
  conforming to
  [`schemas/build/exact_build_identity.schema.json`](../../schemas/build/exact_build_identity.schema.json)
  when present.
- A card's `subject.release_candidate_card_ref` MUST resolve to a
  record conforming to
  [`schemas/release/release_candidate_card.schema.json`](../../schemas/release/release_candidate_card.schema.json)
  when present, and MUST be present on `breaking_change_notice` cards.
- A card's `subject.update_manifest_ref` MUST resolve to a record
  conforming to
  [`schemas/release/update_manifest.schema.json`](../../schemas/release/update_manifest.schema.json)
  when present.
- A card's `change_entries[].advisory_ref` MUST resolve to a record
  conforming to
  [`schemas/security/advisory_record.schema.json`](../../schemas/security/advisory_record.schema.json)
  when present, and MUST be present when `change_class=security`.
- A card's
  `service_health_panel.rows[].outage_notice_ref` MUST resolve to a
  record conforming to
  [`schemas/ops/outage_notice.schema.json`](../../schemas/ops/outage_notice.schema.json)
  when present.
- A card's
  `service_health_panel.rows[].maintenance_notice_ref` MUST resolve to
  a record conforming to
  [`schemas/ops/maintenance_notice.schema.json`](../../schemas/ops/maintenance_notice.schema.json)
  when present.
- A card's `change_entries[].migration_guide_ref` and
  `admin_action_ref` SHOULD resolve to destination descriptors
  conforming to
  [`schemas/docs/destination_descriptor.schema.json`](../../schemas/docs/destination_descriptor.schema.json)
  or to `release_candidate_card` evidence rows.

## Rendering contract

Every card carries a `rendering_contract` block stating which fields
MUST be visible on every surface that renders the card:

- `subject_visible` — version label, channel, and exact-build cue;
- `change_entries_visible` — every `change_entries` entry with its
  change class, kind, evidence cue, and known-limit cue;
- `service_health_panel_visible` — every populated service-tier row
  with its tier, state, source, and freshness cue. MUST be `true`
  when any tier reports a managed-degradation state;
- `freshness_state_visible` — source and freshness state plus the
  stale label when present;
- `admin_note_visible` — audience classes, affected groups, and admin
  action summary when the card is an `admin_note` or carries an
  admin-implicating change entry;
- `access_points_visible` — every populated access-point row;
- `non_blocking_invariants_enforced` is `const true`. The non-blocking
  invariants are structural; surfaces MUST enforce them rather than
  treat them as guidance.

Surfaces MAY add layout, ordering, copy polish, and motion on top, but
they MUST NOT hide a field that the rendering contract marks visible
or override the non-blocking invariants.

## Out of contract

- The eventual UI implementation. This contract freezes the record
  shape and the rendering invariants; surface owners design the UI on
  top.
- Authoring concrete release prose, marketing copy, or migration
  narrative. The card freezes the shape that carries them.
- Operating a status page or an incident-management backend. The
  card surfaces refs into the outage, maintenance, and migration
  notices that own the underlying truth.
- Pricing, commercial availability, or partner naming. The card freezes
  the openness and managed-service shape; concrete commercial language
  is owned outside this milestone.
