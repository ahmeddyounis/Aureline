# ADR 0002 — Renderer text-stack and shaping-fallback

- **Decision id:** D-0001 (see `artifacts/governance/decision_index.yaml#D-0001`)
- **Status:** Accepted
- **Decision date:** 2026-04-19
- **Freeze deadline:** 2026-07-01
- **Owner:** `@ahmedyounis`
- **Backup owner:** `null` (covered by waiver `single-maintainer-backup` in `artifacts/governance/ownership_matrix.yaml#waivers`)
- **Forum:** architecture_council
- **Related requirement ids:** none

## Context

The desktop shell is a custom-rendered surface. Editor, terminal, diffs,
and notebook code cells all feed through the same text-heavy pipeline;
whichever stack it picks becomes a shared floor under every later
shell, editor, benchmark, IME, and accessibility decision. Holding that
floor open invites each subsystem to introduce its own shaping,
fallback, or atlas posture and then discover late that they disagree —
which is exactly the drift the ownership matrix's renderer lane
exists to prevent.

The freeze matters because later work cannot land honestly on top of
an unfrozen stack: the renderer spike cannot claim latency ceilings,
the benchmark lab cannot stabilise traces, and the accessibility /
input-review lane cannot cite a concrete IME and semantic-tree
contract. An unfrozen stack also keeps the build-vs-reuse discipline
under tension, because each contributor can credibly argue for a
different crate graph.

This ADR closes `D-0001` (renderer stack and rendering primitive)
ahead of its `2026-07-01` freeze so the renderer spike, the benchmark
lab, and the accessibility-bridge work can start instrumenting against
concrete hook names rather than against a moving target.

## Decision

Aureline freezes a single renderer stack, one text-shaping direction,
one font-discovery and fallback strategy, one glyph-cache posture,
one invalidation model, and the GPU / windowing assumptions they ride
on. All are stated in terms of contracts and hook names rather than
specific library versions so dependency refresh is a hygiene change,
not a re-litigation.

### Renderer stack

- **GPU abstraction.** `wgpu`-class Rust binding over the native
  graphics API of the host: **Metal** on macOS, **Vulkan** on Linux,
  **Direct3D 12** on Windows. **OpenGL / GLES** is permitted *only*
  as a software / compatibility fallback on Linux hosts where neither
  Vulkan nor a working swapchain is reachable; that path degrades
  visibly (see `degraded_renderer_banner`) and does not claim hot-path
  parity.
- **Windowing.** `winit`-class Rust windowing crate. Each top-level
  Aureline window owns one `winit`-class window, one swap chain, one
  renderer context, and one accessibility node-tree root. The shell
  MAY host multiple windows backed by one workspace authority (per
  section 7.1.11 of the TDD); the renderer does not assume window
  count.
- **Scene model.** A two-layer model: a **text-and-decoration layer**
  painted by the pipeline in §11.5 of the TAD, and an **overlay layer**
  for caret, selection fill, drag ghosts, and composition underlines.
  The overlay layer is re-paintable without touching the glyph atlas,
  so caret blink and composition animation never force glyph re-raster.
- **Colour pipeline.** sRGB surface with per-monitor colour-space
  tagging; HDR / wide-gamut surfaces are explicitly out of scope at
  this decision and open a new row if requested.

### Text shaping

- **Shaper.** A HarfBuzz-compatible shaper exposed through a stable
  `Shaper` trait inside `crates/aureline-text`. The reference
  implementation is a Rust-native HarfBuzz-class engine
  (e.g. `rustybuzz`) so the default hot path carries no native-C
  dependency; an implementation-agnostic trait means the same contract
  can be fulfilled by a platform-native shaper where IME or
  accessibility parity requires it.
- **Platform-native shaper seam.** The `Shaper` trait permits — but
  does not mandate — a CoreText / DirectWrite / Pango adapter as a
  *parallel* implementation, selected per shaping call based on an
  explicit policy (default `rust_native`; `platform_native` only when
  a specific surface declares it needs OS-governed bidi, OS-composed
  IME clusters, or OS-specific typographic shaping). Lanes do not
  swap shaper silently.
