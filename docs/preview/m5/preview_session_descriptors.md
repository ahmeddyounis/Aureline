# M5 preview-session descriptors and source-sync chips

This document is the contract for the M5 preview-session descriptors. It binds
the **first real M5 framework-pack, preview-route, and notebook-adjacent
surfaces** onto a single shared packet so the source revision, runtime target,
device/viewport, data posture, freshness, and source-sync state stop hiding
inside provider-specific extension chrome.

Where the
[source-first preview / browser-runtime inspection matrix](freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix.md)
freezes the *qualification* of each claimed preview/runtime surface, this packet
materializes the *per-session* descriptor each surface presents to the user.

Source remains canonical; the session descriptor is derivative — never a second
writable truth model. Switching between live, mock, captured, or stale preview
states changes governed chips and exports, not bespoke copy or silent icon
changes. A runtime-only view never masquerades as saved source state, and a stale
or downgraded session always exports a precise degraded label and trigger.

## Source of truth

- Packet type: `PreviewSessionDescriptorSet`
  (`crates/aureline-preview/src/preview_session_descriptors/`).
- Boundary schema:
  `schemas/preview/preview_session_descriptor_set.schema.json`.
- Checked support export:
  `artifacts/preview/m5/preview_session_descriptors/support_export.json`.
- Markdown summary:
  `artifacts/preview/m5/preview_session_descriptors.md`.
- Protected fixtures:
  `fixtures/preview/m5/preview_session_descriptors/`.
- Conformance dump: `cargo run -p aureline-preview --example dump_m5_preview_session_descriptors [support|summary]`.

## First-real consumer surfaces

Each first-real consumer surface carries at least one session descriptor:
`framework_pack_preview`, `preview_route`, and `notebook_adjacent_preview`. The
`support_export_projection` surface reuses the same descriptors downstream and is
not a first-real consumer.

## Governed chips

Each session reuses the frozen and per-view vocabularies rather than minting
synonyms, and adds the session-level chips this lane owns:

| Chip | Vocabulary | Source |
| --- | --- | --- |
| Preview session | `PreviewSessionClass` | inspection matrix (reused) |
| Source sync | `SourceSyncClass` | inspection matrix (reused) |
| Runtime origin | `PreviewOriginClass` | preview-origin descriptor (reused) |
| Target kind | `PreviewTargetClass` | preview-target descriptor (reused) |
| Device capability | `DeviceCapabilityClass` | preview-target descriptor (reused) |
| Data posture | `PreviewDataPostureClass` | this packet |
| Freshness | `PreviewFreshnessClass` | this packet |
| Downgrade trigger | `SessionDowngradeTrigger` | this packet |

- **Data posture** keeps the live-vs-mock-vs-captured mode honest: `live`,
  `mock`, `captured`, or `unidentified`.
- **Freshness** keeps a lagging view from claiming current state by silence:
  `fresh`, `aging`, `stale`, or `unknown`.
- **Source revision** is the opaque pointer at the canonical revision the view
  derives from; it is present for source-relative sync states and absent for a
  runtime-only view.

## Auto-downgrade gate

A session may not advertise current/in-sync/fresh state it cannot back:

- A session is downgraded when its freshness is `stale` or `unknown`, its
  source-sync class is `drifted_from_source` or `unidentified_source_sync`, or
  its data posture is `unidentified`.
- A downgraded session records a `downgrade_trigger` and carries a precise,
  non-generic `degraded_label`. A non-downgraded session carries neither.

`PreviewSessionDescriptorSet::validate` rejects a set that:

- omits a required first-real consumer surface, demonstrates no live-vs-mock-or-captured
  posture switch, or demonstrates no downgraded/stale session;
- lets a downgraded session skip its trigger or precise label, or lets a
  non-downgraded session carry a trigger or label;
- lets a `runtime_only_no_source` view claim saved source state or carry a
  canonical source revision;
- declares a `live` data posture without a live runtime, or lets a `captured`
  posture claim a live runtime or write capability;
- declares a source-revision presence inconsistent with the sync state;
- lets a write-capable session run unbacked by source or skip previewing the
  real source diff before commit;
- carries unpaired viewport dimensions or a session without evidence;
- fails any guardrail or consumer-projection invariant; or
- carries raw boundary material in the export.

## Guardrails

- Source remains canonical; the descriptor is derivative — never a second
  writable truth model.
- Runtime state never hides source-mapping uncertainty behind a live chip.
- Inspect-only sessions are never auto-upgraded into write-capable flows.
- Embedded preview/browser boundaries are not blurred into product authority.
- Switching posture changes governed chips and exports, not bespoke copy.
- Stale or downgraded sessions export precise truth downstream.

## Consumers

Product, docs/help, diagnostics, support export, and release-control surfaces
ingest these descriptors directly instead of cloning chip terminology by hand,
and they label downgraded sessions below current in every surface.
