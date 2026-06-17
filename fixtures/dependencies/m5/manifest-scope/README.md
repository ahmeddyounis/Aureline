# Manifest-scope review fixtures

These fixtures exercise the manifest-scope review object
(`aureline-deps`, `manifest_scope_and_source_review`) across the scope and
source-trust cases the lane must keep honest:

- **`member_exact_private_registry.json`** — a member-targeted operation whose
  resolved scope is exactly the target, resolved from a private registry.
- **`member_disclosed_shared_lockfile.json`** — a member change that necessarily
  updates the shared workspace lockfile; the broadening is disclosed, not silent,
  and stays appliable.
- **`member_unconfirmed_broadening_blocked.json`** — a member request whose
  resolution would silently widen to the whole workspace; because it is
  unconfirmed it is **not appliable**.
- **`revoked_mirror_trust_blocked.json`** — an exact member operation whose mirror
  credential has been revoked; trust is blocked and the operation is not
  appliable.

Each file is a `manifest_scope_review` packet validated against
`schemas/deps/manifest-scope-review.schema.json` and the typed model. They use
durable manifest ids, continuity tokens, redacted manifest paths and source
labels, and controlled enum values only; they carry no raw registry URLs,
manifests, lockfiles, package bodies, tokens, or credential material. Every row
binds to the frozen package-state matrix through `references_matrix_id`.
