# Preview-runtime strip, device-target picker, and hot-reload / source-map contract

This document freezes the **surface-level** contract every preview surface
projects to its top-of-surface chrome strip, its device-target picker, and
its hot-reload + source-map status badges. The goal is to make sure no
preview surface — browser, native, or embedded — can present a live or
editable status without simultaneously disclosing runtime class, freshness,
mapping quality, and target kind, and to make sure viewport presets remain
visibly distinct from real runtime targets in the picker.

This contract sits **above** the cross-surface preview-snapshot record and
**below** any consumer surface chrome. It does not implement preview
runtimes, simulators, or source mappers; it freezes how a strip / picker /
hot-reload status is allowed to look across the three preview lanes.

Companion artifacts:

- [`/schemas/preview/preview_runtime_strip.schema.json`](../../schemas/preview/preview_runtime_strip.schema.json)
  — boundary schema for the `preview_runtime_strip_record` every protected
  preview surface emits to render its top-of-surface strip.
- [`/schemas/preview/device_target_descriptor.schema.json`](../../schemas/preview/device_target_descriptor.schema.json)
  — boundary schema for the `device_target_descriptor_record` rows the
  device / viewport picker enumerates.
- [`/schemas/preview/hot_reload_state.schema.json`](../../schemas/preview/hot_reload_state.schema.json)
  — boundary schema for the `hot_reload_state_record` the hot-reload + source-
  map status badge emits, including controlled recovery routes.
- [`/fixtures/preview/preview_runtime_surface_cases/`](../../fixtures/preview/preview_runtime_surface_cases/)
  — worked corpus of strip, picker, and hot-reload cases.
- [`/schemas/preview/preview_snapshot.schema.json`](../../schemas/preview/preview_snapshot.schema.json)
  and [`/docs/architecture/preview_runtime_contract.md`](../architecture/preview_runtime_contract.md)
  — cross-surface preview-snapshot record this contract projects from.
- [`/schemas/security/trust_class.schema.json`](../../schemas/security/trust_class.schema.json)
  — safe-preview trust-class, connectivity-state, and downgrade-trigger
  ladder this contract re-exports.
- [`/schemas/generated/artifact_edit_posture.schema.json`](../../schemas/generated/artifact_edit_posture.schema.json)
  — provenance-state and default-edit-posture vocabulary the surface badges
  pin.

If this document disagrees with the PRD, TAD Appendix CR, or UX spec, those
sources win and this document plus the schemas update in the same change.

## Why a separate surface contract

The cross-surface `preview_snapshot_record` answers four questions about a
preview surface in machine form. The strip, picker, and hot-reload badge
answer the same questions in **chrome form**. Without a frozen surface
contract:

- a preview surface can claim a live runtime in its title bar without
  disclosing freshness or mapping quality on the same row;
- a device picker can list a viewport preset and an attached physical
  device side-by-side under the same group, letting users believe the
  preview ran on a real device of that class;
- a hot-reload badge can collapse `restart_required`, `rebuild_required`,
  `failed`, and `unavailable` into one vague "stale" label that hides
  which source-safe recovery route applies.

This contract closes those gaps with three frozen records:

1. The **strip record** that every protected preview surface MUST project
   on its top-of-surface chrome.
2. The **device-target descriptor** every picker row MUST project, with
   viewport presets and real runtime targets in disjoint groups.
3. The **hot-reload state record** every badge MUST project, including the
   typed source-mapping projection and the closed list of source-safe
   recovery routes for each non-applied state.

## Scope

Frozen at this revision:

- the `preview_runtime_strip_record` shape, its required disclosure floor
  (runtime class, trust state, degraded note, target kind, source revision,
  theme/device identity, session freshness, mapping quality, hot-reload
  state), and the rule that a strip MUST NOT claim "live" or "editable"
  without naming all of them on the same record;
- the `device_target_descriptor_record` shape and the `picker_group_class`
  partition that keeps viewport presets disjoint from real runtime targets
  (browsers, simulators, emulators, physical devices, managed pools);
- the `hot_reload_state_record` shape including the six-class hot-reload
  vocabulary (`applied`, `partial`, `restart_required`, `rebuild_required`,
  `failed`, `unavailable`), the strip-projection three-class source-mapping
  vocabulary (`exact`, `approximate`, `unavailable`), the inspect-to-source
  and edit gating rules per state, and the closed list of source-safe
  recovery routes per non-applied state;
- the rule that every `preview_runtime_strip_record` cites the underlying
  `preview_snapshot_record` by `preview_snapshot_record_ref` and never
  duplicates the snapshot's source-of-truth fields;
