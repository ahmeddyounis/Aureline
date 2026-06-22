# Editor-assist qualification packet

One certification packet that binds the editor-assist micro-surface truth
sources into a per-family claim verdict and **auto-narrows** a claimed editor
family when its assist-surface proof is stale or failing.

The product claims a set of editor families — code files, config files, notebook
cells, request and SQL editors, docs-code blocks, generated files, protected
files, partial-index state, and large-file / restricted mode — support a rich
edit loop: completion, hints, hover, peek, snippet sessions, and decorations.
Each of those micro-surfaces is governed by its own frozen truth lane. This
packet does **not** re-prove those contracts; it consumes them as proof sources
and projects one verdict that About/help, service-health, compatibility, release
automation, and support export all render instead of restating assist-quality
claims by hand.

- Schema: [`schemas/editor/m5-assist-qualification.schema.json`](../../schemas/editor/m5-assist-qualification.schema.json)
- Canonical fixture: [`fixtures/editor/m5-assist-qualification/canonical_packet.json`](../../fixtures/editor/m5-assist-qualification/canonical_packet.json)
- Rust truth source: `crates/aureline-editor/src/m5_assist_qualification`
- Headless emitter: `cargo run --bin aureline_m5_assist_qualification`
- Freeze gate: `cargo test -p aureline-editor --test m5_assist_qualification_replay`

This lane is the certification layer over the micro-surface matrix
([editor-assist matrix](m5-editor-assist.md)), [assist descriptors](m5-assist-descriptors.md),
[completion rows](m5-completion-rows.md), [signature/snippet](m5-signature-snippet.md),
[hover/peek](m5-hover-peek.md), [constrained-assist](m5-constrained-assist.md),
[advanced editing](m5-advanced-editing.md), and the
[assist support packet](m5-assist-support.md). It reuses their vocabularies and
freshness stamps rather than forking new ones.

## Proof dimensions

The closed set of assist-surface claims a family is certified on. Each dimension
cites the upstream lane(s) whose freshness and pass state decide whether the
claim holds, and carries a freshness budget (default 30 days).

| Dimension | Critical | Primary proof source | Governs |
| --- | --- | --- | --- |
| `assist_source_honesty` | yes | assist descriptors | every surface (sources stay distinct/labeled) |
| `precedence` | yes | editor-assist matrix | every surface (editing truth outranks chrome) |
| `completion` | no | completion rows | the completion channel |
| `hint` | no | assist descriptors | inlay-hint / code-lens channels |
| `hover` | no | hover/peek | the hover channel |
| `peek` | no | hover/peek | the peek channel |
| `constrained_file_narrowing` | no | constrained-assist | constrained families only |
| `ime_multi_cursor_safety` | no | signature/snippet | snippet-session + completion channels |
| `accessibility_parity` | no | advanced editing | every surface (keyboard-complete, non-color-only) |

A dimension applies to a family only when the family **claims** at least one of
its governing channels with real fidelity. A surface that blocks peek
(docs-code, request editor) or suppresses completion (large-file mode) is not
penalized when that channel's proof ages out — its claim never promised it.

### Proof state

Each dimension resolves to exactly one state, derived from the upstream lane's
pass state and its capture stamp measured against the packet's evaluation stamp:

- `fresh` — present, passing, and within its freshness budget.
- `stale` — present and was passing, but captured outside its budget (silently
  aged out).
- `failing` — present, but the upstream contract did not hold.
- `missing` — no proof supplied for the dimension.

Only `fresh` keeps a claim fully supported. The freshness derivation lives in
code (`derive_proof_state`), so release automation feeds raw capture stamps and
pass flags and the packet decides the verdict — it never pre-decides it.

## Per-family claim support

For each claimed family, the packet evaluates only the dimensions that family
claims and folds them into one support level:

- `fully_supported` — every claimed dimension is `fresh`.
- `narrowed` — at least one claimed dimension is `stale` or `failing`, but no
  critical safety dimension failed; the claim is degraded and names the
  responsible dimension(s).
- `blocked` — a **critical** dimension (`assist_source_honesty` or
  `precedence`) failed or is missing; the family's assist claim is withdrawn.

Only the two safety dimensions are critical: a mislabeled assist source and
convenience chrome outranking editing truth are correctness violations the edit
loop cannot ship around. Every other dimension degrades honestly — it narrows
the claim and discloses the limit rather than blocking the family.

## Frozen invariants

The packet evaluates these over its own data; a structural regression flips an
invariant to `holds = false` rather than silently shipping.

1. `dimension_set_complete` — every proof dimension resolves to exactly one
   global proof.
2. `every_claimed_family_present` — every claimed editor family has a row.
3. `no_fully_supported_family_with_nonfresh_proof` — a family stays fully
   supported only when every dimension it claims is fresh. This is the guardrail
   against silent aging.
4. `every_downgrade_is_named` — every narrowed or blocked family names the
   responsible dimension(s).
5. `critical_failure_blocks_claim` — a failing or missing critical dimension
   blocks every family that claims it.
6. `release_evidence_dimensions_present` — completion, hint, hover, peek,
   constrained-file narrowing, IME / multi-cursor, and accessibility-parity rows
   are all present.

## Release automation

`project_assist_qualification(evaluated_as_of, proofs)` is the release-automation
entry point. Release automation supplies one `ProofInput` per dimension carrying
the upstream lane's capture stamp, pass flag, and freshness budget; the
projection derives each state and folds the per-family verdicts. Because staleness
is derived from `evaluated_as_of`, re-running the same proof inputs at a later
date automatically downgrades families whose micro-surface evidence has aged past
its budget, even when broad language-pack or navigation proof stays fresh.

`assist_qualification_packet()` is the canonical binding: it reads the real
in-code proof sources (each lane's `all_invariants_hold` and `AS_OF` stamp) and
feeds them to the projection, so the checked-in fixture and the freeze gate pin
the certified state byte-for-byte.

## About / help / service-health / compatibility

These surfaces consume the packet directly. They read `families[].support` for
the per-family badge and `rollup` for the cross-family summary, and render
`assist_qualification_lines` for the export-safe text projection. None of them
restates assist-quality claims by hand.

## Verification

```sh
cargo run --bin aureline_m5_assist_qualification              # JSON
cargo run --bin aureline_m5_assist_qualification -- --lines   # human-readable
cargo test -p aureline-editor --test m5_assist_qualification_replay
cargo test -p aureline-editor m5_assist_qualification
```

## Risks and follow-ups

- **The freshness budget is uniform (30 days) in the canonical binding.** Per
  dimension budgets are already a field on `ProofInput`; tightening them for the
  fastest-moving micro-surfaces is a release-automation tuning follow-up.
- **The canonical packet binds in-code lane invariants, not live CI evidence
  ages.** The release-automation entry point accepts real capture stamps; wiring
  the live evidence pipeline to feed those stamps (so a missed CI refresh ages a
  dimension out) is incremental follow-up as the evidence store matures.
- **Per-channel granularity stops at the dimension.** The packet certifies a
  family at the dimension level; surfacing which exact channel inside a dimension
  aged out is left to the underlying micro-surface lane the dimension cites.
