# Save truth cases

Reviewable save scenarios the VFS / save prototype validates against. Each
fixture describes one ADR-0006 failure case the save pipeline must name with
its own vocabulary — "opened path vs. actual write target vs. rewrite class
vs. degraded / unsupported conditions" — so the pipeline behaviour is
auditable without running Rust code.

The stable corpus ids for these fixtures live in
`fixtures/fs/identity_corpus_manifest.yaml`. Prototype exports should quote
those ids instead of inventing scenario-local labels.

These fixtures are the human-reviewable counterpart to the machine-emitted
save-plan records under `artifacts/fs/save_plan_examples/`. Both sides use
the frozen vocabulary from:

- `docs/adr/0006-vfs-save-cache-identity.md` (identity layers, save pipeline,
  failure cases, watcher posture, capability flags)
- `docs/filesystem/filesystem_identity_vocabulary.md` (cross-surface strings)

Every fixture in this directory MUST correspond to exactly one scenario label
in `crates/aureline-vfs/src/harness.rs::SCENARIOS`. The harness asserts that
alignment in its byte-stability tests, and exported scenario JSON should also
carry the matching `corpus_case_id`, any `related_fixture_ids`, and any
`rename_matrix_row_refs`.

## Index

| Label | Expected outcome | What it exercises |
|---|---|---|
| `local_atomic_save_happy_path` | `committed` | Local POSIX-like root; atomic_replace preferred; compare-before-write holds; manifest recorded. |
| `case_only_difference` | `committed` | Presentation uri differs from canonical by case only; alias convergence fires; atomic commit proceeds on canonical. |
| `symlink_alias` | `committed` | Presentation uri is a symlink into the workspace; resolution chain disclosed; atomic commit proceeds on canonical. |
| `hardlink_sibling` | `committed` | Canonical has one hardlink sibling; alias disclosed; atomic commit on canonical also affects sibling. |
| `unicode_normalization_variant` | `committed` | Presentation uri uses NFD; canonical uses NFC; normalization-variant alias disclosed; atomic commit proceeds on canonical. |
| `external_change_detected` | `external_change_detected` | Sibling writer bumped the generation between open and save; compare-before-write catches the mismatch. |
| `review_required_before_save` | `review_required_before_save` | Workspace policy attaches review_required_before_save; pipeline halts before any bytes move. |
| `read_only_root_blocked` | `read_only_or_policy_blocked` | Archive-like view (read-only overlay); select_save_mode returns blocked. |
| `remote_conditional_conflict` | `save_conflict` | Remote-agent root with conditional_remote_write; sibling commit bumped the revision token; pipeline yields save_conflict (not external_change_detected, because the pre-write condition is what fails). |
| `watcher_degradation` | `committed` | OS watcher drops into fallback_polling mid-session; watcher-health frames emitted; compare-before-write is still the correctness floor and the commit proceeds. |
| `save_participant_failed` | `save_participant_failed` | Save participant (text-normalisation) raises; pipeline records failure and never runs compare-before-write. |

## Schema

Each fixture declares itself via a JSON-pointer-style `__fixture__` block
that names the scenario label, a human summary, and the ADR / vocabulary
section(s) it aligns with. The remaining fields describe the pre-save
identity record, the root capability envelope, the synthetic watcher script,
the save request, and the expected save outcome + watcher-degradation story.

No fixture encodes wall-clock times. Synthetic monotonic tokens
(`mono:HHMM:SS:SS.FRAC`) stand in for timestamps so reruns are byte-stable.

## Adding a new case

1. Add a row to `SCENARIOS` in `crates/aureline-vfs/src/harness.rs` with a
   unique label and the ADR outcome you expect.
2. Add a matching fixture here named `<label>.json`; update the index above.
3. Run `./tools/vfs_proto.sh --emit-scenarios artifacts/fs/save_plan_examples`
   to refresh the machine-emitted records.
4. Confirm the harness tests still pass (`cargo test -p aureline-vfs`).