- the redaction floor on every record: raw URLs, raw absolute paths, raw
  hostnames, raw IP addresses, raw device serial numbers, raw bearer
  tokens, raw session cookies, raw rendered bytes, raw stack frames, and
  raw mock-data bodies never cross any of these boundaries.

Out of scope (named explicitly so the schemas do not creep):

- implementing preview runtimes, simulators, or source mappers;
- minting framework-specific source-mapping algorithms or device emulator
  pipelines;
- the visual-transform manifest schema (only opaque refs are carried);
- the share-sheet UX (the strip carries an opaque ref only);
- any preview-specific telemetry; payloads narrow through the
  telemetry/support registry like every other surface.

## The preview-runtime strip

The strip is the top-of-surface chrome row every protected preview surface
renders. Its boundary record is `preview_runtime_strip_record`. The record
is a projection — it cites the underlying `preview_snapshot_record_ref`
and re-uses the snapshot's vocabulary verbatim — but the strip carries its
own required disclosure floor so a consumer surface cannot present "live"
or "editable" without naming every floor field on the same row.

### Required disclosure floor

A protected strip MUST display all of:

| Field group              | Strip field                                                                 | Source                                                                |
|--------------------------|-----------------------------------------------------------------------------|-----------------------------------------------------------------------|
| Runtime source class     | `runtime_source_class`                                                      | mirrors `preview_runtime_kind_class` from the snapshot                |
| Trust / policy state     | `trust_state` (trust class) + `policy_state_class` + `connectivity_state`   | re-exports the safe-preview ladder                                    |
| Degraded note            | `degraded_note` (typed reason summary)                                      | one of `downgrade_trigger_observations`                               |
| Target kind              | `target_kind_class`                                                         | mirrors `target_environment_class` from the snapshot                  |
| Source revision          | `source_revision_chip`                                                      | redacted projection of `source_revision_anchor`                       |
| Theme / device identity  | `theme_identity` + `device_identity_summary`                                | composes with the design-system theme record + device-target row      |
| Session freshness        | `session_freshness_class`                                                   | mirrors `freshness_class`                                             |
| Mapping quality          | `mapping_quality_class`                                                     | strip-projection of `mapping_confidence_class` (three-class)          |
| Hot-reload state         | `hot_reload_state_class` + `recovery_route_class`                           | mirrors the hot-reload state record                                   |
| Live or editable claim   | `live_or_editable_claim_class`                                              | derived; gated by floor fields below                                  |

The strip MUST NOT claim `live` or `editable` in
`live_or_editable_claim_class` without all of the following being present
on the same record: `runtime_source_class`, `target_kind_class`,
`session_freshness_class`, `mapping_quality_class`, and a non-stale
`hot_reload_state_class`. The schema enforces this through an `allOf` gate.

### Trust / policy state composition

`trust_state` re-exports the two safe-preview classes admissible to
executable preview surfaces (`TrustedLocalActive`, `IsolatedRemoteActive`).
`policy_state_class` re-exports the workspace-policy state vocabulary
(`policy_in_force`, `policy_in_review`, `policy_narrowed`,
`policy_revoked`, `policy_not_applicable`). `connectivity_state` re-exports
the safe-preview connectivity ladder. The strip does not invent parallel
trust or policy classes.

### Degraded note

Every strip in a non-`live` posture MUST surface a `degraded_note` whose
`degraded_reason_trigger` is one of the snapshot's
`downgrade_trigger_observations`. Free-form prose is forbidden in the
typed slot; the `degraded_note_summary` is a redaction-safe sentence.

### Source revision chip

The chip mirrors the snapshot's `source_sync_state` and projects only the
`source_revision_class` and the opaque `source_revision_ref`. Per-path
identity does not project. `drifted_paths_count` MAY project as a typed
count; the strip never displays raw path strings.

### Theme and device identity

`theme_identity` carries a redaction-safe theme label class
(`light_default`, `dark_default`, `high_contrast_light`,
`high_contrast_dark`, `custom_workspace_theme`,
`custom_user_profile_theme`, `external_design_system_theme`,
`theme_not_applicable`) plus an opaque `theme_revision_ref`.

`device_identity_summary` is a redacted projection of the snapshot's
`device_target` row. It carries `device_target_class`,
`viewport_preset_class`, an opaque `device_handle_ref` when present, and
the redacted `device_label`. Raw serial numbers, raw IP addresses, raw
hostnames, and raw account handles never appear.

## The device / viewport picker

