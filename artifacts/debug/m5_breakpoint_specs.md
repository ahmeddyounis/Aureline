# M5 breakpoint-spec evidence

This set is the checked-in proof path for Aureline's typed M5 breakpoint specs and
mapping-state pills: the canonical record every debugger-capable surface reads to show
what a breakpoint requested, where it actually bound, and whether its source mapping is
still trustworthy. It materializes the breakpoint-spec family named by the
[M5 debug-contracts matrix](./m5_debug_contracts.md). Gutters, breakpoint lists,
session headers, notebook cells, replay timelines, and support exports consume these
specs directly.

The published set is
[`fixtures/debug/m5_breakpoint_specs/canonical_set.json`](../../fixtures/debug/m5_breakpoint_specs/canonical_set.json),
frozen against `crates/aureline-debug/src/m5_breakpoint_specs/mod.rs` by the gate at
`crates/aureline-debug/tests/m5_breakpoint_specs.rs`.

## Materialized breakpoints

| Breakpoint | Kind | Scope | Pill (verification · mapping) | Green icon | Needs remap |
|---|---|---|---|---|---|
| `debug.breakpoint:line_verified_exact:0001` | line | workspace_source | Verified | yes | no |
| `debug.breakpoint:conditional_verified_misaligned:0002` | conditional | session_local | Verified · relocated | no | no |
| `debug.breakpoint:line_pending:0003` | line | workspace_source | Pending | no | no |
| `debug.breakpoint:logpoint_unbound_needs_remap:0004` | logpoint | workspace_source | Unbound · needs remap | no | yes |
| `debug.breakpoint:function_unsupported_replay:0005` | function | replay_timeline | Unsupported · no source mapping · replay-only | no | no |
| `debug.breakpoint:data_policy_blocked:0006` | data | session_local | Policy-blocked | no | no |
| `debug.breakpoint:exception_verified:0007` | exception | exception_category | Verified · no source mapping | no | no |
| `debug.breakpoint:notebook_verified_exact:0008` | line | notebook_cell | Verified | yes | no |
| `debug.breakpoint:notebook_needs_remap:0009` | line | notebook_cell | Unbound · needs remap | no | yes |
| `debug.breakpoint:imported_lexical_fallback:0010` | line | workspace_source | Verified · relocated (lexical) | no | no |

The set materializes the full verification vocabulary (pending, verified, unbound,
unsupported, policy-blocked), the full mapping vocabulary (exact, misaligned,
needs-remap, unmapped), every scope, every kind, and every mapping provenance.

## Proof claims

| Claim | Evidence |
|---|---|
| A breakpoint shown anywhere traces back to one canonical spec and one mapping/verification state vocabulary | invariant `breakpoints.one_canonical_pill_vocabulary` + `breakpoints.verification_vocabulary_complete` + `breakpoints.mapping_vocabulary_complete` |
| A green gutter icon never hides unbound, misaligned, replay-only, or policy-blocked reality | invariant `breakpoints.green_never_hides_unverified_misaligned_replay_or_blocked` + the `green_icon_never_hides_a_caveat` freeze-gate test |
| Breakpoint identity survives rename/reformat/import, and degrades to explicit needs-remap rather than silent disappearance | invariant `breakpoints.lost_identity_degrades_to_needs_remap` + the `lost_identity_breakpoints_stay_visible_as_needs_remap` test |
| A lexical (grep) fallback is never replayed as an exact semantic mapping | invariant `breakpoints.lexical_fallback_never_presented_as_exact` |
| Notebook views preserve stable cell identity without pretending remapped breakpoints are exact | invariant `breakpoints.notebook_preserves_stable_cell_identity` + the `notebook_and_replay_scopes_keep_their_anchors` test |
| Replay views preserve stable frame identity and stay replay-only | invariant `breakpoints.replay_preserves_stable_frame_identity_and_stays_replay_only` |
| Support/export packets retain breakpoint verification and mapping state rather than flattening them into rendered chrome | invariant `breakpoints.export_retains_verification_and_mapping_state` + the `fixture_round_trips_and_is_export_safe` test |
| Every cited proof packet and producer exists on disk | the `every_proof_packet_and_producer_exists_on_disk` freeze-gate test |

## Verification

```sh
cargo test -p aureline-debug
```
