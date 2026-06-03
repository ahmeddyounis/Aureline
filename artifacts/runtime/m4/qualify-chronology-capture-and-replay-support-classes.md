# Qualify chronology capture and replay/reverse-debug support classes across debug lanes — M4 truth packet

This document is the reviewer-facing artifact summary for the M4 chronology-capture
and replay/reverse-debug support-class qualification truth packet.

| Field | Value |
|---|---|
| Record kind | `qualify_chronology_capture_and_replay_support_classes_truth_stable_packet` |
| Schema version | 1 |
| Promotion state | **stable** |
| Lanes covered | `local_lane`, `remote_helper_lane`, `container_lane`, `notebook_bridge_lane` |
| Row count | 148 (37 rows per lane × 4 lanes) |
| Consumer projections | 13 |
| Validation findings | 0 |

## What the packet promises

- **Replay support class is derived, not claimed.** Every debug lane declares
  its full replay-support vocabulary (`supported`, `limited`, `view_only`,
  `unsupported`, `policy_blocked`) via explicit `support_class_admission` rows.
  No surface may collapse, paraphrase, or silently promote a class.

- **Chronology capture state is always disclosed.** Each lane admits all four
  capture states (`recorded`, `not_recorded`, `restart_with_recording_available`,
  `capture_unsupported`). The debugger UI, timeline scrubber, and variable
  inspector MUST surface the packet state verbatim rather than eliding
  `not_recorded` or silently hiding the restart-with-recording offer.

- **Mapping quality badges are explicit.** All six badge classes (`exact`,
  `approximate`, `partial`, `unavailable`, `stale`, `mismatched`) are admitted per
  lane. Call-stack and variable-inspector surfaces MUST attest
  `attests_mapping_quality_preserved=true`.

- **Replay surfaces are read-only unless an explicit live-control path is
  active.** Every replay surface binding except `support_export_surface` attests
  `attests_replay_read_only=true`. The support export surface is exempt because
  it emits evidence bundles rather than presenting live replay controls.

- **Inspector state is preserved across replay surfaces.** The variable
  inspector, call-stack panel, evaluate console, and support-export surfaces
  attest `attests_inspector_state_preserved=true`, preventing replayed state from
  polluting the live inspector view.

- **Restart with recording is a packet-qualified offer, not a UI guess.** The
  four restart-posture admissions ensure the UI only surfaces the restart offer
  when the packet shows `available`; the three unavailable variants carry
  explicit reasons rather than silent omission.

- **No raw debug material crosses the boundary.** All rows in the stable packet
  attest `raw_source_material_excluded=true`, `secrets_excluded=true`, and
  `ambient_authority_excluded=true`. Any row violating these flags fires
  `raw_source_material_present` and blocks stable promotion.

- **Lineage is bound per lane.** Each lane carries a `lineage_admission` row
  with a non-empty `execution_context_id_binding`, keeping every chronology
  packet and support export attributable to one execution context.

## Promotion state

**stable.** The packet has zero validation findings. All four lanes pass the
full qualifying criteria: evidence class is bound, all required vocabulary rows
are present, all replay surface bindings carry the required attestations, and no
raw material is admitted.

## Source-of-truth pointers

| Artifact | Path |
|---|---|
| Boundary schema | [`schemas/debug/chronology-replay-support.schema.json`](../../../schemas/debug/chronology-replay-support.schema.json) |
| Contract doc | [`docs/m4/qualify-chronology-capture-and-replay-support-classes.md`](../../../docs/m4/qualify-chronology-capture-and-replay-support-classes.md) |
| Rust contract | [`crates/aureline-debug/src/qualify_chronology_capture_and_replay_support_classes/`](../../../crates/aureline-debug/src/qualify_chronology_capture_and_replay_support_classes/) |
| Stable packet | [`artifacts/runtime/m4/qualify_chronology_capture_and_replay_support_classes_truth_packet.json`](qualify_chronology_capture_and_replay_support_classes_truth_packet.json) |
| Fixture corpus | [`fixtures/runtime/m4/qualify_chronology_capture_and_replay_support_classes/`](../../../fixtures/runtime/m4/qualify_chronology_capture_and_replay_support_classes/) |

## Narrowed-below-stable drills

Any lane that cannot satisfy the full vocabulary coverage at `launch_stable` MUST
narrow that lane's quality row to `launch_stable_below` (or below) and include a
`disclosure_ref` pointing to the documented known limit. The packet's promotion
state is the meet of all lane promotion states; a single narrowed lane narrows the
whole packet.

## How to verify

```sh
# Build the aureline-debug crate.
cargo build -p aureline-debug

# Run the 16 qualification unit tests.
cargo test -p aureline-debug
```
