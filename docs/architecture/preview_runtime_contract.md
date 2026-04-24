# Preview-runtime, source-mapping confidence, and stale-editability contract

This document freezes the cross-surface contract Aureline uses when a
static preview, live preview, inspectable tree/overlay, or visual-edit
surface renders source under a preview runtime. The goal is simple:
preview and visual-design lanes stay source-first, provenance-explicit,
and safely degradable, and consumers across browser, native, and
embedded preview lanes share one vocabulary instead of inventing
per-surface badges.

Companion artifacts:

- [`/schemas/preview/preview_snapshot.schema.json`](../../schemas/preview/preview_snapshot.schema.json)
  — boundary schema for the `preview_snapshot_record` every preview
  surface emits.
- [`/fixtures/preview/source_mapping_cases/`](../../fixtures/preview/source_mapping_cases/)
  — worked corpus of static, live, inspectable, visual-edit, mock,
  captured, stale, unmappable, runtime-unavailable, hot-reload,
  device-mismatch, and share-sheet cases.
- [`/schemas/generated/artifact_edit_posture.schema.json`](../../schemas/generated/artifact_edit_posture.schema.json)
  and [`/docs/architecture/generated_artifact_safe_edit_policy.md`](generated_artifact_safe_edit_policy.md)
  — generated-artifact lineage / drift / regeneration / structured-
  viewer fallback this contract composes with.
- [`/schemas/security/trust_class.schema.json`](../../schemas/security/trust_class.schema.json)
  and [`/docs/security/safe_preview_trust_classes.md`](../security/safe_preview_trust_classes.md)
  — safe-preview trust-class, connectivity-state, downgrade-trigger
  ladder this contract re-exports.
- [`/schemas/execution/context_snapshot.schema.json`](../../schemas/execution/context_snapshot.schema.json)
  — execution-context snapshot this contract references for runtime
  identity instead of restating execution fields.
- [`/schemas/integration/approval_ticket.schema.json`](../../schemas/integration/approval_ticket.schema.json)
  — approval-ticket family for sandbox / trust expansions.

If this document disagrees with the PRD, TAD Appendix CR, or UX spec,
those sources win and this document plus the schema update in the same
change.

## Why freeze this now

Preview and visual-design surfaces are the easiest place for hidden
truth to leak. A live preview that silently drops to a stale snapshot,
an "inspector" that pretends to map every node back to source, a visual
editor that writes back over a runtime it no longer trusts, or a share
link that hides who can revoke it — each of these turns the preview
lane into a parallel source-of-truth. Without one contract:

- a static preview can claim a runtime that never executed;
- a live preview can keep rendering past the source revision it was
  built against without admitting it;
- an inspectable overlay can offer source-jump on nodes that have no
  source span;
- a visual-edit surface can apply a "mapped edit" through a runtime
  whose mapping is approximate or stale;
- a hot-reload state can be hidden behind a freshness label that does
  not distinguish "applied" from "rebuild required";
- a viewport preset can masquerade as a real device target;
- a preview share-sheet can be wired to "anyone with the link" without
  declaring an auth class, expiry, or revoke path.

This contract closes those gaps by freezing one preview-snapshot
record, one source-mapping confidence vocabulary, one stale-editability
ladder, one hot-reload state vocabulary, and one set of disclosure
fields (source-sync chip, device-target row, hot-reload status, share-
sheet block) every preview surface MUST project the same way.

## Scope

Frozen at this revision:

- one shared `preview_snapshot_record` shape covering the four preview
  modes (static preview, live preview, inspectable tree/overlay,
  visual edit) across the three preview lanes (browser, native,
  embedded);
- the source-to-runtime mapping-confidence vocabulary that gates source
  jump, diff, export, apply-mapped-edit, and direct-write actions;
- the stale-editability ladder that maps mapping confidence + hot-
  reload state + connectivity into one allowed-action floor consumer
  surfaces read instead of recomputing per-surface;
- the reserved hot-reload state vocabulary
  (`applied`, `partial`, `restart_required`, `rebuild_required`,
  `failed`, `unavailable`);
- the rule that viewport presets are a presentation choice and never a
  device target — actual browser/device targets carry their own
  device-target class and an opaque device handle;
- the source-sync chip, device-target row, hot-reload status, and
  preview share-sheet fields every preview surface MUST disclose;
- the explicit statuses for source-in-sync, mapped-with-fallback,
  stale-mapping, device-mismatch, and hot-reload-unavailable;
