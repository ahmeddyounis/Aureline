# M5 component-gallery evidence pack

The **evidence pack** is the versioned, machine-readable visual / accessibility
proof for the launch-critical M5 component families. Where the
[host-primitive library](m5-host-primitive.md) ships the host-rendered
*implementation* each family renders through, and the
[component manifests](m5-component-manifest.md) freeze their *contracts*, the
evidence pack is the **reproducible gallery** a shell-quality gate reads instead
of a folder of hand-captured screenshots. It is regenerated from the same
checked-in contract Aureline ships, so the proof can never drift from the
component it certifies.

- Schema: [`schemas/design-system/m5-evidence-pack.schema.json`](../../schemas/design-system/m5-evidence-pack.schema.json)
- Canonical pack: [`fixtures/ui/m5-component-gallery/evidence-pack.json`](../../fixtures/ui/m5-component-gallery/evidence-pack.json)
- Per-component fixtures: `fixtures/ui/m5-component-gallery/evidence-<kind>.json`
- Release packet: [`artifacts/release/m5-design-system-proof/evidence-pack-release.json`](../../artifacts/release/m5-design-system-proof/evidence-pack-release.json)
- Producer: `cargo run -p aureline-design-system --bin aureline_design_system_m5_evidence_pack`

## What a component's evidence records

The pack carries one evidence record per launch-critical family, keyed by the
same `component_kind` as the manifest and host primitive. Each record:

- **renders one gallery scene per controlled state**, derived directly from the
  host primitive's render plan for that state. A scene copies the primitive's
  `rendered_parts`, `non_color_cues` (always including `label_text`), status
  message id, and interactivity, and adds keyboard-journey and
  assistive-technology evidence refs. Because the scenes are minted from the
  checked-in primitive library, the proof is reproducible from the contract
  rather than from a manual capture.
- **captures one appearance variant per axis in the same pack** — see below.
- **attaches its owning identity and computed freshness** — the `component_id`
  and `owner_role` come from the component manifest, and the `freshness` is
  *computed* from `captured_at`, `evaluated_at`, and `freshness_window_days`. The
  derived `claim_gate` certifies, narrows, or blocks the component's
  shell-quality claim.

## Appearance variants live in one pack

Every scene captures seven appearance variants, so high-contrast,
reduced-motion, and zoom evidence is never split into a separate, easily-stale
folder from the normal-theme baseline:

| `variant_kind` | Theme | Motion posture | Zoom |
| -------------- | ----- | -------------- | ---- |
| `normal_dark` | `dark_reference` | `motion_standard` | 100% |
| `normal_light` | `light_parity` | `motion_standard` | 100% |
| `high_contrast_dark` | `high_contrast_dark` | `motion_standard` | 100% |
| `high_contrast_light` | `high_contrast_light` | `motion_standard` | 100% |
| `reduced_motion` | `dark_reference` | `motion_reduced` | 100% |
| `zoom_150` | `dark_reference` | `motion_standard` | 150% |
| `zoom_200` | `dark_reference` | `motion_standard` | 200% |

Each variant carries a `non_color_meaning_present` assertion, a `focus_visible`
assertion (true wherever the scene can receive focus), and a deterministic
`baseline_digest`.

## The baseline digest is the visual-diff

`baseline_digest` is an `fnv1a64:<16 hex>` content hash computed over the scene's
canonical descriptor — the owning component id, state, status id, rendered parts,
non-color cues, interactivity, and the variant's theme / motion / zoom axes. A
content change moves the digest, so the checked-in baseline diff fails *without
anyone comparing a screenshot*. The same function backs both the seed builder and
`M5EvidencePack::validate`, so a stored digest can never silently drift from the
scene it certifies. The digest is metadata, not an image: the pack carries no raw
screenshots.

## Freshness narrows or blocks shell-quality claims

Freshness is the load-bearing governance of the lane. `evidence_freshness`
computes a component's age (evaluation date minus capture date, in days) and
compares it to its window:

- within the window → `current` → `claim_gate: certified`;
- past the window → `stale` → `claim_gate: narrowed` (the component's claim
  auto-narrows to a disclosed reduced posture);
- incomplete scene or variant coverage → `claim_gate: blocked`.

`M5EvidencePack::reevaluate(evaluated_at)` re-derives every component's freshness
and gate as-of a new release date — capture dates, scenes, and digests are
untouched, only freshness and the gate move — so a gate can inspect whether the
checked-in evidence is still current for a given date and narrow claims when it is
not. Narrowing is **per owning component**: the canonical pack staggers capture
dates by family, so re-evaluating at a later date narrows the oldest components
first while freshly-captured ones stay certified (see
`seeded_m5_evidence_pack_stale_narrowed`).

## Release-packet inclusion

`release_packet` projects a `m5_design_system_evidence_pack_release` packet with
one freshness / gate / shape summary per component (scene count, total variant
count, high-contrast / reduced-motion / zoom variant counts, capture date,
freshness, and claim gate), plus the pack totals and the `certified` / `narrowed`
/ `blocked` component counts and the pack-level gate. The release center, QA, and
support exports consume this projection to narrow claims when a component's proof
goes stale.

## Privacy and boundary

Evidence packs are metadata-only truth packets. They carry semantic token
*references*, message *ids*, and content *digests* — never raw color values, raw
screenshots, credential bodies, or provider payloads. The validator scans the
serialized export for forbidden boundary material as defense in depth.

## Drift control

The seed builder in `aureline-design-system` is the single producer of the
checked-in pack fixture, the per-component fixtures, and the release packet, and
the inline tests assert the checked-in artifacts match the seed, validate, are
rendered from the host-primitive render plans, and take their owning identity from
the component manifests, so any drift fails
`cargo test -p aureline-design-system m5_evidence_pack`.