The picker is the chooser surface where a user selects which target the
preview should run against. Its boundary record is
`device_target_descriptor_record`. Every row in the picker is one
descriptor.

### Picker-group partition

`picker_group_class` is the top-level partition every descriptor MUST
declare. The vocabulary is closed and disjoint:

- `viewport_presets_group` — desktop, tablet, phone, custom presets. No
  real runtime target is attached. The descriptor's `is_real_runtime_target`
  MUST be `false`.
- `attached_browsers_group` — actual browser windows / tabs / embedded
  canvases backed by a real renderer process. `is_real_runtime_target`
  MUST be `true`.
- `simulators_group` — OS-level or framework-level simulators (iOS sim,
  Android emulator running as a simulator profile). `is_real_runtime_target`
  MUST be `true`.
- `emulators_group` — instruction-level emulators (full Android emulator
  image, embedded device emulator). `is_real_runtime_target` MUST be
  `true`.
- `physical_devices_group` — tethered or remote-attached physical devices.
  `is_real_runtime_target` MUST be `true`.
- `managed_device_pool_group` — shared device pool reservations.
  `is_real_runtime_target` MUST be `true`.
- `external_handoff_group` — external-browser or external-device handoff
  targets. `is_real_runtime_target` MUST be `true`.

The schema enforces that descriptors in `viewport_presets_group` set
`is_real_runtime_target = false` and a `null` `device_handle_ref`, and
that descriptors in every other group set `is_real_runtime_target = true`
and a non-null `device_handle_ref`.

### Picker-row floor

Every descriptor MUST carry:

