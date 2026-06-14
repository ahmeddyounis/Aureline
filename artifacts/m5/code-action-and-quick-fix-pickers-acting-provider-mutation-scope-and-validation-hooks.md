# M5 evidence pointer — code-action and quick-fix pickers

Evidence pointer for the frozen code-action and quick-fix pickers that
govern the new M5 framework-pack, notebook-cell, docs-artifact,
request/structured-artifact, config-artifact, and generated-source lanes.
This row is a depth-lane proof governed by the canonical M5 evidence
index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## Canonical artifacts

- Truth packet: `artifacts/language/m5/code_action_quick_fix_picker_truth_packet.json`
- Boundary schema: `schemas/language/code_action_quick_fix_picker_truth.schema.json`
- Reviewer contract: `docs/m5/code-action-and-quick-fix-pickers-acting-provider-mutation-scope-and-validation-hooks.md`
- Human-readable rendering: `artifacts/language/m5/code-action-and-quick-fix-pickers-acting-provider-mutation-scope-and-validation-hooks.md`
- Fixture corpus: `fixtures/language/m5/code_action_quick_fix_picker_truth_packet/`
- Owning crate module: `crates/aureline-language/src/code_action_quick_fix_picker_truth_packet/`
- Regenerator: `cargo run -p aureline-language --example dump_code_action_quick_fix_picker_truth_packet`

## Executable proof

`crates/aureline-language/tests/code_action_quick_fix_picker_truth_packet.rs`
loads every fixture and the checked-in packet, asserts the materialized
promotion state, finding counts, and closed token sets, and proves the
picker covers every required artifact-family lane, every picker dimension
on each certified lane, that every mutating action states an apply
posture and never widens inline into protected artifacts without a
preview, and that all ten consumer projections preserve the packet.
Inline unit coverage lives in
`crates/aureline-language/src/code_action_quick_fix_picker_truth_packet/tests.rs`.

## Narrowing rule

Any marketed or support-class row that depends on these pickers narrows
automatically when the packet's evidence is missing, stale, or
downgraded: a lane that loses a concrete acting provider, an
acting-provider label, a picker dimension, an apply posture, a required
preview hash / completeness label / rollback checkpoint ref, an
inspectable disagreement, a visible manual-fix path, a disclosure ref, or
a consumer projection drops **below** `certified` instead of inheriting
an adjacent certified row.
