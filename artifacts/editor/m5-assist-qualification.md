# Editor-assist qualification packet

## Release evidence

This artifact documents the editor-assist qualification packet produced by
`crates/aureline-editor/src/m5_assist_qualification/`. The packet binds the
editor-assist micro-surface proof sources into one per-family claim verdict and
auto-narrows a claimed editor family when its assist-surface proof is stale or
failing, so no editor lane stays fully supported while its completion / hint /
hover / peek / provider / IME / accessibility proof has silently aged out.

## Record family

| Record | Kind | Schema | Version |
|---|---|---|---|
| `AssistQualificationPacket` | `m5_assist_qualification_packet` | `schemas/editor/m5-assist-qualification.schema.json` | 1 |

- Packet id: `m5-assist-qualification:packet:0001`
- As of: `2026-06-22T00:00:00Z`
- Coverage: 10 claimed editor families × 9 proof dimensions
- Overall: all 6 invariants hold; 10/10 families fully supported in the canonical binding

## Proof dimensions (release-evidence rows)

Each dimension carries its own freshness state and failure mode. Completion,
hint, hover, peek, constrained-file narrowing, IME / multi-cursor, and
accessibility-parity are present as explicit rows; assist-source honesty and
precedence are the two critical safety rows.

| Dimension | Critical | Primary proof source |
|---|---|---|
| `assist_source_honesty` | yes | `schemas/editor/m5-assist-descriptors.schema.json` |
| `precedence` | yes | `schemas/editor/m5-editor-assist.schema.json` |
| `completion` | no | `schemas/editor/m5-completion-rows.schema.json` |
| `hint` | no | `schemas/editor/m5-assist-descriptors.schema.json` |
| `hover` | no | `schemas/editor/m5-hover-peek.schema.json` |
| `peek` | no | `schemas/editor/m5-hover-peek.schema.json` |
| `constrained_file_narrowing` | no | `schemas/editor/m5-constrained-assist.schema.json` |
| `ime_multi_cursor_safety` | no | `schemas/editor/m5-signature-snippet.schema.json` |
| `accessibility_parity` | no | `schemas/editor/m5-advanced-editing.schema.json` |

## Auto-narrow contract (all must pass)

1. `dimension_set_complete` — every proof dimension resolves to exactly one global proof.
2. `every_claimed_family_present` — every claimed editor family has a qualification row.
3. `no_fully_supported_family_with_nonfresh_proof` — a family stays fully supported only when every dimension it claims is fresh.
4. `every_downgrade_is_named` — every narrowed or blocked family names the responsible dimension(s).
5. `critical_failure_blocks_claim` — a failing or missing critical dimension blocks every family that claims it.
6. `release_evidence_dimensions_present` — completion/hint/hover/peek, constrained-file narrowing, IME/multi-cursor, and accessibility-parity rows are all present.

## Family coverage

Generated and pinned in `fixtures/editor/m5-assist-qualification/canonical_packet.json`.
In the canonical binding every upstream lane's invariants hold and every proof is
captured on the evaluation date, so every family is fully supported. The packet
narrows or blocks automatically when a proof goes stale, failing, or missing.

| Family | Constrained | Canonical support |
|---|---|---|
| code_file | no | fully_supported |
| config_file | no | fully_supported |
| notebook_cell | yes | fully_supported |
| request_editor | yes | fully_supported |
| sql_editor | yes | fully_supported |
| docs_code_block | yes | fully_supported |
| generated_file | yes | fully_supported |
| protected_file | yes | fully_supported |
| partial_index_state | yes | fully_supported |
| large_file_restricted | yes | fully_supported |

## Verification

Emit the canonical packet:

```sh
cargo run --bin aureline_m5_assist_qualification
cargo run --bin aureline_m5_assist_qualification -- --lines
```

Run the freeze gate (rebuilds the packet from in-code proof sources and asserts
it equals the fixture, plus proves auto-narrowing on stale proof):

```sh
cargo test -p aureline-editor --test m5_assist_qualification_replay
```

Run the unit contract suite:

```sh
cargo test -p aureline-editor m5_assist_qualification
```

Validate the fixture against the schema:

```sh
python3 -c "import json,jsonschema; jsonschema.validate(json.load(open('fixtures/editor/m5-assist-qualification/canonical_packet.json')), json.load(open('schemas/editor/m5-assist-qualification.schema.json')))"
```

## Risks and follow-ups

- **The canonical binding measures in-code lane invariants, not live CI evidence
  ages.** The release-automation entry point (`project_assist_qualification`)
  accepts real capture stamps and budgets; wiring the live evidence store to feed
  those stamps is incremental follow-up.
- **The freshness budget is uniform (30 days).** Per-dimension budgets are a
  field on the proof input; tightening them per micro-surface is a tuning
  follow-up.
- **About/help/service-health/compatibility consume the packet, but the live
  wiring of each surface to read `families[].support` is incremental** as those
  surfaces mature; the packet and its export-safe line projection are the stable
  contract they bind to.
