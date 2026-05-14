# Extension review and revocation alpha fixtures

These fixtures exercise the alpha review packet implemented in
`crates/aureline-extensions/src/review_alpha/` and the draft policy-pack
schema at `schemas/policy/policy_pack_alpha.schema.json`.

The corpus covers:

- install/update review with publisher continuity visible,
- mirror revocation and broken continuity denying install,
- explicit revoke review driven by an emergency-disable policy pack.
- policy-pack and constraint records that validate against the alpha
  policy-pack schema.

The Rust tests load these JSON fixtures directly so the contract remains a
protected proof path rather than a prose-only example.