- the transform-manifest and rollback-checkpoint requirements for any
  mapped edit or visual-edit flow that claims source-safe mutation.

Out of scope (named explicitly so the schema does not creep):

- building any live preview runtime, framework adapter, hot-reload
  pipeline, or visual designer at this milestone;
- minting framework-specific source-mapping algorithms;
- the visual-transform manifest schema itself (this contract only
  carries an opaque ref);
- the share-sheet UX (this contract only declares the typed fields a
  later share-sheet flow MUST emit);
- device emulation, simulator orchestration, or remote-device pool
  brokering;
- any preview-specific telemetry; payloads narrow through the
  telemetry/support registry like every other surface.

## Shared snapshot record

Every protected preview surface emits the same record from
[`preview_snapshot.schema.json`](../../schemas/preview/preview_snapshot.schema.json).
The record does not replace the full preview-runtime configuration or
the visual-transform manifest. It is the compact projection consumers
read when they need one stable answer to four questions:

1. What is the **preview mode** this surface is rendering?
2. What is the **source revision** and **runtime identity** behind it?
3. What is the **mapping confidence** between rendered nodes and
   canonical source right now?
4. What **mutations** (source jump, diff, export, apply mapped edit,
   direct write) are admissible on this surface in this state?

### 1. Preview mode

`preview_mode` is the four-value lane that mirrors TAD Appendix CR.

- `static_preview`
  Binds to a source revision and a preview config. No live runtime
  claim. `target_environment_class` MUST be
  `static_render_only_no_runtime` or
  `extension_host_preview_runtime`. Hot reload is always
  `unavailable`. Visual edit is forbidden.
- `live_preview`
  Binds to a running preview runtime under a declared sandbox profile.
  Source revision, runtime identity, data/mock provenance, and
  freshness are all mandatory. Default edit posture is inspect-only;
  visual edit requires a separate `visual_edit` snapshot.
- `inspectable_tree_overlay`
  Layers a read-only inspector on a preview runtime. Source-jump and
  code-jump are admissible only on nodes that map to a source span.
  `apply_mapped_edit` and `direct_write_to_canonical_source` are
  forbidden by construction (`stale_editability_class` cannot be
  `fully_editable_through_mapped_edits`).
- `visual_edit`
  Layers a typed transform planner on a preview runtime. Mutation
  requires `mapping_confidence_class = exact_source_mapping`,
  `stale_editability_class = fully_editable_through_mapped_edits`,
  `hot_reload_state ∈ {applied, partial}`, and a populated
  `transform_manifest` block (manifest ref + rollback-checkpoint ref
  + round-trip-proof posture proven for the adapter, ideally for the
  specific construct).

### 2. Preview lane

`preview_lane` distinguishes browser, native, and embedded preview
lanes:

- `browser_preview_lane`
- `native_preview_lane`
- `embedded_preview_lane`

The three lanes share **one** vocabulary for source-sync, device-
target, hot-reload, and share-sheet disclosure. A native preview
running in a simulator and a browser preview running in a tab speak
the same source-sync chip language.

### 3. Provenance composition with the artifact-edit-posture record

The snapshot pins `artifact_origin_class` to the constant
`preview_projection` and `do_not_imply_canonical_source` to the
constant `true`. `preview_provenance_state` re-exports the subset of
`provenance_state` from
[`artifact_edit_posture.schema.json`](../../schemas/generated/artifact_edit_posture.schema.json)
that may apply to a preview projection
(`in_sync`, `stale_inputs`, `generator_changed`,
`diverged_from_generator`, `unknown_lineage`, `mock_provenance`).
`default_edit_posture` re-exports a narrower subset
(`inspect_read_only`, `structured_safe_edit`,
`edit_canonical_source`).

This means search, open, review, AI-context, export, and support
surfaces that already speak the artifact-edit-posture vocabulary read
preview snapshots without minting new origin or posture tokens. Mock
preview snapshots remain `inspect_read_only` by the same allOf gate
the artifact-edit-posture schema already enforces.

### 4. Trust-class composition with the safe-preview ladder

`trust_class` re-exports the two safe-preview classes admissible to
executable preview surfaces (`TrustedLocalActive`,
`IsolatedRemoteActive`). `connectivity_state` re-exports the safe-
preview connectivity ladder verbatim. The first twelve
`downgrade_trigger` values re-export the safe-preview ladder so a
preview surface that loses workspace trust, sees a sanitizer fail, or
loses connectivity downgrades through the same vocabulary the editor
and review surfaces use.