- **Segmentation and bidi.** Grapheme, word, and bidi segmentation are
  owned by `crates/aureline-text` using Unicode-aligned algorithms
  (UAX #14, UAX #29, UAX #9). The shaper consumes an already-segmented
  run; renderers do not re-segment.
- **Features.** Ligature control, stylistic sets, and variable-font
  axes are exposed through typed feature flags on the shaping run so
  the buffer, terminal, diff, and notebook surfaces share one
  vocabulary rather than each inventing feature strings.

### Font discovery and fallback

- **Discovery.** A platform-aware font-directory abstraction
  (`fontdb`-class). On macOS it wraps CoreText directories; on Windows
  DirectWrite / registry roots; on Linux `fontconfig` roots plus
  XDG / system directories. The abstraction surfaces a stable font
  handle; consumers never hold raw paths.
- **Fallback chain.** A deterministic, inspectable chain applied per
  shaping run:
  1. **Explicit family** declared by the caller (editor theme,
     terminal profile, notebook cell, diff viewer).
  2. **Script-aware preference group** registered for the shaping
     run's script / language (for example, `han_fallback`,
     `arabic_fallback`, `emoji_fallback`).
  3. **OS system-UI family** for the active locale.
  4. **Last-resort bundled subset.** A bundled, signed Noto-class
     subset ships with the desktop binary so missing-glyph rendering
     never falls through to `.notdef` boxes on a supported host.
- **Fallback transparency.** Every shaped run records which fallback
  stage produced each glyph. The accessibility tree and support
  bundle expose the stage so glyph-substitution bugs are diagnosable
  as `fallback_stage = 3 (system_ui)` rather than as "font issue".
- **Colour-font support.** COLR / CPAL and bitmap sbix tables are
  supported through the rasteriser (see below); emoji sequences
  respect the fallback chain's `emoji_fallback` group before falling
  through to the bundled subset.

### Glyph-cache posture

- **Cache layers.**
  1. **Shape cache** — keyed on `(font_handle, feature_set,
     direction, script, cluster_text_hash)`; invalidated on font
     change, feature change, or text cluster change.
  2. **Raster cache** — keyed on `(glyph_id, font_handle, px_size,
     subpixel_variant, scale_bucket)`; backing store is a
     GPU-resident atlas shard per monitor-scale bucket.
  3. **Line-layout cache** — keyed on `(buffer_line_id, content_hash,
     shaping_features, direction, scale_bucket)`; invalidated on
     buffer edit, theme change, or scale bucket change.
- **Atlas sharding.** One atlas per scale bucket per surface. Moving
  a window between monitors of different scale rebinds the active
  shard rather than rebuilding every glyph, so first paint on the
  new monitor uses already-warm glyphs.
- **Eviction.** LRU by cold-slot rank within a shard. Eviction
  decisions carry a reason code (`lru`, `atlas_full`, `font_unloaded`,
  `scale_bucket_retired`) that surfaces in the renderer's telemetry
  scope.
- **No cross-process atlas.** Each desktop shell process owns its
  atlas; remote and spike processes never share GPU memory.

### Invalidation model

- **Dirty-rect compositor.** Only dirty rectangles are repainted; the
  composite step walks the dirty set and emits exactly one GPU
  submission per frame per surface.
- **Layout invalidation.** Layout invalidation is localised to the
  affected line range and does not cascade into neighbouring lines
  unless wrap or bidi context crosses the edit boundary.
- **Scale invalidation.** A per-surface scale change invalidates the
  active raster-cache shard but not the shape cache. A shape-feature
  change invalidates the shape cache but not the raster cache.
- **Overlay separation.** Caret blink, selection redraw, and IME
  composition underline live in the overlay layer and never
  invalidate glyph raster state.
- **Hidden surfaces.** Panes that are hidden, minimised, or behind
  a full-screen other window MUST stop reflow and animation; the
  renderer exposes an `is_visible` signal per surface so upstream
  reflow loops can cooperate.

### GPU / windowing assumptions

