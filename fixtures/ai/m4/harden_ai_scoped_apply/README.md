# Harden AI Scoped-Apply

This fixture set exercises the stable AI scoped-apply hardening record owned by
`aureline_ai::harden_ai_scoped_apply`. It binds the preview/approval/apply/revert
lifecycle, scoped-apply and multi-file patch honesty, cross-wedge command parity,
route/spend authority truth, and exportable evidence/rollback lineage into one
export-safe packet.

`scoped_apply_packet.json` covers:

- an `applied_kept` lifecycle with a shown preview, granted one-time approval, a
  rollback checkpoint captured before apply, a mutation journal, an apply audit,
  and an available revert handle, with no direct trusted-path apply attempted;
- a `multi_file_bounded` declared scope that the produced patch stays inside;
- a four-file multi-file patch with a content-addressed digest covering modify,
  create, delete, and rename change kinds, where the rename row was disclosed and
  in scope but deselected so it never reached the live tree, and only disclosed,
  approved, in-scope hunks reached the tree;
- all seven launch wedges (palette, menu, keybinding, CLI/headless, deep link,
  automation recipe, AI assistant) sharing the same command descriptor, preview,
  approval, result, and rollback model, disclosing route truth, running the same
  policy checks, and qualifying for the Stable lane;
- route/spend authority truth with disclosed egress, a tainted-context fence, and
  no undisclosed authority widening; and
- the exportable evidence/rollback lineage binding the AI evidence packet, the
  patch-review summary, and the rollback handle.

Verify the checked packet with:

```sh
cargo test -p aureline-ai harden_ai_scoped_apply --no-fail-fast
```

Regenerate the checked artifact, summary, and fixture after intentional changes
with:

```sh
cargo test -p aureline-ai harden_ai_scoped_apply::tests::emit_artifact -- --ignored
```
