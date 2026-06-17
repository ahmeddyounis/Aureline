# Reviewed mutation-flow fixtures

These fixtures exercise the reviewed mutation-flow review object
(`aureline-deps`, `reviewed_mutation_flows`) across the four flows and the
guards the lane must keep honest:

- **`install_native_build_reviewed.json`** — an install that requires a native
  build; the toolchain requirement is labeled explicitly, acknowledged, and
  committable after review.
- **`update_known_scripts_committed.json`** — an update that runs known install
  scripts; the scripts are acknowledged and the mutation is committed with a
  durable, reversible checkpoint.
- **`remove_reverted_recovery.json`** — a removal that a downstream build broke;
  it is recovered from a durable, **reverted** checkpoint rather than a transient
  notification.
- **`regenerate_policy_blocked.json`** — a regenerate/resolve proposed by AI whose
  build script is blocked by policy; the commit gate stays **closed**, the
  resolver version is still disclosed, and automation cannot bypass review.

Each file is a `reviewed_mutation_flows` packet validated against
`schemas/deps/reviewed-mutation-flows.schema.json` and the typed model. They use
durable manifest, lockfile, and checkpoint ids, redacted manifest paths and
source labels, and controlled enum values only; they carry no raw registry URLs,
manifests, lockfiles, package bodies, tokens, or credential material. Every sheet
binds to the frozen package-state matrix through `references_matrix_id` and every
checkpoint is a durable receipt offering revert / open-diff / export-patch.