- **Target GPU API per host.** Metal (macOS 12+), Vulkan 1.2+
  (Linux), Direct3D 12 (Windows 10 22H2+ and Windows 11). These are
  the **claimed** targets; the benchmark lab reports against these
  hosts and no others.
- **Minimum feature set.** sRGB swapchains, monotonic present timing,
  vsync plus allow-tearing mode, and shader model adequate for
  glyph SDF rasterisation. Features beyond this floor (subgroup ops,
  mesh shaders) MUST NOT become load-bearing without a new decision
  row.
- **Windowing assumptions.** One swap chain per window; fractional
  and per-monitor DPI scaling supported; monitor hot-plug and scale
  change supported through the `multi_monitor_scale_change` hook
  (below); fullscreen toggles and desktop / space moves preserve the
  accessibility tree.
- **Software-render fallback.** A software rasteriser (e.g.
  `softbuffer`-class) is permitted as a last-resort path for
  unsupported GPUs. It does not claim hot-path parity and surfaces a
  visible `degraded_renderer_banner`.

### Accessibility bridge

- **Bridge.** A Rust-native cross-platform accessibility bridge
  (`accesskit`-class) feeding **UI Automation** (Windows),
  **NSAccessibility** (macOS), and **AT-SPI** (Linux).
- **Ownership.** The shell / renderer owns the accessibility tree
  lifecycle; text extraction for assistive technologies is served
  from the accessibility node tree, not by screen-scraping the GPU
  surface. A glyph that is visible but not present in the
  accessibility tree is a correctness bug, not a rendering
  optimisation.

### Protected-hot-path hook list

The renderer exposes the following named hooks. They are the canonical
instrumentation surface for the renderer spike and the benchmark lab;
no lane MAY invent alternative names for the same measurement.

| Hook id                          | Fires when                                                                                                                      | Protected hot-path budget |
|----------------------------------|---------------------------------------------------------------------------------------------------------------------------------|---------------------------|
| `warm_start_to_first_paint`      | Process is resident in page cache; the window swaps in its first non-blank frame                                                | yes                       |
| `first_paint`                    | Any surface emits its first frame in a session, including on reopen from a hidden state                                         | yes                       |
| `scroll_frame`                   | A vertical or horizontal scroll gesture submits a frame                                                                         | yes                       |
| `caret_move`                     | Caret primary position changes without selection change                                                                         | yes                       |
| `selection_change`               | Primary selection range or secondary-selection set changes                                                                      | yes                       |
| `ime_composition_update`         | IME composition string, underline segmentation, or caret-inside-composition changes                                             | yes                       |
| `fallback_glyph_resolution`      | A glyph resolves through fallback stage `>=2` for a shaping run                                                                 | yes                       |
| `multi_monitor_scale_change`     | The active surface's DPI / scale bucket changes, including monitor hot-plug and fractional-scale adjust                         | yes                       |
| `atlas_shard_rebind`             | A surface swaps its active raster-cache shard as a consequence of `multi_monitor_scale_change`                                  | yes                       |
| `atlas_eviction`                 | A glyph is evicted from a raster-cache shard                                                                                    | no (observability only)   |
| `frame_submit`                   | The compositor submits a frame to the GPU surface                                                                               | yes                       |
| `reflow_line_range`              | A localised layout reflow touches a contiguous line range                                                                       | yes                       |
| `degraded_renderer_banner`       | The renderer boots on the software fallback or loses its primary adapter mid-session                                            | no (observability only)   |
| `accessibility_tree_update`      | The accessibility node tree publishes a delta                                                                                   | yes                       |

The benchmark lab reports every hot-path hook against its protected
budget on the claimed hardware matrix; non-hot-path hooks are
observability-only and do not gate release.

### Non-goals for the renderer at this decision

Out of scope until a superseding decision row opens:

- WebGPU surfaces inside third-party browsers (browser companions
  ride a separate surface contract, not this one).
- HDR or wide-colour-gamut output.
- Render-graph experiments, frame-graph intermediate representations,
  and multi-pass post-processing beyond the two layers defined above.