The remaining nine `downgrade_trigger` values are preview-specific
extensions:

- `source_revision_drifted`
- `framework_adapter_revision_changed`
- `preview_runtime_restart_required`
- `preview_runtime_rebuild_required`
- `preview_runtime_unavailable`
- `device_target_mismatch`
- `mock_data_set_changed`
- `captured_replay_expired`
- `mapping_invalidated_by_hot_reload`

A preview surface that needs to narrow MUST cite a typed trigger from
this list rather than describing the cause in prose.

## Source-sync chip

The source-sync chip displays `source_sync_state` plus a redacted
`source_revision_anchor`:

- `source_in_sync` — runtime built/loaded against the recorded source
  revision; no edits since.
- `source_drifted_since_render` — source moved after render; the
  snapshot is stale until refreshed. `drifted_paths_count` MAY be
  shown; per-path identity does not project.
- `source_revision_unknown` — runtime did not report a revision.
- `source_revision_redacted` — reserved for support-export contexts
  where revision identity is intentionally projected as opaque.
- `source_unmappable_for_runtime` — runtime kind never reports source
  identity.

The chip is not a freshness label. Freshness is reported separately
through `freshness_class`.

## Device-target row vs. viewport preset

The schema distinguishes `device_target_class` (where the preview
actually rendered) from `viewport_preset_class` (which layout preset
is being displayed). A viewport preset is a presentation choice; it
never claims a real device of that class.

- `viewport_preset_only` means no real device is attached. The
  preview ran in whatever runtime `target_environment_class` names;
  the surface is showing the chosen viewport preset only.
- `tethered_device`, `remote_attached_device`, `managed_device_pool`
  all carry an opaque `device_handle_ref`.
- `browser_window_target`, `browser_tab_target`,
  `embedded_canvas_target` name actual browser or embedded surfaces
  the preview is being rendered into.

When the requested device target could not be reached and the
snapshot fell back to a viewport preset only,
`device_target_mismatch_observed` MUST be `true` and the matching
`device_target_mismatch` trigger MUST appear in
`downgrade_trigger_observations`.

## Hot-reload status

`hot_reload.hot_reload_state` is the spec-frozen six-value vocabulary:

- `applied` — most recent source change is reflected.
- `partial` — some change reflected, some pending; the snapshot may
  still admit `apply_mapped_edit` if mapping is exact and stale-
  editability is fully editable.
- `restart_required` — dev-server or runtime must restart before the
  surface can offer mapped edits again.
- `rebuild_required` — a build step must run; same restriction.
- `failed` — most recent reload errored; the snapshot freezes mapped
  edits and exposes the typed `last_failure_reason_class`.
- `unavailable` — hot reload is not supported by this runtime /
  adapter / target. The forced state for `static_preview`.

The schema enforces that `restart_required`, `rebuild_required`,
`failed`, and `unavailable` all forbid `apply_mapped_edit` from the
allowed-action set.

## Mapping-confidence ladder

`mapping_confidence_class` is the gate every consumer surface reads
before offering source jump, diff, export, or mutation:

- `exact_source_mapping` — the only class that admits `visual_edit`.
- `approximate_mapping` — inspect-only with code-jump on confirmed
  nodes. The chip MUST disclose "mapped with fallback".
- `stale_mapping` — forces inspect-only or freezes the surface.
  Mutation actions are stripped from `effective_allowed_actions`.
- `unmappable_node_present` — at least one rendered node has no
  source span. Visual edit on those nodes MUST NOT be offered.
- `runtime_only_no_source_mapping` — the runtime never produced a
  source map (some remote previews).
- `unknown_mapping` — the strictest downgrade.

## Stale-editability ladder

`stale_editability_class` projects mapping confidence + hot-reload
state + connectivity into one allowed-action floor:

- `fully_editable_through_mapped_edits` — only valid for
  `visual_edit` snapshots that satisfy every mapping / hot-reload /
  trust precondition.
- `inspect_only_due_to_mapping_uncertainty`
- `inspect_only_due_to_stale_runtime`
- `export_only_due_to_runtime_unavailable`
- `frozen_until_resync`
- `frozen_due_to_policy_or_trust`

