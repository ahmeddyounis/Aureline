# Shell spike — composition-path notes

These notes name the seams the desktop shell spike
(`crates/aureline-shell-spike`) exposes for damage and invalidation,
the text-layer / overlay-layer boundary, the placeholder-surface
ownership of each zone, and the trace and log identifiers emitted at
startup and at the input/render boundaries. They are the contract a
later renderer-wiring task implements; the spike does not own the GPU
itself.

The hook vocabulary is normative and lifted verbatim from
[ADR 0002 — Renderer text-stack and shaping-fallback](../adr/0002-renderer-text-stack-and-shaping-fallback.md).
The spike MUST NOT introduce synonyms.

## Damage and invalidation entry points

The spike has exactly one classifier function that decides which zone
and which scene layer an input action damages:

- `aureline_shell_spike::render_path::classify(frame, action)` returns
  a `DamageRecord { zone, layer, rect, hook }`.
- `aureline_shell_spike::render_path::classify_layer(action)` returns
  the `Layer` only, for callers that have already resolved the zone.

Every damaging action produces exactly one record per dispatch. The
compositor is expected to coalesce per zone per frame; the spike does
not coalesce because the trace samples committed under
`artifacts/render/spike_trace_samples/` are intentionally arrival-order
records.

Non-damaging actions (`InputAction::None`) return no record. This is
the seam the eventual renderer crate uses to skip frames where no
input has touched any zone.

## Text-layer / overlay-layer boundary

The spike paints one of two ADR-0002 layers per damage record:

| Layer                  | Hooks routed here                                             | Re-rasters glyphs? |
|------------------------|---------------------------------------------------------------|--------------------|
| `text_and_decoration`  | `reflow_line_range`, `scroll_frame`, `multi_monitor_scale_change` | yes               |
| `overlay`              | `caret_move`, `selection_change`, `ime_composition_update`     | no                 |

The classifier in `render_path` is the only place this mapping lives.
The renderer wiring task may replace the function but may not
contradict the table; doing so reopens ADR 0002.

## Placeholder surface ownership

These are spike render zones, not the canonical product shell zones from
ADR 0016. The spike declares the following ownership posture for its
trace and damage buckets:

| Spike zone         | `SurfaceOwnership` | Reason                                                                 |
|--------------------|--------------------|------------------------------------------------------------------------|
| `title_bar`        | `stable`           | Static decoration; never re-shapes glyphs at runtime.                  |
| `sidebar`          | `stable`           | Placeholder until the panel system lands; treated as static chrome.    |
| `editor_viewport`  | `text_pipeline`    | Hosts the text-and-decoration layer; the only zone that re-shapes.     |
| `status_bar`       | `stable`           | Static decoration; updates are infrequent and treated as full repaints.|

`SurfaceOwnership::for_zone(zone)` is the seam consumers call. Adding a
zone requires bumping `CapabilityManifest::SCHEMA_VERSION` and
updating this table in the same change.

## Trace and log identifiers

The spike emits records at three boundaries:

1. **Startup.** When the binary launches the fixture scene, the timing
   recorder writes `warm_start_to_first_paint` followed by `first_paint`
   with the note `scene.begin` / `scene.first_paint`. These two hooks
   are ADR 0002's startup pair and never fire in any other position.
2. **Input boundary.** Each step in the fixture script labels its
   input event (e.g. `caret_left`, `ime_compose`). The label is carried
   into the timing mark's `note` field so trace consumers can correlate
   a hook to the script step that produced it.
3. **Render boundary.** The compositor closes every scene with one
   `frame_submit` record, noted `scene.end`. This is the seam the
   benchmark lab's frame-cadence assertions hang off.

The capability manifest written to
`artifacts/render/spike_capabilities.json` records the schema version,
the build identity (crate + version + target triple), the backend
selection, the host OS, the zone layout, and the full hook vocabulary.
Trace consumers MUST read the manifest to learn which hooks the spike
considers hot-path, rather than re-deriving the rule.

## Mode of operation

The spike binary (`shell_spike`) runs the fixture scene headlessly in
this revision. Wiring a native `winit` window and a software-render
or `wgpu`-backed surface is a follow-up; the seams in `lib.rs`
(`input_path`, `render_path`, `frame_timing`, `zones`) are the contract
that wiring will satisfy. Until the wiring lands the binary's
`Backend` field is `headless`; the manifest's `backend` field flips to
`native_window` once the surface is bound.
