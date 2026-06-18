# Automation safety labels and cross-surface parity

Automation safety labels are controlled, user-facing terms. They tell a user, at
a glance, where a command may run and what it does before they trust it in a
recipe or macro flow. This page is the reviewer contract for the **one label
source** and the proof that the same labels are projected consistently wherever a
claimed command is surfaced or exported.

## The controlled vocabulary

The vocabulary is closed and frozen. It is the controlled-automation-label set
defined in
[`schemas/automation/automation-manifest.schema.json`](../../schemas/automation/automation-manifest.schema.json)
and re-exported, unchanged, by the contract baseline in
[`schemas/automation/automation-contract-baseline.schema.json`](../../schemas/automation/automation-contract-baseline.schema.json).
No surface invents a new label, a synonym, or a surface-local term.

| Stable id | Display token | Kind | Meaning |
| --- | --- | --- | --- |
| `macro_safe` | Macro-safe | admissibility cue | Captured and replayed locally against explicit UI or editor state only. |
| `recipe_safe` | Recipe-safe | admissibility cue | Admissible as a typed, gated step in a declarative recipe. |
| `headless_safe` | Headless-safe | admissibility cue | Dispatchable from a CLI or headless surface without an interactive UI. |
| `ui_only` | UI-only | admissibility cue | Interactive only; not admissible to a recipe, macro, or headless surface. |
| `approval_required` | Approval required | admissibility cue | Requires an approval ticket before any apply. |
| `writes_files` | Writes files | effect disclosure | Writes files in the workspace or on the device. |
| `runs_process` | Runs process | effect disclosure | Launches or controls a process. |
| `network_call` | Network call | effect disclosure | Performs a network call. |
| `remote_mutation` | Remote mutation | effect disclosure | Mutates remote state. |

The **admissibility cues** say where a command may run. The **effect
disclosures** are the side-effect class — they say what the command does. The
side-effect class is never dropped on any surface.

## The surfaces

Every claimed M5 command projects its label set to each of these surfaces, and a
later surface that wants to show automation posture reuses this projection
instead of inventing its own:

- `command_palette_row` — the command palette / launcher row.
- `recipe_builder` — the declarative recipe builder.
- `macro_recorder` — the macro recorder and replay surface.
- `docs_help` — docs and in-app help.
- `cli_headless_inspect` — the CLI / headless `inspect` projection.
- `support_export` — the redacted support / export packet.
- `release_public_truth` — release notes and the public-truth artifact.

## What parity means

For each command there is exactly **one source label set** (the labels the
command graph owns). Parity holds when, for every surface:

- the surface projects the same stable-id set as the command source;
- every projected label keeps its canonical stable id token and canonical
  display token (no synonyms);
- no effect-disclosure (side-effect) label is dropped;
- the stable ids survive localization, export, and downgrade states; and
- every projected label is inside the frozen vocabulary.

The packet pins these as freeze invariants. A dropped surface, a drifted surface
label set, a synonym display token, a drifted stable id, a dropped side-effect
label, a stable id that does not survive localization / export / downgrade, a
label outside the vocabulary, or a violated invariant **blocks stable**.

## Companion artifacts

- Boundary schema:
  [`schemas/automation/automation-labels.schema.json`](../../schemas/automation/automation-labels.schema.json).
- Reused vocabulary axis:
  [`schemas/automation/automation-contract-baseline.schema.json`](../../schemas/automation/automation-contract-baseline.schema.json).
- Canonical packet and its projections (support export, CLI/headless view,
  compact text): `artifacts/m5/automation/label-parity/`.
- Worked-example and fail-closed mutation fixtures:
  `fixtures/automation/m5/label-parity/`.
- Fail-closed CI gate: `tools/ci/m5/label_parity_check.py`.

The artifacts and fixtures are bit-for-bit derivable from the frozen seed:

```sh
cargo run -q -p aureline-runtime --example dump_m5_label_parity
```

The typed Rust consumer mints the same packet, so
`cargo test -p aureline-runtime --test m5_label_parity` enforces the same
invariants and that the checked-in artifacts match the seed.