The schema's allOf gates enforce that every class except
`fully_editable_through_mapped_edits` strips `apply_mapped_edit` and
`direct_write_to_canonical_source` from `effective_allowed_actions`.

## Allowed and blocked actions

`effective_allowed_actions` enumerates the actions admissible at
snapshot time. Consumer surfaces MUST NOT offer an action absent
from this list. The vocabulary is closed:

- `source_jump`
- `diff_against_source`
- `export_static_snapshot`
- `export_metadata_only`
- `apply_mapped_edit`
- `direct_write_to_canonical_source`
- `open_in_external_handoff`
- `share_via_share_sheet`

`blocked_actions` enumerates explicitly disallowed actions, each
paired with a typed `reason_trigger`. Surfaces SHOULD render the
greyed-out action with the typed reason rather than hiding it
silently — this preserves the rule that a stale or partial preview
degrades to inspect-only or export-only **without hiding why**
source-to-runtime mapping is incomplete.

## Transform manifest and rollback checkpoint

Any mapped edit or visual-edit flow that claims source-safe mutation
populates the `transform_manifest` block:

- `transform_manifest_required = true`
- `transform_manifest_ref` non-null
- `rollback_checkpoint_ref` non-null
- `round_trip_proof_class` non-null

The acceptable round-trip proof classes are:

- `round_trip_proven_for_this_adapter_and_construct`
- `round_trip_proven_for_adapter_only`
- `round_trip_unproven_inspect_only`
- `round_trip_unproven_export_only`

The schema strips `apply_mapped_edit` whenever the proof class is
unproven.

The visual-transform manifest itself is governed by a separate schema
that this contract does not freeze. This contract only carries opaque
refs and the round-trip-proof posture.

## Preview share-sheet (extension point)

`share_sheet` is reserved as a typed extension point. Visibility,
auth/session, expiry, and revoke-path are explicit so a later share-
sheet flow cannot smuggle scope or trust decisions into ad-hoc
payloads. The schema enforces:

- `share_visibility_class ∈ {public_link, tenant_only, organization_only}`
  forbids `share_revoke_path_class = no_revoke_required` and
  `share_auth_class = no_auth_required`.
- `share_link_handle_ref` is opaque; raw URLs, raw tokens, raw
  cookies, and raw expiring credentials never appear.

## Source sync, device target, hot reload, and share fields per lane

| Field group        | Browser preview lane | Native preview lane | Embedded preview lane |
|--------------------|----------------------|---------------------|------------------------|
| Source sync        | required             | required            | required               |
| Device target      | required             | required            | required               |
| Hot reload         | required             | required            | required               |
| Share-sheet block  | optional             | optional            | optional               |

The same field shapes render in all three lanes; lane-specific
adapters do not redefine the chip vocabulary.

## Composition with execution-context, safe-preview, and generated-
artifact lineage models

- The execution-context boundary stays the source of truth for
  runtime target identity, sandbox posture, and policy epoch. The
  preview snapshot records `execution_context_record_ref` and does
  not duplicate execution-context fields.
- The safe-preview boundary stays the source of truth for trust
  class, connectivity state, and downgrade-trigger ladder. The
  preview snapshot re-exports those vocabularies and adds preview-
  specific triggers without minting parallel safe-preview classes.
- The generated-artifact lineage / edit-posture boundary stays the
  source of truth for `artifact_origin_class = preview_projection`,
  the `mock_provenance` rule, and the structured-viewer fallback. The
  preview snapshot pins those constants and re-exports the narrow
  `default_edit_posture` subset.

If a future change widens the preview-snapshot vocabulary, it MUST
land additive-minor on the relevant schema (preview-specific values
on this schema; safe-preview / artifact-edit-posture / execution-
context values on their owning schemas) and bump the corresponding
`*_schema_version` const.

## Change discipline

Adding a new `preview_mode`, `preview_lane`,
`mapping_confidence_class`, `stale_editability_class`,
`hot_reload_state`, `device_target_class`, `viewport_preset_class`,
`source_sync_state`, `share_visibility_class`, `share_auth_class`,
`share_revoke_path_class`, or `downgrade_trigger` value is additive-
minor and bumps `preview_snapshot_schema_version`. Repurposing an
existing value is breaking and requires a new decision row.

Re-exporting a vocabulary from another schema is preferred over
minting a parallel one. Where this contract narrows or extends a
re-export, the gate is documented above; if a future contributor
needs to narrow further, that change lands on the owning schema, not
through a private fork in this directory.