- Shader-based font effects beyond glyph SDF rasterisation.
- In-app video playback or GPU-accelerated image pipelines beyond
  static decoration.
- Remote rendering of the primary editor surface (remote sessions
  still paint locally; this is a separate decision row if revisited).
- Sharing the glyph atlas across processes.

These lines move only by opening a new decision row, not by editing
this ADR.

### Tradeoff table

The structured tradeoff rows live in
`artifacts/architecture/renderer_tradeoff_rows.yaml`. The headline
summary:

| Axis                                         | Chosen stack                                                                 | Best rejected alternative                                                    | Why chosen wins                                                                                           |
|----------------------------------------------|------------------------------------------------------------------------------|------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------|
| **Hot-path performance**                     | `wgpu` + native GPU API + atlas-per-scale                                    | Electron / browser runtime                                                   | Electron-class ceiling is already rejected by AD-002; `wgpu` is the build-lane floor in §4.3              |
| **Accessibility text extraction**            | Accessibility tree published by renderer; `accesskit`-class bridge           | Screen-scrape a CPU-side glyph buffer                                        | Scraping bypasses semantic truth; AT parity requires the tree                                             |
| **IME composition correctness**              | Platform-input adapter + overlay composition layer + optional native shaper  | Own IME composition stack that bypasses OS composition                       | OS composition is authoritative on every platform; owning it would drift from dead keys, AltGr, IME       |
| **Cross-platform portability**               | `wgpu` + `winit` + Rust-native shaper + `accesskit`                          | Per-platform rewrites (Cocoa / Win32 / GTK) sharing a thin core              | Per-platform rewrites triple the support matrix and invert the build-vs-reuse posture                     |
| **Dependency health**                        | Rust-native defaults; platform-native seams optional and explicit            | Bind directly to HarfBuzz C library on every platform                        | Rust-native keeps the default hot path free of a C toolchain dependency while leaving the seam intact     |
| **Maintenance burden**                       | One renderer contract across editor / terminal / diff / notebook             | Per-surface renderers tuned locally                                          | One pipeline is the TDD §7.1.9 posture; divergence would break shared dirty-rect and invalidation logic   |

Each row carries reopen triggers in the YAML (for example: a
benchmark-lab finding that `fallback_glyph_resolution` exceeds its
budget on a claimed host reopens the shaper choice).

### Decision-example fixtures

A small corpus of decision-example fixtures lives under
`fixtures/text/renderer_decision_examples/`. They are short, reviewable
scenarios (mixed-script, CJK, colour emoji, bidi IME, ligatures,
fallback, multi-monitor scale, warm-start timing) used by the renderer
spike and benchmark lab to anchor the hook names above to concrete
inputs and expected observable outcomes. They are not a test suite;
they are the language the ADR's hook list refers to.

## Consequences

- **Frozen:** the renderer stack (GPU abstraction, windowing, scene
  model), the shaper contract, the font-discovery and fallback
  chain, the glyph-cache posture, the invalidation model, and the
  protected-hot-path hook names.
- **Frozen:** the accessibility bridge is an `accesskit`-class
  cross-platform bridge feeding UI Automation, NSAccessibility, and
  AT-SPI. Accessibility text extraction flows from the tree, not
  from the glyph buffer.
- **Frozen:** the protected-hot-path hook names are the canonical
  instrumentation vocabulary. The benchmark lab, the renderer spike,
  and the accessibility / input-review lane MUST use these names
  without inventing synonyms.
- **Permitted:** the `Shaper` trait MAY be implemented by
  platform-native shapers (CoreText, DirectWrite, Pango) when an
  explicit policy declares the surface needs OS-governed behaviour.
  This is not a per-lane choice; it is a policy decision recorded
  against the shaping run.
- **Permitted:** dependency refresh (bumping `wgpu`, `winit`,
  `rustybuzz`, `accesskit`, etc.) is a hygiene change governed by the
  compliance checklist, not a new ADR.
