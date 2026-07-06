# M5 Runtime-Boundary Component Surface Certification (M05-859)

Closing capstone for the B100 runtime-boundary / repair-component lane. Where the
freeze matrix
(`freeze_the_m5_terminal_tab_remote_target_pill_environment_status_strip_toolchain_pin_row_presence_avatar_stack_and_repair_action_card_component_matrix`)
defines the six reusable components, the M05-853..857 primitive lanes narrow each
one, and the M05-858 accessibility lane proves keyboard / screen-reader /
CLI-export parity and auto-narrowing, this lane **certifies** that the shared
component truth holds on every claimed M5 runtime-collaboration-recovery surface
— and automatically narrows any surface that cannot sustain it.

- Rust module: `crates/aureline-shell/src/m5_runtime_boundary_component_certification/`
- Boundary schema: `schemas/ui/m5-runtime-boundary-component-certification.schema.json`
- Release proof: `artifacts/release/m5-runtime-boundary-component-certification-proof/`
  (`support_export.json`, `matrix.csv`, `report.md`)
- Fixtures: `fixtures/ui/m5-runtime-boundary-component-certification/`

## What is certified

The packet is keyed on the claimed **surface** a user runs, shares, switches,
repairs, or exports execution state through — not on the component family it
renders. The ten certified surfaces are:

`terminal`, `notebook_console`, `request_console`, `preview_server`, `debug`,
`run_test`, `collaboration`, `doctor`, `support`, `export`.

Each row certifies its surface across six truth axes — exactly the parity
dimensions the spec requires verifying:

| Axis | Meaning |
| --- | --- |
| `visual` | host boundary, resolved runtime/toolchain, collaboration role, and reversal class are shown on the primary surface |
| `keyboard` | the same boundary/status/role/reversal truth and its actions are reachable without a pointer |
| `screen_reader` | the same truth is announced non-visually, never relying on color or avatar imagery alone |
| `cli_export` | **always-on**: the certified surface state is reconstructable as text / JSON / Markdown for support and automation |
| `degraded_state` | a weakened host/runtime/role/reversal posture honestly downgrades a `Live`/`Ready` claim to degraded / reconnecting / restored / policy-blocked |
| `restore_no_rerun` | a restored session preserves boundary and status truth without silently re-running work |

Each surface also cites the frozen component families it renders
(`consumed_families`). Across the whole packet every one of the six families must
be certified on at least one surface (`all_families_covered`), which is how this
capstone proves the full component matrix runs across the claimed consumers.

## Runtime-support claim ladder

The claim a surface asserts (and the weakest ceiling it is certified down to) is
the reused M05-858 `M5RuntimeSupportClaim` ladder, strongest first:

`live` > `ready` > `degraded` > `reconnecting` > `restored` > `policy_blocked`.

Certification may only **narrow** a claim, never strengthen it.

## Verdict derivation (green / yellow / red)

The `derived_status` on every row is always recomputed from the axis outcomes and
claim narrowing — never asserted. The invariant is **a degraded axis must produce
a visible claim narrowing**.

- **Green** — every axis certified and the claimed runtime-support claim is
  delivered (`claimed_claim == certified_claim`, no `claim_auto_narrow`).
- **Yellow** — an axis is not current and the surface discloses the reduction by
  narrowing its claim to the weakest supported ceiling. The `claim_auto_narrow`
  block must bind to a non-always-on axis that is `disclosed_narrowed`, carry a
  precise (non-generic) `visible_label`, and its `from_claim`/`to_claim` must
  match the row's `claimed_claim`/`certified_claim`. The narrowed axis outcome
  names a frozen `M5RuntimeBoundaryDowngradeTrigger`.
- **Red** — any of: an axis is `undisclosed_drift`; the always-on `cli_export`
  axis is not certified (or copy/export is incomplete); the certified claim is
  stronger than the claimed one; a degraded axis is retained behind a full claim
  with no narrowing; or the narrowing block is inconsistent (spurious, wrongly
  bound, generic-labelled, or bound to the always-on axis). Red surfaces block
  the release; gaps are expressed as narrowed (yellow) claims or blocked (red)
  rows, never as hidden exceptions.

Every row cites exactly one canonical release-proof bundle —
`artifacts/release/m5-runtime-boundary-proof/support_export.json`, the frozen
runtime-boundary component release proof — rather than cloning per-surface
evidence, and records the M05-858 accessibility support export as supporting
evidence. The packet is metadata-only: raw file paths, remote hosts, credentials,
and device identifiers never cross this boundary.

## Seed certification

The checked-in packet certifies all ten surfaces: **6 green / 4 yellow / 0 red**.

| Surface | Claimed → Certified | Status | Binding axis |
| --- | --- | --- | --- |
| terminal | live → live | green | — |
| notebook_console | live → live | green | — |
| request_console | ready → ready | green | — |
| preview_server | ready → ready | green | — |
| run_test | ready → ready | green | — |
| export | restored → restored | green | — |
| debug | live → degraded | yellow | degraded_state |
| collaboration | live → reconnecting | yellow | degraded_state |
| doctor | ready → policy_blocked | yellow | degraded_state |
| support | ready → restored | yellow | restore_no_rerun |

## Regenerating the proof

The on-disk `support_export.json` is the `include_str!` canonical for the
round-trip test. Regenerate the artifacts and fixtures after any change to the
seeded builder:

```
GEN_RUNTIME_CERT_ARTIFACTS=1 cargo test -p aureline-shell \
  m5_runtime_boundary_component_certification::tests::generate_artifacts
```

Then rebuild so the baked-in `include_str!` picks up the new content, and run:

```
cargo test -p aureline-shell --lib m5_runtime_boundary_component_certification
```