- `descriptor_id` (opaque)
- `picker_group_class`
- `device_target_class` (re-export of the snapshot's class)
- `viewport_preset_class`
- `is_real_runtime_target` (boolean, gated as above)
- `availability_class` (`available_now`, `available_with_warmup`,
  `available_remote_only`, `unavailable_temporarily`,
  `unavailable_permanently`, `requires_approval`, `requires_pairing`)
- `redacted_label` (short, reviewer-safe)

### Disclosure rules

A picker UI MUST render the seven groups in seven visually distinct
sections with a group header. A descriptor SHALL NOT be moved between
groups by a downstream surface; if reclassification is needed, the
descriptor is re-emitted with the corrected `picker_group_class`.

## Hot-reload state and source-mapping projection

The hot-reload status badge carries the boundary record
`hot_reload_state_record`. The record is a strip-level projection of the
deeper `hot_reload_block` and `mapping_confidence_class` fields on the
preview-snapshot record.

### Hot-reload state vocabulary

`hot_reload_state_class` is the spec-frozen six-value vocabulary:

- `applied` — most recent source change is reflected.
- `partial` — some change reflected, some pending. The badge MAY admit
  `apply_mapped_edit` if mapping is exact and stale-editability is fully
  editable.
- `restart_required` — dev-server or runtime must restart before the
  surface can offer mapped edits again.
- `rebuild_required` — a build step must run; same restriction.
- `failed` — most recent reload errored. The badge exposes a typed
  `last_failure_reason_class` and the recovery routes admissible from
  this state.
- `unavailable` — hot reload is not supported by this runtime / adapter /
  target. The forced state for any `static_preview` strip.

### Source-mapping state (strip projection)

`source_mapping_state_class` is the **three-class** strip-projection of
`mapping_confidence_class` from the cross-surface snapshot:

| Strip projection | Snapshot mapping confidence                                        |
|------------------|--------------------------------------------------------------------|
| `exact`          | `exact_source_mapping`                                             |
| `approximate`    | `approximate_mapping` or `unmappable_node_present`                 |
| `unavailable`    | `stale_mapping`, `runtime_only_no_source_mapping`, `unknown_mapping` |

The schema enforces the projection map: a strip-projected `exact` MUST
correspond to an `exact_source_mapping` snapshot, an `approximate`
projection MUST correspond to an `approximate_mapping` or
`unmappable_node_present` snapshot, and an `unavailable` projection MUST
correspond to one of the three remaining snapshot classes.

### Inspect-to-source and edit gating per state

The strip badge resolves two boolean gates the consumer surface reads
instead of recomputing per-surface:

- `inspect_to_source_admissible` — true iff
  `source_mapping_state_class ∈ {exact, approximate}` and
  `hot_reload_state_class ∈ {applied, partial, restart_required,
  rebuild_required}`. `failed` and `unavailable` set this to false.
- `mapped_edit_admissible` — true iff
  `source_mapping_state_class = exact` AND
  `hot_reload_state_class ∈ {applied, partial}` AND the underlying
  snapshot is a `visual_edit` snapshot in `fully_editable_through_mapped_edits`.

A strip whose `mapped_edit_admissible = true` MUST cite a
`transform_manifest_ref` on the underlying snapshot record. The schema
enforces this through an `allOf` gate.

### Source-safe recovery routes

`recovery_route_class` is the closed vocabulary of recovery routes
admissible from a non-`applied` hot-reload state:

- `no_recovery_required_applied` — `applied` only.
- `no_recovery_required_static_preview_unavailable` — `unavailable` on a
  `static_preview`. No live runtime exists; recovery is not applicable.
- `wait_for_partial_to_settle` — `partial` only.
- `restart_runtime_recovery` — `restart_required` only.
- `rebuild_then_reload_recovery` — `rebuild_required` only.
- `inspect_only_with_diff_against_source_recovery` — `failed` with
  mapping `exact` or `approximate`. The user inspects the failure,
  diffs, and re-runs.
- `export_metadata_only_recovery` — `failed` with mapping `unavailable`,
  or runtime unavailable. The strip exports a metadata-only packet so
  the user has a record without a working preview.
- `open_canonical_source_recovery` — any non-`applied` state where the
  source-mapping is `exact` or `approximate`. The user opens the
  canonical source and resolves manually.
- `open_runtime_logs_recovery` — `failed` only. The user opens the
  typed runtime log surface (no raw bytes cross this boundary; only
  opaque log refs).
- `request_managed_runtime_recovery` — `unavailable` or `failed` on a
  managed-runtime target. The user requests a fresh managed runtime
  through the integration approval flow.

The schema enforces the per-state allowed recovery-route set. A `failed`
or `rebuild_required` state MUST cite at least one source-safe recovery
route from this list and MUST NOT collapse into a vague "stale" label.

## Composition with the cross-surface preview-snapshot record

Every `preview_runtime_strip_record` carries
`preview_snapshot_record_ref` pointing at the underlying snapshot
record. The strip never re-asserts source-of-truth fields the snapshot
already owns (mapping confidence detail, full hot-reload block,
transform-manifest body, share-sheet body); it carries the
strip-projection vocabularies above and an opaque ref to the snapshot.

Every `device_target_descriptor_record` carries an optional
`bound_preview_snapshot_record_ref` for the snapshot the descriptor was
selected for, and an `availability_observed_at` timestamp.

Every `hot_reload_state_record` carries
`preview_snapshot_record_ref` and an `observed_at` timestamp. The strip-
projection mapping above is the only allowed projection from the
six-class snapshot mapping vocabulary.

## Composition with the safe-preview, artifact-edit-posture, and execution-context boundaries

- The execution-context boundary stays the source of truth for runtime
  target identity, sandbox posture, and policy epoch. The strip cites
  `execution_context_record_ref` through the snapshot only and does not
  duplicate execution-context fields.
- The safe-preview boundary stays the source of truth for trust class,
  connectivity state, and downgrade-trigger ladder. The strip
  re-exports those vocabularies; new triggers land additive-minor on
  the safe-preview schema and on the preview-snapshot schema.
- The generated-artifact lineage / edit-posture boundary stays the
  source of truth for `artifact_origin_class = preview_projection`, the
  `mock_provenance` rule, and the structured-viewer fallback.

## Per-lane field requirements

| Field group                        | Browser preview lane | Native preview lane | Embedded preview lane |
|------------------------------------|----------------------|---------------------|------------------------|
| Strip required-floor               | required             | required            | required               |
| Picker descriptor required-floor   | required             | required            | required               |
| Hot-reload state record            | required             | required            | required               |

The same record shapes render in all three lanes; lane-specific adapters
do not redefine the strip vocabulary, the picker partition, or the hot-
reload vocabulary.

## Change discipline

Adding a new `runtime_source_class`, `target_kind_class`, `picker_group_class`,
`availability_class`, `theme_identity_class`, `policy_state_class`,
`live_or_editable_claim_class`, `mapping_quality_class`,
`source_mapping_state_class`, `hot_reload_state_class`,
`recovery_route_class`, or `degraded_reason_trigger` value is additive-
minor and bumps the corresponding `*_schema_version` const. Repurposing
an existing value is breaking and requires a new decision row.

Re-exporting a vocabulary from another schema is preferred over minting
a parallel one. Where this contract narrows or projects a re-export
(notably the three-class strip projection of mapping confidence), the
gate is documented above; if a future contributor needs to widen the
projection, that change lands here and on the snapshot schema together.