- **Follow-up:** the renderer spike instruments every hot-path hook
  before claiming latency budgets. The benchmark lab stabilises
  traces against the claimed GPU targets. The accessibility /
  input-review lane (tracked as dependency `DEP-0003`) stands up
  its packet family against `accessibility_tree_update` and
  `ime_composition_update`.
- **Follow-up:** a windowing-state ADR and an accessibility-bridge
  ADR MAY narrow specific subsurfaces above this floor, but cannot
  widen the rejected-alternatives list.
- **Ratifies:** the hot-path hook names become the vocabulary used by
  benchmark rows, renderer fitness functions, and release-evidence
  claims that cite renderer performance.

## Alternatives considered

- **Electron / Chromium-embedded shell (the rejected AD-002
  alternative).** Ship the editor inside a browser runtime and ride
  the browser's text and accessibility stacks. Rejected: AD-002 in
  the TAD already rejects this for latency, memory, and custom-text
  reasons, and the `Explicitly rejected architectural patterns` table
  in §4.4 reinforces the ban. Adopting it here would contradict the
  protected renderer lane's existence.
- **Platform-native shells (one per OS) with a thin shared core.**
  Use Cocoa / Win32 / GTK directly per host and share only data
  structures. Rejected: triples the support matrix, loses the one
  pipeline the TDD §7.1.9 depends on, and breaks the benchmark
  lab's ability to compare frames across hosts. It would also force
  three accessibility trees, three dirty-rect compositors, and three
  glyph caches — an unbounded maintenance surface for a
  solo-maintainer posture.
- **CPU-only renderer.** Skip the GPU entirely and ride a software
  rasteriser as the steady state. Rejected: cannot hit the TAD's
  latency ceiling on scroll, first paint, or large buffers. Kept
  only as a last-resort fallback with a visible degraded banner.
- **Stock browser-embedded text and shaping for editor.** Use a
  `WebView`-class surface for the editor specifically. Rejected:
  bidi, IME, ligature, and fallback behaviour are not controllable
  enough to meet the PRD's text-correctness bar, and the
  accessibility tree becomes shared with an opaque host.
- **Per-surface renderers tuned locally.** Let editor, terminal,
  diff, and notebook each pick their own renderer. Rejected:
  divergent invalidation, divergent font fallback, duplicated glyph
  atlases, and shell-wide regressions that look like "it only
  broke in the diff pane" bugs. The TAD §11.5 commits to one
  pipeline; this ADR commits to it in writing.
- **HarfBuzz C bindings on every platform.** Ship the C HarfBuzz
  library as the default shaper everywhere. Rejected: the
  Rust-native HarfBuzz-class engine covers the default correctness
  surface, keeps the hot path free of a C build-toolchain
  dependency, and the platform-native shaper seam already covers
  the IME and OS-typography cases where the C library would be the
  obvious choice.
- **Defer to a later milestone.** Leave `D-0001` open and let the
  narrowing default apply on `2026-07-01`. Rejected: the narrowing
  default (collapse to the shell spike's surface) is strictly
  narrower than the frozen stack above, and it would leave the
  benchmark lab, the renderer spike, and the accessibility /
  input-review lane without a contract to instrument against
  during the most expensive three months of pre-implementation
  work.

The `D-0001` default-if-unresolved narrowing would have frozen the
renderer to the shell spike's minimum surface and deferred all
advanced rendering primitives. Accepting this ADR replaces that
narrowing with the frozen stack, shaper contract, fallback chain,
glyph-cache posture, invalidation model, GPU / windowing assumptions,
and hook list above; the narrowed posture does not apply.

## Reopen triggers

Each of the following MUST open a new decision row (not an edit of
this ADR):

- a claimed benchmark host exceeds the protected budget on any
  hot-path hook and the fix would require swapping a stack element;
- the `fallback_glyph_resolution` hook consistently crosses its budget
  on a claimed locale, implicating the shaper or the fallback chain;
- a platform accessibility API changes in a way that `accesskit`-class
  cannot bridge inside its support window;
- an IME or platform-input change invalidates the overlay-layer
  composition assumption;
- adding HDR / wide-colour, WebGPU browser-companion parity, or
  cross-process atlas sharing to the product scope.

## Platform-specific risk notes

- **macOS.** Metal behaves predictably; the risk lane is IME behaviour
  during input-source switches and Live Text / OS overlay coexistence
  on the editor surface. The `ime_composition_update` hook is the
  trace anchor.
- **Windows.** Direct3D 12 requires current driver support on
  Windows 10 22H2 and Windows 11; older Windows hosts fall to the
  software fallback with the degraded banner. The risk lane is
  DirectWrite-governed line layout differing subtly from the
  Rust-native shaper for certain script/locale combinations;
  the platform-native shaper seam covers the escape hatch.
- **Linux.** Display-server diversity (X11, Wayland, mixed fractional
  scaling) is the dominant risk. `multi_monitor_scale_change` is the
  first-class trace anchor. Fontconfig roots and XDG directories
  vary across distributions; the font-discovery abstraction MUST
  tolerate both. AT-SPI availability differs across desktop
  environments; the accessibility claim narrows to the tested
  desktop profile.
- **All platforms.** Virtual machines and remote-desktop sessions
  frequently degrade GPU capability; the software fallback and the
  degraded banner are the first-class path for those environments,
  not a silent regression.

## Benchmark-measurement expectations

- Every protected-hot-path hook reports latency to the benchmark
  lab on the claimed GPU targets (Metal / Vulkan / D3D12) and on
  the software fallback. Hot-path hooks have budgets; non-hot-path
  hooks are observability-only.
- The benchmark lab's reproducibility pack for renderer claims
  names the GPU family, driver version, display configuration,
  shaper policy (`rust_native` or `platform_native`), font set,
  and fallback-chain composition at measurement time.
- A benchmark result that crosses a protected budget on a claimed
  host is a `red` lane state; repeated `yellow` on the same hook
  forces a scope correction per the milestone-scorecard rules.

## Source anchors

- `.t2/docs/Aureline_Technical_Architecture_Document.md:401` —
  AD-002: "custom native shell on wgpu/winit-class stack with
  accessibility bridge".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:425` —
  "wgpu/windowing stack, text shaping, accessibility bridges".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:450` —
  "Flagship desktop shell built on a browser/Electron runtime …
  conflicts with Aureline's latency, memory, and custom
  accessibility/rendering thesis".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1462` —
  Rendering pipeline stages (shaping → atlas → dirty-region →
  draw-list → GPU submit → accessibility tree).
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1469` —
  Rendering rules (dirty-region repaint, localised invalidation,
  hidden-pane animation stop).
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4452` —
  Accessibility architecture requirements (semantic tree,
  keyboard-complete command graph, focus model independent of paint
  order).
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4479` —
  "IME composition across editor, palette, settings, terminal,
  and rename inputs".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4482` —
  "font fallback across mixed scripts and emoji".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4521` —
  Windowing and displays: DPI scaling, multi-monitor, fullscreen,
  spaces.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4524` —
  Accessibility bridges for custom-rendered surfaces.
- `.t2/docs/Aureline_Technical_Design_Document.md:1256` —
  Rendering pipeline stages restated in the TDD.
- `.t2/docs/Aureline_Technical_Design_Document.md:1263` — Hot-path
  protection list (first paint, file switch, typing, scrolling,
  palette, warm quick-open, inline diagnostics).

## Linked artifacts

- Decision register row: `artifacts/governance/decision_index.yaml#D-0001`
- RFC: none.
- Tradeoff table (machine form):
  `artifacts/architecture/renderer_tradeoff_rows.yaml`.
- Decision-example fixtures:
  `fixtures/text/renderer_decision_examples/`.
- Affected lanes: `crates/aureline-render`, `crates/aureline-text`,
  `crates/aureline-bench`,
  `artifacts/governance/ownership_matrix.yaml#scorecard_lane_index:benchmark_lab`,
  `artifacts/governance/ownership_matrix.yaml#scorecard_lane_index:accessibility_input_review`.

## Supersession history

First acceptance. No supersession.
